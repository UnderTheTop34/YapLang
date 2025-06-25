use emojis::Emoji;
use unicode_segmentation::UnicodeSegmentation;

pub fn emoji_to_string(emoji_string: String) -> String {
    let mut output = String::new();
    for emoji in split_after_emojis(&*emoji_string) {
        if let Some(emoji) = emojis::get(&*emoji){
            output += *emoji.shortcode();
        }else{
            output += &emoji;
        }

    }

    output
}


fn split_after_emojis(input: &str) -> Vec<String> {
    let graphemes = UnicodeSegmentation::graphemes(input, true).collect::<Vec<_>>();
    let mut result = Vec::new();
    let mut current = String::new();

    for g in graphemes {
        current.push_str(g);
        if is_emoji(g) {
            result.push(current.clone());
            current.clear();
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

// VERY simple emoji detection (refine as needed)
fn is_emoji(s: &str) -> bool {
    let first = s.chars().next().unwrap_or('\0');
    // Basic emoji unicode ranges
    (first >= '\u{1F300}' && first <= '\u{1FAD0}') // emojis & symbols
        || (first >= '\u{2600}' && first <= '\u{26FF}') // misc symbols
        || (first >= '\u{2700}' && first <= '\u{27BF}') // dingbats
}
