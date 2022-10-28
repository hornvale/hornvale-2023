use crate::scripting_language::native_function::error::Error;
use crate::scripting_language::value::Value;
use crate::scripting_language::virtual_machine::VirtualMachine;

#[named]
pub fn uptime(vm: &VirtualMachine, args: &[Value]) -> Result<Value, Error> {
  trace_enter!();
  trace_var!(vm);
  trace_var!(args);
  let time = vm.start_time.elapsed().as_secs_f64();
  trace_var!(time);
  let result = Value::Number(time);
  trace_var!(result);
  trace_exit!();
  Ok(result)
}
