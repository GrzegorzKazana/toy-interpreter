pub fn split_str_at(input: &String, idx: usize) -> (String, String) {
    let head = input.chars().take(idx).collect();
    let tail = input.chars().skip(idx).collect();

    (head, tail)
}

pub fn split_str_trim(input: &String, idx: usize) -> (String, String) {
    let (head, tail) = split_str_at(input, idx);

    (head, tail.trim().to_string())
}

pub fn partition<T, F: Fn(&T) -> bool>(arr: &[T], pred: F) -> Option<(&[T], &T, &[T])> {
    arr.iter()
        .rposition(pred)
        .map(|idx| (&arr[..idx], &arr[idx], &arr[idx + 1..]))
}

pub fn option_seq2<T, U>((o_a, o_b): (Option<T>, Option<U>)) -> Option<(T, U)> {
    if o_a.is_some() && o_b.is_some() {
        Option::Some((o_a.unwrap(), o_b.unwrap()))
    } else {
        Option::None
    }
}

pub fn option_seq3<T, U, V>(
    (o_a, o_b, o_c): (Option<T>, Option<U>, Option<V>),
) -> Option<(T, U, V)> {
    if o_a.is_some() && o_b.is_some() && o_c.is_some() {
        Option::Some((o_a.unwrap(), o_b.unwrap(), o_c.unwrap()))
    } else {
        Option::None
    }
}
