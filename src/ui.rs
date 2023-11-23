use crate::config::{Command, Config};
use gdk4::Texture;
use gtk4::glib::signal::Propagation;
use gtk4::glib::source::Priority;
use gtk4::glib::{Bytes, ControlFlow, MainContext, Receiver, Sender};
use gtk4::prelude::*;
use gtk4::{
    Application, CallbackAction, DirectionType, EventControllerKey, GestureClick, Image, Label,
    ListBox, ListBoxRow, Orientation, Shortcut, ShortcutController, ShortcutTrigger, Window,
};

use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
//pub mod config;

pub fn build_ui(app: &Application, config: &Config) {
    let window = build_window(app);
    let list_box = build_list(config);
    window.set_child(Some(&list_box));
    // let (sender, receiver): (Sender<Message>, Receiver<Message>) =
    //     MainContext::channel::<Message>(Priority::DEFAULT);
    window.present();
    // receiver.attach(None, move |msg| {
    //     match msg {
    //         Message::CloseWindow => window.close(),
    //         Message::Move(direction) => list_box.emit_move_focus(direction),
    //     }
    //     ControlFlow::Continue
    // });
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
    // window.set_margin(Edge::Left, 200);
    // window.set_anchor(Edge::Left, true);
    // Boxは非対称のサイズに子供を指定できるみたい。
    //builder.add_from_string(include_str!("system.ui")).unwrap();
    window.set_property("name", "window");
    let trigger = ShortcutTrigger::parse_string("Escape").unwrap();
    let action = CallbackAction::new(move |widget, _| {
        let window: &Window = widget.downcast_ref().unwrap();
        window.close();
        true
    });
    let builder = Shortcut::builder().trigger(&trigger).action(&action);
    let controller = ShortcutController::new();
    controller.add_shortcut(builder.build());

    window.add_controller(controller);
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
    let shortcut_controller = ShortcutController::new();
    let ctrl_p_trigger = ShortcutTrigger::parse_string("<Control>p").unwrap();
    let ctrl_p_action = CallbackAction::new(move |widget, _| {
        let list_box: &ListBox = widget.downcast_ref().unwrap();
        list_box.emit_move_focus(DirectionType::TabBackward);
        true
    });
    let ctrl_p_shortcut = Shortcut::builder()
        .trigger(&ctrl_p_trigger)
        .action(&ctrl_p_action)
        .build();
    shortcut_controller.add_shortcut(ctrl_p_shortcut);

    let ctrl_n_trigger = ShortcutTrigger::parse_string("<Control>n").unwrap();
    let ctrl_n_action = CallbackAction::new(move |widget, _| {
        let list_box: &ListBox = widget.downcast_ref().unwrap();
        list_box.emit_move_focus(DirectionType::TabForward);
        true
    });
    let ctrl_n_shortcut = Shortcut::builder()
        .trigger(&ctrl_n_trigger)
        .action(&ctrl_n_action)
        .build();
    shortcut_controller.add_shortcut(ctrl_n_shortcut);

    let controller_key = EventControllerKey::new();
    controller_key.connect_key_pressed(|k, key, c, modifier_type| {
        println!("hey, {}, {}", key, c);
        Propagation::Proceed
    });
    list_box.add_controller(controller_key);
    list_box.add_controller(shortcut_controller);
    list_box
}

fn build_entry(label: &str, icon: &str, command: &Command) -> gtk4::Box {
    let bx = gtk4::Box::new(Orientation::Horizontal, 16);
    let bytes = Bytes::from(icon.as_bytes());
    let texture = Texture::from_bytes(&bytes).unwrap();
    let image = Image::from_paintable(Some(&texture));
    image.set_pixel_size(24);
    bx.append(&image);
    bx.append(&Label::new(Some(label)));
    let click_controller = GestureClick::new();
    //bx.add_css_class("entry");
    let a = String::from(&command.command);
    click_controller.connect_pressed(move |_, _, _, _| {
        println!("{}", &a);
        //emit_move_focus(&self, direction: DirectionType);
    });
    bx.set_property("name", "entry");
    bx.add_controller(click_controller);
    bx
}
