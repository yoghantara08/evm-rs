use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvmError {
    StackUnderflow,
    StackOverflow,
    OutOfGas,
    InvalidJump,
    InvalidOpcode(u8),
    WriteProtection,
    CallDepthExceeded,
    InvalidMemoryAccess,
    ReturnDataOutOfBounds,
}

impl fmt::Display for EvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StackUnderflow => write!(f, "stack underflow"),
            Self::StackOverflow => write!(f, "stack overflow"),
            Self::OutOfGas => write!(f, "out of gas"),
            Self::InvalidJump => write!(f, "invalid jump destination"),
            Self::InvalidOpcode(op) => write!(f, "invalid opcode: 0x{op:02x}"),
            Self::WriteProtection => write!(f, "write protection violation"),
            Self::CallDepthExceeded => write!(f, "call depth exceeded"),
            Self::InvalidMemoryAccess => write!(f, "invalid memory access"),
            Self::ReturnDataOutOfBounds => write!(f, "return data out of bounds"),
        }
    }
}

impl std::error::Error for EvmError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_messages() {
        assert_eq!(EvmError::StackUnderflow.to_string(), "stack underflow");
        assert_eq!(EvmError::StackOverflow.to_string(), "stack overflow");
        assert_eq!(EvmError::OutOfGas.to_string(), "out of gas");
        assert_eq!(EvmError::InvalidJump.to_string(), "invalid jump destination");
        assert_eq!(
            EvmError::InvalidOpcode(0xEF).to_string(),
            "invalid opcode: 0xef"
        );
    }
}
