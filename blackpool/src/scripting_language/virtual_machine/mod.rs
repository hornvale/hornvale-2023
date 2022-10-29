use cpu_time::ProcessTime;
use std::collections::hash_map::Entry;
use std::fmt::Debug;

use crate::scripting_language::bound_method::BoundMethod;
use crate::scripting_language::chunk::Chunk;
use crate::scripting_language::class::Class as ClassStruct;
use crate::scripting_language::closure::upvalue::Upvalue;
use crate::scripting_language::closure::Closure as ClosureStruct;
use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::garbage_collection::trace::formatter::Formatter as TraceFormatter;
use crate::scripting_language::garbage_collection::trace::Trace;
use crate::scripting_language::instance::Instance;
use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::interpreter::Interpreter;
use crate::scripting_language::native_function::NativeFunction;
use crate::scripting_language::standard_library;
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
  /// The reference to the initializer.
  pub init_string: Reference<String>,
  /// Open upvalues.
  pub open_upvalues: Vec<Reference<Upvalue>>,
  /// Start time.
  pub start_time: ProcessTime,
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
    let init_string = garbage_collector.intern("init".to_owned());
    trace_var!(init_string);
    let open_upvalues = Vec::new();
    trace_var!(open_upvalues);
    let start_time = ProcessTime::now();
    trace_var!(start_time);
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
    self.push(Value::Function(function))?;
    let closure = self.alloc(ClosureStruct::new(function));
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
          trace_var!(a);
          let b = self.pop()?;
          trace_var!(b);
          self.push(Value::Boolean(a == b))?;
        },
        NotEqual => {
          let a = self.pop()?;
          trace_var!(a);
          let b = self.pop()?;
          trace_var!(b);
          self.push(Value::Boolean(a != b))?;
        },
        GreaterThan => self.binary_arithmetic_operation(GreaterThan, |a, b| b > a, Value::Boolean)?,
        LessThan => self.binary_arithmetic_operation(LessThan, |a, b| b < a, Value::Boolean)?,
        GreaterThanOrEqual => self.binary_arithmetic_operation(GreaterThanOrEqual, |a, b| b >= a, Value::Boolean)?,
        LessThanOrEqual => self.binary_arithmetic_operation(LessThanOrEqual, |a, b| b <= a, Value::Boolean)?,
        Return => {
          let call_frame = self.call_frames.pop().unwrap();
          trace_var!(call_frame);
          let return_value = self.pop()?;
          trace_var!(return_value);
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
          trace_var!(class_name);
          let class_object = ClassStruct::new(class_name);
          trace_var!(class_object);
          let class_reference = self.alloc(class_object);
          trace_var!(class_reference);
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
      }
    }
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
        self.did_encounter_runtime_error("Operands must be numbers.");
        return Err(Error::RuntimeError(RuntimeError::InappropriateOperands(
          instruction,
          b,
          a,
        )));
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

  /// Set a value in the stack directly.
  #[named]
  pub fn set_in_stack(&mut self, offset: usize, value: Value) {
    trace_enter!();
    trace_var!(offset);
    trace_var!(value);
    let max_index = self.stack.len() - 1;
    trace_var!(max_index);
    let index = max_index - offset;
    self.stack[index] = value;
    trace_exit!();
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
    debug!("marking upvalues for garbage collection");
    for &upvalue in &self.open_upvalues {
      self.garbage_collector.mark_object(upvalue);
    }
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
  pub fn get_current_closure(&self) -> &ClosureStruct {
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

  /// Capture an upvalue.
  #[named]
  pub fn capture_upvalue(&mut self, location: usize) -> Result<Reference<Upvalue>, Error> {
    trace_enter!();
    trace_var!(location);
    for &upvalue_ref in &self.open_upvalues {
      let upvalue = self.garbage_collector.deref(upvalue_ref);
      trace_var!(upvalue);
      if upvalue.location == location {
        return Ok(upvalue_ref);
      }
    }
    let upvalue = Upvalue::new(location);
    trace_var!(upvalue);
    let upvalue = self.alloc(upvalue);
    trace_var!(upvalue);
    self.open_upvalues.push(upvalue);
    trace_var!(upvalue);
    Ok(upvalue)
  }

  /// Call the value on top of the stack as a function.
  #[named]
  pub fn call_value(&mut self, argument_count: usize) -> Result<(), Error> {
    trace_enter!();
    trace_var!(argument_count);
    let callee = self.peek(argument_count)?;
    match callee {
      Value::BoundMethod(bound_reference) => {
        let bound = self.garbage_collector.deref(bound_reference);
        trace_var!(bound);
        let method = bound.method;
        trace_var!(method);
        let receiver = bound.receiver;
        trace_var!(receiver);
        self.set_in_stack(argument_count, receiver);
        self.call(method, argument_count)?;
      },
      Value::Closure(closure) => self.call(closure, argument_count)?,
      Value::NativeFunction(native_function) => {
        let start = self.stack.len() - argument_count;
        trace_var!(start);
        let result = native_function.0(self, &self.stack[start..])?;
        trace_var!(result);
        self.stack.truncate(start - 1);
        self.push(result)?;
      },
      Value::Class(class_reference) => {
        let instance_object = Instance::new(class_reference);
        trace_var!(instance_object);
        let instance_reference = self.alloc(instance_object);
        trace_var!(instance_reference);
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
    trace_exit!();
    Ok(())
  }

  /// Invoke a method with arguments.
  #[named]
  pub fn invoke(&mut self, name: Reference<String>, argument_count: usize) -> Result<(), Error> {
    trace_enter!();
    trace_var!(name);
    trace_var!(argument_count);
    let receiver = self.peek(argument_count)?;
    trace_var!(receiver);
    if let Value::Instance(instance_reference) = receiver {
      let instance = self.garbage_collector.deref(instance_reference);
      trace_var!(instance);
      if let Some(&field) = instance.fields.get(&name) {
        trace_var!(field);
        self.set_in_stack(argument_count, field);
        self.call_value(argument_count)?;
      } else {
        let class = instance.class;
        trace_var!(class);
        self.invoke_from_class(class, name, argument_count)?;
      }
    } else {
      self.did_encounter_runtime_error("Only instances have methods.");
      return Err(Error::RuntimeError(RuntimeError::CalledMethodOnNonInstance));
    }
    trace_exit!();
    Ok(())
  }

  /// Invoke a method call via class.
  #[named]
  fn invoke_from_class(
    &mut self,
    class_reference: Reference<ClassStruct>,
    name_reference: Reference<String>,
    argument_count: usize,
  ) -> Result<(), Error> {
    trace_enter!();
    trace_var!(class_reference);
    trace_var!(name_reference);
    trace_var!(argument_count);
    let class = self.garbage_collector.deref(class_reference);
    trace_var!(class);
    if let Some(&method_value) = class.methods.get(&name_reference) {
      if let Value::Closure(closure_reference) = method_value {
        self.call(closure_reference, argument_count)?;
      } else {
        return Err(Error::RuntimeError(RuntimeError::CalledNonClosureMethod));
      }
    } else {
      let name = self.garbage_collector.deref(name_reference);
      let message = format!("Undefined property '{}'.", name);
      self.did_encounter_runtime_error(&message);
      return Err(Error::RuntimeError(RuntimeError::CalledNonexistentMethod));
    }
    trace_exit!();
    Ok(())
  }

  /// Call a closure.
  #[named]
  pub fn call(&mut self, closure_reference: Reference<ClosureStruct>, argument_count: usize) -> Result<(), Error> {
    trace_enter!();
    trace_var!(closure_reference);
    trace_var!(argument_count);
    let closure = self.garbage_collector.deref(closure_reference);
    trace_var!(closure);
    let function = self.garbage_collector.deref(closure.function);
    trace_var!(function);
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
      trace_var!(start);
      let end = start + argument_count;
      trace_var!(end);
      debug!(
        "Calling {} {} with arguments ({:#?})",
        closure,
        function,
        &self.stack[start..end]
      );
      let call_frame = CallFrame::new(closure_reference, start);
      self.call_frames.push(call_frame);
    }
    trace_exit!();
    Ok(())
  }

  /// Close upvalues; that is, move them from the stack to the heap.
  #[named]
  pub fn close_upvalues(&mut self, last: usize) -> Result<(), Error> {
    trace_enter!();
    trace_var!(last);
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
    trace_exit!();
    Ok(())
  }

  /// Define a native function.
  #[named]
  pub fn define_native_function(&mut self, name: &str, native_function: NativeFunction) -> Result<(), Error> {
    trace_enter!();
    trace_var!(name);
    trace_var!(native_function);
    let name_reference = self.garbage_collector.intern(name.to_owned());
    trace_var!(name_reference);
    self
      .globals
      .insert(name_reference, Value::NativeFunction(native_function));
    trace_exit!();
    Ok(())
  }

  /// Encountered an error.
  #[named]
  pub fn did_encounter_runtime_error(&self, message: &str) {
    trace_enter!();
    trace_var!(message);
    let frame = self.get_current_frame();
    trace_var!(frame);
    eprintln!("{}", message);
    let chunk = self.get_current_chunk();
    trace_var!(chunk);
    let line_number = chunk.instructions.line_numbers[frame.instruction_pointer - 1];
    eprintln!("[line {}] in script", line_number);
    trace_exit!();
  }

  /// Bind a method to a class.
  #[named]
  pub fn bind_method(
    &mut self,
    class_reference: Reference<ClassStruct>,
    name_reference: Reference<String>,
  ) -> Result<(), Error> {
    trace_enter!();
    trace_var!(class_reference);
    trace_var!(name_reference);
    let class = self.garbage_collector.deref(class_reference);
    trace_var!(class);
    if let Some(method) = class.methods.get(&name_reference) {
      let receiver = self.peek(0)?;
      trace_var!(receiver);
      let method = match method {
        Value::Closure(closure) => *closure,
        _ => panic!("Inconsistent state. Method is not closure"),
      };
      trace_var!(method);
      let bound = BoundMethod::new(receiver, method);
      trace_var!(bound);
      let bound_reference = self.alloc(bound);
      trace_var!(bound_reference);
      self.pop()?;
      self.push(Value::BoundMethod(bound_reference))?;
    } else {
      let name = self.garbage_collector.deref(name_reference);
      trace_var!(name);
      self.did_encounter_runtime_error(&format!("Undefined property '{}'.", name));
      return Err(Error::RuntimeError(RuntimeError::UndefinedProperty(name.to_string())));
    }
    trace_exit!();
    Ok(())
  }

  /// Defines a method.
  #[named]
  pub fn define_method(&mut self, name: Reference<String>) -> Result<(), Error> {
    trace_enter!();
    trace_var!(name);
    let method_value = self.peek(0)?;
    if let Value::Class(class_reference) = self.peek(1)? {
      let class_object = self.garbage_collector.deref_mut(class_reference);
      class_object.methods.insert(name, method_value);
      self.pop()?;
    } else {
      return Err(Error::RuntimeError(RuntimeError::DefinedMethodOutsideClassContext));
    }
    trace_exit!();
    Ok(())
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
  pub fn test_vm4() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut vm = VirtualMachine::new();
    vm.interpret(&"2 != 3;".to_string())?;
    vm.interpret(&"2 > 3;".to_string())?;
    vm.interpret(&"2 >= 3;".to_string())?;
    vm.interpret(&"2 == 2;".to_string())?;
    vm.interpret(&"2 == 3;".to_string())?;
    vm.interpret(&"2 != 2;".to_string())?;
    vm.interpret(&"!(2 > 3);".to_string())?;
    vm.interpret(&"!(2 >= 3);".to_string())?;
    vm.interpret(&"2 < 3;".to_string())?;
    vm.interpret(&"2 <= 3;".to_string())?;
    vm.interpret(&"2 - 3;".to_string())?;
    vm.interpret(&"3 - 2;".to_string())?;
    vm.interpret(&"2 + 3;".to_string())?;
    vm.interpret(&"3 + 2;".to_string())?;
    vm.interpret(&"2 * -4;".to_string())?;
    vm.interpret(&"3 * 2;".to_string())?;
    vm.interpret(&"-4 / 2;".to_string())?;
    vm.interpret(&"2 / 4;".to_string())?;
    vm.interpret(&"nil;".to_string())?;
    vm.interpret(&"true;".to_string())?;
    vm.interpret(&"false;".to_string())?;
    trace_exit!();
    Ok(())
  }
}
