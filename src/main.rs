use dirs_next as dirs;
use enum_map::{Enum, EnumMap};
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Serialize)]
struct ColorFile<'a> {
    colors: Colors<'a>,
}

#[derive(Serialize)]
struct Colors<'a> {
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
enum Color {
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
}

fn main() -> io::Result<()> {
    let mut colors_path = dirs::cache_dir().unwrap();
    colors_path.push("wal/colors");

    let f = File::open(colors_path)?;
    let f = BufReader::new(f);
    let c: EnumMap<Color, String> =
        f.lines()
            .enumerate()
            .fold(EnumMap::new(), |mut map, (i, c)| {
                let color = Color::get_color(i);
                map[color] = "0x".to_string() + &c.unwrap()[1..];
                map
            });

    let primary = GroundList {
        background: &c[Color::One],
        foreground: "0xFFFFFF",
    };

    let cursor = CursorList {
        text: &c[Color::One],
        cursor: &c[Color::Eight],
    };

    let normal = ColorList {
        black: &c[Color::One],
        red: &c[Color::Two],
        green: &c[Color::Three],
        yellow: &c[Color::Four],
        blue: &c[Color::Five],
        magenta: &c[Color::Six],
        cyan: &c[Color::Seven],
        white: &c[Color::Eight],
    };

    let bright = ColorList {
        black: &c[Color::Nine],
        red: &c[Color::Ten],
        green: &c[Color::Eleven],
        yellow: &c[Color::Twelve],
        blue: &c[Color::Thirteen],
        magenta: &c[Color::Fourteen],
        cyan: &c[Color::Fifteen],
        white: &c[Color::Sixteen],
    };

    let mut conf_path = dirs::config_dir().unwrap();
    conf_path.push("alacritty/colors.yml");

    let mut conf = File::create(conf_path)?;
    let colors = Colors {
        primary,
        cursor,
        normal,
        bright,
    };

    // [4..] because I want to skip the first 4 bytes (---\n) as this file will later be appended
    // to the main config file
    let colors =
        &serde_yaml::to_string(&ColorFile { colors }).expect("Failed to convert to string")[4..];

    conf.write(colors.as_bytes())?;

    Ok(())
}
