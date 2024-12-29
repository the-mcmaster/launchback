use button::{Button, ButtonStroke};
use midir::{MidiInput, MidiOutput};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread::{self, sleep};
use std::time::Duration;

const MK2_NAME: &str = "Launchpad MK2";

mod button;

mod keyboard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("STARTING MAIN FOR THE FIRST TIME");

    'main: loop {
    // Create a MIDI input object
    let midi_in = MidiInput::new("MIDI Input")?;
    let midi_out = MidiOutput::new("MIDI OUTPUT")?;
    let ports = midi_in.ports();
    let out_ports = midi_out.ports();

    let devices = ports.iter()
        .filter(|port| midi_in
            .port_name(&port).unwrap_or_default()
            .chars().take(MK2_NAME.chars().count()).collect::<String>()
            .eq(MK2_NAME)
        )
        .collect::<Vec<_>>();

    let out_devices = out_ports.iter()
    .filter(|port| midi_out
        .port_name(&port).unwrap_or_default()
        .chars().take(MK2_NAME.chars().count()).collect::<String>()
        .eq(MK2_NAME)
    )
    .collect::<Vec<_>>();

    if devices.is_empty() || out_devices.is_empty() {
        println!("No MIDI devices found. Please connect your Launchpad MK2.");
        sleep(Duration::from_secs(1));
        continue 'main;
    }

    let port = &devices[0]; // Assuming the first port is the Launchpad MK2
    let out_port = &out_devices[0];
    println!("Using MIDI port: {}", midi_in.port_name(port)?);
    println!("Using MIDI port: {}", midi_out.port_name(out_port)?);

    // Channel to monitor disconnection events
    let (midi_conns_tx, midi_conns_rx) = channel::<()>();
    let midi_conns_tx_clone = midi_conns_tx.clone();

    let port_name = midi_in.port_name(&port)?;

    let mut conn_out = midi_out.connect(&out_port, "midir-output")?;

    // Connect to the MIDI device
    let _conn_in = midi_in.connect(
        port,
        "midir-read-input",
        move |timestamp, message, _| {
            let button_stroke = ButtonStroke::from([message[0], message[1], message[2]]);
            println!(
                "{}: {:?} ({} bytes): BUTTON {:?}",
                timestamp,
                message,
                message.len(),
                if message.len() == 3 {
                    format!("{:?}", button_stroke)
                } else {
                    "UNKNOWN".to_string()
                }
            );

            if let ButtonStroke::Press(Button::Mute) = button_stroke {
                println!("PRESSTING A");
                inputbot::KeybdKey::AKey.press();
            } else if let ButtonStroke::Release(Button::Mute) = button_stroke {
                println!("UNPRESSTING A");
                inputbot::KeybdKey::AKey.release();
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
    thread::spawn(move || {
        loop {
            let midi_in = MidiInput::new("MIDI Input").unwrap();
            let ports = midi_in.ports();
            let current_port = ports.iter().find(|port| midi_in.port_name(port).unwrap().eq(&port_name));
            if current_port.is_none() {
                _ = midi_conns_tx_clone.send(());
            }
            sleep(Duration::from_millis(200));
        }
    });

    println!("Connected. Listening for MIDI messages. Disconnect the device to test.");

    'main_loop: loop {
        // Handle disconnection event
        match midi_conns_rx.recv_timeout(Duration::from_millis(150)) {
            Ok(_device_disconnect) => {
                println!("MIDI device disconnected. Reseting...");
                continue 'main;
            }
            Err(RecvTimeoutError::Timeout) => {
                println!("Retting Mainloop...");
                continue 'main_loop
            }
            Err(RecvTimeoutError::Disconnected) => {
                println!("WARN: Disconnection: Sender unexpectedly disconnected. Reseting...");
                continue 'main;
            }
        }
    }
}}
