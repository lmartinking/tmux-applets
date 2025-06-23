use std::fmt;
use std::fs;

use nix::unistd;

#[derive(Debug, PartialEq)]
pub enum AppletError {
    CPUCountError,
    CPUInfoError,
}

impl std::error::Error for AppletError {}

impl fmt::Display for AppletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppletError: {:?}", self)
    }
}

type Result<T> = std::result::Result<T, AppletError>;

fn cpu_count() -> Result<u32>
{
    let count = match unistd::sysconf(unistd::SysconfVar::_NPROCESSORS_ONLN) {
        Ok(Some(c)) => c as u32,
        Ok(None) => return Err(AppletError::CPUCountError),
        Err(_) => return Err(AppletError::CPUCountError),
    };
    Ok(count)
}

#[derive(Debug, PartialEq)]
pub struct CPUInfo {
    pub min_freq: u32,
    pub max_freq: u32,
    pub cur_freq: u32,
}

fn read_u32_from_file(path: &str) -> Option<u32>
{
    let vstr = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => return None
    };

    let vstr = vstr.trim_end();

    match vstr.parse::<u32>() {
        Ok(v) => Some(v),
        Err(_) => return None
    }
}

fn cpu_info(cpu_index: u32) -> Result<CPUInfo>
{
    let min_freq_path = format!("/sys/bus/cpu/devices/cpu{cpu_index}/cpufreq/scaling_min_freq");
    let max_freq_path = format!("/sys/bus/cpu/devices/cpu{cpu_index}/cpufreq/scaling_max_freq");
    let cur_freq_path = format!("/sys/bus/cpu/devices/cpu{cpu_index}/cpufreq/scaling_cur_freq");

    match (read_u32_from_file(&min_freq_path), read_u32_from_file(&max_freq_path), read_u32_from_file(&cur_freq_path)) {
        (Some(min), Some(max), Some(cur)) => Ok(CPUInfo { min_freq: min, max_freq: max, cur_freq: cur }),
        _ => Err(AppletError::CPUInfoError)
    }
}

impl fmt::Display for CPUInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPUInfo: {:?}", self)
    }
}

fn main() -> Result<()> {
    let c = cpu_count()?;

    for i in 0..c {
        let info = cpu_info(i)?;
        println!("CPU {i} Info: {info}");
    }

    Ok(())
}
