use std::{env, fmt};

use tmux_applets::{cpu, mem, ping};

#[derive(Debug)]
enum AppletError {
    MissingArgument,
    CPUApplet(cpu::CPUAppletError),
    MemApplet(mem::MemAppletError),
    PingApplet(ping::PingAppletError),
}

impl fmt::Display for AppletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppletError: {:?}", self)
    }
}

impl From<cpu::CPUAppletError> for AppletError {
    fn from(error: cpu::CPUAppletError) -> Self {
        AppletError::CPUApplet(error)
    }
}

impl From<mem::MemAppletError> for AppletError {
    fn from(error: mem::MemAppletError) -> Self {
        AppletError::MemApplet(error)
    }
}

impl From<ping::PingAppletError> for AppletError {
    fn from(error: ping::PingAppletError) -> Self {
        AppletError::PingApplet(error)
    }
}

impl std::error::Error for AppletError {}

type Result<T> = std::result::Result<T, AppletError>;

const USAGE: &str = "
usage: tmux-applets <applet> [<args>]

available applets:

    cpu: show cpu frequency usage

        optional parameters:
          s:XX.YY set the saturation (eg: s:50.0)
          l:XX.YY set the lightness  (eg: l:75.0)

    mem: show memory usage

        optional parameters:
          pct-text  show the memory usage percentage as text inside the box
          s:XX.YY   set the saturation (eg: s:50.0)
          l:XX.YY   set the lightness  (eg: l:75.0)

    ping: ping a host

        required parameters:
          <host>    the host to ping

        optional parameters:
          s:XX.YY   set the saturation (eg: s:50.0)
          l:XX.YY   set the lightness  (eg: l:75.0)

NOTE: Saturation and Lightness parameters must be in the range [0.0, 100.0]
";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{USAGE}");
        return Err(AppletError::MissingArgument);
    }

    match args[1].as_str() {
        "-h" | "--help" | "help" => {
            println!("{USAGE}");
            Ok(())
        }
        "mem" => Ok(mem::applet(&args[2..])?),
        "cpu" => Ok(cpu::applet(&args[2..])?),
        "ping" => Ok(ping::applet(&args[2..])?),
        _ => {
            println!("{USAGE}");
            Err(AppletError::MissingArgument)
        }
    }
}
