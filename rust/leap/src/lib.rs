
pub fn is_leap_year(year: i32) -> bool {
    match year {
        y if y % 400 == 0 => true,
        y if y % 100 == 0 => false,
        y => y % 4 == 0,
    }
}
