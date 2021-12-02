use std::fs;

fn main() {
    let filename = "input/data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let sonar_readings: Vec<u32> = contents
        .split('\n')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut current_item: Option<u32> = None;
    let mut deeper_count = 0;

    for reading in sonar_readings {
        if let Some(current_item) = current_item {
            if reading > current_item {
                deeper_count += 1;
            }
        }
        current_item = Some(reading);
    }

    println!("Number of deeper readings {:?}", deeper_count);
}
