extern crate daemon;

use daemon::{State,Daemon,DaemonRunner};
use std::env;
use std::fmt::{self};
use std::fs::OpenOptions;
use std::io::{Error,Write};
use std::sync::mpsc::Receiver;
use std::thread;

static SERVICE_NAME: &'static str = "kunoichi";

fn main() {
  log(&*format!("application {:?} initialising.", SERVICE_NAME));
  let daemon = Daemon {
    name: SERVICE_NAME.to_string()
  };
  let mut service_state = ServiceState::Stopped;
  daemon.run(move |rx: Receiver<State>| {
    for signal in rx.iter() {
      match signal {
        State::Start => {
          match service_state {
            ServiceState::Stopped | ServiceState::StartFailed => {
              service_state = ServiceState::Starting;
              log(&*format!("service {:?} {:?}", SERVICE_NAME, service_state));
              service_state = match start_service() {
                Ok(_) => ServiceState::Started,
                Err(_) => ServiceState::StartFailed,
              };
              log(&*format!("service {:?} {:?}", SERVICE_NAME, service_state));
            },
            ServiceState::Started => {
              log(&*format!("service {:?} is already in the {:?} state.", SERVICE_NAME, service_state));
            },
            _ => {
              log(&*format!("service {:?} is in the {:?} state, so cannot be started.", SERVICE_NAME, service_state));
            },
          }
        },
        State::Reload => {
          match service_state {
            ServiceState::Started | ServiceState::StopFailed => {
              service_state = ServiceState::Stopping;
              log(&*format!("service {:?} {:?}", SERVICE_NAME, service_state));
              service_state = match stop_service() {
                Ok(_) => ServiceState::Stopped,
                Err(_) => ServiceState::StopFailed,
              };
              log(&*format!("service {:?} {:?}", SERVICE_NAME, service_state));
            },
            ServiceState::Stopped => {
              log(&*format!("service {:?} is already in the {:?} state.", SERVICE_NAME, service_state));
            },
            _ => {
              log(&*format!("service {:?} is in the {:?} state, so cannot be stopped.", SERVICE_NAME, service_state));
            },
          }
          match service_state {
            ServiceState::Stopped | ServiceState::StartFailed => {
              service_state = ServiceState::Starting;
              log(&*format!("service {:?} {:?}", SERVICE_NAME, service_state));
              service_state = match start_service() {
                Ok(_) => ServiceState::Started,
                Err(_) => ServiceState::StartFailed,
              };
              log(&*format!("service {:?} {:?}", SERVICE_NAME, service_state));
            },
            ServiceState::Started => {
              log(&*format!("service {:?} is already in the {:?} state.", SERVICE_NAME, service_state));
            },
            _ => {
              log(&*format!("service {:?} is in the {:?} state, so cannot be started.", SERVICE_NAME, service_state));
            },
          }
        },
        State::Stop => {
          match service_state {
            ServiceState::Started | ServiceState::StopFailed => {
              service_state = ServiceState::Stopping;
              log(&*format!("service {:?} {:?}", SERVICE_NAME, service_state));
              service_state = match stop_service() {
                Ok(_) => ServiceState::Stopped,
                Err(_) => ServiceState::StopFailed,
              };
              log(&*format!("service {:?} {:?}", SERVICE_NAME, service_state));
            },
            ServiceState::Stopped => {
              log(&*format!("service {:?} is already in the {:?} state.", SERVICE_NAME, service_state));
            },
            _ => {
              log(&*format!("service {:?} is in the {:?} state, so cannot be stopped.", SERVICE_NAME, service_state));
            },
          }
        }
      };
    }
  }).unwrap();
  log(&*format!("application {:?} terminating.", SERVICE_NAME));
}

#[derive(Clone,Copy,Debug)]
enum ServiceState {
  Started,
  Starting,
  StartFailed,
  Stopped,
  Stopping,
  StopFailed,
}
impl fmt::Display for ServiceState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[allow(unused_must_use)]
fn start_service() -> Result<(), Error> {
  unimplemented!();
}

fn stop_service() -> Result<(), Error> {
  unimplemented!();
}

#[allow(unused_must_use)]
fn log(message: &str) {
  log_safe(message);
}

fn log_safe(message: &str) -> Result<(), Error> {
  println! ("{}", message);
  // todo: either add timestamp or (preferred) lose file in favour of eventlog(windows)/syslog(linux)
  let path = try! (env::current_exe()).with_extension("log");
  let mut file = try! (OpenOptions::new().create(true).write(true).append(true).open(&path));
  try! (file.write(message.as_bytes()));
  try! (file.write(b"\n"));
  Ok(())
}