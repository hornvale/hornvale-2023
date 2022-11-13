use cpu_time::ProcessTime;
use std::collections::hash_map::Entry;
use std::fmt::Debug;

use crate::scripting::bound_method::BoundMethod;
use crate::scripting::chunk::Chunk;
use crate::scripting::class::Class as ClassStruct;
use crate::scripting::closure::upvalue::Upvalue;
use crate::scripting::closure::Closure as ClosureStruct;
use crate::scripting::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting::garbage_collection::reference::Reference;
use crate::scripting::garbage_collection::trace::formatter::Formatter as TraceFormatter;
use crate::scripting::garbage_collection::trace::Trace;
use crate::scripting::instance::Instance;
use crate::scripting::instruction::Instruction;
use crate::scripting::interpreter::Interpreter;
use crate::scripting::native_function::NativeFunction;
use crate::scripting::standard_library;
use crate::scripting::table::Table;
use crate::scripting::value::Value;

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
  /// The reference to the initializer.
  pub init_string: Reference<String>,
  /// Open upvalues.
  pub open_upvalues: Vec<Reference<Upvalue>>,
  /// Start time.
  pub start_time: ProcessTime,
}

impl VirtualMachine {
  /// Constructor.
  pub fn new() -> Self {
    let call_frames = Vec::with_capacity(CALL_FRAMES_MAX);
    let stack = Vec::with_capacity(STACK_SIZE_MAX);
    let mut garbage_collector = GarbageCollector::new();
    let globals = Table::new();
    let init_string = garbage_collector.intern("init".to_owned());
    let open_upvalues = Vec::new();
    let start_time = ProcessTime::now();
    let mut result = Self {
      call_frames,
      stack,
      garbage_collector,
      globals,
      init_string,
      open_upvalues,
      start_time,
    };
    result
      .define_native_function("clock", NativeFunction(standard_library::uptime))
      .unwrap();

    result
  }

  /// Interpret some source code.
  pub fn interpret(&mut self, source: &str) -> Result<(), Error> {
    let mut interpreter = Interpreter::default();
    let function = interpreter.compile(source, &mut self.garbage_collector)?;

    self.push(Value::Function(function))?;
    let closure = self.alloc(ClosureStruct::new(function));

    self.call_frames.push(CallFrame::new(closure, 0));
    self.run()?;
    Ok(())
  }

