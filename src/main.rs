mod color;

use anyhow::{Context, Result};
use color::{Color, ColorFile};
use dirs_next as dirs;
use enum_map::EnumMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;

// make old method optional
#[derive(StructOpt)]
#[structopt(
    name = "alacritty_wal", 
    version = env!("CARGO_PKG_VERSION"), 
    about = "Gets wal colors and modifies alacritty config to add them."
)]
struct Opt {
    /// If passed, a colors.yml file will be created in the alacritty configuration directory, instead of overwriting the current config.
    /// This file can be concatenated with the original config (as long as it doesn't contain a colors section) to create a whole new config
    #[structopt(short, long)]
    manual: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut colors_path = dirs::cache_dir().unwrap();
    colors_path.push("wal/colors");

    let f = File::open(&colors_path).with_context(|| {
        format!(
            "Failed to open wal colors from path: {}",
            colors_path.display()
        )
    })?;
    let f = BufReader::new(f);
    let c: EnumMap<Color, String> =
        f.lines()
            .enumerate()
            .fold(EnumMap::new(), |mut map, (i, c)| {
                let color = Color::get_color(i);
                map[color] = "0x".to_string() + &c.unwrap()[1..];
                map
            });

    let colors = Color::get_colors(&c);

    let mut conf_path = dirs::config_dir().unwrap();
    if opt.manual {
        conf_path.push("alacritty/colors.yml");

        let mut conf_file = File::create(conf_path)?;

        let colors = serde_yaml::to_vec(&ColorFile { colors })?;

        // [4..] because I want to skip the first 4 bytes (---\n) as this file will later be appended
        // to the main config file
        conf_file.write(&colors[4..])?;
    } else {
        conf_path.push("alacritty/alacritty.yml");

        let conf_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&conf_path)?;

        let mut conf: serde_yaml::Value = serde_yaml::from_reader(&conf_file)?;

        conf["colors"] = serde_yaml::to_value(colors)?;
        serde_yaml::to_writer(File::create(&conf_path)?, &conf)?;
    }

    Ok(())
}
