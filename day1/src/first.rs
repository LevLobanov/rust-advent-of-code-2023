use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>>{
    let input_file= File::open("src/input.txt")?;
    let input_file_buffer = BufReader::new(input_file);

    let mut calibration_values_sum = 0;
    for fline_res in input_file_buffer.lines() {
        let fline = fline_res?;
        let mut calibration_value = 0;
        for start_char in fline.chars() {
            if start_char.is_digit(10) {
                calibration_value += start_char.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for end_char in fline.chars().rev() {
            if end_char.is_digit(10) {
                calibration_value += end_char.to_digit(10).unwrap();
                break;
            }
        }
        calibration_values_sum += calibration_value;
    }

    println!("Total sum of calibration values: {}", calibration_values_sum);
    Ok(())
}
