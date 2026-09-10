use error::ParserError;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod error;

fn main() -> Result<(), ParserError> {
    let csv_path = Path::new("./color_srgb.csv");
    let mut file = File::open(csv_path)?;
    let mut csv = String::new();

    file.read_to_string(&mut csv)?;

    let csv_data: Vec<&str> = csv.split_inclusive("\n").collect();

    #[derive(Debug)]
    struct Color {
        name: String,
        hex: String,
        rgb: String,
    }

    let mut names: Vec<Color> = Vec::new();

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
                new_data.push(new_line_without_quotes);
                new_line_without_quotes = "".to_string();
            } else if started_with_quote == true {
                if started_quote_number == 1 {
                    if c.to_string().as_str() != "\"" {
                        new_line_with_quotes += c.to_string().as_str();
                    }
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

        let color_data = Color {
            name: new_data[0].to_string(),
            hex: new_data[1].to_string(),
            rgb: new_data[2].to_string(),
        };

        names.push(color_data);
    }

    println!("names: {:#?}", names);

    Ok(())
}
