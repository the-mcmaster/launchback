//! TODO

use std::{thread::sleep, time::Duration};

use inputbot::{KeybdKey, MouseButton};

use crate::button::{Button, ButtonStroke};

macro_rules! grid_bind {
    (($row:expr, $col:expr), $value:expr) => {
        Press(Grid($row, $col)).bind(|| {
            $value.press();
        });
        Release(Grid($row, $col)).bind(|| {
            $value.release();
        });
    };
}

pub fn pre_initialization() {
    use ButtonStroke::*;
    use Button::*;

    Press(Up).bind(|| {
        MouseButton::LeftButton.press();
        sleep(Duration::from_millis(30));
        MouseButton::LeftButton.release();
    });

    grid_bind!((7, 3), KeybdKey::QKey);
    grid_bind!((7, 4), KeybdKey::WKey);
    grid_bind!((7, 5), KeybdKey::EKey);
    grid_bind!((7, 6), KeybdKey::RKey);

    grid_bind!((6, 3), KeybdKey::AKey);
    grid_bind!((6, 4), KeybdKey::SKey);
    grid_bind!((6, 5), KeybdKey::DKey);
    
    grid_bind!((5, 2), KeybdKey::LShiftKey);

    grid_bind!((3, 6), KeybdKey::GKey);
}