use std::string::ToString;
use unicode_segmentation::UnicodeSegmentation;
use crate::executer::commands::command::Command;
use crate::util;
use crate::util::variable::Variable;
use crate::util::variable::VariableKind::String;

pub fn make_string_function() -> Command {
    Command::new("🧵".to_string(), 2, {|exec, args |
        {
            let name = args[0].to_string();
            let chars: Vec<&str> = UnicodeSegmentation::graphemes(args[1].as_str(), true).collect();
            let str_value = util::emoji_to_string::emoji_to_string(chars.join("").to_string());
            exec.variables.insert(name, Variable::new(String, exec.current_scope, true, str_value));
            return None;
        }
    })
}

#[cfg(test)]
mod test {
    use crate::executer::commands::mkstr_fn::make_string_function;
    use crate::executer::executer::Executer;
    use crate::util::variable::{Variable, VariableKind};

    #[test]
    fn test() {
        {
            let mut executer = Executer::new(Vec::new());
            let mut command = make_string_function();
            command.execute(&mut executer, vec!["😈".to_string(), "💚🆙🔚🐍🐍".to_string()]);
            assert_eq!(*executer.variables.get("😈").unwrap(), Variable::new(VariableKind::String, executer.current_scope, true, "guess".to_string()));
        }
    }
}