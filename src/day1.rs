use crate::utils;

pub fn run() {
  let day_one_data = utils::read_data("day_one");
  day_one_part_one(&day_one_data);
  day_one_part_two(&day_one_data);
}

fn day_one_part_one(sonar_readings: &Vec<u32>) {
  let mut current_item: Option<u32> = None;
  let mut deeper_count = 0;

  for reading in sonar_readings {
    if let Some(current_item) = current_item {
      if *reading > current_item {
        deeper_count += 1;
      }
    }
    current_item = Some(*reading);
  }

  assert_eq!(1715, deeper_count);
  println!("Number of deeper readings {:?}", deeper_count);
}

fn day_one_part_two(sonar_readings: &Vec<u32>) {
  let mut deeper_count = 0;
  for (index, reading) in sonar_readings.iter().enumerate() {
    if let Some(comparator) = sonar_readings.get(index + 3) {
      if comparator > reading {
        deeper_count += 1;
      }
    }
  }
  println!("Number of deeper readings {:?}", deeper_count);
}
