use regex::Regex;

#[derive(Debug)]
pub  struct InputParams {
  pub regex: Regex,
  pub input: Option<String>,
  pub with_line_numbers: bool,
}