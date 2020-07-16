use dirs_next as dirs;
use serde::Serialize;
use std::collections::HashMap;
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

fn get_color_name(n: usize) -> &'static str {
    match n {
        0 => "color1",
        1 => "color2",
        2 => "color3",
        3 => "color4",
        4 => "color5",
        5 => "color6",
        6 => "color7",
        7 => "color8",
        8 => "color9",
        9 => "color10",
        10 => "color11",
        11 => "color12",
        12 => "color13",
        13 => "color14",
        14 => "color15",
        15 => "color16",
        _ => panic!("No such color!"),
    }
}

fn main() -> io::Result<()> {
    let mut colors_path = dirs::cache_dir().unwrap();
    colors_path.push("wal/colors");

    let f = File::open(colors_path)?;
    let f = BufReader::new(f);
    let c: HashMap<&str, String> = f
        .lines()
        .enumerate()
        .map(|(i, c)| (get_color_name(i), "0x".to_string() + &c.unwrap()[1..]))
        .collect();

    let primary = GroundList {
        background: &c["color1"],
        foreground: &c["color2"],
    };

    let cursor = CursorList {
        text: &c["color1"],
        cursor: &c["color8"],
    };

    let normal = ColorList {
        black: &c["color1"],
        red: &c["color2"],
        green: &c["color3"],
        yellow: &c["color4"],
        blue: &c["color5"],
        magenta: &c["color6"],
        cyan: &c["color7"],
        white: &c["color8"],
    };

    let bright = ColorList {
        black: &c["color9"],
        red: &c["color10"],
        green: &c["color11"],
        yellow: &c["color12"],
        blue: &c["color13"],
        magenta: &c["color14"],
        cyan: &c["color15"],
        white: &c["color16"],
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
    let colors =
        &serde_yaml::to_string(&ColorFile { colors }).expect("Failed to convert to string")[4..];

    conf.write(colors.as_bytes())?;

    Ok(())
}
