pub enum ExecutionError{
    CommandNotFound(ExecutionErrorDetails),
}



pub struct ExecutionErrorDetails{
    pub error_code : u32,
    pub line: u32,
    pub column: u32,
}

impl ExecutionErrorDetails{
    pub fn new(error_code : u32, line: u32, column: u32) -> ExecutionErrorDetails{
        ExecutionErrorDetails{ error_code, line, column }
    }

    pub fn empty() -> ExecutionErrorDetails{
        ExecutionErrorDetails{ error_code : 0, line : 0, column : 0 }
    }

    pub fn make_error_code(error_code: u32) -> ExecutionErrorDetails{
        ExecutionErrorDetails{ error_code, line : 0, column : 0 }
    }
}