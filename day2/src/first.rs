use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open("src/input.txt")?;
    let input_file_bufreader = BufReader::new(input_file);

    let mut games_ids_sum = 0;

    'outer_for_blk: for (i, line) in input_file_bufreader.lines().enumerate() {
        let line = line?;
        let data_part = line.split(':').collect::<Vec<&str>>()[1];
        let ball_sequences: Vec<&str> = data_part.split(&[',', ';'][..]).collect();
        for seq in ball_sequences {
            let sequence_parts: Vec<&str> = seq.split(' ').collect();
            let amount = sequence_parts[1];
            let color = sequence_parts[2];
            match color {
                "red" => {
                    if amount.parse::<u8>()? > 12 {
                        continue 'outer_for_blk;
                    }
                },
                "green" => {
                    if amount.parse::<u8>()? > 13 {
                        continue 'outer_for_blk;
                    }
                },
                "blue" => {
                    if amount.parse::<u8>()? > 14 {
                        continue 'outer_for_blk;
                    }
                },
                _ => {}
            }
        }
        games_ids_sum += i + 1;
    }

    println!("Total sum of all possible games IDs: {games_ids_sum}");

    Ok(())
}