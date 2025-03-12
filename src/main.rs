use clap::Parser;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use slint::ComponentHandle;

use geo_quiz::logic::{AppLogic, AppWindow};

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
    let logic = Arc::new(Mutex::new(AppLogic::new(
        args.easy_first,
        args.learn_mode,
        args.initials_on,
    )));
    let ui = AppWindow::new()?;
    let (update, cat) = logic.lock().unwrap().get_stat();
    ui.invoke_update_screen(update, cat.into());
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
