use ruint::aliases::U256;

#[derive(Debug, Clone)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    fn expand(&mut self, offset: usize, size: usize) {
        let needed = offset + size;
        if needed > self.data.len() {
            let words = (needed + 31) / 32;
            self.data.resize(words * 32, 0);
        }
    }

    pub fn get_u256(&mut self, offset: usize) -> U256 {
        self.expand(offset, 32);
        let bytes: [u8; 32] = self.data[offset..offset + 32]
            .try_into()
            .expect("slice is 32 bytes");
        U256::from_be_bytes(bytes)
    }

    pub fn set_u256(&mut self, offset: usize, value: U256) {
        self.expand(offset, 32);
        let bytes = value.to_be_bytes::<32>();
        self.data[offset..offset + 32].copy_from_slice(&bytes);
    }

    pub fn set_byte(&mut self, offset: usize, value: u8) {
        self.expand(offset, 1);
        self.data[offset] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruint::uint;

    #[test]
    fn starts_empty() {
        let memory = Memory::new();
        assert_eq!(memory.size(), 0);
    }

    #[test]
    fn set_and_get_u256() {
        let mut memory = Memory::new();
        memory.set_u256(0, uint!(0xff_U256));
        assert_eq!(memory.get_u256(0), uint!(0xff_U256));
        assert_eq!(memory.size(), 32);
    }

    #[test]
    fn memory_grows_in_32_byte_words() {
        let mut memory = Memory::new();
        memory.set_u256(31, uint!(1_U256));
        assert_eq!(memory.size(), 64);
    }

    #[test]
    fn set_byte() {
        let mut memory = Memory::new();
        memory.set_byte(0, 0xAB);
        assert_eq!(memory.size(), 32);
        let val = memory.get_u256(0);
        let expected = uint!(0xAB_U256) << 248;
        assert_eq!(val, expected);
    }

    #[test]
    fn read_uninitialized_returns_zero() {
        let mut memory = Memory::new();
        memory.set_byte(0, 0xFF);
        let val = memory.get_u256(0);
        let expected = uint!(0xFF_U256) << 248;
        assert_eq!(val, expected);
    }
}
