use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use slint::{ComponentHandle, Image};

use geo_quiz::logic::{AppLogic, AppWindow};
macro_rules! update_country {
    ($ui:ident, $stat:ident) => {{
        // $ui.set_photo_path($img.image);
        if let Some(stat) = $stat {
            let level = if stat.score != 0 { 3 } else { 0 };
            $ui.set_info_level(level);
            $ui.invoke_set_score(stat.score as i32);
            $ui.set_country_name(stat.name.into());
            $ui.set_capital(stat.capital.into());
            $ui.set_other_info(stat.other_info.into());
            $ui.set_out_of(stat.out_of as i32);
            $ui.set_num(stat.num as i32);
            $ui.set_flag(Image::load_from_path(&stat.svg_path).unwrap());
        }
    }};
}

fn main() -> Result<(), Box<dyn Error>> {
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));

    let logic = Arc::new(Mutex::new(AppLogic::new()));
    let ui = AppWindow::new()?;
    let stat = Some(logic.lock().unwrap().get_stat());
    update_country!(ui, stat);

    ui.on_next({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move |result: i32| {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();
            let stat = logic.next(result as u32);
            update_country!(ui, stat);
        }
    });
    ui.on_prev({
        let ui_handle = ui.as_weak();
        let logic_ref = logic.clone();
        move || {
            let ui = ui_handle.unwrap();
            let mut logic = logic_ref.lock().unwrap();
            let stat = logic.prev();
            update_country!(ui, stat);
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
