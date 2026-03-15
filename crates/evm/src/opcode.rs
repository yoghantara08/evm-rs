// Opcode byte constants
pub const STOP: u8 = 0x00;
pub const ADD: u8 = 0x01;
pub const MUL: u8 = 0x02;
pub const SUB: u8 = 0x03;
pub const DIV: u8 = 0x04;
pub const SDIV: u8 = 0x05;
pub const MOD: u8 = 0x06;
pub const SMOD: u8 = 0x07;
pub const ADDMOD: u8 = 0x08;
pub const MULMOD: u8 = 0x09;
pub const EXP: u8 = 0x0A;
pub const SIGNEXTEND: u8 = 0x0B;

pub const POP: u8 = 0x50;

pub const PUSH1: u8 = 0x60;
pub const PUSH32: u8 = 0x7F;

/// Return human-readable name for an opcode byte.
pub fn opcode_name(op: u8) -> String {
    match op {
        STOP => "STOP".into(),
        ADD => "ADD".into(),
        MUL => "MUL".into(),
        SUB => "SUB".into(),
        DIV => "DIV".into(),
        SDIV => "SDIV".into(),
        MOD => "MOD".into(),
        SMOD => "SMOD".into(),
        ADDMOD => "ADDMOD".into(),
        MULMOD => "MULMOD".into(),
        EXP => "EXP".into(),
        SIGNEXTEND => "SIGNEXTEND".into(),
        POP => "POP".into(),
        0x60..=0x7F => format!("PUSH{}", op - 0x5F),
        _ => format!("UNKNOWN(0x{op:02x})"),
    }
}

/// Static gas cost for an opcode.
pub fn opcode_gas(op: u8) -> u64 {
    match op {
        STOP => 0,
        ADD | MUL | SUB => 3,                     // G_verylow
        DIV | SDIV | MOD | SMOD | SIGNEXTEND => 5, // G_low
        ADDMOD | MULMOD => 8,                      // G_mid
        EXP => 10,                                 // base cost (+ 50 per exponent byte)
        POP => 2,                                  // G_base
        0x60..=0x7F => 3,                          // PUSH: G_verylow
        _ => 0,                                    // unknown opcodes halt immediately
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_names() {
        assert_eq!(opcode_name(0x00), "STOP");
        assert_eq!(opcode_name(0x01), "ADD");
        assert_eq!(opcode_name(0x60), "PUSH1");
        assert_eq!(opcode_name(0x7F), "PUSH32");
        assert_eq!(opcode_name(0x50), "POP");
        assert_eq!(opcode_name(0xEE), "UNKNOWN(0xee)");
    }

    #[test]
    fn static_gas_costs() {
        assert_eq!(opcode_gas(0x01), 3); // ADD
        assert_eq!(opcode_gas(0x02), 3); // MUL
        assert_eq!(opcode_gas(0x03), 3); // SUB
        assert_eq!(opcode_gas(0x04), 5); // DIV
        assert_eq!(opcode_gas(0x06), 5); // MOD
        assert_eq!(opcode_gas(0x08), 8); // ADDMOD
        assert_eq!(opcode_gas(0x09), 8); // MULMOD
        assert_eq!(opcode_gas(0x60), 3); // PUSH1
        assert_eq!(opcode_gas(0x7F), 3); // PUSH32
        assert_eq!(opcode_gas(0x00), 0); // STOP
        assert_eq!(opcode_gas(0x50), 2); // POP
        assert_eq!(opcode_gas(0x0A), 10); // EXP
        assert_eq!(opcode_gas(0x0B), 5); // SIGNEXTEND
    }
}
