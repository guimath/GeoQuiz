pub mod info_parse;
mod logic;
use std::{
    error::Error,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

#[cfg(target_os = "android")]
use slint::android::android_activity::AndroidApp;
 
use slint::{ComponentHandle, LogicalSize, Model, ModelRc, SharedString, VecModel};

use logic::{AppLogic, AppWindow, ScoreStatSlint};

fn vec_to_model(vec: &Vec<String>) -> ModelRc<SharedString> {
    let vc = vec.clone();
    let v: VecModel<SharedString> = vc.iter().map(|x| SharedString::from(x)).collect();
    ModelRc::new(v)
}

pub fn main() {
    let path = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    init(path).unwrap();
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: AndroidApp) {
    // use slint::android::android_activity::{InputStatus, MainEvent, PollEvent};
    // use slint::android::android_activity::input::{InputEvent, KeyAction, Keycode};

    let path = app.external_data_path().unwrap();
    slint::android::init(app).unwrap();
    // slint::android::init_with_event_listener(app.clone(), move |event| {
    //     if let PollEvent::Main(main_event) = event{
    //         // if *main_event != MainEvent::InputAvailable{
    //         //     return;
    //         // }
    //         match main_event {
    //             MainEvent::InputAvailable { .. } => {
    //                 // redraw_pending = true;
    //                 match app.input_events_iter() {
    //                     Ok(mut iter) => {
    //                         loop {
    //                             // info!("loop");
    //                             let read_input = iter.next(|event| {
    //                                 let handled = match event {
    //                                     InputEvent::KeyEvent(key_event) => {
    //                                         // info!("{:?}", key_event);
    //                                         if key_event.key_code() == Keycode::Back {
    //                                             if key_event.action() == KeyAction::Down {
    //                                                 eprintln!("back arrow detected");
    //                                             }
    //                                             InputStatus::Handled
    //                                         } else {
    //                                             InputStatus::Unhandled
    //                                         }
    //                                     }
    //                                     _ => {
    //                                         InputStatus::Unhandled
    //                                     }
    //                                 };
    //                                 handled
    //                             });
                    
    //                             if !read_input {
    //                                 // info!("stop loop");
    //                                 break;
    //                             }
    //                         }
    //                     }
    //                     Err(err) => {
    //                         eprintln!("Failed to get input events iterator: {err:?}");
    //                     }
    //                 }
    //             },
    //             _ => (),
    //         }
    //     }
    // }).unwrap();
    // eprintln!("{:?}", path.clone());
    init(path).unwrap()
}

fn init(path: PathBuf) -> Result<(), Box<dyn Error>> {
    // slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));
    let logic = Arc::new(Mutex::new(AppLogic::new(path)));
    let ui = AppWindow::new()?;
    ui.window().set_size(LogicalSize {
        width: 1000.0,
        height: 1000.0,
    });
    {
        let logic_lock = logic.lock().unwrap();
        ui.set_search_countries_mask(logic_lock.search_changed("".into()).as_slice().into());
        ui.set_search_all_countries(logic_lock.all_names.as_slice().into());
        ui.set_all_categories_name(vec_to_model(&logic_lock.all_cat_names));
        ui.set_txt_categories_name(vec_to_model(&logic_lock.txt_cat_names));
        ui.set_sub_categories_name(vec_to_model(&logic_lock.sub_cat_names));
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
            logic.prepare_main_play(info_type as usize, guess_types.into());
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
                main_max: stat.main_max.into(),
                choice_max: stat.choice_max.into(),
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
    ui.on_link({
        move | url | {
            // TODO fix for android
            open::that(url.as_str()).unwrap();
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
