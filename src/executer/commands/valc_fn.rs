use unicode_segmentation::UnicodeSegmentation;
use crate::executer::commands::command::Command;
use crate::util;
use crate::util::variable::VariableKind;

// Value change function
pub fn valc_function() -> Command {
    Command::new("✉️"/*I chose a random symbol for this*/.to_string(), 2, {|executer, args |
        {
            let chars: Vec<&str> = UnicodeSegmentation::graphemes(args[1].as_str(), true).collect();

            if let Some(current_var) = executer.variables.get(args[0].as_str()){
                match current_var.kind {
                    VariableKind::String => {
                        let new_text = util::emoji_to_string::emoji_to_string(chars.join("").to_string());
                        let mut var_clone = current_var.clone();
                        var_clone.value = new_text.clone();

                        executer.variables.remove(args[0].as_str());
                        executer.variables.insert(args[0].to_string(), var_clone);
                    }
                    _ => {}
                }
            }
            return None
        }
    })
}

#[cfg(test)]
mod test {
    use crate::executer::commands::valc_fn::valc_function;
    use crate::executer::executer::Executer;

    #[test]
    fn test_valc() {
        {
            let mut exec = Executer::new(vec!["🧵😃🦷💚🆙🔚🐍🐍".to_string()]);
            let mut command = valc_function();

            exec.execute(| x | do_nothing(x));
            command.execute(&mut exec, vec!["😃".to_string(), "📈🛕🔚🤟🤟🐙".to_string()]);

            assert_eq!(exec.variables.get("😃").unwrap().value, "Hello");
        }
    }

    fn do_nothing(_arg1: String){

    }
}