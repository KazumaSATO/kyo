// use gtk::prelude::*;
// use gtk::{glib, Application, ApplicationWindow};
// use gtk4 as gtk;

// fn main() -> glib::ExitCode {
//     let app = Application::builder()
//         .application_id("org.example.HelloWorld")
//         .build();

//     app.connect_activate(|app| {
//         // We create the main window.
//         let window = ApplicationWindow::builder()
//             .application(app)
//             .default_width(320)
//             .default_height(200)
//             .title("Hello, World!")
//             .build();

//         // Show the window.
//         window.present();
//     });

//     app.run()
// }

//use gtk4 as gtk;
use gtk4::builders::ApplicationBuilder;
use gtk4::prelude::*;
use gtk4::{Application, Builder};
fn main() {
    let application: ApplicationBuilder = Application::builder();
    let builder = Builder::new();
    builder.add_from_file("a.ui");
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();
    app.connect_activate(|app| {
        builder.object("window");
    });
    app.run();
    println!("Hello, world!");
}
