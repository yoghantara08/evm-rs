use ruint::aliases::U256;

pub type Address = [u8; 20];

pub trait Database {
    fn balance(&self, address: &Address) -> U256;
    fn code(&self, address: &Address) -> Vec<u8>;
    fn code_hash(&self, address: &Address) -> U256;
    fn storage(&self, address: &Address, slot: &U256) -> U256;
    fn set_storage(&mut self, address: &Address, slot: U256, value: U256);
    fn block_hash(&self, number: u64) -> U256;
}

pub struct StubDatabase;

impl Database for StubDatabase {
    fn balance(&self, _address: &Address) -> U256 { U256::ZERO }
    fn code(&self, _address: &Address) -> Vec<u8> { Vec::new() }
    fn code_hash(&self, _address: &Address) -> U256 { U256::ZERO }
    fn storage(&self, _address: &Address, _slot: &U256) -> U256 { U256::ZERO }
    fn set_storage(&mut self, _address: &Address, _slot: U256, _value: U256) {}
    fn block_hash(&self, _number: u64) -> U256 { U256::ZERO }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_database_returns_zeros() {
        let db = StubDatabase;
        assert_eq!(db.balance(&[0u8; 20]), U256::ZERO);
        assert!(db.code(&[0u8; 20]).is_empty());
        assert_eq!(db.code_hash(&[0u8; 20]), U256::ZERO);
        assert_eq!(db.storage(&[0u8; 20], &U256::ZERO), U256::ZERO);
        assert_eq!(db.block_hash(0), U256::ZERO);
    }
}
