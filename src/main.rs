use cubeos_service::*;
use log::{info, error};
use clap_derive::*;
use clap::*;

mod eps;
use eps::Eps;

#[derive(Parser, Debug, Clone, ValueEnum, PartialEq)]
enum StartStop {
    #[clap(name = "start")]
    Start,
    #[clap(name = "stop")]
    Stop,
}
impl From<StartStop> for isis_eps_api::BusChannel {
    fn from(start_stop: StartStop) -> Self {
        match start_stop {
            StartStop::Start => isis_eps_api::BusChannel::On,
            StartStop::Stop => isis_eps_api::BusChannel::Off,
        }
    }
}
impl std::fmt::Display for StartStop {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            StartStop::Start => "Start",
            StartStop::Stop => "Stop",
        })
    }
}

#[derive(Parser)]
struct Args {
    #[clap(default_value = "start")]
    start_stop: StartStop,
    channels: Option<Vec<u8>>,
}

fn main() {
    let _ = Logger::init();
    
    let args = Args::parse();

    let channels = match args.channels {
        Some(channels) => channels,
        None => vec![3],
    };
    for channel in channels {
        match Eps::set_single_output(args.start_stop.clone().into(), channel) {
            Ok(_) => info!("{} channel {}", args.start_stop.to_string(), channel),
            Err(e) => error!("Failed to {} channel {}: {:?}", args.start_stop.to_string(), channel, e),
        }
    }
}
