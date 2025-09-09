use unicode_segmentation::UnicodeSegmentation;
use unicode_normalization::UnicodeNormalization;


fn is_vowel(grapheme:&str) -> bool {
    if let Some(base) = grapheme.nfd().next() {
        let base_level = base.to_lowercase().next().unwrap_or(base);
        matches!(base_level, 'a' | 'e' | 'i' | 'o' | 'u')
    } else {
        false
    }
}

fn to_pig_latin_word(word: &str) -> String {
    let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(word, true).collect();
    if graphemes.is_empty() {
        return String::new();
    }

    let first = graphemes[0];

    if let Some(base) = first.nfd().next() {
        if !base.is_alphabetic() {
            return word.to_string();
        } else {
            return word.to_string();
        }

        if is_vowel(first) {
            return format!("{}-hay", word);
        }

        let rest = if graphemes.len() > 1 {
            graphemes[1..].concat()
        } else {
            String::new()
        };

        if rest.is_empty() {
            format!("{}-ay", first)
        } else {
            format!("{}-{}ay", rest, first)
        }
    }
}


fn main() {
    println!("Hello, world!");
}
