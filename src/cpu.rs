use std::fmt;
use std::fs;

use nix::unistd;

use colorsys::Rgb;

use crate::common::{parse_colour_param, pct_value_hsl};

#[derive(Debug, PartialEq)]
pub enum CPUAppletError {
    CPUCount,
    CPUInfo,
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
        Ok(None) => return Err(CPUAppletError::CPUCount),
        Err(_) => return Err(CPUAppletError::CPUCount),
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

    vstr.parse::<u32>().ok()
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
    adj as f32 / range as f32
}

fn cpu_info(cpu_index: u32) -> Result<CPUInfo> {
    let min_freq_path = format!("/sys/bus/cpu/devices/cpu{cpu_index}/cpufreq/scaling_min_freq");
    let max_freq_path = format!("/sys/bus/cpu/devices/cpu{cpu_index}/cpufreq/scaling_max_freq");
    let cur_freq_path = format!("/sys/bus/cpu/devices/cpu{cpu_index}/cpufreq/scaling_cur_freq");

    match (read_u32_from_file(&min_freq_path), read_u32_from_file(&max_freq_path), read_u32_from_file(&cur_freq_path)) {
        (Some(min), Some(max), Some(cur)) => Ok(CPUInfo { min_freq: min, max_freq: max, cur_freq: cur }),
        _ => Err(CPUAppletError::CPUInfo),
    }
}

impl fmt::Display for CPUInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPUInfo: {:?}", self)
    }
}

pub fn applet(args: &[String]) -> Result<()> {
    let mut colour_s: Option<f32> = None;
    let mut colour_l: Option<f32> = None;

    for arg in args {
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

    eprintln!("Saturation: {:?} Lightness: {:?}", colour_s, colour_l);

    let c = cpu_count()?;

    for i in 0..c {
        let info = cpu_info(i)?;
        let norm = normalise_cur_freq(&info) * 100.0;

        let c = pct_value_hsl(norm, colour_s, colour_l);
        let rgb = Rgb::from(&c);

        eprintln!("CPU {i} Info: {info} Norm: {norm:.0}% RGB: {}", rgb.to_hex_string());

        print!("#[bg={}]  ", rgb.to_hex_string());
    }
    println!("#[default]");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalise_cur_freq() {
        assert_eq!(0.0, normalise_cur_freq(&CPUInfo { min_freq: 100, max_freq: 200, cur_freq: 99 }));
        assert_eq!(0.0, normalise_cur_freq(&CPUInfo { min_freq: 100, max_freq: 200, cur_freq: 100 }));
        assert_eq!(0.5, normalise_cur_freq(&CPUInfo { min_freq: 100, max_freq: 200, cur_freq: 150 }));
        assert_eq!(1.0, normalise_cur_freq(&CPUInfo { min_freq: 100, max_freq: 200, cur_freq: 201 }));
    }
}
