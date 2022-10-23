/// Format a nullary opcode in a useful way.
#[macro_export]
macro_rules! format_nullary_opcode {
  ($opcode: expr) => {{
    format!("{0:<16}", stringify!($opcode))
  }};
}

macro_rules! test_instructions {
  ([$($instruction:expr),*], [$($start_stack:expr),*] => [$($end_stack:expr),*]) => {{
    #[allow(unreachable_patterns)]
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    { // Begin test scope.
      use crate::scripting_language::instruction::Instruction;
      use Instruction::*;
      use crate::scripting_language::program::Program;
      use crate::scripting_language::virtual_machine::VirtualMachine;
      info!("\n\n------------------ Starting test! ------------------\n");
      let mut dump = String::new();
      let mut program = Program::default();
      let mut line = 0;
      $(line += 1; program.instructions.append($instruction, line);)*
      print_var!(line);
      print_var!(program);
      program.instructions.dump(&mut dump).unwrap();
      println!("{}", dump);
      let mut vm = VirtualMachine::default();
      // We want the first values listed above to be the top of the stack, so
      // we have to create a vector, then reverse it.
      let mut start_stack = Vec::new();
      $(start_stack.push($start_stack);)*
      start_stack.reverse();
      vm.stack = start_stack;
      let result = vm.run(&program);
      print_var!(result);
      result.unwrap();
      let mut end_stack = vm.stack.clone();
      // Now we want to match the first values of the end stack listed above to
      // the values of the stack as they're popped.
      $({
        use Value::*;
        let expected = $end_stack;
        let actual = end_stack.pop().unwrap();
        match (expected, actual) {
          (Number(expected), Number(actual)) => assert_approx_eq!(expected, actual, 0.1),
          (expected, actual) => assert_eq!(expected, actual, "Failed to match expected values from stack (expected: {}, actual: {})", expected, actual),
        }
      })*
    }
  }};
}
