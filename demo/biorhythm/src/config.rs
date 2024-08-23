use chrono::NaiveDate;
use std::process;

pub struct Config {
    pub birth_date: Option<NaiveDate>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub num_days: i64,
    pub interactive: bool,
    pub use_color: bool,
}

pub fn parse_args(args: &[String]) -> Config {
    let mut config = Config {
        birth_date: None,
        start_date: None,
        end_date: None,
        num_days: 365,
        interactive: false,
        use_color: true,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--start_date" | "-s" => {
                config.start_date = Some(NaiveDate::parse_from_str(&args[i + 1], "%Y-%m-%d").expect("Invalid start date"));
                i += 2;
            }
            "--end_date" | "-e" => {
                config.end_date = Some(NaiveDate::parse_from_str(&args[i + 1], "%Y-%m-%d").expect("Invalid end date"));
                i += 2;
            }
            "--days" | "-d" => {
                config.num_days = args[i + 1].parse().expect("Invalid number of days");
                i += 2;
            }
            "--interactive" | "-i" => {
                config.interactive = true;
                i += 1;
            }
            "--no_color" | "-nc" => {
                config.use_color = false;
                i += 1;
            }
            _ => {
                println!("Unknown option: {}", args[i]);
                process::exit(1);
            }
        }
    }

    config
}