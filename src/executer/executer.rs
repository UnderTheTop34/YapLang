use crate::executer::ExecutionError::{ExecutionError, ExecutionErrorDetails};
macro_rules! nop { () => (); }

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
        if line.len() == 0{
            return None;
        }

        let first_symbol = line.chars().nth(0).unwrap();

        match first_symbol {
            '🎺' => {
                self.output.push("Would print something now.".to_string());
                return None;
            }
            _ => { Some(ExecutionError::CommandNotFound(ExecutionErrorDetails::new(1, 0, 0)))}
        }
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
            assert_eq!(executer.output, vec!["Would print something now."]);
        }

        {
            let mut executer = Executer::new(vec!["🎺".to_string(), "🎺".to_string()]);
            executer.execute({|x| do_nothing(x) });
            assert_eq!(executer.output, vec!["Would print something now.", "Would print something now."]);
        }
    }

    fn do_nothing(x: String) {

    }
}