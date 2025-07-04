use unicode_segmentation::UnicodeSegmentation;
use rand::Rng;

pub fn remove_comments(text: String) -> String {
    let mut output: String = String::new();

    let random_insertions = ["🎺📈🐬🐙🛰🤓🐙🎾🛰🏃🌌☁️🍺\n", "🎺😃🤟🥟🛕🔚\n"];
    let mut line_number = 0;
    let mut rng = rand::rng();

    for l in text.lines() {
        if rng.random::<u8>() == 0 && line_number > 2{
            output += random_insertions[line_number % random_insertions.len()];
        }
        let mut last_char = "n";
        let mut line = "".to_string();
        for str in UnicodeSegmentation::graphemes(l, true).collect::<Vec<&str>>() {
            if last_char == "/" && str == "/" {
                line.remove(line.len() - 1);
                if line.is_empty() {
                    break;
                }
                output += line.as_str();
                if line_number != text.lines().count() - 1 {
                    output.push('\n');
                }
                line.clear();
                break;
            }
            last_char = str;
            line.push_str(str);
        }
        if !line.is_empty() {
            output += line.as_str();
            if line_number != text.lines().count() - 1 {
                output.push('\n');
            }
        }
        line_number += 1;
    }

    output
}


#[cfg(test)]
mod tests {
    use crate::util::remove_comments::remove_comments;

    #[test]
    fn test_remove_comments() {
        assert_eq!(remove_comments("// print(Hello, world)\n🎺📈🛕🔚🤟🤟🐙🛰📈🪟🐙💍🤟//🥟📣".to_string()).as_str(), "🎺📈🛕🔚🤟🤟🐙🛰📈🪟🐙💍🤟");
    }
}