  /// Run the chunk.
  pub fn run(&mut self) -> Result<(), Error> {
    loop {
      let instruction =
        self.get_current_chunk().instructions.instructions[self.get_current_frame().instruction_pointer];
      self.get_current_frame_mut().instruction_pointer += 1;

      use Instruction::*;
      use Value::*;
      debug_var!(self.get_current_frame().instruction_pointer);
      debug_var!(instruction);
      match instruction {
        Constant(index) => {
          let constant = self.get_current_chunk().constants.constants[index as usize];

          self.push(constant)?;
        },
        Negate => {
          let pop = self.pop()?;

          match pop {
            Number(pop) => {
              let answer = -pop;

              self.push(Value::Number(answer))?;
            },
            _ => {
              self.did_encounter_runtime_error("Operand must be a number.");
              return Err(Error::RuntimeError(RuntimeError::InappropriateOperand(Negate, pop)));
            },
          }
        },
        Add => {
          let a = self.pop()?;
          let b = self.pop()?;

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
              self.did_encounter_runtime_error("Operands must be two numbers or two strings.");
              return Err(Error::RuntimeError(RuntimeError::InappropriateOperands(
                instruction,
                b,
                a,
              )));
            },
          }
        },
        Subtract => self.binary_arithmetic_operation(Subtract, |a, b| b - a, Value::Number)?,
        Multiply => self.binary_arithmetic_operation(Multiply, |a, b| b * a, Value::Number)?,
        Divide => self.binary_arithmetic_operation(Divide, |a, b| b / a, Value::Number)?,
        Equal => {
          let a = self.pop()?;
          let b = self.pop()?;

          self.push(Value::Boolean(a == b))?;
        },
        NotEqual => {
          let a = self.pop()?;
          let b = self.pop()?;

          self.push(Value::Boolean(a != b))?;
        },
        GreaterThan => self.binary_arithmetic_operation(GreaterThan, |a, b| b > a, Value::Boolean)?,
        LessThan => self.binary_arithmetic_operation(LessThan, |a, b| b < a, Value::Boolean)?,
        GreaterThanOrEqual => self.binary_arithmetic_operation(GreaterThanOrEqual, |a, b| b >= a, Value::Boolean)?,
        LessThanOrEqual => self.binary_arithmetic_operation(LessThanOrEqual, |a, b| b <= a, Value::Boolean)?,
        Return => {
          let call_frame = self.call_frames.pop().unwrap();
          let return_value = self.pop()?;

          self.close_upvalues(call_frame.index)?;
          if self.call_frames.is_empty() {
            return Ok(());
          } else {
            self.stack.truncate(call_frame.index);
            self.push(return_value)?;
          }
        },
        True => self.push(Value::Boolean(true))?,
        False => self.push(Value::Boolean(false))?,
        Instruction::Nil => self.push(Value::Nil)?,
        Pop => {
          self.pop()?;
        },
        Print => {
          let value = self.pop()?;
          let formatter = TraceFormatter::new(value, &self.garbage_collector);
          println!("{}", formatter);
        },
        Not => {
          let value = self.pop()?;
          let answer = self.is_falsey(&value);

          self.push(Value::Boolean(answer))?;
        },
        DefineGlobal(index) => {
          let identifier = self.get_current_chunk().read_string(index);
          let value = self.pop()?;

          self.globals.insert(identifier, value);
        },
        GetGlobal(index) => {
          let identifier = self.get_current_chunk().read_string(index);

          match self.globals.get(&identifier) {
            Some(&value) => self.push(value)?,
            None => {
              let identifier = self.garbage_collector.deref(identifier);
              self.did_encounter_runtime_error(&format!("Undefined variable '{}'.", identifier));
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
            let global_name = self.garbage_collector.deref(identifier);
            self.did_encounter_runtime_error(&format!("Undefined variable '{}'.", global_name));
            return Err(Error::RuntimeError(RuntimeError::UndefinedVariable(
              identifier.to_string(),
            )));
          }
        },
        GetLocal(index) => {
          let index = index as usize + self.get_current_frame().index;
          let value = self.stack[index as usize];
          self.push(value)?;
        },
        SetLocal(index) => {
          let index = index as usize + self.get_current_frame().index;
          let value = self.peek(0)?;
          self.stack[index] = value;
        },
        GetUpvalue(index) => {
          let value = {
            let upvalue_reference = self.get_current_closure().upvalues[index as usize];
            let upvalue = self.garbage_collector.deref(upvalue_reference);
            if let Some(value) = upvalue.closed {
              value
            } else {
              self.stack[upvalue.location]
            }
          };
          self.push(value)?;
        },
        SetUpvalue(index) => {
          let upvalue_reference = self.get_current_closure().upvalues[index as usize];
          let value = self.peek(0)?;
          let mut upvalue = self.garbage_collector.deref_mut(upvalue_reference);
          if upvalue.closed.is_none() {
            self.stack[upvalue.location] = value;
          } else {
            upvalue.closed = Some(value);
          }
        },
        CloseUpvalue => {
          let stack_top = self.stack.len() - 1;
          self.close_upvalues(stack_top)?;
          self.pop()?;
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
        Instruction::Closure(index) => {
          let function_value = self.get_current_chunk().constants.constants[index as usize];
          if let Value::Function(function) = function_value {
            let upvalue_count = self.garbage_collector.deref(function).upvalues.len();
            let mut closure = ClosureStruct::new(function);
            for i in 0..upvalue_count {
              let upvalue = self.garbage_collector.deref(function).upvalues[i];
              let object_upvalue = if upvalue.is_local {
                let location = self.get_current_frame().index + upvalue.index as usize;
                self.capture_upvalue(location)?
              } else {
                self.get_current_closure().upvalues[upvalue.index as usize]
              };
              closure.upvalues.push(object_upvalue);
            }
            let closure = self.alloc(closure);
            self.push(Value::Closure(closure))?;
          }
        },
        Call(argument_count) => {
          self.call_value(argument_count as usize)?;
        },
        Instruction::Class(index) => {
          let class_name = self.get_current_chunk().read_string(index);
          let class_object = ClassStruct::new(class_name);
          let class_reference = self.alloc(class_object);

          self.push(Value::Class(class_reference))?;
        },
        Method(index) => {
          let method_name = self.get_current_chunk().read_string(index);
          self.define_method(method_name)?;
        },
        Invoke((name_index, argument_count)) => {
          let name = self.get_current_chunk().read_string(name_index);
          self.invoke(name, argument_count as usize)?;
        },
        SetProperty(index) => {
          if let Value::Instance(instance_reference) = self.peek(1)? {
            let property_name = self.get_current_chunk().read_string(index);
            let value = self.pop()?;
            let instance = self.garbage_collector.deref_mut(instance_reference);
            instance.fields.insert(property_name, value);
            self.pop()?;
            self.push(value)?;
          } else {
            self.did_encounter_runtime_error("Only instances have fields.");
            return Err(Error::RuntimeError(RuntimeError::AccessedPropertyOnNonInstance));
          }
        },
        GetProperty(index) => {
          if let Value::Instance(instance_reference) = self.peek(0)? {
            let instance = self.garbage_collector.deref(instance_reference);
            let class = instance.class;
            let property_name = self.get_current_chunk().read_string(index);
            let value = instance.fields.get(&property_name);
            match value {
              Some(&value) => {
                self.pop()?;
                self.push(value)?;
              },
              None => {
                self.bind_method(class, property_name)?;
              },
            }
          } else {
            self.did_encounter_runtime_error("Only instances have properties.");
            return Err(Error::RuntimeError(RuntimeError::AccessedPropertyOnNonInstance));
          }
        },
        Inherit => {
          let pair = (self.peek(0)?, self.peek(1)?);
          if let (Value::Class(subclass_reference), Value::Class(superclass_reference)) = pair {
            let superclass = self.garbage_collector.deref(superclass_reference);
            let methods = superclass.methods.clone();
            let mut subclass = self.garbage_collector.deref_mut(subclass_reference);
            subclass.methods = methods;
            self.pop()?;
          } else {
            self.did_encounter_runtime_error("Superclass must be a class.");
            return Err(Error::RuntimeError(RuntimeError::AttemptedToSubclassNonClass));
          }
        },
        GetSuper(index) => {
          let method_name = self.get_current_chunk().read_string(index);
          if let Value::Class(superclass) = self.pop()? {
            self.bind_method(superclass, method_name)?;
          } else {
            self.did_encounter_runtime_error("Could not find a superclass");
            return Err(Error::RuntimeError(RuntimeError::CouldNotFindRequestedSuperclass));
          }
        },
        SuperInvoke((index, argument_count)) => {
          let method_name = self.get_current_chunk().read_string(index);
          if let Value::Class(class) = self.pop()? {
            self.invoke_from_class(class, method_name, argument_count as usize)?;
          } else {
            self.did_encounter_runtime_error("super invoke with no class");
            return Err(Error::RuntimeError(RuntimeError::CouldNotFindRequestedSuperclass));
          }
        },
      }
    }
  }

