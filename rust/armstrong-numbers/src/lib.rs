trait DigitIterator {
    fn digits(self) -> impl Iterator<Item = u32>;
}

impl DigitIterator for u32 {
    fn digits(self) -> impl Iterator<Item = u32> {
        let mut n = self;
        std::iter::from_fn(move || {
            if n == 0 {
                None
            } else {
                let digit = n % 10;
                n /= 10;
                Some(digit)
            }
        })
    }
}

pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits: u32 = num.digits().count() as u32;

    num.digits().fold(0, |sum, num| sum + num.pow(num_digits)) == num
}
