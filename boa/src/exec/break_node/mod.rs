use super::{Executable, Interpreter, InterpreterState};
use crate::{builtins::value::Value, syntax::ast::node::Break, Result};

#[cfg(test)]
mod tests;

impl Executable for Break {
    fn run(&self, interpreter: &mut Interpreter) -> Result<Value> {
        interpreter.set_current_state(InterpreterState::Break(self.label().map(String::from)));

        Ok(Value::undefined())
    }
}
