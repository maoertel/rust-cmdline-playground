use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

use regex::{Regex, Match};

use crate::input_parameters::InputParams;

#[derive(Debug)]
pub struct Processor {
  pub input_params: InputParams
}

impl Processor {
  pub fn run(&self) {
    match &self.input_params {
      InputParams { regex, input: Some(inp), with_line_numbers } => {
        let file = File::open(inp).unwrap();
        let reader = BufReader::new(file);
        process(reader, regex.clone(), &get_matching_lines, *with_line_numbers)
      }
      InputParams { regex, input: None, with_line_numbers } => {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process(reader, regex.clone(), &get_matching_lines, *with_line_numbers)
      }
    };
  }
}

fn process<T: BufRead + Sized>(
  reader: T,
  regex: Regex,
  f: &dyn Fn(T, Regex) -> Vec<(usize, String)>,
  with_line_numbers: bool,
) {
  for (num, line) in f(reader, regex) {
    if with_line_numbers { println!("{} {}", num, line) } else { println!("{}", line) }
  }
}

fn get_matching_lines<T: BufRead + Sized>(reader: T, regex: Regex) -> Vec<(usize, String)> {
  reader
    .lines()
    .enumerate()
    .map(|(line_number, maybe_line)| {
      let line = maybe_line.unwrap();
      match regex.find(&line) {
        Some(_) => Some((line_number, line)),
        None => None
      }
    })
    .filter(|it| it.is_some())
    .map(|it| it.unwrap())
    .collect()
}