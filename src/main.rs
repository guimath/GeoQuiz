use clap::Parser;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use slint::{ComponentHandle, LogicalSize, Model, SharedString, VecModel};

use geo_quiz::logic::{AppLogic, AppWindow, ImageType, InfoType};

/// args
#[derive(Parser)]
struct Cli {
    /// show well know flags first (according to own score)
    #[clap(long, short)]
    easy_first: bool,
    /// enable learn mode : all info is displayed by default
    #[clap(long, short)]
    learn_mode: bool,
    /// show initials by default
    #[clap(long, short)]
    initials_on: bool,
}
fn main() -> Result<(), Box<dyn Error>> {
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));
    let args = Cli::parse();
    let logic = Arc::new(Mutex::new(AppLogic::default()));

    let ui = AppWindow::new()?;
    ui.window().set_size(LogicalSize {
        width: 1000.0,
        height: 1000.0,
    });
    ui.set_learn_mode(args.learn_mode);
    ui.set_initials_on(args.initials_on);
    ui.on_start_play({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |selected, easy_first, hard_mode, image| {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();

            let s: &VecModel<SharedString> = selected.as_any().downcast_ref().unwrap();
            let selected: Vec<String> = s.iter().map(|x| x.into()).collect();
            let info_types: [InfoType; 3] = [
                selected[0].parse::<InfoType>().unwrap(),
                selected[1].parse::<InfoType>().unwrap(),
                selected[2].parse::<InfoType>().unwrap(),
            ];
            let img = image.parse::<ImageType>().unwrap();
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
