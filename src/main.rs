use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use clap::Parser;

use slint::{ComponentHandle, Image};

use geo_quiz::logic::{AppLogic, AppWindow};
macro_rules! update_country {
    ($ui:ident, $stat:ident, $args:ident) => {{
        // $ui.set_photo_path($img.image);
        if let Some(stat) = $stat {
            let level = if stat.score != 0 || $args.learn_mode { 3 } else { 0 };
            $ui.set_info_level(level);
            $ui.invoke_set_score(stat.score as i32);
            let first_letter = stat.name.chars().nth(0).unwrap_or(' ');
            $ui.invoke_set_country_name(stat.name.into(), first_letter.into(), $args.initials_on);
            let first_letter = stat.capital.chars().nth(0).unwrap_or(' ');
            $ui.invoke_set_capital(stat.capital.into(), first_letter.into(), $args.initials_on);
            $ui.set_other_info(stat.other_info.into());
            $ui.set_out_of(stat.out_of as i32);
            $ui.set_num(stat.num as i32);
            $ui.set_flag(Image::load_from_path(&stat.svg_path).unwrap());
        }
    }};
}


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
    let logic = Arc::new(Mutex::new(AppLogic::new(args.easy_first)));
    let ui = AppWindow::new()?;
    let stat = Some(logic.lock().unwrap().get_stat());
    update_country!(ui, stat, args);
    ui.on_next({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |result: i32| {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();
            let stat = logic.next(result as u32);
            update_country!(ui, stat, args);
        }
    });
    ui.on_prev({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move || {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();
            let stat = logic.prev();
            update_country!(ui, stat, args);
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
