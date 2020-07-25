use enum_map::{Enum, EnumMap};
use serde::Serialize;

#[derive(Serialize)]
pub struct ColorFile<'a> {
    pub colors: Colors<'a>,
}

#[derive(Serialize)]
pub struct Colors<'a> {
    primary: GroundList<'a>,
    cursor: CursorList<'a>,
    normal: ColorList<'a>,
    bright: ColorList<'a>,
}

#[derive(Serialize)]
struct GroundList<'a> {
    background: &'a str,
    foreground: &'a str,
}

#[derive(Serialize)]
struct CursorList<'a> {
    text: &'a str,
    cursor: &'a str,
}

#[derive(Serialize)]
struct ColorList<'a> {
    black: &'a str,
    red: &'a str,
    green: &'a str,
    yellow: &'a str,
    blue: &'a str,
    magenta: &'a str,
    cyan: &'a str,
    white: &'a str,
}

#[derive(Enum)]
pub enum Color {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
}

impl Color {
    pub fn get_color(n: usize) -> Color {
        match n {
            0 => Color::One,
            1 => Color::Two,
            2 => Color::Three,
            3 => Color::Four,
            4 => Color::Five,
            5 => Color::Six,
            6 => Color::Seven,
            7 => Color::Eight,
            8 => Color::Nine,
            9 => Color::Ten,
            10 => Color::Eleven,
            11 => Color::Twelve,
            12 => Color::Thirteen,
            13 => Color::Fourteen,
            14 => Color::Fifteen,
            15 => Color::Sixteen,
            _ => panic!("No such color!"),
        }
    }

    pub fn get_colors<'a>(colors: &'a EnumMap<Color, String>) -> Colors<'a> {
        let primary = GroundList {
            background: &colors[Color::One],
            foreground: &colors[Color::Sixteen],
        };

        let cursor = CursorList {
            text: &colors[Color::One],
            cursor: &colors[Color::Eight],
        };

        let normal = ColorList {
            black: &colors[Color::One],
            red: &colors[Color::Two],
            green: &colors[Color::Three],
            yellow: &colors[Color::Four],
            blue: &colors[Color::Five],
            magenta: &colors[Color::Six],
            cyan: &colors[Color::Seven],
            white: &colors[Color::Eight],
        };

        let bright = ColorList {
            black: &colors[Color::Nine],
            red: &colors[Color::Ten],
            green: &colors[Color::Eleven],
            yellow: &colors[Color::Twelve],
            blue: &colors[Color::Thirteen],
            magenta: &colors[Color::Fourteen],
            cyan: &colors[Color::Fifteen],
            white: &colors[Color::Sixteen],
        };

        Colors {
            primary,
            cursor,
            normal,
            bright,
        }
    }
}
