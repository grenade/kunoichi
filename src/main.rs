extern crate daemon;

use daemon::{State,Daemon,DaemonRunner};
use std::env;
use std::fs::OpenOptions;
use std::io::{Error,Write};
use std::sync::mpsc::Receiver;

fn main() {
  log("application initialising...");
  let daemon = Daemon {
    name: "kunoichi".to_string()
  };
  let mut running = false;
  daemon.run(move |rx: Receiver<State>| {
    for signal in rx.iter() {
      match signal {
        State::Start => {
          service_start(&mut running);
        },
        State::Reload => {
          service_stop(&mut running);
          service_start(&mut running);
        },
        State::Stop => {
          service_stop(&mut running);
        }
      };
    }
  }).unwrap();
  log("application terminating...");
}

fn service_start(running: &mut bool) {
  match running {
    &mut true => log("service is already started."),
    &mut false => {
      log("service starting...");
      *running = true;
      // todo: start service and make a mess
      log("service started.");
    },
  }
}

fn service_stop(running: &mut bool) {
  match running {
    &mut true => {
      log("service stopping...");
      *running = false;
      // todo: stop service and clean up
      log("service stopped.");
    },
    &mut false => log("service is already stopped."),
  }
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