  /// Binary arithmetic operator.
  #[inline]
  pub fn binary_arithmetic_operation<T>(
    &mut self,
    instruction: Instruction,
    function: fn(f64, f64) -> T,
    valuate: fn(T) -> Value,
  ) -> Result<(), Error> {
    let a = self.pop()?;
    let b = self.pop()?;

    use Value::*;
    match (a, b) {
      (Number(a), Number(b)) => self.push(valuate(function(a, b)))?,
      (_, _) => {
        self.did_encounter_runtime_error("Operands must be numbers.");
        return Err(Error::RuntimeError(RuntimeError::InappropriateOperands(
          instruction,
          b,
          a,
        )));
      },
    }
    Ok(())
  }

  /// Push a value to the stack.
  #[inline]
  pub fn push(&mut self, value: Value) -> Result<(), Error> {
    if self.stack.len() > STACK_SIZE_MAX {
      return Err(Error::RuntimeError(RuntimeError::StackOverflow));
    }

    self.stack.push(value);
    Ok(())
  }

  /// Push a value to the stack.
  #[inline]
  pub fn pop(&mut self) -> Result<Value, Error> {
    if self.stack.is_empty() {
      return Err(Error::RuntimeError(RuntimeError::StackUnderflow));
    }
    let result = self.stack.pop().unwrap();
    Ok(result)
  }

