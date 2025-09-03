use std::fmt;
use std::fs;

use colorsys::Rgb;

use crate::common::{parse_colour_param, pct_value_hsl};

#[derive(Debug, PartialEq)]
pub enum MemAppletError {
    MemInfoUnavailable,
}

impl std::error::Error for MemAppletError {}

impl fmt::Display for MemAppletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MemAppletError: {:?}", self)
    }
}

type Result<T> = std::result::Result<T, MemAppletError>;

#[derive(Debug, PartialEq)]
pub struct MemInfo {
    pub total: u32,
    pub used: u32,
    pub available: u32,
}

const MEM_INFO_PATH: &str = "/proc/meminfo";

fn read_meminfo() -> Result<MemInfo> {
    let data = fs::read_to_string(MEM_INFO_PATH).or(Err(MemAppletError::MemInfoUnavailable))?;

    let mut info = MemInfo { available: 0, total: 0, used: 0 };

    for line in data.lines() {
        if info.total != 0 && info.available != 0 {
            break;
        }

        let mut parts = line.split_whitespace();
        let Some(key) = parts.next() else { continue };
        let Some(value) = parts.next() else { continue };

        match key {
            "MemTotal:" => {
                info.total = value.parse::<u32>().unwrap_or(0);
            }
            "MemAvailable:" => {
                info.available = value.parse::<u32>().unwrap_or(0);
            }
            _ => continue,
        }
    }

    if info.available > 0 && info.total > 0 {
        info.used = info.total - info.available
    }

    Ok(info)
}

fn normalise_mem_usage(info: &MemInfo) -> f32 {
    info.used as f32 / info.total as f32
}

pub fn applet(args: &[String]) -> Result<()> {
    let mut colour_s: Option<f32> = None;
    let mut colour_l: Option<f32> = None;
    let mut show_pct_text: bool = false;

    for arg in args {
        if arg == "pct-text" {
            show_pct_text = true;
            continue;
        }
        if let Some(s) = parse_colour_param(arg, "s") {
            if (0.0..=100.0).contains(&s) {
                colour_s = Some(s);
            } else {
                eprintln!("Saturation {s} out of range [0, 100.0]");
            }
        };
        if let Some(l) = parse_colour_param(arg, "l") {
            if (0.0..=100.0).contains(&l) {
                colour_l = Some(l);
            } else {
                eprintln!("Lightness {l} out of range [0, 100.0]");
            }
        }
    }

    let info = read_meminfo()?;
    let norm = normalise_mem_usage(&info);

    eprintln!("Mem Info: {:?} Pct: {:.2}", info, norm);

    let c = pct_value_hsl(norm, colour_s, colour_l);
    let rgb = Rgb::from(&c);

    if show_pct_text {
        let val = format!("{:.0}", ((norm * 100.0).round()));
        print!("#[bg={}]{:>2}", rgb.to_hex_string(), val);
    } else {
        print!("#[bg={}]  ", rgb.to_hex_string());
    }
    println!("#[default]");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_mem_info() {
        let m = read_meminfo();
        assert_eq!(true, m.is_ok());
        let m = m.unwrap();
        assert_ne!(0, m.total);
        assert_ne!(0, m.available);
        assert_ne!(0, m.used);
        println!("mem_info: {:?}", m);
    }
}
