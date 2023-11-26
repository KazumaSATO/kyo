use config::load_config;
use gtk4::prelude::*;
use gtk4::Application;
use style::load_css;

pub mod config;
pub mod style;
pub mod ui;
//fn main() -> glib::ExitCode {
fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("dev.nryotaro.kanami")
        .build();
    app.connect_startup(|_| load_css(&None));
    // Connect to "activate" signal of `app`
    app.connect_activate(|app| ui::build_ui(&app, load_config(None)));
    // Run the application
    app.run();
}
