use clap::ArgMatches;
use regex::Regex;

#[derive(Debug)]
pub struct InputParams {
  pub regex: Regex,
  pub input: Option<String>,
  pub with_line_numbers: bool,
}

pub struct ParameterParser {
  pub(crate) args: ArgMatches
}

impl ParameterParser {
  pub fn get_input_params(&self) -> InputParams {
    let pattern = self.args.value_of("pattern").unwrap();
    let regex = Regex::new(pattern).unwrap();
    let input = self.args.value_of("input").map(|it| String::from(it));
    let with_line_numbers = self.args.is_present("line_numbers");

    InputParams { regex, input, with_line_numbers }
  }
}
