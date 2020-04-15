pub fn split_str_at(input: &str, idx: usize) -> (String, String) {
    let head = input.chars().take(idx).collect();
    let tail = input.chars().skip(idx).collect();

    (head, tail)
}

pub fn split_str_trim(input: &str, idx: usize) -> (String, String) {
    let (head, tail) = split_str_at(input, idx);

    (head, tail.trim().to_string())
}

pub fn split_str_at_start(splitter: &str, input: &str) -> Option<(String, String)> {
    if input.starts_with(splitter) {
        Option::Some(split_str_trim(input, splitter.len()))
    } else {
        Option::None
    }
}

pub fn split_while_number(input: &str) -> Option<(String, String)> {
    let split_idx = input.chars().take_while(|c| c.is_digit(10)).count();

    if split_idx > 0 {
        Option::Some(split_str_trim(input, split_idx))
    } else {
        Option::None
    }
}

pub fn split_while_alphanumeric(input: &str) -> Option<(String, String)> {
    let split_idx = input.chars().take_while(|c| c.is_alphanumeric()).count();

    if split_idx > 0 {
        Option::Some(split_str_trim(input, split_idx))
    } else {
        Option::None
    }
}

pub fn flatten_result<T, U>(nested_result: Result<Result<T, U>, U>) -> Result<T, U> {
    nested_result.and_then(|x| x)
}
