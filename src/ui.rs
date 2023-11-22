use crate::config::{Command, Config};
use gdk4::Texture;
use gtk4::glib::source::Priority;
use gtk4::glib::{Bytes, ControlFlow, MainContext, Receiver, Sender};
use gtk4::prelude::*;
use gtk4::{
    Application, Box, CallbackAction, GestureClick, Image, Label, ListBox, Orientation, Shortcut,
    ShortcutController, ShortcutTrigger, Window,
};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
//pub mod config;

pub fn build_ui(app: &Application, config: &Config) {
    let window = build_window(app);
    let list_box = build_list(config);
    window.set_child(Some(&list_box));

    let (sender, receiver): (Sender<Message>, Receiver<Message>) =
        MainContext::channel::<Message>(Priority::DEFAULT);

    let shortcut_controller = build_shortcut_controller(sender);
    window.add_controller(shortcut_controller);
    window.present();
    receiver.attach(None, move |msg| {
        window.close();
        ControlFlow::Continue
    });
}

fn build_window(app: &Application) -> Window {
    // https://stackoverflow.com/questions/66942543/how-do-we-build-gui-with-glade-gtk-rs-in-rust
    // https://github.com/pachi/visol/blob/master/src/window.rs
    let builder = Window::builder();
    let window = builder.application(app).build();
    window.init_layer_shell();
    window.set_keyboard_mode(KeyboardMode::OnDemand);
    //window.set_layer(Layer::Overlay);
    window.set_layer(Layer::Top);
    window.set_default_width(240);
    window.set_property("name", "window");
    // window.set_margin(Edge::Left, 200);
    // window.set_anchor(Edge::Left, true);
    // Boxは非対称のサイズに子供を指定できるみたい。

    //builder.add_from_string(include_str!("system.ui")).unwrap();
    window
}

fn build_list(config: &Config) -> ListBox {
    let builder = ListBox::builder();
    let list_box = builder.build();

    let models = vec![
        ("Lock", include_str!("lock.svg"), &config.lock),
        ("Sleep", include_str!("sleep.svg"), &config.sleep),
        ("Poweroff", include_str!("poweroff.svg"), &config.poweroff),
    ];
    for (label, icon, command) in models {
        list_box.append(&build_entry(label, icon, command));
    }
    list_box
}
fn build_entry(label: &str, icon: &str, command: &Command) -> Box {
    let bx = Box::new(Orientation::Horizontal, 16);
    let bytes = Bytes::from(icon.as_bytes());
    let texture = Texture::from_bytes(&bytes).unwrap();
    let image = Image::from_paintable(Some(&texture));
    image.set_pixel_size(24);
    bx.append(&image);
    bx.append(&Label::new(Some(label)));
    let click_controller = GestureClick::new();
    let a = String::from(&command.command);
    click_controller.connect_pressed(move |_, _, _, _| {
        println!("{}", &a);
        //emit_move_focus(&self, direction: DirectionType);
    });
    bx.set_property("name", "entry");
    bx.add_controller(click_controller);
    bx
}

fn build_shortcut_controller<'a>(sender: Sender<Message>) -> ShortcutController {
    let shortcut_controller = ShortcutController::new();
    shortcut_controller.add_shortcut(build_shortcut_escape(sender.clone()));
    shortcut_controller
}

enum Message {
    CloseWindow,
}
fn build_shortcut_escape(sender: Sender<Message>) -> Shortcut {
    let trigger = ShortcutTrigger::parse_string("Escape").unwrap();
    let action = CallbackAction::new(move |_, _| {
        sender.send(Message::CloseWindow).unwrap();
        true
    });
    let builder = Shortcut::builder().trigger(&trigger).action(&action);

    builder.build()
}