  /// Peek at a value on the stack.
  #[inline]
  pub fn peek(&self, offset: usize) -> Result<Value, Error> {
    if self.stack.is_empty() {
      return Err(Error::RuntimeError(RuntimeError::StackUnderflow));
    }
    let max_index = self.stack.len() - 1;
    let index = max_index - offset;
    let result = self.stack[index];
    Ok(result)
  }

  /// Set a value in the stack directly.
  pub fn set_in_stack(&mut self, offset: usize, value: Value) {
    let max_index = self.stack.len() - 1;
    let index = max_index - offset;
    self.stack[index] = value;
  }

  /// Is this "falsey" or not?
  #[inline]
  pub fn is_falsey(&mut self, value: &Value) -> bool {
    use Value::*;

    match value {
      Nil => true,
      Boolean(value) => !value,
      _ => false,
    }
  }

  /// Allocate an object.
  pub fn alloc<T: Trace + 'static + Debug>(&mut self, object: T) -> Reference<T> {
    self.mark_and_sweep();

    self.garbage_collector.alloc(object)
  }

  /// Eliminates duplicate string references.
  pub fn intern(&mut self, name: String) -> Reference<String> {
    self.mark_and_sweep();

    self.garbage_collector.intern(name)
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
  pub fn mark_and_sweep(&mut self) {
    if self.garbage_collector.should_collect() {
      debug!("Beginning garbage collection.");
      self.mark_roots();
      self.garbage_collector.collect_garbage();
      debug!("Concluding garbage collection.");
    }
  }

  /// Mark roots.
  fn mark_roots(&mut self) {
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
    debug!("marking upvalues for garbage collection");
    for &upvalue in &self.open_upvalues {
      self.garbage_collector.mark_object(upvalue);
    }
    debug!("marking {} global variables for garbage collection", self.globals.len());
    self.garbage_collector.mark_table(&self.globals);
    self.garbage_collector.mark_object(self.init_string);
  }

  /// Get current frame.
  pub fn get_current_frame(&self) -> &CallFrame {
    let result = self.call_frames.last().unwrap();

    result
  }

  /// Get current frame mutable.
  pub fn get_current_frame_mut(&mut self) -> &mut CallFrame {
    let result = self.call_frames.last_mut().unwrap();

    result
  }

  /// Get current closure.
  pub fn get_current_closure(&self) -> &ClosureStruct {
    let closure = self.get_current_frame().closure;

    self.garbage_collector.deref(closure) as _
  }

  /// Get current chunk.
  pub fn get_current_chunk(&self) -> &Chunk {
    let closure = self.get_current_closure();
    let function = self.garbage_collector.deref(closure.function);

    &function.chunk as _
  }

  /// Capture an upvalue.
  pub fn capture_upvalue(&mut self, location: usize) -> Result<Reference<Upvalue>, Error> {
    for &upvalue_ref in &self.open_upvalues {
      let upvalue = self.garbage_collector.deref(upvalue_ref);

      if upvalue.location == location {
        return Ok(upvalue_ref);
      }
    }
    let upvalue = Upvalue::new(location);
    let upvalue = self.alloc(upvalue);

    self.open_upvalues.push(upvalue);
    Ok(upvalue)
  }

  /// Call the value on top of the stack as a function.
  pub fn call_value(&mut self, argument_count: usize) -> Result<(), Error> {
    let callee = self.peek(argument_count)?;
    match callee {
      Value::BoundMethod(bound_reference) => {
        let bound = self.garbage_collector.deref(bound_reference);
        let method = bound.method;
        let receiver = bound.receiver;

        self.set_in_stack(argument_count, receiver);
        self.call(method, argument_count)?;
      },
      Value::Closure(closure) => self.call(closure, argument_count)?,
      Value::NativeFunction(native_function) => {
        let start = self.stack.len() - argument_count;
        let result = native_function.0(self, &self.stack[start..])?;

        self.stack.truncate(start - 1);
        self.push(result)?;
      },
      Value::Class(class_reference) => {
        let instance_object = Instance::new(class_reference);
        let instance_reference = self.alloc(instance_object);

        self.set_in_stack(argument_count, Value::Instance(instance_reference));
        let class = self.garbage_collector.deref(class_reference);
        if let Some(&initializer) = class.methods.get(&self.init_string) {
          if let Value::Closure(initializer) = initializer {
            return self.call(initializer, argument_count);
          } else {
            self.did_encounter_runtime_error("Initializer is not closure");
            return Err(Error::RuntimeError(RuntimeError::ClassInitializerWasNotAClosure));
          }
        } else if argument_count > 0 {
          let message = format!("Expected 0 arguments but got {}.", argument_count);
          self.did_encounter_runtime_error(&message);
          return Err(Error::RuntimeError(RuntimeError::ClassInitializerCalledWithArguments(
            argument_count,
          )));
        }
      },
      value => {
        self.did_encounter_runtime_error("Can only call functions and classes.");
        return Err(Error::RuntimeError(RuntimeError::CalledUncallableValue(value)));
      },
    }
    Ok(())
  }

  /// Invoke a method with arguments.
  pub fn invoke(&mut self, name: Reference<String>, argument_count: usize) -> Result<(), Error> {
    let receiver = self.peek(argument_count)?;

    if let Value::Instance(instance_reference) = receiver {
      let instance = self.garbage_collector.deref(instance_reference);

      if let Some(&field) = instance.fields.get(&name) {
        self.set_in_stack(argument_count, field);
        self.call_value(argument_count)?;
      } else {
        let class = instance.class;

        self.invoke_from_class(class, name, argument_count)?;
      }
    } else {
      self.did_encounter_runtime_error("Only instances have methods.");
      return Err(Error::RuntimeError(RuntimeError::CalledMethodOnNonInstance));
    }
    Ok(())
  }

  /// Invoke a method call via class.
  fn invoke_from_class(
    &mut self,
    class_reference: Reference<ClassStruct>,
    name_reference: Reference<String>,
    argument_count: usize,
  ) -> Result<(), Error> {
    let class = self.garbage_collector.deref(class_reference);

    if let Some(&method_value) = class.methods.get(&name_reference) {
      if let Value::Closure(closure_reference) = method_value {
        self.call(closure_reference, argument_count)?;
      } else {
        return Err(Error::RuntimeError(RuntimeError::CalledNonClosureMethod));
      }
    } else {
      let name = self.garbage_collector.deref(name_reference);
      self.did_encounter_runtime_error(&format!("Undefined property '{}'.", name));
      return Err(Error::RuntimeError(RuntimeError::CalledNonexistentMethod));
    }
    Ok(())
  }

  /// Call a closure.
  pub fn call(&mut self, closure_reference: Reference<ClosureStruct>, argument_count: usize) -> Result<(), Error> {
    let closure = self.garbage_collector.deref(closure_reference);
    let function = self.garbage_collector.deref(closure.function);

    if argument_count != function.arity {
      let message = format!("Expected {} arguments but got {}.", function.arity, argument_count);
      self.did_encounter_runtime_error(&message);
      return Err(Error::RuntimeError(
        RuntimeError::CalledFunctionWithWrongNumberOfArguments(argument_count, function.arity),
      ));
    } else if self.call_frames.len() == CALL_FRAMES_MAX {
      return Err(Error::RuntimeError(RuntimeError::StackOverflow));
    } else {
      let start = self.stack.len() - argument_count - 1;
      let end = start + argument_count;

      debug!(
        "Calling {} {} with arguments ({:#?})",
        closure,
        function,
        &self.stack[start..end]
      );
      let call_frame = CallFrame::new(closure_reference, start);
      self.call_frames.push(call_frame);
    }
    Ok(())
  }

  /// Close upvalues; that is, move them from the stack to the heap.
  pub fn close_upvalues(&mut self, last: usize) -> Result<(), Error> {
    let mut i = 0;
    while i != self.open_upvalues.len() {
      let upvalue_reference = self.open_upvalues[i];
      let upvalue = self.garbage_collector.deref_mut(upvalue_reference);
      if upvalue.location >= last {
        self.open_upvalues.remove(i);
        let location = upvalue.location;
        upvalue.closed = Some(self.stack[location]);
      } else {
        i += 1;
      }
    }
    Ok(())
  }

  /// Define a native function.
  pub fn define_native_function(&mut self, name: &str, native_function: NativeFunction) -> Result<(), Error> {
    let name_reference = self.garbage_collector.intern(name.to_owned());

    self
      .globals
      .insert(name_reference, Value::NativeFunction(native_function));
    Ok(())
  }

  /// Encountered an error.
  pub fn did_encounter_runtime_error(&self, message: &str) {
    let frame = self.get_current_frame();

    eprintln!("{}", message);
    let chunk = self.get_current_chunk();
    let line_number = chunk.instructions.line_numbers[frame.instruction_pointer - 1];
    eprintln!("[line {}] in script", line_number);
  }

  /// Bind a method to a class.
  pub fn bind_method(
    &mut self,
    class_reference: Reference<ClassStruct>,
    name_reference: Reference<String>,
  ) -> Result<(), Error> {
    let class = self.garbage_collector.deref(class_reference);

    if let Some(method) = class.methods.get(&name_reference) {
      let receiver = self.peek(0)?;
      let method = match method {
        Value::Closure(closure) => *closure,
        _ => panic!("Inconsistent state. Method is not closure"),
      };
      let bound = BoundMethod::new(receiver, method);
      let bound_reference = self.alloc(bound);

      self.pop()?;
      self.push(Value::BoundMethod(bound_reference))?;
    } else {
      let name = self.garbage_collector.deref(name_reference);

      self.did_encounter_runtime_error(&format!("Undefined property '{}'.", name));
      return Err(Error::RuntimeError(RuntimeError::UndefinedProperty(name.to_string())));
    }
    Ok(())
  }

  /// Defines a method.
  pub fn define_method(&mut self, name: Reference<String>) -> Result<(), Error> {
    let method_value = self.peek(0)?;
    if let Value::Class(class_reference) = self.peek(1)? {
      let class_object = self.garbage_collector.deref_mut(class_reference);
      class_object.methods.insert(name, method_value);
      self.pop()?;
    } else {
      return Err(Error::RuntimeError(RuntimeError::DefinedMethodOutsideClassContext));
    }
    Ok(())
  }
}

