const NUMBERS: [&str; 11] = [
    "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn get_strings_for_bottles_to_take_down(
    start_bottles: u32,
    take_down: u32,
) -> Option<&'static [&'static str]> {
    let end_bottles = start_bottles - take_down;
    NUMBERS.get(end_bottles as usize..1 + start_bottles as usize)
}

fn bottle_number_string_pair_to_verse(pair: (&&str, &&str)) -> String {
    let pair_0_plural = if *pair.0 == "One" { "" } else { "s" };
    let pair_1_plural = if *pair.1 == "One" { "" } else { "s" };
    format!(
        "{} green bottle{} hanging on the wall,\n\
        {} green bottle{} hanging on the wall,\n\
        And if one green bottle should accidentally fall,\n\
        There'll be {} green bottle{} hanging on the wall.",
        pair.0,
        pair_0_plural,
        pair.0,
        pair_0_plural,
        pair.1.to_lowercase(),
        pair_1_plural
    )
}

fn reversed_pairs<'a>(input: &'a [&'a str]) -> impl Iterator<Item = (&'a &'a str, &'a &'a str)> {
    input.iter().rev().zip(input.iter().rev().skip(1))
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    if let Some(bottle_strings_from_end_to_start) =
        get_strings_for_bottles_to_take_down(start_bottles, take_down)
    {
        reversed_pairs(bottle_strings_from_end_to_start)
            .map(bottle_number_string_pair_to_verse)
            .collect::<Vec<String>>()
            .join("\n\n")
    } else {
        "".to_string()
    }
}
