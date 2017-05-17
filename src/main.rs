extern crate daemon;

use daemon::{State,Daemon,DaemonRunner};
use std::env;
use std::fs::OpenOptions;
use std::io::{Error,Write};
use std::sync::mpsc::Receiver;

fn main() {
  log("kunoichi application started.");
  let daemon = Daemon {
    name: "kunoichi".to_string()
  };
  daemon.run(move |rx: Receiver<State>| {
    log("kunoichi service started.");
    for signal in rx.iter() {
      match signal {
        State::Start => {
          log("kunoichi service: Start");
          // do stuff
        },
        State::Reload => {
          log("kunoichi service: Reload");
          // do stuff
        },
        State::Stop => {
          log("kunoichi service: Stop");
          // do stuff
        }
      };
    }
    log("kunoichi service finished.");
  }).unwrap();
  log("kunoichi application finished.");
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