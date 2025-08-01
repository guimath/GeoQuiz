pub mod info_parse;
mod logic;
use std::{
    error::Error,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use slint::{ComponentHandle, LogicalSize, Model, ModelRc, SharedString, VecModel};

use logic::{AppLogic, AppWindow, HyperLinkClick, ScoreStatSlint};

use crate::info_parse::AllInfos;

use lazy_static::lazy_static;

lazy_static! {
    static ref ALL_INFOS: AllInfos = info_parse::get_data();
}

fn vec_to_model(vec: &Vec<String>) -> ModelRc<SharedString> {
    ModelRc::new(
        vec.iter()
            .map(SharedString::from)
            .collect::<VecModel<SharedString>>(),
    )
}
fn arr_to_model(vec: &[&str]) -> ModelRc<SharedString> {
    ModelRc::new(
        vec.iter()
            .map(|&s| s.into())
            .collect::<VecModel<SharedString>>(),
    )
}

pub fn main() {
    let path = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    init(path).unwrap();
}

#[cfg(target_os = "android")]
fn start_android_action_view(url: String) -> Result<(), Box<dyn std::error::Error>> {
    use jni::objects::{JObject, JValue};
    // Create a VM for executing Java calls
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm() as _) }?;
    let activity = unsafe { jni::objects::JObject::from_raw(ctx.context() as _) };
    // let context = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
    let mut env = vm.attach_current_thread()?;

    // activity
    let intent_class = env.find_class("android/content/Intent")?;
    let action_view = env.get_static_field(&intent_class, "ACTION_VIEW", "Ljava/lang/String;")?;

    // Create Uri object
    let uri_class = env.find_class("android/net/Uri")?;
    let url = env.new_string(url)?;
    let uri = env.call_static_method(
        &uri_class,
        "parse",
        "(Ljava/lang/String;)Landroid/net/Uri;",
        &[JValue::Object(&JObject::from(url))],
    )?;
    // Create new ACTION_VIEW intent with the uri
    let intent = env.alloc_object(&intent_class)?;
    env.call_method(
        &intent,
        "<init>",
        "(Ljava/lang/String;Landroid/net/Uri;)V",
        &[action_view.borrow(), uri.borrow()],
    )?;

    // Start the intent activity.
    env.call_method(
        &activity,
        "startActivity",
        "(Landroid/content/Intent;)V",
        &[JValue::Object(&intent)],
    )?;

    Ok(())
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: slint::android::android_activity::AndroidApp) {
    let path = app.external_data_path().unwrap();
    slint::android::init(app).unwrap();
    // eprintln!("{:?}", path.clone());
    // path = storage/emulated/0/Android/data/com.example.geo_quiz/files
    init(path).unwrap()
}

