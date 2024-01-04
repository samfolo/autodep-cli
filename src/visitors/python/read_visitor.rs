// use rustpython_parser::ast::{self, located::Located};

pub struct PythonModuleReadVisitor {
    function_calls: Vec<String>,
}

impl PythonModuleReadVisitor {
    pub fn new() -> Self {
        PythonModuleReadVisitor {
            function_calls: Vec::new(),
        }
    }

    pub fn function_calls(&self) -> &Vec<String> {
        &self.function_calls
    }
}

impl PythonModuleReadVisitor {}
