use unicode_segmentation::unicode_segmentation;
use unicode_normalization::UnicodeNormalization;

fn is_vowel(grapheme:&str) -> bool {
    if let Some(base) = grapheme.ntd().next() () {
        let base_level = base.to_lowercase().next().unwrap_or(base);
        matches!(base_level, 'a' | 'e' | 'i' | 'o' | 'u')
    } else {
        false
    }
}


fn main() {
    println!("Hello, world!");
}
