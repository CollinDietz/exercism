fn divisible_by(n: u32, d: u32) -> bool {
    n % d == 0
}

pub fn raindrops(n: u32) -> String {
    let answer = format!(
        "{}{}{}",
        if divisible_by(n, 3) { "Pling" } else { "" },
        if divisible_by(n, 5) { "Plang" } else { "" },
        if divisible_by(n, 7) { "Plong" } else { "" },
    );

    if answer.is_empty() {
        n.to_string()
    } else {
        answer
    }
}
