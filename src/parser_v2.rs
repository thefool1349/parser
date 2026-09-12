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
    let mut parsed_data: Vec<Color> = Vec::new();
    let mut indx = 0;
    for line in &csv_line {
        if indx == 0 {
            indx += 1;
            continue;
        }

        let mut starts_with_quote = false;
        let mut quote_number = 0;
        let mut first_index_assigned = true;
        let mut last_index_assigned = true;
        let mut total_coma = 0;
        let mut first_coma_index = 0;
        let mut second_coma_index = 0;
        for (i, c) in line.chars().enumerate() {
            if c == '"' {
                starts_with_quote = true;
                quote_number += 1;
            }
            if quote_number == 1 && starts_with_quote == true && first_index_assigned {
                first_index_assigned = false;
            }
            if quote_number == 2 && starts_with_quote == true && last_index_assigned {
                last_index_assigned = false;
            }
            if c == ',' {
                total_coma += 1;
                if total_coma == 1 {
                    first_coma_index = i;
                }
                if total_coma == 2 {
                    second_coma_index = i;
                }
            }
        }
        let color = Color {
            name: &line[0..first_coma_index],
            hex: &line[(first_coma_index + 1)..second_coma_index],
            rgb: &line[(second_coma_index + 1)..],
        };

        parsed_data.push(color);
    }

    println!("parsed_data: {:#?}", parsed_data);
    Ok(())
}
