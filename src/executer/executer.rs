use unicode_segmentation::UnicodeSegmentation;
use crate::executer::execution_error::{ExecutionError};
use crate::executer::commands::{command::Command, print_fn::print_function};


pub struct Executer {
    pub code: Vec<String>,
    pub output: Vec<String>,
}

impl Executer {
    pub fn new(code: Vec<String>) -> Executer {
        Executer {code, output: vec![] }
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
            print_function()
        ];

        // Loop over all commands to find the right one
        for mut command in command_list {
            if first_symbol == command.name{
                // Execute said command & check for errors
                if let Some(error) = command.execute(self, vec![chars[1..].join("")]){
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
        Executer {code: self.code.clone(), output: self.output.clone() }
    }
}

#[cfg(test)]
mod tests {
    use crate::executer::executer::Executer;

    #[test]
    fn test_line_execute(){
        {
            let mut executer = Executer::new(vec![]);
            executer.execute({ |x| do_nothing(x) });
            assert_eq!(executer.output, Vec::<String>::new());
        }


        {
            let mut executer = Executer::new(vec!["🎺".to_string()]);
            executer.execute({|x| do_nothing(x) });
            assert_eq!(executer.output, vec![""]);
        }

        {
            let mut executer = Executer::new(vec!["🎺⚽️".to_string(), "🎺".to_string()]);
            executer.execute({|x| do_nothing(x) });
            assert_eq!(executer.output, vec!["s", ""]);
        }
    }

    fn do_nothing(x: String) {

    }
}