use std::cell::RefCell;
use std::fmt::Display;
use std::os::unix::raw::time_t;
use std::ptr::addr_of_mut;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, pin_mut, SinkExt, StreamExt, TryFutureExt};
use tokio::sync::mpsc;
use tokio::time::Instant;

use crate::app::AppResult;
use crate::event::Event::Tick;

/// Terminal events.
#[derive(Clone, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),

    UiUpdate(String),
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Event::UiUpdate(string) => { String::from(string) }
            _ => { String::from("") }
        };
        write!(f, "{}", str)
    }
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,
    /// Event handler thread.
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        let _sender = sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                  _ = _sender.closed() => {
                    break;
                  }
                  _ = tick_delay => {
                    _sender.send(Event::Tick).unwrap();
                  }
                  Some(Ok(evt)) = crossterm_event => {
                    match evt {
                      CrosstermEvent::Key(key) => {
                        if key.kind == crossterm::event::KeyEventKind::Press {
                          _sender.send(Event::Key(key)).unwrap();
                        }
                      },
                      CrosstermEvent::Mouse(mouse) => {
                        _sender.send(Event::Mouse(mouse)).unwrap();
                      },
                      CrosstermEvent::Resize(x, y) => {
                        _sender.send(Event::Resize(x, y)).unwrap();
                      },
                      CrosstermEvent::FocusLost => {
                      },
                      CrosstermEvent::FocusGained => {
                      },
                      CrosstermEvent::Paste(_) => {
                      },
                    }
                  }
                }
                ;
            }
        });
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub async fn next(&mut self) -> AppResult<Event> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "This is an IO error",
            )))
    }

    pub fn send(&self, event: Event) -> AppResult<()> {
        self.sender.send(Event::UiUpdate(
            event.to_string()
        )).map(|e| { e })
            .unwrap_or_else(|e1| { e1; });

        Ok(())
    }
}
