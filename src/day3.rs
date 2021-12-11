use crate::utils;

pub fn run() {
  let data: Vec<String> = utils::read_data("day_three");

  part_a_take2(&data);
  part_b(&data);
}

fn part_a_take2(data: &Vec<String>) {
  let row_length: u32 = data.first().unwrap().len() as u32;
  // threshold for considering the result to be "on"
  let threshold = data.len() / 2;

  // convert input data to decimal
  let data: Vec<u64> = data
    .iter()
    .map(|x| u64::from_str_radix(x, 2).unwrap())
    .collect();

  let binary_string: String = (0..row_length)
    .rev()
    .map(|position| data.iter().filter(|x| (*x & (1 << position)) != 0).count() > threshold)
    .map(|x| if x { "1" } else { "0" })
    .collect();

  let gamma_bits = u32::from_str_radix(&binary_string, 2).unwrap();
  let mask = u32::pow(2, row_length) - 1;
  let epsilon_bits = gamma_bits ^ mask;

  let power = gamma_bits * epsilon_bits;

  println!("Power: {}", power);
  assert_eq!(841526, power)
}

fn part_b(data: &Vec<String>) {
  let row_length: u32 = data.first().unwrap().len() as u32;
  // threshold for considering the result to be "on"
  let threshold = data.len() / 2;

  // convert input data to decimal
  let data: Vec<u64> = data
    .iter()
    .map(|x| u64::from_str_radix(x, 2).unwrap())
    .collect();

  let mut remaining_values: Vec<u64> = data.clone();

  for position in (0..row_length).rev() {
    let on_count = remaining_values
      .iter()
      .filter(|x| (*x & (1 << position)) != 0)
      .count();
    let is_on = (on_count * 2) >= remaining_values.len();

    remaining_values.retain(|x| ((*x & (1 << position)) != 0) == is_on);

    if remaining_values.len() == 1 {
      break;
    }
  }

  let oxygen_generator = remaining_values[0];

  let mut remaining_values: Vec<u64> = data.clone();

  for position in (0..row_length).rev() {
    let on_count = remaining_values
      .iter()
      .filter(|x| (*x & (1 << position)) != 0)
      .count();

    let is_on = (on_count * 2) >= remaining_values.len();

    remaining_values.retain(|x| ((*x & (1 << position)) != 0) != is_on);

    if remaining_values.len() == 1 {
      break;
    }
  }

  let co2_scrubber = remaining_values[0];

  assert_eq!(4790390, co2_scrubber * oxygen_generator);
  println!("life support rating: {:?}", co2_scrubber * oxygen_generator)
}

// Lame naive first effort!
fn part_a(data: &Vec<String>) {
  let data: Vec<&str> = data.iter().map(AsRef::as_ref).collect();

  let bit_rows: Vec<Vec<u32>> = data
    .iter()
    .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
    .collect();

  let sum_rows: [u32; 12] = bit_rows
    .iter()
    .fold([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], |acc, x| {
      let mut new_val = acc;
      for (index, value) in acc.iter().enumerate() {
        new_val[index] = u32::from(x[index]) + value;
      }

      new_val
    });

  let threshold: u32 = (bit_rows.len() as u32) / 2;

  let gamma_bits = sum_rows
    .map(|x| x > threshold)
    .map(|x| if x { "1" } else { "0" })
    .join("");

  let epsilon_bits = sum_rows
    .map(|x| x < threshold)
    .map(|x| if x { "1" } else { "0" })
    .join("");

  let gamma_dec = isize::from_str_radix(&gamma_bits, 2).unwrap();
  let epsilon_dec = isize::from_str_radix(&epsilon_bits, 2).unwrap();
  let power = gamma_dec * epsilon_dec;

  println!(
    "result: {:?} g: {:?} {:?} e: {:?} {:?}, power: {:?}",
    sum_rows, gamma_bits, gamma_dec, epsilon_bits, epsilon_dec, power
  );

  assert_eq!(841526, power)
}
