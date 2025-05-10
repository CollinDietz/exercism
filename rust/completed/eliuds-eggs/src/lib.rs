pub fn egg_count(display_value: u32) -> usize {
    (0..32)
        .filter(|&n| display_value & 2_u32.pow(n) != 0)
        .count()
}
