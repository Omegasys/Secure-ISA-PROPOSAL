use crate::{encoding::Instruction, opcode::Opcode, IsaContext};

/// HW_ACCEL:
/// Delegates computation to hardware (GPU / HSM / FPGA / secure chip)
pub fn execute(instr: Instruction, ctx: &IsaContext) -> Result<(), String> {
    if !ctx.hw_accel_enabled {
        return Err("Hardware acceleration disabled".into());
    }

    if instr.opcode != Opcode::HW_ACCEL {
        return Err("Invalid opcode for HW_ACCEL".into());
    }

    let task_id = instr.operand_a;

    println!(
        "[HW_ACCEL] Offloading task {} to hardware accelerator (flags={})",
        task_id, instr.flags
    );

    // Placeholder:
    // - route to GPU / HSM / SIMD / crypto engine

    Ok(())
}
