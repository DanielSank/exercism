use int_enum::IntEnum;
use anyhow::Context;
use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;


#[repr(u32)]
#[derive(Copy, Debug, Clone, IntEnum, Display, EnumIter, PartialEq)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}


impl TryFrom<u32> for ResistorColor {
    type Error = anyhow::Error;
    fn try_from(number: u32) -> Result<Self, Self::Error> {
        Self::from_int(number).with_context(|| "Invalid resistor value")
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    ResistorColor::try_from(value)
        .as_ref()  // Result<T, E> -> Result<&T, E>
        .map(ToString::to_string)  // Result<T, E> -> Result<P, E>
        .unwrap_or_else(|_| "value out of range".to_string())  // Result<T, E> -> T
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::iter().collect()
}
