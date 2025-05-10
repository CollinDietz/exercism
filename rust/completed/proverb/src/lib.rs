pub fn build_proverb(list: &[&str]) -> String {
  if !list.is_empty() {
    list.windows(2).fold(String::new(), |acc, items| {
        acc + &format!("For want of a {} the {} was lost.\n", items[0], items[1])
    }) +&format!("And all for the want of a {}.", list[0])
  }
else { String::new()}
}
