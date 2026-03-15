use clap::{Parser, Subcommand};
use evm::context::ExecutionContext;
use evm::database::StubDatabase;
use evm::interpreter::{execute_with_trace, ExecutionResult};

#[derive(Parser)]
#[command(name = "evm-rs", about = "EVM bytecode interpreter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute EVM bytecode
    Run {
        /// Hex-encoded bytecode (without 0x prefix)
        #[arg(long)]
        bytecode: String,

        /// Enable step-by-step execution trace
        #[arg(long)]
        trace: bool,

        /// Gas limit (default: 1000000)
        #[arg(long, default_value = "1000000")]
        gas: u64,
    },
    /// Disassemble bytecode to human-readable opcodes
    Disasm {
        /// Hex-encoded bytecode (without 0x prefix)
        #[arg(long)]
        bytecode: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { bytecode, trace, gas } => {
            let bytecode = hex::decode(bytecode.trim_start_matches("0x"))
                .expect("invalid hex bytecode");
            let ctx = ExecutionContext {
                code: bytecode,
                gas_limit: gas,
                ..Default::default()
            };
            let mut db = StubDatabase;

            if trace {
                let (result, steps) = execute_with_trace(&ctx, &mut db);
                for step in &steps {
                    let stack_str: Vec<String> =
                        step.stack.iter().map(|v| format!("0x{v:02x}")).collect();
                    let instr = if let Some(ref operand) = step.operand {
                        format!("{} 0x{}", step.opcode_name, hex::encode(operand))
                    } else {
                        step.opcode_name.clone()
                    };
                    println!(
                        "PC={:04}  {:14} | Stack: [{}] | Gas: {}",
                        step.pc, instr, stack_str.join(", "), step.gas_used,
                    );
                }
                print_result(&result);
            } else {
                let result = evm::interpreter::execute(&ctx, &mut db);
                print_result(&result);
            }
        }
        Commands::Disasm { bytecode } => {
            let bytecode = hex::decode(bytecode.trim_start_matches("0x"))
                .expect("invalid hex bytecode");
            for line in evm::disasm::disassemble(&bytecode) {
                println!("{line}");
            }
        }
    }
}

fn print_result(result: &ExecutionResult) {
    match result {
        ExecutionResult::Success { gas_used, .. } => {
            println!("--- Execution complete: Success | Gas used: {gas_used} ---");
        }
        ExecutionResult::Revert { gas_used, .. } => {
            println!("--- Execution complete: Revert | Gas used: {gas_used} ---");
        }
        ExecutionResult::Halt { reason } => {
            println!("--- Execution halted: {reason} ---");
        }
    }
}
