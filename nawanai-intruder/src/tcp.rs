use std::{
    fmt::{Display, Formatter},
    net::SocketAddr,
};

use slint::ComponentHandle;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

const BUFFER_SIZE: usize = 6;

const FLAG_POWER_ACTION: u8 = 0b00000001;
const FLAG_NOTIFY: u8 = 0b00000010;
const FLAG_MISC: u8 = 0b00000100;

const FLAG_POWER_SHUTDOWN: u8 = 0b00000001;
const FLAG_POWER_RESTART: u8 = 0b00000010;

const FLAG_NOTIFY_FAKE_UPDATE: u8 = 0b00000001;
const FLAG_NOTIFY_VIRUS_DETECTED: u8 = 0b00000010;

const FLAG_MISC_RUNAWAY: u8 = 0b00000001;
const FLAG_MISC_MOUSE_GRAVITY: u8 = 0b00000010;

pub enum Message {
    PowerAction(i32),
    Notification(i32),
    Misc(i32),
    Quit,
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::PowerAction(i) => write!(f, "PowerAction({})", i),
            Self::Notification(i) => write!(f, "Notification({})", i),
            Self::Misc(i) => write!(f, "Misc({})", i),
            Self::Quit => write!(f, "Quit"),
        }
    }
}

pub struct TCPListener {
    pub channel: UnboundedSender<Message>,
    worker_thread: std::thread::JoinHandle<()>,
}

impl TCPListener {
    pub fn new(ui: &crate::AppWindow) -> Self {
        let (channel, r) = tokio::sync::mpsc::unbounded_channel();
        let worker_thread = std::thread::spawn({
            let weak = ui.as_weak();
            move || {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(listen_loop(r, weak))
                    .unwrap()
            }
        });
        Self {
            channel,
            worker_thread,
        }
    }

    pub fn join(self) -> std::thread::Result<()> {
        let _ = self.channel.send(Message::Quit);
        self.worker_thread.join()
    }
}

async fn listen_loop(
    mut r: UnboundedReceiver<Message>,
    handle: slint::Weak<crate::AppWindow>,
) -> tokio::io::Result<()> {
    let listener = TcpListener::bind("192.168.50.76:7878").await?;
    let mut connections: Vec<(TcpStream, SocketAddr)> = Vec::new();
    let mut buffers = vec![[0; BUFFER_SIZE]];

    let connections_ptr = &mut connections as *mut Vec<(TcpStream, SocketAddr)>;

    loop {
        tokio::select! {
            connection = listener.accept() => {
                connections.push(connection?);
                let conns = connections.len();
                handle.clone().upgrade_in_event_loop(move |h| {
                    h.set_connections(conns as i32);
                }).unwrap();
            }
            message = r.recv() => {
                if let Some(msg) = message {
                    println!("[Worker Thread] Got Message {}", msg);
                    match msg {
                        Message::PowerAction(id) => {
                            let mut packet = Vec::new();
                            packet.push(FLAG_POWER_ACTION);
                            packet.push(match id {
                                0 => FLAG_POWER_SHUTDOWN,
                                1 => FLAG_POWER_RESTART,
                                _ => { continue; }
                            });
                            for (stream, _) in connections.iter_mut() {
                                stream.write(&packet.to_vec()).await.unwrap();
                            }
                        },
                        Message::Notification(id) => {
                            let mut packet = Vec::new();
                            packet.push(FLAG_NOTIFY);
                            packet.push(match id {
                                0 => FLAG_NOTIFY_FAKE_UPDATE,
                                1 => FLAG_NOTIFY_VIRUS_DETECTED,
                                _ => { continue; }
                            });
                            for (stream, _) in connections.iter_mut() {
                                stream.write(&packet.to_vec()).await.unwrap();
                            }
                        }
                        Message::Misc(id) => {
                            let mut packet = Vec::new();
                            packet.push(FLAG_MISC);
                            packet.push(match id {
                                1 => FLAG_MISC_RUNAWAY,
                                2 => FLAG_MISC_MOUSE_GRAVITY,
                                _ => { continue; }
                            });
                            for (stream, _) in connections.iter_mut() {
                                stream.write(&packet.to_vec()).await.unwrap();
                            }
                        }
                        Message::Quit => {
                            break Ok(());
                        }
                    }
                }
            }
            d = async {
                let mut disconnected = Vec::new();
                for (i, (stream, _addr)) in connections.iter_mut().enumerate() {
                    match stream.read(&mut buffers[i]).await {
                        Ok(n) if n == 0 => {
                            disconnected.push(i);
                        }
                        Err(_e) => {
                            // Handle read error if needed
                        }
                        _ => {} // For other cases, do nothing
                    }
                }
                disconnected
            } => {
                if !d.is_empty() {
                    let c = unsafe {&mut *connections_ptr};
                    println!("{} connection(s) closed", d.len());
                    for i in d {
                        c.remove(i);
                    }
                }
            }
        };
    }
}
