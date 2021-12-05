use crate::utils;

pub fn run() {
  let data: Vec<String> = utils::read_data("day_two");

  part_a(&data);
  part_b(&data);
}

fn part_a(data: &Vec<String>) {
  let mut depth: u32 = 0;
  let mut distance: u32 = 0;

  for instruction in data {
    let items: Vec<&str> = instruction.split(' ').collect();
    let command = items[0];
    let value: u32 = items[1].parse().unwrap();

    match command {
      "forward" => distance += value,
      "up" => depth -= value,
      "down" => depth += value,
      _ => panic!("unexpected command"),
    }
  }

  println!("position product: {}", depth * distance);
}

fn part_b(data: &Vec<String>) {
  let mut depth: u32 = 0;
  let mut distance: u32 = 0;
  let mut aim: u32 = 0;

  for instruction in data {
    let items: Vec<&str> = instruction.split(' ').collect();
    let command = items[0];
    let value: u32 = items[1].parse().unwrap();

    match command {
      "forward" => {
        distance += value;
        depth += aim * value;
      }
      "up" => aim -= value,
      "down" => aim += value,
      _ => panic!("unexpected command"),
    }
  }

  println!("position product: {}", depth * distance);
}
