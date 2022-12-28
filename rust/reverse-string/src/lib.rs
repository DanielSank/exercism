pub fn reverse(input: &str) -> String {
    let mut characters: Vec<char> = vec![];
    for ch in input.chars().rev() { characters.push(ch) }
    String::from_iter(characters)
}
