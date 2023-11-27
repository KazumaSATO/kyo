use config::read_config;
use gtk4::prelude::*;
use gtk4::Application;
use style::load_css;

pub mod config;
pub mod style;
pub mod ui;

fn main() {
    let app = Application::builder()
        .application_id("dev.nryotaro.kanami")
        .build();
    app.connect_startup(|_| load_css(&None));
    app.connect_activate(|app| ui::build_ui(&app, read_config(None as Option<String>)));
    app.run();
}
