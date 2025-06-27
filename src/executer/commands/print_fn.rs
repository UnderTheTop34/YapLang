use unicode_segmentation::UnicodeSegmentation;
use crate::executer::commands::command::Command;
use crate::util;

pub fn print_function() -> Command {
    Command::new("🎺".to_string(), 1, {|executer, args |
        {
            let chars: Vec<&str> = UnicodeSegmentation::graphemes(args[0].as_str(), true).collect();
            let text = util::emoji_to_string::emoji_to_string(chars.join("").to_string());
            executer.output.push(text);

            return None;
        }
    })
}

#[cfg(test)]
mod test {
    use crate::executer::commands::print_fn::*;
    use crate::executer::executer::Executer;

    #[test]
    fn test_print_fn() {
        let mut executer = Executer::new(vec![]);
        let mut command = print_function();
        command.execute(&mut executer, vec!["🛕🔚🤟🤟🐙 🪟🐙💍🤟🥟".to_string()]);

        assert_eq!(executer.output, vec!["hello world".to_string()]);
    }
}
