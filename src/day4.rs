use crate::utils;
use std::fmt;
use std::fs;
use std::str::FromStr;

pub fn run() {
  let data: Vec<String> = utils::read_data("day_four");

  // part_a(&data);
  // part_b(&data);

  parse(data);
}

fn parse(data: Vec<String>) {
  let numbers_drawn: Vec<u8> = data[0]
    .split(',')
    .map(|x| x.parse::<u8>().unwrap())
    .collect();

  println!("{:?}", numbers_drawn);

  let rows: Vec<Vec<u8>> = data
    .iter()
    .skip(1)
    .filter(|x| *x != "")
    .map(|r| {
      r.split_whitespace()
        .map(|x| x.parse::<u8>().unwrap())
        .collect()
    })
    .collect();

  let columns: Vec<Vec<u8>> = rows
    .clone()
    .chunks(5)
    .flat_map(|rows| (0..5).map(|position| rows.iter().map(|row| row[position]).collect()))
    .collect();

  println!("{:?}", columns);
}

// make a 5 element array from each row and column of each board
// map each number to the position it will appear in the sequence
// get the max position from each row
// pick the smallest one
// thats the winning row

// take all the indexes in the winning puzzle that are <= the index in last step
// map them back to the numbers
// calculate the value

// Board
// -> values: [[5]; 10]
// -> indexes: [[5]; 10]
