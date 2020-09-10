extern crate clap;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

use clap::{App, Arg, ArgMatches};
use regex::Regex;

use input_parameters::InputParams;
use processor::Processor;

mod processor;
mod input_parameters;

fn main() {
  let args = get_args();

  let input_params = eval_input_params(args);

  let processor = Processor { input_params };
  processor.run();
}

fn get_args<'a>() -> ArgMatches<'a> {
  App::new("grep-lite")
    .version("0.1")
    .about("searches for patterns")
    .arg(
      Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true)
    )
    .arg(
      Arg::with_name("input")
        .help("File to search")
        .takes_value(true)
        .required(false)
    )
    .arg(
      Arg::with_name("line_numbers")
        .help("Output with line numbers")
        .short("l")
        .long("lines")
        .takes_value(false)
        .required(false)
    )
    .get_matches()
}

fn eval_input_params(args: ArgMatches) -> InputParams {
  let pattern = args.value_of("pattern").unwrap();
  let regex = Regex::new(pattern).unwrap();
  let input = args.value_of("input").map(|it| String::from(it));
  let with_line_numbers = args.is_present("line_numbers");

  InputParams { regex, input, with_line_numbers }
}
