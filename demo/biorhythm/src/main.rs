mod config;
mod date_utils;
mod biorhythm;
mod chart;

use chrono::{Local, Duration};
use std::process;
use std::io::{self, Write};

use crate::date_utils::{get_date, get_single_char};
use crate::chart::print_chart;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut config = config::parse_args(&args);

    loop {
        if config.birth_date.is_none() {
            config.birth_date = Some(get_date("Enter birth date (YYYY-MM-DD): ", None));
        }

        if config.interactive || config.start_date.is_none() {
            let default_start = Local::now().date_naive();
            config.start_date = Some(get_date("Enter start date for chart (YYYY-MM-DD): ", Some(default_start)));
        }

        if config.interactive || config.end_date.is_none() {
            print!("Enter number of days to chart [{}]: ", config.num_days);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if !input.is_empty() {
                config.num_days = input.parse().expect("Invalid number of days");
            }
            config.end_date = Some(config.start_date.unwrap() + Duration::days(config.num_days - 1));
        }

        print_chart(config.birth_date.unwrap(), config.start_date.unwrap(), config.end_date.unwrap(), config.use_color);

        if config.interactive {
            print!("Press 'r' to restart, <esc> to exit, any other key for next period: ");
            io::stdout().flush().unwrap();
            
            match get_single_char() {
                27 => process::exit(0), // <esc>
                114 => { // 'r'
                    config.start_date = None;
                    config.end_date = None;
                    continue;
                }
                _ => {
                    config.start_date = Some(config.end_date.unwrap() + Duration::days(1));
                    config.end_date = None;
                }
            }
        } else {
            break;
        }
    }
}
