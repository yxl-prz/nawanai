#![windows_subsystem = "windows"]

slint::include_modules!();

mod tcp;
mod util;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let tcp_listener = tcp::TCPListener::new(&ui);

    ui.on_power_action({
        let chan = tcp_listener.channel.clone();
        move |action_id| {
            chan.send(tcp::Message::PowerAction(action_id)).unwrap();
        }
    });

    ui.on_notify({
        let chan = tcp_listener.channel.clone();
        move |id| {
            chan.send(tcp::Message::Notification(id)).unwrap();
        }
    });

    ui.on_misc({
        let chan = tcp_listener.channel.clone();
        move |id| {
            chan.send(tcp::Message::Misc(id)).unwrap();
        }
    });

    ui.run().unwrap();
    tcp_listener.join().unwrap();
    Ok(())
}
