use std::fmt;
use std::fs;

use nix::unistd;

use colorsys::{Hsl, Rgb};

#[derive(Debug, PartialEq)]
pub enum CPUAppletError {
    CPUCountError,
    CPUInfoError,
}

impl std::error::Error for CPUAppletError {}

impl fmt::Display for CPUAppletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPUAppletError: {:?}", self)
    }
}

type Result<T> = std::result::Result<T, CPUAppletError>;

pub fn cpu_count() -> Result<u32> {
    let count = match unistd::sysconf(unistd::SysconfVar::_NPROCESSORS_ONLN) {
        Ok(Some(c)) => c as u32,
        Ok(None) => return Err(CPUAppletError::CPUCountError),
        Err(_) => return Err(CPUAppletError::CPUCountError),
    };
    Ok(count)
}

#[derive(Debug, PartialEq)]
pub struct CPUInfo {
    pub min_freq: u32,
    pub max_freq: u32,
    pub cur_freq: u32,
}

fn read_u32_from_file(path: &str) -> Option<u32> {
    let vstr = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let vstr = vstr.trim_end();

    match vstr.parse::<u32>() {
        Ok(v) => Some(v),
        Err(_) => return None,
    }
}

// convert the current cpu frequency into a percentage, range [0.0, 1.0]
fn normalise_cur_freq(cpu: &CPUInfo) -> f32 {
    let mut c = cpu.cur_freq;
    if c < cpu.min_freq {
        c = cpu.min_freq
    }
    if c > cpu.max_freq {
        c = cpu.max_freq
    }
    let range = cpu.max_freq - cpu.min_freq;
    let adj = c - cpu.min_freq;
    return adj as f32 / range as f32;
}

fn pct_to_hue(pct: f32) -> f32 {
    const LEFT_STOP: f32 = 0.0; // Red
    const RIGHT_STOP: f32 = 120.0; // Green
    assert!(RIGHT_STOP > LEFT_STOP);
    // Note: pct is inverted as we want to transition from green to red
    LEFT_STOP + ((1.0 - pct) * (RIGHT_STOP - LEFT_STOP))
}

const DEFAULT_SATURATION: f32 = 100.0;
const DEFAULT_LIGHTNESS: f32 = 50.0;

fn cpu_freq_hsl(norm: f32, s: Option<f32>, l: Option<f32>) -> Hsl {
    let hue = pct_to_hue(norm);
    let s = s.unwrap_or(DEFAULT_SATURATION);
    let l = l.unwrap_or(DEFAULT_LIGHTNESS);
    Hsl::from((hue, s, l))
}

fn cpu_info(cpu_index: u32) -> Result<CPUInfo> {
    let min_freq_path = format!("/sys/bus/cpu/devices/cpu{cpu_index}/cpufreq/scaling_min_freq");
    let max_freq_path = format!("/sys/bus/cpu/devices/cpu{cpu_index}/cpufreq/scaling_max_freq");
    let cur_freq_path = format!("/sys/bus/cpu/devices/cpu{cpu_index}/cpufreq/scaling_cur_freq");

    match (
        read_u32_from_file(&min_freq_path),
        read_u32_from_file(&max_freq_path),
        read_u32_from_file(&cur_freq_path),
    ) {
        (Some(min), Some(max), Some(cur)) => Ok(CPUInfo {
            min_freq: min,
            max_freq: max,
            cur_freq: cur,
        }),
        _ => Err(CPUAppletError::CPUInfoError),
    }
}

impl fmt::Display for CPUInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPUInfo: {:?}", self)
    }
}

// Parse a colour parameter in the form: `{key}:{val}` where `{val}` is a float
fn parse_colour_param(p: &str, key: &str) -> Option<f32> {
    let parts: Vec<&str> = p.split(":").collect();
    if parts.len() != 2 {
        return None;
    }

    let k = parts[0];
    let v = parts[1];
    if key != k {
        return None;
    }

    v.parse::<f32>().ok()
}

pub fn applet(args: &[String]) -> Result<()> {
    let mut colour_s: Option<f32> = None;
    let mut colour_l: Option<f32> = None;

    for arg in args {
        if let Some(s) = parse_colour_param(arg, "s") {
            if s >= 0.0 && s <= 100.0 {
                colour_s = Some(s);
            } else {
                eprintln!("Saturation {s} out of range [0, 100.0]");
            }
        };
        if let Some(l) = parse_colour_param(arg, "l") {
            if l >= 0.0 && l <= 100.0 {
                colour_l = Some(l);
            } else {
                eprintln!("Lightness {l} out of range [0, 100.0]");
            }
        }
    }

    eprintln!("Saturation: {:?} Lightness: {:?}", colour_s, colour_l);

    let c = cpu_count()?;

    for i in 0..c {
        let info = cpu_info(i)?;
        let norm = normalise_cur_freq(&info) * 100.0;

        let c = cpu_freq_hsl(norm, colour_s, colour_l);
        let rgb = Rgb::from(&c);

        eprintln!(
            "CPU {i} Info: {info} Norm: {norm:.0}% RGB: {}",
            rgb.to_hex_string()
        );

        print!("#[bg={}]  ", rgb.to_hex_string());
    }
    println!("#[default]");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsl_to_rgb_red() {
        let hsl = Hsl::from((0.0, 100.0, 50.0));
        let rgb: [u8; 3] = Rgb::from(&hsl).into();
        assert_eq!((255, 0, 0), rgb.into());
    }

    #[test]
    fn test_hsl_to_rgb_green() {
        let hsl = Hsl::from((120.0, 100.0, 50.0));
        let rgb: [u8; 3] = Rgb::from(&hsl).into();
        assert_eq!((0, 255, 0), rgb.into());
    }

    #[test]
    fn test_normalise_cur_freq() {
        assert_eq!(
            0.0,
            normalise_cur_freq(&CPUInfo {
                min_freq: 100,
                max_freq: 200,
                cur_freq: 99
            })
        );
        assert_eq!(
            0.0,
            normalise_cur_freq(&CPUInfo {
                min_freq: 100,
                max_freq: 200,
                cur_freq: 100
            })
        );
        assert_eq!(
            0.5,
            normalise_cur_freq(&CPUInfo {
                min_freq: 100,
                max_freq: 200,
                cur_freq: 150
            })
        );
        assert_eq!(
            1.0,
            normalise_cur_freq(&CPUInfo {
                min_freq: 100,
                max_freq: 200,
                cur_freq: 201
            })
        );
    }
}
