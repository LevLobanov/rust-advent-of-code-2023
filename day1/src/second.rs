use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>>{
    let input_file= File::open("src/input.txt")?;
    let input_file_buffer = BufReader::new(input_file);

    let str_variants = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    let mut calibration_values_sum = 0;
    for fline_res in input_file_buffer.lines() {
        let fline = fline_res?;
        let mut calibration_value = 0;

        'start_blk: for i in 0..fline.len() {
            for (var_i, str_var) in str_variants.iter().enumerate() {
                if fline[i..].starts_with(str_var) {
                    calibration_value += ((var_i % 9) + 1) * 10;
                    break 'start_blk;
                }
            }
        }
        'end_blk: for i in 0..fline.len() {
            for (var_i, str_var) in str_variants.iter().enumerate() {
                if fline[..(fline.len() - i)].ends_with(str_var) {
                    calibration_value += (var_i % 9) + 1;
                    break 'end_blk;
                }
            }
        }
        calibration_values_sum += calibration_value;
    }

    println!("Total sum of calibration values: {}", calibration_values_sum);
    Ok(())
}