use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::ParserError;

pub fn parser_v2() -> Result<(), ParserError> {
    let csv_path = Path::new("./color_srgb.csv");
    let mut file = File::open(csv_path)?;
    let mut csv = String::new();

    file.read_to_string(&mut csv)?;

    let csv_line: Vec<&str> = csv.lines().collect();

    #[derive(Debug)]
    struct Color<'a> {
        name: &'a str,
        hex: &'a str,
        rgb: &'a str,
    }

    for line in &csv_line {
        let mut starts_with_quote = false;
        let mut quote_number = 0;
        let mut index_of_first_quote: usize = 0;
        let mut index_of_last_quote: usize = 0;
        let mut first_index_assigned = true;
        let mut last_index_assigned = true;
        for (i, c) in line.chars().enumerate() {
            if c == '"' {
                starts_with_quote = true;
                quote_number += 1;
            }
            if quote_number == 1 && starts_with_quote == true && first_index_assigned {
                index_of_first_quote = i;
                first_index_assigned = false;
            }
            if quote_number == 2 && starts_with_quote == true && last_index_assigned {
                index_of_last_quote = i;
                last_index_assigned = false;
            }
        }
        println!(
            "index of first quote: {}, index of last quote: {}",
            index_of_first_quote, index_of_last_quote
        );
    }

    // println!("csv_line: {:#?}", csv_line);
    Ok(())
}