fn init(path: PathBuf) -> Result<(), Box<dyn Error>> {
    // slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));
    // let all_infos = info_parse::get_data();

    let all_names: Vec<SharedString> = ALL_INFOS
        .all_countries
        .iter()
        .map(|x| x.infos[0].full.as_str().into())
        .collect();
    let logic = Arc::new(Mutex::new(AppLogic::new(&path, &ALL_INFOS)));
    let ui = AppWindow::new()?;
    ui.window().set_size(LogicalSize {
        width: 1000.0,
        height: 1000.0,
    });
    {
        let logic_lock = logic.lock().unwrap();
        ui.set_search_countries_mask(logic_lock.search_changed("".into()).as_slice().into());
        ui.set_search_all_countries(all_names.as_slice().into());
        ui.set_all_categories_name(vec_to_model(logic_lock.get_all_categories_name()));
        ui.set_txt_categories_name(vec_to_model(&ALL_INFOS.info_names));
        ui.set_sub_categories_name(arr_to_model(&logic::SUB_CAT_NAMES));
        ui.set_users(vec_to_model(&logic_lock.list_users()));
        ui.invoke_set_active_user_look_up(logic_lock.get_active_user().into());
    }

    ui.on_look_up_search_changed({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |search| {
            let ui = ui_handle.unwrap();
            let logic = logic_ref.lock().unwrap();
            ui.set_search_countries_mask(logic.search_changed(search.into()).as_slice().into());
        }
    });
    ui.on_look_up_selected({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |search| {
            let ui = ui_handle.unwrap();
            let logic = logic_ref.lock().unwrap();
            ui.invoke_update_look_up_selected(logic.look_up_selected(search as usize));
        }
    });
    ui.on_look_up_current({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move || {
            let ui = ui_handle.unwrap();
            let logic = logic_ref.lock().unwrap();
            ui.invoke_update_look_up_current(logic.look_up_current());
        }
    });

    ui.on_set_play_config({
        let logic_ref = logic.clone();
        move |params| {
            let mut logic = logic_ref.lock().unwrap();
            logic.set_config(params);
        }
    });
    //  MAIN PLAY
    ui.on_start_play({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |guess_types, info_type| {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();

            let down_ref: &VecModel<i32> = guess_types.as_any().downcast_ref().unwrap();
            let v: Vec<usize> = down_ref.iter().map(|x| x as usize).collect();
            let guess_types = [v[0], v[1], v[2]];
            logic.prepare_main_play(info_type as usize, guess_types);
            let (update, cat) = logic.get_stat();
            ui.invoke_update_screen(update, cat.into());
        }
    });
    ui.on_next({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |result: i32| {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();
            if let Some((update, cat)) = logic.next(result as u32) {
                ui.invoke_update_screen(update, cat.into());
            } else if logic.is_at_end() {
                let (v, max) = logic.get_play_scores(false);
                ui.invoke_show_play_scores(v.as_slice().into(), max);
            }
        }
    });
    ui.on_prev({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move || {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();
            if let Some((update, cat)) = logic.prev() {
                ui.invoke_update_screen(update, cat.into());
            }
        }
    });

    //  CHOICE PLAY
    ui.on_choice_start_play({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |info_type, guess_type| {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();
            logic.prepare_choice_play(info_type as usize, guess_type as usize);
            ui.invoke_update_choice(logic.get_choices());
        }
    });
    ui.on_choice_changed({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |guesses, next, found| {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();
            // let down_ref: &VecModel<bool> = guesses.as_any().downcast_ref().unwrap();
            // let v: Vec<bool> = down_ref.iter().collect();
            if let Some(info) = logic.choice_changed(guesses, next, found) {
                ui.invoke_update_choice(info);
            } else if logic.is_at_end() {
                let (v, max) = logic.get_play_scores(true);
                ui.invoke_show_play_scores(v.as_slice().into(), max);
            }
        }
    });
    // SCORE
    ui.on_score_user_selected({
        let logic_ref = logic.clone();
        move |name| {
            let mut logic = logic_ref.lock().unwrap();
            logic.score_user_selected(name.into());
        }
    });
    ui.on_score_user_change({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |name, delete| {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();
            logic.score_user_change(name.into(), delete);
            ui.set_users(vec_to_model(&logic.list_users()));
            ui.invoke_set_active_user(logic.get_active_user().into());
        }
    });
    ui.on_score_rename_user({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |name1, name2| {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();
            logic.score_rename_user(name1.into(), name2.into());
            ui.set_users(vec_to_model(&logic.list_users()));
            ui.invoke_set_active_user(logic.get_active_user().into());
        }
    });
    ui.on_score_filter_changed({
        let logic_ref = logic.clone();
        move |all| {
            let mut logic = logic_ref.lock().unwrap();
            logic.score_filter_changed(all);
        }
    });
    ui.on_score_sub_cat_changed({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |sub_cat_idx| {
            let ui = ui_handle.unwrap();
            let logic = logic_ref.lock().unwrap();
            let stat = logic.score_sub_cat_changed(sub_cat_idx as usize);

            ui.invoke_update_score(ScoreStatSlint {
                main_avg: stat.main_avg.into(),
                main_last: stat.main_last.into(),
                choice_avg: stat.choice_avg.into(),
                choice_last: stat.choice_last.into(),
                main_max: stat.main_max,
                choice_max: stat.choice_max,
            });
        }
    });
    ui.on_get_play_scores({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |choice_play| {
            let ui = ui_handle.unwrap();
            let logic = logic_ref.lock().unwrap();
            let (v, max) = logic.get_play_scores(choice_play);
            ui.invoke_show_play_scores(v.as_slice().into(), max);
        }
    });
    ui.on_save_score({
        let logic_ref = logic.clone();
        move || {
            let logic = logic_ref.lock().unwrap();
            logic.save_scores();
        }
    });
    ui.global::<HyperLinkClick>().on_hl_clicked(|url| {
        if webbrowser::open(url.as_str()).is_err() {
            // webbrowser only works on http for android
            #[cfg(target_os = "android")]
            start_android_action_view(url.to_string()).unwrap();
        }
    });
    ui.on_close({
        // let logic_ref = logic.clone();
        move || {
            // let logic = logic_ref.lock().unwrap();
            // logic.save_scores();
            slint::quit_event_loop().unwrap();
        }
    });
    ui.window().on_close_requested({
        // let logic_ref = logic.clone();
        move || {
            // let logic = logic_ref.lock().unwrap();
            // logic.save_scores();
            slint::CloseRequestResponse::HideWindow
        }
    });
    ui.run()?;
    Ok(())
}
