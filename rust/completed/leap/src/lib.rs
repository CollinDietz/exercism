pub fn divisible_by(number: u64, divisor: u64) -> bool {
    return number % divisor == 0;
}

pub fn is_leap_year(year: u64) -> bool {
    divisible_by(year, 4)
        && (divisible_by(year, 100) && divisible_by(year, 400) || !divisible_by(year, 100))
}
