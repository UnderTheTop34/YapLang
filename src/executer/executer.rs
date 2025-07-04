use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use crate::executer::execution_error::{ExecutionError};
use crate::executer::commands::{command::Command, print_fn::print_function, panic_fn::panic_function, mkstr_fn::make_string_function, valc_fn::valc_function};
use crate::util::variable::Variable;
use crate::util::emoji_to_string::ARGUMENT_SEPERATOR_EMOJI;
use crate::util::variable_replacer::replace_variables_in_emoji_strings;

pub struct Executer {
    pub code: Vec<String>,
    pub output: Vec<String>,
    pub variables: HashMap<String, Variable>,
    pub current_scope: u16,
}

impl Executer {
    pub fn new(code: Vec<String>) -> Executer {
        Executer {code, output: vec![], variables: HashMap::new(), current_scope: 0 }
    }

    pub fn execute(&mut self, output_function: fn(String)) {
        for line in self.clone().code.iter() {
            let line = line.clone();
            let output_position = self.output.len();

            self.execute_line(line);

            if self.output.len() > output_position {
                for i in output_position..self.output.len() {
                    output_function(self.output[i].clone()); // The output function handles printing or visual display
                }
            }
        }
    }

    fn execute_line(&mut self, line: String) -> Option<ExecutionError>{
        let chars: Vec<&str> = UnicodeSegmentation::graphemes(line.as_str(), true).collect();
        if chars.len() == 0{
            return None;
        }

        let first_symbol = chars[0];

        let command_list: Vec<Command> = vec![
            print_function(),
            panic_function(),
            make_string_function(),
            valc_function(),

        ];


        // Prepare arguments (split them at the teeth emoji)
        let mut arguments: Vec<String> = Vec::new();
        let mut current_argument: Vec<String> = Vec::new();

        for char in chars {
            if char == ARGUMENT_SEPERATOR_EMOJI {
                arguments.push(current_argument.join("").clone());
                current_argument = Vec::new();
                continue;
            }
            current_argument.push(char.to_string());
        }

        arguments.push(current_argument.join("").clone());


        // Loop over all commands to find the right one
        for mut command in command_list {
            if first_symbol == command.name{
                // Remove the amount of chars used to specify the command from the first argument, as they're not really part of it
                arguments[0] = arguments[0].split_at(command.name.len()).1.to_string();

                // Replace variables in remaining arguments
                for i in 0..arguments.len(){
                    arguments[i] = replace_variables_in_emoji_strings(arguments[i].clone(), self.variables.clone());
                }
                // Execute said command & check for errors
                if let Some(error) = command.execute(self, arguments){
                    return Some(error);
                }
                break;
            }
        }

        None
    }


}

impl Clone for Executer {
    fn clone(&self) -> Executer {
        Executer {code: self.code.clone(), output: self.output.clone(), variables: self.variables.clone(), current_scope: self.current_scope.clone()}
    }
}

#[cfg(test)]
mod tests {
    use crate::executer::executer::Executer;

    #[test]
    fn test_line_execute(){
        {
            let mut executer = Executer::new(vec![]);
            executer.execute(|x| do_nothing(x));
            assert_eq!(executer.output, Vec::<String>::new());
        }


        {
            let mut executer = Executer::new(vec!["🎺".to_string()]);
            executer.execute(|x| do_nothing(x));
            assert_eq!(executer.output, vec![""]);
        }

        {
            let mut executer = Executer::new(vec!["🎺⚽️".to_string(), "🎺".to_string()]);
            executer.execute(|x| do_nothing(x));
            assert_eq!(executer.output, vec!["s", ""]);
        }
    }

    fn do_nothing(_x: String) {

    }
}