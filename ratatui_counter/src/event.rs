// Event handlers n general
use crossterm::event::{KeyEvent, MouseEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
// TODO: refactorings for usage via SegQueue over std

use color_eyre::Result;

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    // TODO: create a holder for resize to refer to as point x/y or width/height
}

#[derive(Debug)]
pub struct EventHandler {
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,

    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

// Impl for cross channel even communications

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();

        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("Unable to poll for event") {
                        match event::read().expect("Unable to read event") {
                            CrosstermEvent::Key(ev) => {
                                if ev.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(ev))
                                } else {
                                    Ok(())
                                }
                            }

                            CrosstermEvent::Mouse(ev) => sender.send(Event::Mouse(ev)),

                            CrosstermEvent::Resize(width, height) => {
                                sender.send(Event::Resize(width, height))
                            }

                            _ => unimplemented!(),
                        }
                        .expect("Failed to send the terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender
                            .send(Event::Tick)
                            .expect("Failed to send the tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
