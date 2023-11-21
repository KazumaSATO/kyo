use crate::config::Config;
use gtk4::glib::source::Priority;
use gtk4::glib::{ControlFlow, MainContext, Receiver, Sender};
use gtk4::prelude::*;
use gtk4::{
    Application, CallbackAction, ListBox, Shortcut, ShortcutController, ShortcutTrigger, Window,
};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
//pub mod config;

pub fn build_ui(app: &Application, config: &Config) {
    let window = build_window(app);
    let list_box = build_list();
    window.set_child(Some(&list_box));

    let (sender, receiver): (Sender<Message>, Receiver<Message>) =
        MainContext::channel::<Message>(Priority::DEFAULT);

    let shortcut_controller = build_shortcut_controller(&window, sender);
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
    // window.set_margin(Edge::Left, 200);
    // window.set_anchor(Edge::Left, true);
    // Boxは非対称のサイズに子供を指定できるみたい。

    //builder.add_from_string(include_str!("system.ui")).unwrap();
    window
}

fn build_list() -> ListBox {
    let builder = ListBox::builder();
    builder.build()
}

fn build_shortcut_controller<'a>(
    window: &'a Window,
    sender: Sender<Message>,
) -> ShortcutController {
    let shortcut_controller = ShortcutController::new();
    // let (sender, receiver): (Sender<Message>, Receiver<Message>) =
    //     MainContext::channel::<Message>(Priority::DEFAULT);

    shortcut_controller.add_shortcut(build_shortcut_escape(sender.clone()));
    // receiver.attach(None, |x| {
    //     window.close();
    //     ControlFlow::Continue
    // });
    shortcut_controller
}
//#[derive(Copy, Clone)]
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

fn message_handler<'a>(window: &'a Window) -> impl FnMut(Message) -> ControlFlow + 'a {
    |x| {
        window.close();
        ControlFlow::Continue
    }
}
