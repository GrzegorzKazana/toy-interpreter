pub fn split_str_at(input: &String, idx: usize) -> (String, String) {
    let head = input.chars().take(idx).collect();
    let tail = input.chars().skip(idx).collect();

    (head, tail)
}

pub fn split_str_trim(input: &String, idx: usize) -> (String, String) {
    let (head, tail) = split_str_at(input, idx);

    (head, tail.trim().to_string())
}
