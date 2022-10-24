use crate::scripting_language::compiler::Compiler;
use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::program::Program;
use crate::scripting_language::value::Value;

pub mod constants;
use constants::*;
pub mod error;
use error::runtime::RuntimeError;
use error::Error;

/// The `VirtualMachine` type.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualMachine {
  /// Points at the next instruction to read.
  pub instruction_pointer: usize,
  /// The stack.
  pub stack: Vec<Value>,
}

impl VirtualMachine {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let instruction_pointer = 0;
    trace_var!(instruction_pointer);
    let stack = Vec::with_capacity(STACK_SIZE_MAX);
    trace_var!(stack);
    let result = Self {
      instruction_pointer,
      stack,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Interpret some source code.
  #[named]
  pub fn interpret(&mut self, source: &String) -> Result<Option<Value>, Error> {
    trace_enter!();
    trace_var!(source);
    self.instruction_pointer = 0;
    self.stack = Vec::with_capacity(STACK_SIZE_MAX);
    let program = self.compile(source)?;
    let result = self.run(&program)?;
    println!("{:#?}", program);
    println!("{:#?}", result);
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Compile the source code.
  #[named]
  pub fn compile(&mut self, source: &String) -> Result<Program, Error> {
    trace_enter!();
    trace_var!(source);
    let mut compiler = Compiler::default();
    trace_var!(compiler);
    let mut result = Program::default();
    compiler.compile(source, &mut result)?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Run the program.
  #[named]
  pub fn run(&mut self, program: &Program) -> Result<Option<Value>, Error> {
    trace_enter!();
    trace_var!(program);
    loop {
      let instruction_option = program.instructions.instructions.get(self.instruction_pointer);
      if instruction_option.is_none() {
        break;
      }
      let instruction = *instruction_option.unwrap();
      trace_var!(instruction);
      use Instruction::*;
      use Value::*;
      match instruction {
        Constant(index) => {
          let constant = program.constants.constants[index as usize];
          trace_var!(constant);
          self.push(constant)?;
        },
        LongConstant(index) => {
          let constant = program.constants.constants[index as usize];
          trace_var!(constant);
          self.push(constant)?;
        },
        Negate => {
          let pop = self.pop()?;
          trace_var!(pop);
          match pop {
            Number(pop) => {
              let answer = -pop;
              trace_var!(answer);
              self.push(Value::Number(answer))?;
            },
            _ => {
              return Err(Error::RuntimeError(RuntimeError::InappropriateOperand(Negate, pop)));
            },
          }
        },
        Add => self.binary_arithmetic_operation(Add, |a, b| b + a, Value::Number)?,
        Subtract => self.binary_arithmetic_operation(Subtract, |a, b| b - a, Value::Number)?,
        Multiply => self.binary_arithmetic_operation(Multiply, |a, b| b * a, Value::Number)?,
        Divide => self.binary_arithmetic_operation(Divide, |a, b| b / a, Value::Number)?,
        Equal => {
          let a = self.pop()?;
          trace_var!(a);
          let b = self.pop()?;
          trace_var!(b);
          use Value::*;
          match (a, b) {
            (Number(a), Number(b)) => self.push(Value::Boolean(a == b))?,
            (Boolean(a), Boolean(b)) => self.push(Value::Boolean(b == a))?,
            (_, _) => {
              return Err(Error::RuntimeError(RuntimeError::InappropriateOperands(
                instruction,
                b,
                a,
              )))
            },
          }
        },
        NotEqual => self.binary_arithmetic_operation(NotEqual, |a, b| b != a, Value::Boolean)?,
        GreaterThan => self.binary_arithmetic_operation(GreaterThan, |a, b| b > a, Value::Boolean)?,
        LessThan => self.binary_arithmetic_operation(LessThan, |a, b| b < a, Value::Boolean)?,
        GreaterThanOrEqual => self.binary_arithmetic_operation(GreaterThanOrEqual, |a, b| b >= a, Value::Boolean)?,
        LessThanOrEqual => self.binary_arithmetic_operation(LessThanOrEqual, |a, b| b <= a, Value::Boolean)?,
        Return => break,
        True => self.push(Value::Boolean(true))?,
        False => self.push(Value::Boolean(false))?,
        Instruction::Nil => self.push(Value::Nil)?,
        Not => {
          let value = self.pop()?;
          trace_var!(value);
          let answer = self.is_falsey(&value);
          self.push(Value::Boolean(answer))?;
        },
      }
      self.instruction_pointer += 1;
    }
    let result = self.pop().ok();
    println!("{:#?}", result);
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Binary arithmetic operator.
  #[named]
  #[inline]
  pub fn binary_arithmetic_operation<T>(
    &mut self,
    instruction: Instruction,
    function: fn(f64, f64) -> T,
    valuate: fn(T) -> Value,
  ) -> Result<(), Error> {
    trace_enter!();
    trace_var!(instruction);
    let a = self.pop()?;
    trace_var!(a);
    let b = self.pop()?;
    trace_var!(b);
    use Value::*;
    match (a, b) {
      (Number(a), Number(b)) => self.push(valuate(function(a, b)))?,
      (_, _) => {
        return Err(Error::RuntimeError(RuntimeError::InappropriateOperands(
          instruction,
          b,
          a,
        )))
      },
    }
    trace_exit!();
    Ok(())
  }

  /// Push a value to the stack.
  #[named]
  #[inline]
  pub fn push(&mut self, value: Value) -> Result<(), Error> {
    trace_enter!();
    trace_var!(value);
    if self.stack.len() > STACK_SIZE_MAX {
      return Err(Error::RuntimeError(RuntimeError::StackOverflow));
    }
    trace_var!(self.stack);
    self.stack.push(value);
    trace_exit!();
    Ok(())
  }

  /// Push a value to the stack.
  #[named]
  #[inline]
  pub fn pop(&mut self) -> Result<Value, Error> {
    trace_enter!();
    if self.stack.is_empty() {
      return Err(Error::RuntimeError(RuntimeError::StackUnderflow));
    }
    trace_var!(self.stack);
    let result = self.stack.pop().unwrap();
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Is this "falsey" or not?
  #[named]
  #[inline]
  pub fn is_falsey(&mut self, value: &Value) -> bool {
    trace_enter!();
    trace_var!(value);
    use Value::*;
    let result = match value {
      Nil => true,
      Boolean(value) => !value,
      _ => false,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_vm() {
    init();
    trace_enter!();
    use crate::scripting_language::value::Value::*;
    test_instructions!([Return], [] => []);
    test_instructions!([Return], [Number(53.0)] => [Number(53.0)]);
    test_instructions!([Negate], [Number(53.0)] => [Number(-53.0)]);
    test_instructions!([Negate], [Number(-53.0)] => [Number(53.0)]);
    test_instructions!([Add], [Number(-53.0), Number(4.0)] => [Number(-49.0)]);
    test_instructions!([Add], [Number(4.0), Number(-53.0)] => [Number(-49.0)]);
    trace_exit!();
  }
}
