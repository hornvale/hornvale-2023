use crate::scripting_language::native_function::error::Error;
use crate::scripting_language::value::Value;
use crate::scripting_language::virtual_machine::VirtualMachine;

/// Get the time since the program started.

pub fn uptime(vm: &VirtualMachine, _args: &[Value]) -> Result<Value, Error> {
  let time = vm.start_time.elapsed().as_secs_f64();
  let result = Value::Number(time);

  Ok(result)
}
