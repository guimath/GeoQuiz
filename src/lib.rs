mod logic;
pub mod info_parse;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use slint::{ComponentHandle, LogicalSize, Model, VecModel};

use logic::{AppLogic, AppWindow, ImageType, InfoType};

pub fn main() {
    init().unwrap();
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: slint::android::AndroidApp) {
    app.disable_motion_axis(slint::android::android_activity::input::Axis::Y);
    slint::android::init(app).unwrap();
    init().unwrap()
}

fn init() -> Result<(), Box<dyn Error>> {
    // slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));
    let logic = Arc::new(Mutex::new(AppLogic::default()));

    let ui = AppWindow::new()?;
    ui.window().set_size(LogicalSize {
        width: 1000.0,
        height: 1000.0,
    });
    ui.on_start_play({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |selected, easy_first, hard_mode, image| {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();

            let s: &VecModel<i32> = selected.as_any().downcast_ref().unwrap();
            let selected: Vec<i32> = s.iter().collect();
            let info_types: [InfoType; 3] = [
                InfoType::from_int(selected[0]),
                InfoType::from_int(selected[1]),
                InfoType::from_int(selected[2]),
            ];
            let img = ImageType::from_int(image);
            logic.prepare_infos(easy_first, hard_mode, info_types, img);
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
    ui.on_close({
        let logic_ref = logic.clone();
        move || {
            let logic = logic_ref.lock().unwrap();
            logic.save_scores();
            slint::quit_event_loop().unwrap();
        }
    });
    ui.run()?;
    Ok(())
}
