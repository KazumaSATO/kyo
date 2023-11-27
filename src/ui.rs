use crate::config::Config;
use gdk4::Texture;
use gio::ActionEntry;
use gtk4::glib::Bytes;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, CallbackAction, DirectionType, Image, Label, ListBox,
    Orientation, Shortcut, ShortcutController, ShortcutTrigger,
};

use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

pub(crate) fn build_ui(app: &Application, config: Config) {
    let window = build_window(app);
    let list_box = build_list(config);
    window.set_child(Some(&list_box));
    window.present();
}

fn build_window(app: &Application) -> ApplicationWindow {
    let builder = ApplicationWindow::builder();
    let window = builder.application(app).build();
    window.init_layer_shell();
    window.set_keyboard_mode(KeyboardMode::OnDemand);
    window.set_layer(Layer::Top);
    window.set_default_width(240);
    let trigger = ShortcutTrigger::parse_string("Escape").unwrap();
    let action = CallbackAction::new(move |widget, _| {
        widget.activate_action("win.close", None).unwrap();
        true
    });
    let builder = Shortcut::builder().trigger(&trigger).action(&action);
    let controller = ShortcutController::new();
    controller.add_shortcut(builder.build());
    window.add_controller(controller);
    let action_close: ActionEntry<ApplicationWindow> = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();
    window.add_action_entries([action_close]);
    window
}

fn build_list(config: Config) -> ListBox {
    let list_box = ListBox::builder().name("list").build();
    let models = vec![
        ("Lock", include_str!("lock.svg"), "lock"),
        ("Sleep", include_str!("sleep.svg"), "sleep"),
        ("Poweroff", include_str!("poweroff.svg"), "poweroff"),
    ];
    for (label, icon, css_id) in models {
        list_box.append(&build_entry(label, icon, css_id));
    }
    let shortcut_controller = ShortcutController::new();
    for (key, direction) in [
        ("<Control>p", DirectionType::TabBackward),
        ("<Control>n", DirectionType::TabForward),
    ] {
        shortcut_controller.add_shortcut(create_shortcut(key, direction));
    }
    list_box.add_controller(shortcut_controller);

    list_box.connect_row_activated(move |list_box, row| {
        match row.index() {
            0 => config.run_lock(),
            1 => config.run_sleep(),
            2 => config.run_poweroff(),
            _ => (),
        }
        list_box.activate_action("win.close", None).unwrap();
    });
    list_box
}

fn build_entry(label: &str, icon: &str, css_id: &str) -> gtk4::Box {
    let bx = gtk4::Box::new(Orientation::Horizontal, 16);
    let bytes = Bytes::from(icon.as_bytes());
    let texture = Texture::from_bytes(&bytes).unwrap();
    let image = Image::from_paintable(Some(&texture));
    image.set_pixel_size(24);
    bx.append(&image);
    bx.append(&Label::new(Some(label)));
    bx.set_property("name", css_id);
    bx
}

fn create_shortcut(key: &str, direction_type: DirectionType) -> Shortcut {
    let ctrl_p_trigger = ShortcutTrigger::parse_string(key).unwrap();
    let ctrl_p_action = CallbackAction::new(move |widget, _| {
        let list_box: &ListBox = widget.downcast_ref().unwrap();
        list_box.emit_move_focus(direction_type);
        true
    });

    Shortcut::builder()
        .trigger(&ctrl_p_trigger)
        .action(&ctrl_p_action)
        .build()
}
