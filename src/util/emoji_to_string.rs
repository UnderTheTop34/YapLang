use emojis;
use unicode_segmentation::UnicodeSegmentation;

pub fn emoji_to_string(emoji_string: String) -> String {
    let mut output = String::new();
    let chars: Vec<&str> = UnicodeSegmentation::graphemes(emoji_string.as_str(), true).collect();
    for emoji in chars {
        if let Some(emoji) = emojis::get(&*emoji){
            if let Some(emoji_code) =  emoji.shortcode(){
                output.push(emoji_code.chars().nth(0).unwrap());
            }
        }else{
            output += &emoji;
        }

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
    }
}