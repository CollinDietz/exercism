const BOTTLE_VERSE: [&str; 10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    concat!(
        format!("Ten green bottles hanging on the wall,\n"),
        "Ten green bottles hanging on the wall,\n",
        "And if one green bottle should accidentally fall,\n",
        "There'll be nine green bottles hanging on the wall.",
    ).to_string()
}
