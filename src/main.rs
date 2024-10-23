extern crate clap;

use clap::{Arg, App, SubCommand};
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::prelude::*;
use std::io;
use std::process::Command;

const STATEFILE: &'static str = "/tmp/virtualserial";
const SOCATFILE: &'static str = "/tmp/virtualserial_socat";

struct State {
  pid: i32,
  master: String,
  slave: String,
  baud_rate: i32,
}

impl fmt::Debug for State {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(
      fmt,
      "State({}, {}, {}, {})",
      &self.pid.to_string(),
      &self.master,
      &self.slave,
      &self.baud_rate.to_string(),
    )
  }
}

fn get_user_id() -> i32 {
  let output = Command::new("id")
    .arg("-u")
    .output()
    .expect("failed to execute process");
  let output_string = String::from_utf8_lossy(&output.stdout);
  return output_string.trim().parse::<i32>().unwrap();
}

fn get_group_id() -> i32 {
  let output = Command::new("id")
    .arg("-g")
    .output()
    .expect("failed to execute process");
  let output_string = String::from_utf8_lossy(&output.stdout);
  return output_string.trim().parse::<i32>().unwrap();
}

fn kill_instance(pid: String) {
  Command::new("kill")
    .arg(pid.to_string())
    .output()
    .expect("failed to execute process");
}

fn stateline_to_instance(stateline: String) -> State {
  let line_items: Vec<String> = stateline.split(' ').map(String::from).collect();
  return State{
    pid: line_items[0].parse::<i32>().unwrap(),
    master: line_items[1].to_string(),
    slave: line_items[2].to_string(),
    baud_rate: line_items[3].parse::<i32>().unwrap(),
  };
}

fn get_state() -> Vec<State> {
  let mut file;
  if Path::new(STATEFILE).exists() {
    file = File::open(STATEFILE).expect("File not found");
  } else {
    file = File::create(STATEFILE).expect("Could not create file");
  }

  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("Could not read file");

  let lines: Vec<String> = contents.split('\n').map(String::from).filter(|x| x.len() > 0).collect();
  let instances: Vec<State> = lines.into_iter().map(stateline_to_instance).collect();

  return instances;
}

fn print_state(instances: Vec<State>) {
  println!("Current instances:");

  if instances.len() == 0 {
    println!("No instances yet. See `virtualserial help create`.");
  }

  for instance in instances {
    println!(
      "[{}], {} -> {} at baud {}",
      instance.pid, instance.master, instance.slave, instance.baud_rate
    );
  }
}

fn spawn_instance(master: String, slave: String, baud_rate: i32, user_id: i32, group_id: i32) -> u32 {
  let child = Command::new("socat")
    .args(&[
        "-d", "-d", "-d", "-d", "-lf", SOCATFILE,
        format!(
          "pty,link={},raw,echo=0,ispeed={},ospeed={},user={},group={}",
          master, baud_rate, baud_rate, user_id, group_id,
        ).as_str(),
        format!(
          "pty,link={},raw,echo=0,ispeed={},ospeed={},user={},group={}",
          slave, baud_rate, baud_rate, user_id, group_id,
        ).as_str(),
    ])
    .spawn()
    .expect("failed to execute child");
  return child.id();
}

fn create_instance(master: String, slave: String, baud_rate: i32, user_id: i32, group_id: i32) {
  let mut file = OpenOptions::new().append(true).open(STATEFILE).unwrap();

  let pid = spawn_instance(master.to_string(), slave.to_string(), baud_rate, user_id, group_id);

  let line = format!("{} {} {} {}", pid, master, slave, baud_rate);

  if let Err(e) = writeln!(file, "{}", line) {
    eprintln!("Couldn't write to file: {}", e);
  }

  println!("Created instance: {}", line);
}

fn remove_instance_from_file(pid: String) {
  let infile = File::open(STATEFILE).unwrap();
  let reader = io::BufReader::new(infile);

  let lines: Vec<String> = reader.lines()
    .map(|x| x.unwrap())
    .filter(|x| !x.starts_with(&pid))
    .collect();

  let mut outfile = File::create(STATEFILE).unwrap();

  for line in lines {
    writeln!(outfile, "{}", line).unwrap();
  }
}

fn remove_instance(pid: String) {
  remove_instance_from_file(pid.to_string());
  kill_instance(pid.to_string());
  println!("Instance killed: {}", pid);
}

fn main() {
  let matches = App::new("virtualserial")
    .version("0.1.0")
    .author("Vlad-Stefan Harbuz <vlad@vlad.website>")
    .about("Virtual serial ports for macOS.")
    .subcommand(
      SubCommand::with_name("show")
        .about("Shows current serial pairs.")
    )
    .subcommand(
      SubCommand::with_name("kill")
        .about("Kills a serial port pair instance.")
        .arg(
          Arg::with_name("pid")
            .required(true)
            .takes_value(true)
            .help("The PID of the instance to kill. Find it by using `virtualserial show`.")
        )
    )
    .subcommand(
      SubCommand::with_name("create")
        .about("Creates a new serial pair.")
        .arg(
          Arg::with_name("master_path")
            .required(true)
            .takes_value(true)
            .help("The path to create the master port at, e.g. ./master.")
        )
        .arg(
          Arg::with_name("slave_path")
            .required(true)
            .takes_value(true)
            .help(
              "The path to create the master port at, e.g. ./slave.")
        )
        .arg(
          Arg::with_name("baud_rate")
            .required(true)
            .takes_value(true)
            .help("The baud rate to use, e.g. 9600.")
        )
    )
    .get_matches();

  if let Some(_) = matches.subcommand_matches("show") {
    print_state(get_state());
  } else if let Some(matches) = matches.subcommand_matches("create") {
    let user_id = get_user_id();
    let group_id = get_group_id();
    let master_path = matches.value_of("master_path").unwrap().to_string();
    let slave_path = matches.value_of("slave_path").unwrap().to_string();
    let baud_rate = matches.value_of("baud_rate").unwrap().parse::<i32>().unwrap();
    create_instance(master_path, slave_path, baud_rate, user_id, group_id);
  } else if let Some(matches) = matches.subcommand_matches("kill") {
    let pid = matches.value_of("pid").unwrap().to_string();
    remove_instance(pid);
  }
}
