use crate::{encoding::Instruction, opcode::Opcode, IsaContext};

pub enum PowerState {
    Sleep,
    Hibernate,
    PowerOff,
}

/// POWER MANAGEMENT ISA INSTRUCTIONS
pub fn execute(instr: Instruction, _ctx: &IsaContext) -> Result<PowerState, String> {
    match instr.opcode {
        Opcode::SLEEP => {
            println!("[POWER] Entering SLEEP mode");
            Ok(PowerState::Sleep)
        }

        Opcode::HIBERNATE => {
            println!("[POWER] Entering HIBERNATION mode");
            Ok(PowerState::Hibernate)
        }

        Opcode::POWER_OFF => {
            println!("[POWER] SYSTEM SHUTDOWN INITIATED");
            Ok(PowerState::PowerOff)
        }

        _ => Err("Invalid power opcode".into()),
    }
}
