use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use chrono::NaiveDate;

fn read_lines(filename: String) -> io::Result<io::Lines<BufReader<File>>> {
    // Open the file in read-only mode.
    let file = File::open(filename)?;
    // Read the file line by line, and return an iterator of the lines of the file.
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
struct Line {
    date: chrono::NaiveDate,
    value: f64,
}
impl Line {
    pub fn from_csv_data(s: &str) -> Result<Self, String> {
        Line::from_str(s, ",")
    }

    pub fn from_input_text(s: &str) -> Result<Self, String> {
        Line::from_str(s, "|")
    }

    fn from_str(s: &str, split_by: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(split_by).map(|l| l.trim()).collect();
        if parts.len() < 2 {
            return Err(format!("Error: bad input => {}", s));
        }

        let date = parts.get(0).unwrap();
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| format!("Error: invalid date {}", date))?;

        let value = parts
            .get(1)
            .unwrap()
            .parse::<f64>()
            .map_err(|_| "Error: not a positive number")?;

        if value < 0.0 {
            return Err("Error: not a positive number".to_string());
        }

        Ok(Line { date, value })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(filename) = args.get(1) else {
        panic!("Error: could not open file.");
    };

    let Ok(lines) = read_lines(filename.to_string()) else {
        panic!("Error: could not open file.");
    };

    let Ok(csv_lines) = read_lines("assets/btc/data.csv".to_string()) else {
        panic!("Error: could not open data.csv file.");
    };

    let mut csv_lines: Vec<Line> = csv_lines
        .skip(1)
        .map(|line| Line::from_csv_data(line.unwrap().as_str()).unwrap())
        .collect();
    csv_lines.sort_by(|a, b| b.date.partial_cmp(&a.date).unwrap());
    let csv_lines = &csv_lines;

    lines
        .skip(1)
        .map(|line| Line::from_input_text(line.unwrap().as_str()))
        .for_each(|input_line| {
            let input_line = match input_line {
                Ok(l) => l,
                Err(err) => return println!("{err}"),
            };

            if input_line.value > 1000.0 {
                return println!("Error: too large a number");
            }
            if input_line.date < csv_lines.last().unwrap().date {
                return println!("Error: date too low => {}", input_line.date);
            }

            let csv_line = csv_lines
                .into_iter()
                .find(|csv_line| {
                    csv_line.date == input_line.date || csv_line.date < input_line.date
                })
                .unwrap();

            // solve rounding and trailing issue
            let total = (input_line.value * csv_line.value * 100_000_000.0).round() / 100_000_000.0;
            println!("{} => {} = {}", input_line.date, input_line.value, total)
        });
}
