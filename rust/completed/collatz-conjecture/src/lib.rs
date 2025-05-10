fn step(n: u64) -> u64 {
    if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
}

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        None
    } else {
        Some(
            std::iter::successors(Some(n), |&x| if x == 1 { None } else { Some(step(x)) }).count()
                as u64
                - 1,
        )
    }
}
