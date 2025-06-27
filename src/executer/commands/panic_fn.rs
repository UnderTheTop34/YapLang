use std::string::ToString;
use unicode_segmentation::UnicodeSegmentation;
use colorize;
use colorize::AnsiColor;
use std::process::exit;
use std::eprint;
use crate::executer::commands::command::Command;
use crate::util;

pub fn panic_function() -> Command {
    Command::new("⚽️".to_string(), 1, {|_, args |
        {
            let chars: Vec<&str> = UnicodeSegmentation::graphemes(args[0].as_str(), true).collect();
            let mut text = util::emoji_to_string::emoji_to_string(chars.join("").to_string());
            text = ("Not this again!\n".to_string() + text.as_str()).red().to_string();
            eprint!("{}", text);
            exit(101);
        }
    })
}
