use std::fmt;
use std::fs;
use std::str::FromStr;

pub fn read_data<T: FromStr>(file: &str) -> Vec<T>
where
  <T as FromStr>::Err: fmt::Debug,
{
  let filename = format!("input/{}.txt", file);
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

  contents
    .split('\n')
    .map(|x| x.parse::<T>().unwrap())
    .collect()
}
