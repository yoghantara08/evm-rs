use crate::opcode;

pub fn disassemble(bytecode: &[u8]) -> Vec<String> {
    let mut lines = Vec::new();
    let mut pc = 0;

    while pc < bytecode.len() {
        let op = bytecode[pc];
        let name = opcode::opcode_name(op);

        if (0x60..=0x7F).contains(&op) {
            let n = (op - 0x5F) as usize;
            let end = (pc + 1 + n).min(bytecode.len());
            let operand = &bytecode[pc + 1..end];
            lines.push(format!("{pc:04}: {name} 0x{}", hex::encode(operand)));
            pc += 1 + n;
        } else {
            lines.push(format!("{pc:04}: {name}"));
            pc += 1;
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disassemble_simple() {
        let bytecode = hex::decode("6001600201").unwrap();
        let result = disassemble(&bytecode);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "0000: PUSH1 0x01");
        assert_eq!(result[1], "0002: PUSH1 0x02");
        assert_eq!(result[2], "0004: ADD");
    }

    #[test]
    fn disassemble_push32() {
        let mut bytecode = vec![0x7F];
        bytecode.extend_from_slice(&[0x00; 31]);
        bytecode.push(0x01);
        let result = disassemble(&bytecode);
        assert_eq!(result.len(), 1);
        assert!(result[0].starts_with("0000: PUSH32 0x"));
    }

    #[test]
    fn disassemble_unknown_opcode() {
        let bytecode = vec![0xEF];
        let result = disassemble(&bytecode);
        assert_eq!(result[0], "0000: UNKNOWN(0xef)");
    }
}
