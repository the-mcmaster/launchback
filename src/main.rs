use button::ButtonStroke;
use midir::{MidiInput, MidiOutput};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Arc, LazyLock, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

const MK2_NAME: &str = "Launchpad MK2";

mod button;

mod keyboard;

type ButtonCallback = Arc<Mutex<Arc<dyn Fn() + Send + Sync + 'static>>>;

fn new(callback: Arc<dyn Fn() + Send + Sync + 'static>) -> ButtonCallback {
    Arc::new(Mutex::new(callback))
}

#[rustfmt::skip]
fn blank_binds() -> [ButtonCallback; 80] {
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

pub(crate) static LAUCHPAD_MK2_BOARD_BINDS: LazyLock<([ButtonCallback; 80], [ButtonCallback; 80])> =
    LazyLock::new(|| (blank_binds(), blank_binds()));

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    keyboard::pre_initialization();
    
    'main: loop {
        // Create a MIDI input object
        let midi_in = MidiInput::new("MIDI Input")?;
        let midi_out = MidiOutput::new("MIDI OUTPUT")?;
        let ports = midi_in.ports();
        let out_ports = midi_out.ports();

        let devices = ports
            .iter()
            .filter(|port| {
                midi_in
                    .port_name(port)
                    .unwrap_or_default()
                    .chars()
                    .take(MK2_NAME.chars().count())
                    .collect::<String>()
                    .eq(MK2_NAME)
            })
            .collect::<Vec<_>>();

        let out_devices = out_ports
            .iter()
            .filter(|port| {
                midi_out
                    .port_name(port)
                    .unwrap_or_default()
                    .chars()
                    .take(MK2_NAME.chars().count())
                    .collect::<String>()
                    .eq(MK2_NAME)
            })
            .collect::<Vec<_>>();

        if devices.is_empty() || out_devices.is_empty() {
            /* println!("No MIDI devices found. Please connect your Launchpad MK2."); */
            sleep(Duration::from_secs(1));
            continue 'main;
        }

        let port = &devices[0]; // Assuming the first port is the Launchpad MK2
        let out_port = &out_devices[0];
        /* println!("Using MIDI port: {}", midi_in.port_name(port)?);
        println!("Using MIDI port: {}", midi_out.port_name(out_port)?); */

        // Channel to monitor disconnection events
        let (midi_conns_tx, midi_conns_rx) = channel::<()>();
        let midi_conns_tx_clone = midi_conns_tx.clone();

        let port_name = midi_in.port_name(port)?;

        let mut conn_out = midi_out.connect(out_port, "midir-output")?;

        // Connect to the MIDI device
        let _conn_in = midi_in.connect(
            port,
            "midir-read-input",
            move |_timestamp, message, _| {
                let button_stroke = ButtonStroke::from([message[0], message[1], message[2]]);

                match button_stroke {
                    ButtonStroke::Press(button) => {
                        LAUCHPAD_MK2_BOARD_BINDS.0[button.index()].lock().unwrap()()
                    }
                    ButtonStroke::Release(button) => {
                        LAUCHPAD_MK2_BOARD_BINDS.1[button.index()].lock().unwrap()()
                    }
                }

                let data = match button_stroke {
                    ButtonStroke::Press(button) => [0x90, button.id(), 127],
                    ButtonStroke::Release(button) => [0x80, button.id(), 0],
                };

                _ = conn_out.send(&data);
            },
            (),
        )?;

        // Spawn a thread to monitor for disconnection
        thread::spawn(move || loop {
            let midi_in = MidiInput::new("MIDI Input").unwrap();
            let ports = midi_in.ports();
            let current_port = ports
                .iter()
                .find(|port| midi_in.port_name(port).unwrap().eq(&port_name));
            if current_port.is_none() {
                _ = midi_conns_tx_clone.send(());
            }
            sleep(Duration::from_millis(200));
        });

        println!("Connected. Listening for MIDI messages. Disconnect the device to test.");

        'main_loop: loop {
            // Handle disconnection event
            match midi_conns_rx.recv_timeout(Duration::from_millis(150)) {
                Ok(_device_disconnect) => {
                    continue 'main;
                }
                Err(RecvTimeoutError::Timeout) => {
                    continue 'main_loop;
                }
                Err(RecvTimeoutError::Disconnected) => {
                    eprintln!("warn: Midi Monitor's Sender disconnected unexpectedly. Reseting...");
                    continue 'main;
                }
            }
        }
    }
}
