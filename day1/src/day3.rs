use crate::utils;

pub fn run() {
  let data: Vec<String> = utils::read_data("day_three");

  part_a(&data);
  part_b(&data);
}

fn part_a(data: &Vec<String>) {
  let data: Vec<&str> = data.iter().map(AsRef::as_ref).collect();

  let bit_rows: Vec<Vec<u32>> = data
    .iter()
    .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
    .collect();

  let sum_rows: [u32; 10] = bit_rows
    .iter()
    .fold([0, 0, 0, 0, 0, 0, 0, 0, 0, 0], |acc, x| {
      let mut new_val = acc;
      for (index, value) in acc.iter().enumerate() {
        new_val[index] = u32::from(x[index]) + value;
      }

      new_val
    });

  let threshold: u32 = (bit_rows.len() as u32) / 2;

  let back_to_bits = sum_rows.map(|x| x > threshold);

  println!("{:?}", back_to_bits)
}

fn part_b(data: &Vec<String>) {}
