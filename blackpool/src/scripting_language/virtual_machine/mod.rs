use std::collections::hash_map::Entry;
use std::fmt::Debug;

use crate::scripting_language::chunk::Chunk;
use crate::scripting_language::closure::Closure;
use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::garbage_collection::trace::formatter::Formatter as TraceFormatter;
use crate::scripting_language::garbage_collection::trace::Trace;
use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::interpreter::Interpreter;
use crate::scripting_language::table::Table;
use crate::scripting_language::value::Value;

pub mod call_frame;
use call_frame::CallFrame;
pub mod constants;
use constants::*;
pub mod error;
use error::runtime::RuntimeError;
use error::Error;

/// The `VirtualMachine` type.
#[derive(Debug)]
pub struct VirtualMachine {
  /// The callframes.
  pub call_frames: Vec<CallFrame>,
  /// The stack.
  pub stack: Vec<Value>,
  /// The garbage collector.
  pub garbage_collector: GarbageCollector,
  /// Global variables.
  pub globals: Table,
  /// The last thing popped from the stack.
  pub last_pop: Option<Value>,
  /// The reference to the initializer.
  pub init_string: Reference<String>,
}

impl VirtualMachine {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let call_frames = Vec::with_capacity(CALL_FRAMES_MAX);
    trace_var!(call_frames);
    let stack = Vec::with_capacity(STACK_SIZE_MAX);
    trace_var!(stack);
    let mut garbage_collector = GarbageCollector::new();
    trace_var!(garbage_collector);
    let globals = Table::new();
    trace_var!(globals);
    let last_pop = None;
    trace_var!(last_pop);
    let init_string = garbage_collector.intern("main".to_owned());
    trace_var!(init_string);
    let result = Self {
      call_frames,
      stack,
      garbage_collector,
      globals,
      last_pop,
      init_string,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Interpret some source code.
  #[named]
  pub fn interpret(&mut self, source: &str) -> Result<(), Error> {
    trace_enter!();
    trace_var!(source);
    let mut interpreter = Interpreter::default();
    trace_var!(interpreter);
    let function = interpreter.compile(source, &mut self.garbage_collector)?;
    trace_var!(function);
    let closure = self.alloc(Closure::new(function));
    trace_var!(closure);
    self.call_frames.push(CallFrame::new(closure, 0));
    self.run()?;
    trace_exit!();
    Ok(())
  }

  /// Run the chunk.
  #[named]
  pub fn run(&mut self) -> Result<(), Error> {
    trace_enter!();
    loop {
      let instruction =
        self.get_current_chunk().instructions.instructions[self.get_current_frame().instruction_pointer];
      self.get_current_frame_mut().instruction_pointer += 1;
      trace_var!(instruction);
      use Instruction::*;
      use Value::*;
      debug_var!(self.stack);
      debug_var!(self.get_current_frame().instruction_pointer);
      debug_var!(instruction);
      match instruction {
        Constant(index) => {
          let constant = self.get_current_chunk().constants.constants[index as usize];
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
        Add => {
          let a = self.pop()?;
          trace_var!(a);
          let b = self.pop()?;
          trace_var!(b);
          use Value::*;
          match (&a, &b) {
            (Number(a), Number(b)) => self.push(Value::Number(a + b))?,
            (String(a), String(b)) => {
              let b = self.garbage_collector.deref(*b);
              let a = self.garbage_collector.deref(*a);
              let result = format!("{}{}", b, a);
              let result = self.intern(result);
              let value = Value::String(result);
              self.push(value)?;
            },
            (_, _) => {
              return Err(Error::RuntimeError(RuntimeError::InappropriateOperands(
                instruction,
                b,
                a,
              )))
            },
          }
        },
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
            (String(a), String(b)) => self.push(Value::Boolean(a == b))?,
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
        Pop => {
          self.pop()?;
        },
        Print => {
          let value = self.pop()?;
          trace_var!(value);
          let formatter = TraceFormatter::new(value, &self.garbage_collector);
          println!("{}", formatter);
        },
        Not => {
          let value = self.pop()?;
          trace_var!(value);
          let answer = self.is_falsey(&value);
          trace_var!(answer);
          self.push(Value::Boolean(answer))?;
        },
        DefineGlobal(index) => {
          let identifier = self.get_current_chunk().read_string(index);
          trace_var!(identifier);
          let value = self.pop()?;
          trace_var!(value);
          self.globals.insert(identifier, value);
        },
        GetGlobal(index) => {
          let identifier = self.get_current_chunk().read_string(index);
          trace_var!(identifier);
          match self.globals.get(&identifier) {
            Some(&value) => self.push(value)?,
            None => {
              let identifier = self.garbage_collector.deref(identifier);
              return Err(Error::RuntimeError(RuntimeError::UndefinedVariable(
                identifier.to_string(),
              )));
            },
          }
        },
        SetGlobal(index) => {
          let identifier = self.get_current_chunk().read_string(index);
          let value = self.peek(0)?;
          if let Entry::Occupied(mut entry) = self.globals.entry(identifier) {
            entry.insert(value);
          } else {
            return Err(Error::RuntimeError(RuntimeError::UndefinedVariable(
              identifier.to_string(),
            )));
          }
        },
        GetLocal(index) => {
          let value = self.stack[index as usize];
          self.push(value)?;
        },
        SetLocal(index) => {
          let i = index as usize;
          let value = self.peek(0)?;
          self.stack[i] = value;
        },
        Jump(offset) => {
          self.get_current_frame_mut().instruction_pointer += offset as usize;
        },
        JumpIfFalse(offset) => {
          if self.peek(0)?.is_falsey() {
            debug!("branch taken");
            self.get_current_frame_mut().instruction_pointer += offset as usize;
          }
        },
        Loop(offset) => {
          self.get_current_frame_mut().instruction_pointer -= offset as usize;
        },
      }
    }
    debug_var!(self.stack);
    trace_exit!();
    Ok(())
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
    self.last_pop = Some(result);
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Peek at a value on the stack.
  #[named]
  #[inline]
  pub fn peek(&self, offset: usize) -> Result<Value, Error> {
    trace_enter!();
    trace_var!(offset);
    if self.stack.is_empty() {
      return Err(Error::RuntimeError(RuntimeError::StackUnderflow));
    }
    let max_index = self.stack.len() - 1;
    trace_var!(max_index);
    let index = max_index - offset;
    trace_var!(index);
    let result = self.stack[index];
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

  /// Allocate an object.
  #[named]
  pub fn alloc<T: Trace + 'static + Debug>(&mut self, object: T) -> Reference<T> {
    trace_enter!();
    trace_var!(object);
    self.mark_and_sweep();
    let result = self.garbage_collector.alloc(object);
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Eliminates duplicate string references.
  #[named]
  pub fn intern(&mut self, name: String) -> Reference<String> {
    trace_enter!();
    trace_var!(name);
    self.mark_and_sweep();
    let result = self.garbage_collector.intern(name);
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Mark and sweep GC.
  ///
  /// As the name implies, mark-sweep works in two phases:
  ///
  /// Mark: We start with the roots and traverse or trace through all of the
  /// objects those roots refer to. This is a classic graph traversal of all of
  /// the reachable objects. Each time we visit an object, we mark it in some
  /// way.
  ///
  /// Sweep: Once the mark phase completes, every reachable object in the heap
  /// has been marked. That means any unmarked object is unreachable and ripe
  /// for reclamation. We go through all the unmarked objects and free each
  /// one.
  ///
  /// @see https://craftinginterpreters.com/garbage-collection.html
  #[named]
  pub fn mark_and_sweep(&mut self) {
    trace_enter!();
    if self.garbage_collector.should_collect() {
      debug!("Beginning garbage collection.");
      self.mark_roots();
      self.garbage_collector.collect_garbage();
      debug!("Concluding garbage collection.");
    }
    trace_exit!();
  }

  /// Mark roots.
  #[named]
  fn mark_roots(&mut self) {
    trace_enter!();
    // Mark everything on the stack as a root object.
    debug!("marking {} values on stack for garbage collection", self.stack.len());
    for &value in &self.stack {
      debug!("marking value {:#?} on stack", value);
      self.garbage_collector.mark_value(value);
    }
    debug!("marking values frame for garbage collection");
    for &frame in &self.call_frames {
      self.garbage_collector.mark_object(frame.closure)
    }
    /*
    debug!("marking upvalues for garbage collection");
    for &upvalue in &self.open_upvalues {
        self.garbage_collector.mark_object(upvalue);
    }
    */
    debug!("marking {} global variables for garbage collection", self.globals.len());
    self.garbage_collector.mark_table(&self.globals);
    self.garbage_collector.mark_object(self.init_string);
  }

  /// Get current frame.
  #[named]
  pub fn get_current_frame(&self) -> &CallFrame {
    trace_enter!();
    let result = self.call_frames.last().unwrap();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Get current frame mutable.
  #[named]
  pub fn get_current_frame_mut(&mut self) -> &mut CallFrame {
    trace_enter!();
    let result = self.call_frames.last_mut().unwrap();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Get current closure.
  #[named]
  pub fn get_current_closure(&self) -> &Closure {
    trace_enter!();
    let closure = self.get_current_frame().closure;
    trace_var!(closure);
    let result = self.garbage_collector.deref(closure);
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Get current chunk.
  #[named]
  pub fn get_current_chunk(&self) -> &Chunk {
    trace_enter!();
    let closure = self.get_current_closure();
    trace_var!(closure);
    let function = self.garbage_collector.deref(closure.function);
    trace_var!(function);
    let result = &function.chunk;
    trace_var!(result);
    trace_exit!();
    result
  }
}

impl Default for VirtualMachine {
  #[named]
  fn default() -> Self {
    trace_enter!();
    let result = Self::new();
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

  #[named]
  #[test]
  pub fn test_vm2() {
    init();
    trace_enter!();
    let mut vm = VirtualMachine::new();
    let line = "!(5 - 4 > 3 * 2 == !nil);".to_string();
    vm.interpret(&line).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(true));
    trace_exit!();
  }

  #[named]
  #[test]
  #[should_panic]
  pub fn test_vm3() {
    init();
    trace_enter!();
    let mut vm = VirtualMachine::new();
    let line = "invalid input".to_string();
    // Should panic.
    vm.interpret(&line).unwrap();
    trace_exit!();
  }

  #[named]
  #[test]
  pub fn test_vm4() {
    init();
    trace_enter!();
    let mut vm = VirtualMachine::new();
    vm.interpret(&"2 != 3;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(true));
    vm.interpret(&"2 > 3;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(false));
    vm.interpret(&"2 >= 3;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(false));
    vm.interpret(&"2 == 2;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(true));
    vm.interpret(&"2 == 3;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(false));
    vm.interpret(&"2 != 2;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(false));
    vm.interpret(&"!(2 > 3);".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(true));
    vm.interpret(&"!(2 >= 3);".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(true));
    vm.interpret(&"2 < 3;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(true));
    vm.interpret(&"2 <= 3;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(true));
    vm.interpret(&"2 - 3;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Number(-1.0));
    vm.interpret(&"3 - 2;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Number(1.0));
    vm.interpret(&"2 + 3;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Number(5.0));
    vm.interpret(&"3 + 2;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Number(5.0));
    vm.interpret(&"2 * -4;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Number(-8.0));
    vm.interpret(&"3 * 2;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Number(6.0));
    vm.interpret(&"-4 / 2;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Number(-2.0));
    vm.interpret(&"2 / 4;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Number(0.5));
    vm.interpret(&"nil;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Nil);
    vm.interpret(&"true;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(true));
    vm.interpret(&"false;".to_string()).unwrap();
    assert_eq!(vm.last_pop.unwrap(), Value::Boolean(false));
    trace_exit!();
  }
}
