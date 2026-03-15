use crate::context::ExecutionContext;
use crate::database::Database;
#[cfg(test)]
use crate::database::StubDatabase;
use crate::error::EvmError;
use crate::gas::Gas;
use crate::memory::Memory;
use crate::opcode;
use crate::stack::Stack;
use ruint::aliases::U256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Log {
    pub address: [u8; 20],
    pub topics: Vec<U256>,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum ExecutionResult {
    Success {
        gas_used: u64,
        return_data: Vec<u8>,
        logs: Vec<Log>,
    },
    Revert {
        gas_used: u64,
        return_data: Vec<u8>,
    },
    Halt {
        reason: EvmError,
    },
}

/// Two's complement negation for U256.
fn twos_complement(v: U256) -> U256 {
    (!v).wrapping_add(U256::from(1))
}

/// Execute bytecode and also return the final stack contents (top-first).
#[cfg(test)]
pub fn execute_returning_stack(
    ctx: &ExecutionContext,
    db: &mut dyn Database,
) -> (ExecutionResult, Vec<U256>) {
    let bytecode = &ctx.code;
    let mut pc: usize = 0;
    let mut stack = Stack::new();
    let mut memory = Memory::new();
    let mut gas = Gas::new(ctx.gas_limit);
    let logs: Vec<Log> = Vec::new();

    let result = run_loop(bytecode, &mut pc, &mut stack, &mut memory, &mut gas, logs, ctx, db);
    let stack_values: Vec<U256> = (0..stack.len()).map(|i| stack.peek(i).unwrap()).collect();
    (result, stack_values)
}

pub fn execute(ctx: &ExecutionContext, db: &mut dyn Database) -> ExecutionResult {
    let bytecode = &ctx.code;
    let mut pc: usize = 0;
    let mut stack = Stack::new();
    let mut memory = Memory::new();
    let mut gas = Gas::new(ctx.gas_limit);
    let logs: Vec<Log> = Vec::new();

    run_loop(bytecode, &mut pc, &mut stack, &mut memory, &mut gas, logs, ctx, db)
}

fn run_loop(
    bytecode: &[u8],
    pc: &mut usize,
    stack: &mut Stack,
    _memory: &mut Memory,
    gas: &mut Gas,
    logs: Vec<Log>,
    _ctx: &ExecutionContext,
    _db: &mut dyn Database,
) -> ExecutionResult {
    loop {
        let op = bytecode.get(*pc).copied().unwrap_or(opcode::STOP);

        // Deduct static gas cost before execution
        let cost = opcode::opcode_gas(op);
        if let Err(e) = gas.consume(cost) {
            return ExecutionResult::Halt { reason: e };
        }

        match op {
            opcode::STOP => {
                return ExecutionResult::Success {
                    gas_used: gas.used(),
                    return_data: Vec::new(),
                    logs,
                };
            }

            opcode::ADD => {
                let a = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let b = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                if let Err(e) = stack.push(a.wrapping_add(b)) { return ExecutionResult::Halt { reason: e }; }
            }
            opcode::MUL => {
                let a = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let b = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                if let Err(e) = stack.push(a.wrapping_mul(b)) { return ExecutionResult::Halt { reason: e }; }
            }
            opcode::SUB => {
                let a = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let b = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                if let Err(e) = stack.push(a.wrapping_sub(b)) { return ExecutionResult::Halt { reason: e }; }
            }
            opcode::DIV => {
                let a = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let b = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let result = if b.is_zero() { U256::ZERO } else { a / b };
                if let Err(e) = stack.push(result) { return ExecutionResult::Halt { reason: e }; }
            }
            opcode::SDIV => {
                let a = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let b = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let result = if b.is_zero() {
                    U256::ZERO
                } else {
                    let a_neg = a.bit(255);
                    let b_neg = b.bit(255);
                    let a_abs = if a_neg { twos_complement(a) } else { a };
                    let b_abs = if b_neg { twos_complement(b) } else { b };
                    let quot = a_abs / b_abs;
                    if a_neg != b_neg { twos_complement(quot) } else { quot }
                };
                if let Err(e) = stack.push(result) { return ExecutionResult::Halt { reason: e }; }
            }
            opcode::MOD => {
                let a = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let b = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let result = if b.is_zero() { U256::ZERO } else { a % b };
                if let Err(e) = stack.push(result) { return ExecutionResult::Halt { reason: e }; }
            }
            opcode::SMOD => {
                let a = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let b = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let result = if b.is_zero() {
                    U256::ZERO
                } else {
                    let a_neg = a.bit(255);
                    let b_neg = b.bit(255);
                    let a_abs = if a_neg { twos_complement(a) } else { a };
                    let b_abs = if b_neg { twos_complement(b) } else { b };
                    let rem = a_abs % b_abs;
                    if a_neg { twos_complement(rem) } else { rem }
                };
                if let Err(e) = stack.push(result) { return ExecutionResult::Halt { reason: e }; }
            }
            opcode::ADDMOD => {
                let a = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let b = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let n = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let result = if n.is_zero() { U256::ZERO } else { a.add_mod(b, n) };
                if let Err(e) = stack.push(result) { return ExecutionResult::Halt { reason: e }; }
            }
            opcode::MULMOD => {
                let a = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let b = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let n = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let result = if n.is_zero() { U256::ZERO } else { a.mul_mod(b, n) };
                if let Err(e) = stack.push(result) { return ExecutionResult::Halt { reason: e }; }
            }
            opcode::EXP => {
                let base = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let exponent = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let exp_bytes = if exponent.is_zero() { 0u64 } else { (exponent.bit_len() as u64 + 7) / 8 };
                if let Err(e) = gas.consume(50 * exp_bytes) { return ExecutionResult::Halt { reason: e }; }
                let result = base.pow(exponent);
                if let Err(e) = stack.push(result) { return ExecutionResult::Halt { reason: e }; }
            }
            opcode::SIGNEXTEND => {
                let b = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let x = match stack.pop() { Ok(v) => v, Err(e) => return ExecutionResult::Halt { reason: e } };
                let result = if b < U256::from(31) {
                    let bit_index = b.to::<usize>() * 8 + 7;
                    let sign_bit = U256::from(1) << bit_index;
                    let mask = sign_bit - U256::from(1) | sign_bit;
                    if x & sign_bit != U256::ZERO {
                        x | !mask
                    } else {
                        x & mask
                    }
                } else {
                    x
                };
                if let Err(e) = stack.push(result) { return ExecutionResult::Halt { reason: e }; }
            }

            // PUSH1..PUSH32
            0x60..=0x7F => {
                let n = (op - 0x5F) as usize;
                let mut bytes = [0u8; 32];
                let available = bytecode.len().saturating_sub(*pc + 1);
                let copy_len = n.min(available);
                bytes[32 - n..32 - n + copy_len]
                    .copy_from_slice(&bytecode[*pc + 1..*pc + 1 + copy_len]);
                let value = U256::from_be_bytes(bytes);
                if let Err(e) = stack.push(value) {
                    return ExecutionResult::Halt { reason: e };
                }
                *pc += n;
            }

            opcode::POP => {
                if let Err(e) = stack.pop() {
                    return ExecutionResult::Halt { reason: e };
                }
            }

            _ => {
                return ExecutionResult::Halt {
                    reason: EvmError::InvalidOpcode(op),
                };
            }
        }

        *pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruint::uint;

    fn run(bytecode: &[u8]) -> ExecutionResult {
        let ctx = ExecutionContext {
            code: bytecode.to_vec(),
            gas_limit: 10_000,
            ..Default::default()
        };
        let mut db = StubDatabase;
        execute(&ctx, &mut db)
    }

    fn run_returning_stack(bytecode: &[u8]) -> U256 {
        let ctx = ExecutionContext {
            code: bytecode.to_vec(),
            gas_limit: 100_000,
            ..Default::default()
        };
        let mut db = StubDatabase;
        let (result, stack) = execute_returning_stack(&ctx, &mut db);
        match result {
            ExecutionResult::Success { .. } => {},
            other => panic!("expected Success, got {other:?}"),
        }
        stack[0]
    }

    #[test]
    fn stop() {
        let result = run(&[0x00]);
        match result {
            ExecutionResult::Success { gas_used, .. } => assert_eq!(gas_used, 0),
            other => panic!("expected Success, got {other:?}"),
        }
    }

    #[test]
    fn empty_bytecode_is_stop() {
        let result = run(&[]);
        assert!(matches!(result, ExecutionResult::Success { .. }));
    }

    #[test]
    fn push1() {
        let result = run(&[0x60, 0x42, 0x00]);
        match result {
            ExecutionResult::Success { gas_used, .. } => assert_eq!(gas_used, 3),
            other => panic!("expected Success, got {other:?}"),
        }
    }

    #[test]
    fn push32() {
        let mut bytecode = vec![0x7F];
        bytecode.extend_from_slice(&[0xFF; 32]);
        bytecode.push(0x00);
        let result = run(&bytecode);
        assert!(matches!(result, ExecutionResult::Success { gas_used, .. } if gas_used == 3));
    }

    #[test]
    fn pop() {
        let result = run(&[0x60, 0x01, 0x50, 0x00]);
        match result {
            ExecutionResult::Success { gas_used, .. } => assert_eq!(gas_used, 5),
            other => panic!("expected Success, got {other:?}"),
        }
    }

    #[test]
    fn pop_empty_stack_halts() {
        let result = run(&[0x50]);
        assert!(matches!(result, ExecutionResult::Halt { reason: EvmError::StackUnderflow }));
    }

    #[test]
    fn invalid_opcode_halts() {
        let result = run(&[0xEF]);
        assert!(matches!(result, ExecutionResult::Halt { reason: EvmError::InvalidOpcode(0xEF) }));
    }

    #[test]
    fn out_of_gas() {
        let ctx = ExecutionContext {
            code: vec![0x60, 0x01, 0x00],
            gas_limit: 1,
            ..Default::default()
        };
        let mut db = StubDatabase;
        let result = execute(&ctx, &mut db);
        assert!(matches!(result, ExecutionResult::Halt { reason: EvmError::OutOfGas }));
    }

    #[test]
    fn add() {
        let val = run_returning_stack(&[0x60, 0x01, 0x60, 0x02, 0x01, 0x00]);
        assert_eq!(val, uint!(3_U256));
    }

    #[test]
    fn add_overflow_wraps() {
        let mut bytecode = vec![0x7F];
        bytecode.extend_from_slice(&[0xFF; 32]);
        bytecode.extend_from_slice(&[0x60, 0x01]);
        bytecode.push(0x01);
        bytecode.push(0x00);
        let val = run_returning_stack(&bytecode);
        assert_eq!(val, U256::ZERO);
    }

    #[test]
    fn mul() {
        let val = run_returning_stack(&[0x60, 0x03, 0x60, 0x07, 0x02, 0x00]);
        assert_eq!(val, uint!(21_U256));
    }

    #[test]
    fn sub() {
        let val = run_returning_stack(&[0x60, 0x03, 0x60, 0x05, 0x03, 0x00]);
        assert_eq!(val, uint!(2_U256));
    }

    #[test]
    fn div() {
        let val = run_returning_stack(&[0x60, 0x02, 0x60, 0x0A, 0x04, 0x00]);
        assert_eq!(val, uint!(5_U256));
    }

    #[test]
    fn div_by_zero() {
        let val = run_returning_stack(&[0x60, 0x00, 0x60, 0x0A, 0x04, 0x00]);
        assert_eq!(val, U256::ZERO);
    }

    #[test]
    fn sdiv_positive() {
        let val = run_returning_stack(&[0x60, 0x02, 0x60, 0x04, 0x05, 0x00]);
        assert_eq!(val, uint!(2_U256));
    }

    #[test]
    fn sdiv_neg_dividend() {
        let neg4 = (!U256::from(4)).wrapping_add(U256::from(1));
        let neg2 = (!U256::from(2)).wrapping_add(U256::from(1));
        let mut bytecode = Vec::new();
        bytecode.push(0x60); bytecode.push(0x02);
        bytecode.push(0x7F);
        bytecode.extend_from_slice(&neg4.to_be_bytes::<32>());
        bytecode.push(0x05);
        bytecode.push(0x00);
        let val = run_returning_stack(&bytecode);
        assert_eq!(val, neg2);
    }

    #[test]
    fn sdiv_both_neg() {
        let neg4 = (!U256::from(4)).wrapping_add(U256::from(1));
        let neg2 = (!U256::from(2)).wrapping_add(U256::from(1));
        let mut bytecode = Vec::new();
        bytecode.push(0x7F);
        bytecode.extend_from_slice(&neg2.to_be_bytes::<32>());
        bytecode.push(0x7F);
        bytecode.extend_from_slice(&neg4.to_be_bytes::<32>());
        bytecode.push(0x05);
        bytecode.push(0x00);
        let val = run_returning_stack(&bytecode);
        assert_eq!(val, uint!(2_U256));
    }

    #[test]
    fn mod_op() {
        let val = run_returning_stack(&[0x60, 0x03, 0x60, 0x0A, 0x06, 0x00]);
        assert_eq!(val, uint!(1_U256));
    }

    #[test]
    fn mod_by_zero() {
        let val = run_returning_stack(&[0x60, 0x00, 0x60, 0x0A, 0x06, 0x00]);
        assert_eq!(val, U256::ZERO);
    }

    #[test]
    fn smod_positive() {
        let val = run_returning_stack(&[0x60, 0x03, 0x60, 0x0A, 0x07, 0x00]);
        assert_eq!(val, uint!(1_U256));
    }

    #[test]
    fn smod_neg_dividend() {
        let neg10 = (!U256::from(10)).wrapping_add(U256::from(1));
        let neg1 = (!U256::from(1)).wrapping_add(U256::from(1));
        let mut bytecode = Vec::new();
        bytecode.push(0x60); bytecode.push(0x03);
        bytecode.push(0x7F);
        bytecode.extend_from_slice(&neg10.to_be_bytes::<32>());
        bytecode.push(0x07);
        bytecode.push(0x00);
        let val = run_returning_stack(&bytecode);
        assert_eq!(val, neg1);
    }

    #[test]
    fn addmod() {
        let val = run_returning_stack(&[0x60, 0x03, 0x60, 0x02, 0x60, 0x0A, 0x08, 0x00]);
        assert_eq!(val, U256::ZERO);
    }

    #[test]
    fn addmod_zero_mod() {
        let val = run_returning_stack(&[0x60, 0x00, 0x60, 0x02, 0x60, 0x0A, 0x08, 0x00]);
        assert_eq!(val, U256::ZERO);
    }

    #[test]
    fn mulmod() {
        let val = run_returning_stack(&[0x60, 0x04, 0x60, 0x03, 0x60, 0x0A, 0x09, 0x00]);
        assert_eq!(val, uint!(2_U256));
    }

    #[test]
    fn exp_op() {
        let val = run_returning_stack(&[0x60, 0x03, 0x60, 0x02, 0x0A, 0x00]);
        assert_eq!(val, uint!(8_U256));
    }

    #[test]
    fn exp_zero_exponent() {
        let val = run_returning_stack(&[0x60, 0x00, 0x60, 0x05, 0x0A, 0x00]);
        assert_eq!(val, uint!(1_U256));
    }

    #[test]
    fn exp_gas_dynamic_cost() {
        let result = run(&[0x60, 0xFF, 0x60, 0x02, 0x0A, 0x00]);
        match result {
            ExecutionResult::Success { gas_used, .. } => assert_eq!(gas_used, 66),
            other => panic!("expected Success, got {other:?}"),
        }
    }

    #[test]
    fn signextend_negative() {
        let val = run_returning_stack(&[0x60, 0xFF, 0x60, 0x00, 0x0B, 0x00]);
        assert_eq!(val, U256::MAX);
    }

    #[test]
    fn signextend_positive() {
        let val = run_returning_stack(&[0x60, 0x7F, 0x60, 0x00, 0x0B, 0x00]);
        assert_eq!(val, uint!(0x7F_U256));
    }
}
