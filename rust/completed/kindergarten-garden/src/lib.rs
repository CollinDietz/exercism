const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

static PLANT_MAP: std::sync::LazyLock<std::collections::HashMap<char, &'static str>> =
    std::sync::LazyLock::new(|| {
        let mut map = std::collections::HashMap::new();
        map.insert('G', "grass");
        map.insert('C', "clover");
        map.insert('R', "radishes");
        map.insert('V', "violets");
        map
    });

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_index = STUDENTS.iter().position(|&s| s == student).unwrap();
    diagram
        .split('\n')
        .flat_map(|row| row.chars().skip(2 * student_index).take(2))
        .map(|f| PLANT_MAP[&f])
        .collect::<Vec<&str>>()
}
