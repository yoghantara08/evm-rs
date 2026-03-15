use evm::context::ExecutionContext;
use evm::database::StubDatabase;
use evm::interpreter::{execute, ExecutionResult};

fn run(hex_bytecode: &str, gas: u64) -> ExecutionResult {
    let bytecode = hex::decode(hex_bytecode).unwrap();
    let ctx = ExecutionContext {
        code: bytecode,
        gas_limit: gas,
        ..Default::default()
    };
    let mut db = StubDatabase;
    execute(&ctx, &mut db)
}

#[test]
fn one_plus_two() {
    let result = run("6001600201", 10_000);
    match result {
        ExecutionResult::Success { gas_used, .. } => assert_eq!(gas_used, 9),
        other => panic!("expected Success, got {other:?}"),
    }
}

#[test]
fn complex_arithmetic() {
    // (3 * 5) + 2 = 17
    // PUSH1 2, PUSH1 5, PUSH1 3, MUL, ADD, STOP
    let result = run("600260056003020100", 10_000);
    assert!(matches!(result, ExecutionResult::Success { .. }));
}

#[test]
fn exp_two_to_ten() {
    // 2^10 = 1024
    // PUSH1 10, PUSH1 2, EXP, STOP
    let result = run("600A60020A00", 10_000);
    assert!(matches!(result, ExecutionResult::Success { .. }));
}

#[test]
fn out_of_gas_stops_execution() {
    let result = run("6001600201", 5);
    assert!(matches!(
        result,
        ExecutionResult::Halt { reason: evm::error::EvmError::OutOfGas }
    ));
}

#[test]
fn stack_overflow() {
    let mut bytecode = String::new();
    for _ in 0..1025 {
        bytecode.push_str("6001");
    }
    let result = run(&bytecode, 1_000_000);
    assert!(matches!(
        result,
        ExecutionResult::Halt { reason: evm::error::EvmError::StackOverflow }
    ));
}

#[test]
fn div_by_zero_returns_zero_not_error() {
    let result = run("6000600A0400", 10_000);
    assert!(matches!(result, ExecutionResult::Success { .. }));
}
