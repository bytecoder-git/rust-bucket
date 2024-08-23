use chrono::NaiveDate;
use std::io::{self, Write, Read};

pub fn get_date(prompt: &str, default: Option<NaiveDate>) -> NaiveDate {
    loop {
        print!("{}", prompt);
        if let Some(date) = default {
            print!("[{}] ", date.format("%Y-%m-%d"));
        }
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.is_empty() && default.is_some() {
            return default.unwrap();
        }

        match NaiveDate::parse_from_str(input, "%Y-%m-%d") {
            Ok(date) => return date,
            Err(_) => println!("Invalid date format. Please use YYYY-MM-DD."),
        }
    }
}

pub fn get_single_char() -> u8 {
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer).unwrap();
    buffer[0]
}