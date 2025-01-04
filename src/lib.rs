use button::ButtonStroke;
use midir::{MidiInput, MidiOutput};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Arc, LazyLock, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

pub const MK2_NAME: &str = "Launchpad MK2";

pub mod button;

pub mod keyboard;

pub type ButtonCallback = Arc<Mutex<Arc<dyn Fn() + Send + Sync + 'static>>>;

pub fn new(callback: Arc<dyn Fn() + Send + Sync + 'static>) -> ButtonCallback {
    Arc::new(Mutex::new(callback))
}

#[rustfmt::skip]
pub fn blank_binds() -> [ButtonCallback; 80] {
    [
        new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())),
        new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())),
        new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())),
        new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())),
        new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())),
        new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())),
        new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())),
        new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())),
        new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())), new(Arc::new(|| ())),
    ]
}

pub static LAUCHPAD_MK2_BOARD_BINDS: LazyLock<([ButtonCallback; 80], [ButtonCallback; 80])> =
    LazyLock::new(|| (blank_binds(), blank_binds()));