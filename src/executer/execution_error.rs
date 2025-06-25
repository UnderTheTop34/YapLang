pub enum ExecutionError{
    #[allow(dead_code)]
    CommandNotFound(ExecutionErrorDetails),
}



pub struct ExecutionErrorDetails{
    #[allow(dead_code)]
    pub error_code : u32,
    #[allow(dead_code)]
    pub line: u32,
    #[allow(dead_code)]
    pub column: u32,
}

impl ExecutionErrorDetails{

    #[allow(dead_code)]
    pub fn new(error_code : u32, line: u32, column: u32) -> ExecutionErrorDetails{
        ExecutionErrorDetails{ error_code, line, column }
    }

    #[allow(dead_code)]
    pub fn empty() -> ExecutionErrorDetails{
        ExecutionErrorDetails{ error_code : 0, line : 0, column : 0 }
    }

    #[allow(dead_code)]
    pub fn make_error_code(error_code: u32) -> ExecutionErrorDetails{
        ExecutionErrorDetails{ error_code, line : 0, column : 0 }
    }
}