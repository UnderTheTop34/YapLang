use crate::executer::executer::Executer;
use crate::executer::execution_error::ExecutionError;

pub struct Command{
    pub name: String,
    pub args_count: u8,
    pub function: fn(executer: &mut Executer, arguments: Vec<String>) -> Option<ExecutionError>,
}

impl Command {
    pub fn new(name: String, amount_of_arguments: u8, f: fn(executer: &mut Executer, arguments: Vec<String>) -> Option<ExecutionError>) -> Self {
        Command {name, args_count: amount_of_arguments, function: f }
    }

    pub fn execute(&mut self, executer: &mut Executer, arguments: Vec<String>) -> Option<ExecutionError> {
        if arguments.len() != self.args_count as usize {
            panic!("Command {} must have exactly {} argument(s), but {} were found.", self.name, self.args_count, arguments.len());
        }
        (self.function)(executer, arguments)
    }

}

impl Clone for Command {
    fn clone(&self) -> Self {
        Command {name: self.name.clone(), args_count: self.args_count, function: self.function }
    }
}