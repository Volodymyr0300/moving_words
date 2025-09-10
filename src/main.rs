fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' |
        'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}

fn to_pig_latin_word(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();

    if chars.is_empty() {
        return String::new();
    }

    let first = chars[0];

    if is_vowel(first) {
        let mut result = String::from(word);
        result.push_str("-hay");
        return result;
    } else {
        let mut result = String::new();

        for i in 1..chars.len() {
            result.push(chars[i]);
        }

        result.push('-');
        result.push(first);
        result.push_str("ay");

        return result;

    }
}

fn to_pig_latin_text(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut results : Vec<String> = Vec::new();

    for word in words {
        results.push(to_pig_latin_word(word));
    }

    results.join(" ")
}

fn main() {
    let examples = [
        "first",
        "apple",
        "banana split",
        "rust language",
        "",
    ];

    for example in examples.iter() {
        println!("{} -> {}", example, to_pig_latin_text(example));
    }
}
