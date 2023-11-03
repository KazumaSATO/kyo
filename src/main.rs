use gdk4::Display;

use glib::{clone, source, ControlFlow, MainContext};
use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Builder, CallbackAction, CssProvider, ListBox, Shortcut,
    ShortcutController, ShortcutTrigger,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::sync::mpsc;

//fn main() -> glib::ExitCode {
fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("dev.nryotaro.kanami")
        .build();
    app.connect_startup(|_| load_css());
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // https://stackoverflow.com/questions/66942543/how-do-we-build-gui-with-glade-gtk-rs-in-rust
    // https://github.com/pachi/visol/blob/master/src/window.rs
    let builder = Builder::new();

    builder.add_from_file("src/system.ui").expect("fail");
    let window: ApplicationWindow = builder.object("window").expect("fail2");
    let list: ListBox = builder.object("list").expect("fail3");
    list.connect_row_activated(move |_list_box, _option_list_box| {
        println!("doge");
    });
    // Prevent the window from closing just after the application starts.
    window.set_application(Some(app));
    //window.set_opacity(0.1);
    window.init_layer_shell();
    window.set_keyboard_mode(KeyboardMode::OnDemand);
    //window.set_layer(Layer::Overlay);
    window.set_layer(Layer::Top);
    // window.set_margin(Edge::Left, 200);
    // window.set_anchor(Edge::Left, true);
    // Boxは非対称のサイズに子供を指定できるみたい。
    let shortcut_controller = ShortcutController::new();
    let trigger = ShortcutTrigger::parse_string("Escape").unwrap();

    let (sender, receiver) = glib::MainContext::channel::<i32>(glib::source::Priority::DEFAULT);

    let shortcut = Shortcut::builder()
        .trigger(&trigger)
        .action(&CallbackAction::new(move |_, b| {
            println!("kemo");
            sender.send(1).unwrap();
            true
        }))
        .build();

    shortcut_controller.add_shortcut(shortcut);
    window.add_controller(shortcut_controller);
    window.present();
    // main_context.spawn_local(clone!(@weak window => async move {
    // //
    //         while let Ok(res) = receiver.recv() {
    //             println!("foobar {res}");
    //             if res == 1 {
    //                 //window.close();
    //             } else {

    //                 println!("doge2");

    //             }
    //         }
    receiver.attach(None, move |x| {
        window.close();
        glib::ControlFlow::Continue
    });

    println!("present");
    //window.show();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("system.css"));

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
