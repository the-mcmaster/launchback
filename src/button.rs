#[derive(Debug)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    Session,
    User1,
    User2,
    Mixer,

    Volume,
    Pan,
    SendA,
    SendB,
    Stop,
    Mute,
    Solo,
    RecordArm,

    Grid(u8, u8)
}

impl Button {
    const UP: u8 = 104;
    const DOWN: u8 = 105;
    const LEFT: u8 = 106;
    const RIGHT: u8 = 107;
    const SESSION: u8 = 108;
    const USER1: u8 = 109;
    const USER2: u8 = 110;
    const MIXER: u8 = 111;

    const VOLUME: u8 = 89;
    const PAN: u8 = 79;
    const SENDA: u8 = 69;
    const SENDB: u8 = 59;
    const STOP: u8 = 49;
    const MUTE: u8 = 39;
    const SOLO: u8 = 29;
    const RECORD_ARM: u8 = 19;

    pub fn id(&self) -> u8 {
        match self {
            Button::Up => Button::UP,
            Button::Down => Button::DOWN,
            Button::Left => Button::LEFT,
            Button::Right => Button::RIGHT,
            Button::Session => Button::SESSION,
            Button::User1 => Button::USER1,
            Button::User2 => Button::USER2,
            Button::Mixer => Button::MIXER,
            Button::Volume => Button::VOLUME,
            Button::Pan => Button::PAN,
            Button::SendA => Button::SENDA,
            Button::SendB => Button::SENDB,
            Button::Stop => Button::STOP,
            Button::Mute => Button::MUTE,
            Button::Solo => Button::SOLO,
            Button::RecordArm => Button::RECORD_ARM,
            Button::Grid(row, col) => 10*row + col,
        }
    }
}

#[derive(Debug)]
pub enum ButtonStroke {
    Press(Button),
    Release(Button),
}

impl From<[u8; 3]> for ButtonStroke {
    fn from(data: [u8; 3]) -> Self {
        const TOP_CONTROLS: u8 = 176;
        const LOWER_NINE_ROWS: u8 = 144;

        use Button::*;
        let (button, pressed) = match data {
            [TOP_CONTROLS, control_button_id @ Button::UP..=Button::MIXER, press] => {
                let button = match control_button_id {
                    Button::UP => Up,
                    Button::DOWN => Down,
                    Button::LEFT => Left,
                    Button::RIGHT => Right,
                    Button::SESSION => Session,
                    Button::USER1 => User1,
                    Button::USER2 => User2,
                    Button::MIXER => Mixer,
                    _ => unimplemented!()
                };

                (button, press != 0)
            }
            [LOWER_NINE_ROWS, button @ 11..=89, press] => {
                let row = button / 10;
                let col = button % 10;

                use Button::*;
                let button = if col == 9 {match button {
                    Button::VOLUME => Volume,
                    Button::PAN => Pan,
                    Button::SENDA => SendA,
                    Button::SENDB => SendB,
                    Button::STOP => Stop,
                    Button::MUTE => Mute,
                    Button::SOLO => Solo,
                    Button::RECORD_ARM => RecordArm,
                    _ => unimplemented!()
                }} else {
                    Grid(row, col)
                };

                (button, press != 0)
            }
            _ => todo!()
        };

        match pressed {
            true => Self::Press(button),
            false => Self::Release(button),
        }
    }
}