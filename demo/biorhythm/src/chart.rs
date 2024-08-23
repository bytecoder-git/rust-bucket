use chrono::{NaiveDate, Duration};
use crate::biorhythm::{calculate_biorhythm, format_biorhythm, get_biorhythm_char};

const CHART_WIDTH: usize = 72;
const BIORHYTHM_WIDTH: usize = 21;

pub fn print_chart(birth_date: NaiveDate, start_date: NaiveDate, end_date: NaiveDate, use_color: bool) {
    println!("\n{:^72}", "BIORHYTHM");
    println!("{:<10} {:^21} {:^21} {:^21}", "DATE", "PHYSICAL", "EMOTIONAL", "INTELLECTUAL");
    println!("{}", "-".repeat(CHART_WIDTH));

    let mut current_date = start_date;
    while current_date <= end_date {
        let days_since_birth = (current_date - birth_date).num_days() as f64;

        let physical = calculate_biorhythm(days_since_birth, 23.0);
        let emotional = calculate_biorhythm(days_since_birth, 28.0);
        let intellectual = calculate_biorhythm(days_since_birth, 33.0);

        let (color_code, reset_code) = if use_color {
            get_color_code(physical, emotional, intellectual)
        } else {
            ("", "")
        };

        println!("{}{} {} {} {}{}",
            color_code,
            current_date.format("%Y-%m-%d"),
            format_biorhythm_line(physical, 'P'),
            format_biorhythm_line(emotional, 'E'),
            format_biorhythm_line(intellectual, 'I'),
            reset_code
        );

        current_date += Duration::days(1);
    }
}

fn format_biorhythm_line(value: f64, label: char) -> String {
    let formatted = format_biorhythm(value, BIORHYTHM_WIDTH);
    let char_representation = get_biorhythm_char(value);
    formatted.replacen('|', &label.to_string(), 1)
        .replace('+', &char_representation.to_string())
        .replace('-', &char_representation.to_string())
}

fn get_color_code(physical: f64, emotional: f64, intellectual: f64) -> (&'static str, &'static str) {
    if is_triple_positive(physical, emotional, intellectual) {
        return ("\x1B[42m\x1B[30m", "\x1B[0m"); // Green background, black text
    }
    if is_triple_negative(physical, emotional, intellectual) {
        return ("\x1B[41m\x1B[30m", "\x1B[0m"); // Red background, black text
    }

    let positive_count = [physical, emotional, intellectual].iter().filter(|&&x| x > 0.0).count();
    
    match positive_count {
        0 => ("\x1B[31m", "\x1B[0m"), // Red text
        1 => ("\x1B[35m", "\x1B[0m"), // Pink text
        2 => ("\x1B[33m", "\x1B[0m"), // Yellow text
        3 => ("\x1B[32m", "\x1B[0m"), // Green text
        _ => unreachable!(),
    }
}

fn is_triple_positive(physical: f64, emotional: f64, intellectual: f64) -> bool {
    physical > 0.9 && emotional > 0.9 && intellectual > 0.9
}

fn is_triple_negative(physical: f64, emotional: f64, intellectual: f64) -> bool {
    physical < -0.9 && emotional < -0.9 && intellectual < -0.9
}