impl Default for VirtualMachine {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_vm() {
    init();
  }

  #[test]
  pub fn test_vm2() {
    init();
    let mut vm = VirtualMachine::new();
    let line = "!(5 - 4 > 3 * 2 == !nil);".to_string();
    vm.interpret(&line).unwrap();
  }

  #[test]
  #[should_panic]
  pub fn test_vm3() {
    init();
    let mut vm = VirtualMachine::new();
    let line = "invalid input".to_string();
    // Should panic.
    vm.interpret(&line).unwrap();
  }

  #[test]
  pub fn test_vm4() -> Result<(), Error> {
    init();
    let mut vm = VirtualMachine::new();
    vm.interpret("2 != 3;")?;
    vm.interpret("2 > 3;")?;
    vm.interpret("2 >= 3;")?;
    vm.interpret("2 == 2;")?;
    vm.interpret("2 == 3;")?;
    vm.interpret("2 != 2;")?;
    vm.interpret("!(2 > 3);")?;
    vm.interpret("!(2 >= 3);")?;
    vm.interpret("2 < 3;")?;
    vm.interpret("2 <= 3;")?;
    vm.interpret("2 - 3;")?;
    vm.interpret("3 - 2;")?;
    vm.interpret("2 + 3;")?;
    vm.interpret("3 + 2;")?;
    vm.interpret("2 * -4;")?;
    vm.interpret("3 * 2;")?;
    vm.interpret("-4 / 2;")?;
    vm.interpret("2 / 4;")?;
    vm.interpret("nil;")?;
    vm.interpret("true;")?;
    vm.interpret("false;")?;
    Ok(())
  }
}
