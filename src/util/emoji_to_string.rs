use std::string::ToString;
use emojis;
use unicode_segmentation::UnicodeSegmentation;

pub static UPPERCASE_EMOJI: &'static str = "📈";

pub fn emoji_to_string(emoji_string: String) -> String {
    let mut output = String::new();
    let chars: Vec<&str> = UnicodeSegmentation::graphemes(emoji_string.as_str(), true).collect();

    let mut capatialize_next_char = false;

    for emoji in chars {
        if emoji == UPPERCASE_EMOJI {
            if capatialize_next_char {
                panic!("Can't make uppercase identifier uppercase.")
            }
            capatialize_next_char = true;
            continue;
        }
        if let Some(emoji) = emojis::get(&*emoji){
            if let Some(char) =  emoji.shortcode().unwrap().chars().nth(0){
                let mut char = char;
                if capatialize_next_char {
                    char = char.to_ascii_uppercase();
                }
                output.push(char);
            }
        }else{
            output += &emoji;
        }
        // This can be done since continue exits the loop after the uppercase identifier.
        capatialize_next_char = false;
    }

    output
}


#[cfg(test)]
mod tests {
    use crate::util::emoji_to_string::emoji_to_string;
    #[test]
    fn test_emoji_to_string() {
        assert_eq!(emoji_to_string("📦✈️🐍🥟🧚🐍🧚".to_string()), "pasdfsf".to_string());
        assert_eq!(emoji_to_string("📦✈️🐍🥟k🐍 ".to_string()), "pasdks ".to_string());
        assert_eq!(emoji_to_string("📈📦📈✈️🐍📈🥟k🐍 ".to_string()), "PAsDks ".to_string());
    }
}