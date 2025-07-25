use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open("src/input.txt")?;
    let input_file_bufreader = BufReader::new(input_file);

    let mut sets_powers_sum = 0;

    for line in input_file_bufreader.lines() {
        let line = line?;
        let data_part = line.split(':').collect::<Vec<&str>>()[1];
        let ball_sequences: Vec<&str> = data_part.split(&[',', ';'][..]).collect();

        let mut red_minimum_balls: u32 = 0;
        let mut green_minimum_balls: u32 = 0;
        let mut blue_minimum_balls: u32 = 0;

        for seq in ball_sequences {
            let sequence_parts: Vec<&str> = seq.split(' ').collect();
            let amount = sequence_parts[1];
            let color = sequence_parts[2];
            match color {
                "red" => {
                    red_minimum_balls = max(red_minimum_balls, amount.parse::<u8>()?.into());
                },
                "green" => {
                    green_minimum_balls = max(green_minimum_balls, amount.parse::<u8>()?.into());
                },
                "blue" => {
                    blue_minimum_balls = max(blue_minimum_balls, amount.parse::<u8>()?.into());
                },
                _ => {}
            }
        }
        sets_powers_sum += red_minimum_balls * green_minimum_balls * blue_minimum_balls;
    }

    println!("Total sum of powers of all sets: {sets_powers_sum}");

    Ok(())
}