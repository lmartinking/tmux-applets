use std::fmt;
use std::fs;

#[derive(Debug, PartialEq)]
pub enum MemAppletError {
    MemInfoError,
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
    let data = fs::read_to_string(MEM_INFO_PATH).or(Err(MemAppletError::MemInfoError))?;

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

pub fn applet(_args: &[String]) -> Result<()> {
    let info = read_meminfo()?;

    eprintln!("Mem Info: {:?}", info);

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
