const PI: f64 = std::f64::consts::PI;

pub fn calculate_biorhythm(days: f64, cycle: f64) -> f64 {
    (2.0 * PI * days / cycle).sin()
}

pub fn format_biorhythm(value: f64, width: usize) -> String {
    let half_width = (width - 1) / 2;
    let position = (value * half_width as f64).round() as i32;
    let mut result = vec![' '; width];
    result[half_width] = '|';
    if position != 0 {
        let index = (half_width as i32 + position) as usize;
        result[index] = if value > 0.0 { '+' } else { '-' };
    }
    result.into_iter().collect()
}

pub fn get_biorhythm_char(value: f64) -> char {
    match value {
        x if x > 0.8 => '+',
        x if x > 0.6 => '+',
        x if x > 0.4 => '+',
        x if x > 0.2 => '+',
        x if x == 0.0 => 'o',
        x if x > -0.2 => '–',
        x if x > -0.4 => '-',
        x if x > -0.6 => '-',
        x if x > -0.8 => '-',
        _ => '-'
    }
}
