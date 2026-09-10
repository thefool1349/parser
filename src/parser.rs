use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::ParserError;

pub fn parser() -> Result<(), ParserError> {
    let csv_path = Path::new("./color_srgb.csv");
    let mut file = File::open(csv_path)?;
    let mut csv = String::new();

    file.read_to_string(&mut csv)?;

    // the file is empty
    if csv.is_empty() {
        return Err(ParserError::InvalidCsv);
    }

    let csv_data: Vec<&str> = if csv.contains("\r\n") {
        csv.split_inclusive("\r\n").collect()
    } else {
        csv.split_inclusive("\n").collect()
    };

    let lol: Vec<String> = csv_data
        .iter()
        .map(|s| {
            if s.contains("\r\n") {
                s.replace("\r\n", "")
            } else if s.contains("\n") {
                s.replace("\n", "")
            } else {
                s.replace("\n", "")
            }
        })
        .collect();

    let csv_data: Vec<&str> = lol.iter().map(|s| s.as_str()).collect();

    #[derive(Debug)]
    struct Color {
        name: String,
        hex: String,
        rgb: String,
    }

    let mut names: Vec<Color> = Vec::new();

    // the file contains only headers
    if csv_data.len() == 1 {
        return Err(ParserError::InvalidCsv);
    }

    let mut count = 0;
    for color in &csv_data {
        if count == 0 {
            count = count + 1;
            continue;
        }

        let mut new_data: Vec<String> = Vec::new();
        let mut new_line_without_quotes = String::new();
        let mut new_line_with_quotes = String::new();
        let mut started_with_quote = false;
        let mut started_quote_number: u8 = 0;
        for c in color.chars() {
            if c.to_string().as_str() == "\"" {
                started_with_quote = true;
                started_quote_number += 1;
            }
            if c.to_string().as_str() == "," && started_with_quote == false {
                if new_line_without_quotes.is_empty() {
                    return Err(ParserError::InvalidCsv);
                }
                new_data.push(new_line_without_quotes);

                new_line_without_quotes = "".to_string();
            } else if started_with_quote == true {
                if started_quote_number == 1 {
                    if c.to_string().as_str() != "\"" {
                        new_line_with_quotes += c.to_string().as_str();
                    }
                }
                if started_quote_number == 1 && new_line_with_quotes.contains("\n") {
                    return Err(ParserError::InvalidCsv);
                }
                if started_quote_number == 2 {
                    new_data.push(new_line_with_quotes);
                    new_line_with_quotes = "".to_string();
                    started_quote_number = 0;
                    started_with_quote = false;
                }
            }
            if c.to_string().as_str() != "," {
                new_line_without_quotes += c.to_string().as_str();
            }
        }

        // if the row does not have the length of 3
        if new_data.len() != 3 {
            return Err(ParserError::InvalidCsv);
        }

        let color_data = Color {
            name: new_data[0].to_string(),
            hex: new_data[1].to_string(),
            rgb: new_data[2].to_string(),
        };

        names.push(color_data);
    }

    // println!("names: {:#?}", names);

    Ok(())
}
