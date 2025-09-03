use std::fmt;
use std::process::Command;

use colorsys::Rgb;

use crate::common::{parse_colour_param, pct_value_hsl};

#[derive(Debug, PartialEq)]
pub enum PingAppletError {
    PingMissingHost,
    PingError,
}

impl std::error::Error for PingAppletError {}

impl fmt::Display for PingAppletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PingAppletError: {:?}", self)
    }
}

type Result<T> = std::result::Result<T, PingAppletError>;

fn ping_host(host: &str) -> Result<()> {
    match Command::new("ping").arg("-q").arg("-c").arg("1").arg("-w").arg("1").arg(host).output() {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(PingAppletError::PingError),
    }
}

pub fn applet(args: &[String]) -> Result<()> {
    let mut host: &str = "";
    let mut colour_s: Option<f32> = None;
    let mut colour_l: Option<f32> = None;

    for arg in args {
        if let Some(s) = parse_colour_param(arg, "s") {
            if (0.0..=100.0).contains(&s) {
                colour_s = Some(s);
            } else {
                eprintln!("Saturation {s} out of range [0, 100.0]");
            }
            continue;
        };
        if let Some(l) = parse_colour_param(arg, "l") {
            if (0.0..=100.0).contains(&l) {
                colour_l = Some(l);
            } else {
                eprintln!("Lightness {l} out of range [0, 100.0]");
            }
            continue;
        }

        if !host.is_empty() {
            eprintln!("Ignoring extra argument: {arg}");
            continue;
        }

        host = arg;
    }

    if host.is_empty() {
        return Err(PingAppletError::PingMissingHost);
    }

    eprintln!("Saturation: {:?} Lightness: {:?}", colour_s, colour_l);
    eprintln!("Pinging host: {host}");

    let value = match ping_host(host) {
        // NOTE: These are inverted from what you might expect, as pct_value_hsl grades along a line from green -> red
        Ok(_) => 0.0,
        Err(_) => 1.0,
    };

    let c = pct_value_hsl(value, colour_s, colour_l);
    let rgb = Rgb::from(&c);

    print!("#[bg={}]  #[default]", rgb.to_hex_string());

    Ok(())
}
