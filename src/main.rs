extern crate daemon;

use daemon::{State,Daemon,DaemonRunner};
use std::env;
use std::fs::OpenOptions;
use std::io::{Error,Write};
use std::sync::mpsc::Receiver;

fn main() {
  log("application starting.");
  let daemon = Daemon {
    name: "kunoichi".to_string()
  };
  daemon.run(move |rx: Receiver<State>| {
    for signal in rx.iter() {
      match signal {
        State::Start => {
          service_start();
        },
        State::Reload => {
          service_stop();
          service_start();
        },
        State::Stop => {
          service_stop();
        }
      };
    }
  }).unwrap();
  log("application terminating.");
}

fn service_start() {
  log("service starting...");
  // todo: start kunoich if not running
  log("service started.");
}
fn service_stop() {
  log("service stopping...");
  // todo: stop kunoich if running
  log("service stopped.");
}

#[allow(unused_must_use)]
fn log(message: &str) {
  log_safe(message);
}

fn log_safe(message: &str) -> Result<(), Error> {
  println! ("{}", message);
  let path = try! (env::current_exe()).with_extension("log");
  let mut file = try! (OpenOptions::new().create(true).write(true).append(true).open(&path));
  try! (file.write(message.as_bytes()));
  try! (file.write(b"\n"));
  Ok(())
}