extern crate clap;

use clap::{App, load_yaml};

use processor::Processor;

use crate::input_parameters::ParameterParser;

mod processor;
mod input_parameters;

fn main() {
  let yaml = load_yaml!("cli.yml");
  let arguments = App::from(yaml).get_matches();

  let parser = ParameterParser { args: arguments };
  let input_params = parser.get_input_params();
  let processor = Processor { input_params };

  processor.run();
}
