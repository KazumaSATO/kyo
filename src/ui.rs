use crate::config::{Command, Config};
use gdk4::Texture;
use gio::ActionEntry;
use glib::clone;
use gtk4::glib::signal::Propagation;
use gtk4::glib::source::Priority;
use gtk4::glib::{Bytes, ControlFlow, MainContext, Receiver, Sender};
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, CallbackAction, DirectionType, EventControllerKey,
    GestureClick, Image, Label, ListBox, ListBoxRow, Orientation, SelectionMode, Shortcut,
    ShortcutController, ShortcutTrigger, Window,
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

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

fn build_window(app: &Application) -> ApplicationWindow {
    // https://stackoverflow.com/questions/66942543/how-do-we-build-gui-with-glade-gtk-rs-in-rust
    // https://github.com/pachi/visol/blob/master/src/window.rs
    let builder = ApplicationWindow::builder();
    let window = builder.application(app).build();
    window.init_layer_shell();
    window.set_keyboard_mode(KeyboardMode::OnDemand);
    //window.set_layer(Layer::Overlay);
    window.set_layer(Layer::Top);
    window.set_default_width(240);
    // window.set_margin(Edge::Left, 200);
    // window.set_anchor(Edge::Left, true);
    // Boxは非対称のサイズに子供を指定できるみたい。
    window.set_property("name", "window");
    let trigger = ShortcutTrigger::parse_string("Escape").unwrap();
    let action = CallbackAction::new(move |widget, _| {
        widget.activate_action("win.close", None).unwrap();
        // let window: &ApplicationWindow = widget.downcast_ref().unwrap();
        // window.close();
        true
    });
    let builder = Shortcut::builder().trigger(&trigger).action(&action);
    let controller = ShortcutController::new();
    controller.add_shortcut(builder.build());
    window.add_controller(controller);

    // Add action "close" to `window` taking no parameter
    let action_close: ActionEntry<ApplicationWindow> = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();
    window.add_action_entries([action_close]);
    window
}

fn build_list(config: &Config) -> ListBox {
    let builder = ListBox::builder()
        .activate_on_single_click(true)
        .selection_mode(SelectionMode::None);
    let list_box = builder.build();

    let models = vec![
        ("Lock", include_str!("lock.svg")),
        ("Sleep", include_str!("sleep.svg")),
        ("Poweroff", include_str!("poweroff.svg")),
    ];
    for (label, icon) in models {
        list_box.append(&build_entry(label, icon));
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
    list_box.add_controller(shortcut_controller);
    let controller_key = EventControllerKey::new();

    let (sender, receiver) = MainContext::channel::<()>(Priority::DEFAULT);
    controller_key.connect_key_pressed(move |k, key, c, modifier_type| {
        // Enter
        if c == 36 {
            //sender.send(()).unwrap();
        }
        Propagation::Proceed
    });
    list_box.connect_row_activated(|_, _| {
        println!("foobar");
    });
    list_box.add_controller(controller_key);
    receiver.attach(
        None,
        clone!(@weak list_box => @default-return ControlFlow::Continue, move |_| {
            let selected_index = list_box.get_index();
            println!("{}", selected_index);

        list_box.activate_action("win.close", None).unwrap();
            ControlFlow::Continue
        }),
    );

    list_box
}

fn build_entry(label: &str, icon: &str) -> ListBoxRow {
    let bx = gtk4::Box::new(Orientation::Horizontal, 16);
    let bytes = Bytes::from(icon.as_bytes());
    let texture = Texture::from_bytes(&bytes).unwrap();
    let image = Image::from_paintable(Some(&texture));
    image.set_pixel_size(24);
    bx.append(&image);
    bx.append(&Label::new(Some(label)));
    // let click_controller = GestureClick::new();
    // //bx.add_css_class("entry");
    // click_controller.connect_pressed(move |_, _, _, _| {
    //     //emit_move_focus(&self, direction: DirectionType);
    // });
    // bx.add_controller(click_controller);
    bx.set_property("name", "entry");
    let builder = ListBoxRow::builder();
    let row = builder.child(&bx).build();
    row.connect_activate(|a| {
        println!("doge");
    });
    row.set_property("a", 3);
    //let c: i32 = row.property("a");
    //println!("{}", c);
    row
    //bx
}

trait Entries {
    fn get_index(&self) -> u32;
}

impl Entries for ListBox {
    fn get_index(&self) -> u32 {
        let selected = self.selected_row().unwrap();
        let mut i: i32 = 0;
        let mut res = 0;
        while let Some(found) = self.row_at_index(i) {
            if selected == found {
                res = i;
                break;
            }
            i += 1;
        }
        res as u32
    }
}
