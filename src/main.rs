// use gtk4::builders::ApplicationBuilder;
// use gtk4::prelude::*;
// use gtk4::{Application, Builder};
// fn main() {
//     let application: ApplicationBuilder = Application::builder();
//     let builder = Builder::new();
//     builder.add_from_file("a.ui");
//     let app = Application::builder()
//         .application_id("org.example.HelloWorld")
//         .build();
//     app.connect_activate(|app| {
//         //builder.object("window");
//     });
//     app.run();
//     println!("Hello, world!");
// }

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Builder, ListBox};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

//fn main() -> glib::ExitCode {
fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("dev.nryotaro.kanami")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // https://stackoverflow.com/questions/66942543/how-do-we-build-gui-with-glade-gtk-rs-in-rust
    // https://github.com/pachi/visol/blob/master/src/window.rs
    let builder = Builder::new();
    println!("doge");
    builder.add_from_file("src/system.ui").expect("fail");
    let window: ApplicationWindow = builder.object("window").expect("fail2");
    let list: ListBox = builder.object("list").expect("fail3");
    let a = 3;
    list.connect_row_activated(move |list_box, option_list_box| {
        println!("doge");
        a;
    });

    // Prevent the window from closing just after the application starts.
    window.set_application(Some(app));
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_margin(Edge::Left, 200);
    window.set_anchor(Edge::Left, true);

    window.show();
}
