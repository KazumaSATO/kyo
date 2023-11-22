use config::{load_config, Config};
use gdk4::{Display, Texture};

use gdk_pixbuf::{Colorspace, Pixbuf};
use glib::{clone, source, Bytes, ControlFlow, ExitCode, MainContext};
use gtk4::prelude::*;
use gtk4::{
    gdk_pixbuf, glib, Application, ApplicationWindow, Builder, CallbackAction, CssProvider, Image,
    ListBox, Shortcut, ShortcutController, ShortcutTrigger,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
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
    app.connect_activate(|app| ui::build_ui(&app, &load_config(None)));
    // Run the application
    app.run();
}

fn build_ui2(app: &Application, config: &Config) {
    // https://stackoverflow.com/questions/66942543/how-do-we-build-gui-with-glade-gtk-rs-in-rust
    // https://github.com/pachi/visol/blob/master/src/window.rs

    let builder = Builder::new();
    builder.add_from_string(include_str!("system.ui")).unwrap();
    let window: ApplicationWindow = builder.object("window").expect("fail2");
    let list: ListBox = builder.object("list").expect("fail3");

    let sleep = include_str!("sleep.svg");
    let colored = sleep.replace("#000000", "#aaaaaa");
    let sleep_bytes = Bytes::from(colored.as_bytes());

    let sleep_texture = Texture::from_bytes(&sleep_bytes).unwrap();
    //let image = Image::from_paintable(Some(&sleep_texture));
    let image: Image = builder.object("sleep_icon").unwrap();
    image.set_paintable(Some(&sleep_texture));
    image.set_pixel_size(50);
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

    //window.show();
}
