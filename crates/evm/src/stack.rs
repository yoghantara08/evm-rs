use crate::error::EvmError;
use ruint::aliases::U256;

const MAX_STACK_DEPTH: usize = 1024;

#[derive(Debug, Clone)]
pub struct Stack {
    data: Vec<U256>,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(64),
        }
    }

    pub fn push(&mut self, value: U256) -> Result<(), EvmError> {
        if self.data.len() >= MAX_STACK_DEPTH {
            return Err(EvmError::StackOverflow);
        }
        self.data.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<U256, EvmError> {
        self.data.pop().ok_or(EvmError::StackUnderflow)
    }

    pub fn peek(&self, depth: usize) -> Result<U256, EvmError> {
        if depth >= self.data.len() {
            return Err(EvmError::StackUnderflow);
        }
        Ok(self.data[self.data.len() - 1 - depth])
    }

    pub fn swap(&mut self, depth: usize) -> Result<(), EvmError> {
        let len = self.data.len();
        if depth >= len {
            return Err(EvmError::StackUnderflow);
        }
        self.data.swap(len - 1, len - 1 - depth);
        Ok(())
    }

    pub fn dup(&mut self, depth: usize) -> Result<(), EvmError> {
        let value = self.peek(depth - 1)?;
        self.push(value)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruint::uint;

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::new();
        stack.push(uint!(42_U256)).unwrap();
        assert_eq!(stack.pop().unwrap(), uint!(42_U256));
    }

    #[test]
    fn pop_empty_underflows() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop(), Err(EvmError::StackUnderflow));
    }

    #[test]
    fn push_overflow_at_1024() {
        let mut stack = Stack::new();
        for i in 0..1024 {
            stack.push(U256::from(i)).unwrap();
        }
        assert_eq!(stack.push(U256::ZERO), Err(EvmError::StackOverflow));
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
        stack.push(uint!(10_U256)).unwrap();
        stack.push(uint!(20_U256)).unwrap();
        assert_eq!(stack.peek(0).unwrap(), uint!(20_U256));
        assert_eq!(stack.peek(1).unwrap(), uint!(10_U256));
    }

    #[test]
    fn peek_out_of_bounds() {
        let stack = Stack::new();
        assert_eq!(stack.peek(0), Err(EvmError::StackUnderflow));
    }

    #[test]
    fn swap() {
        let mut stack = Stack::new();
        stack.push(uint!(1_U256)).unwrap();
        stack.push(uint!(2_U256)).unwrap();
        stack.push(uint!(3_U256)).unwrap();
        stack.swap(1).unwrap();
        assert_eq!(stack.peek(0).unwrap(), uint!(2_U256));
        assert_eq!(stack.peek(1).unwrap(), uint!(3_U256));
    }

    #[test]
    fn dup() {
        let mut stack = Stack::new();
        stack.push(uint!(5_U256)).unwrap();
        stack.push(uint!(10_U256)).unwrap();
        stack.dup(1).unwrap();
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.peek(0).unwrap(), uint!(10_U256));
    }

    #[test]
    fn len_and_is_empty() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        stack.push(U256::ZERO).unwrap();
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 1);
    }
}
