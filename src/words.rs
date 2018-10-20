pub fn pig_latin(s: &str) -> String {
    let (first_char, remainder) = car_cdr(s);
    match first_char.chars().next() {
        Some(c) if is_vowel(c) => format!("{}-hay", s),
        Some(_) => format!("{}-{}ay", remainder, first_char),
        _ => s.to_string()
    }
}

fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'  => true,
        _ => false
    }
}
