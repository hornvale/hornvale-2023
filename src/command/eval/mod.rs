use crate::action::Action;
use crate::entity::PlayerId;
use crate::scripting::virtual_machine::VirtualMachine;
use crate::systems::command_processor::Data as CommandProcessorData;
use anyhow::Error;

/// The `Eval` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Eval {
  pub player_id: PlayerId,
  pub string: String,
  pub original_input: String,
}

impl Eval {
  pub fn get_action(&self, _data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    let mut vm = VirtualMachine::new();
    vm.interpret(&self.string)?;
    Ok(None)
  }
}
