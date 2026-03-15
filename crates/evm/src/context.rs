use crate::database::Address;
use ruint::aliases::U256;

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub address: Address,
    pub caller: Address,
    pub origin: Address,
    pub value: U256,
    pub calldata: Vec<u8>,
    pub gas_limit: u64,
    pub gas_price: U256,
    pub code: Vec<u8>,
    pub block_number: u64,
    pub block_timestamp: u64,
    pub block_coinbase: Address,
    pub block_gas_limit: u64,
    pub chain_id: u64,
    pub base_fee: U256,
    pub difficulty: U256,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            address: [0u8; 20],
            caller: [0u8; 20],
            origin: [0u8; 20],
            value: U256::ZERO,
            calldata: Vec::new(),
            gas_limit: 0,
            gas_price: U256::ZERO,
            code: Vec::new(),
            block_number: 0,
            block_timestamp: 0,
            block_coinbase: [0u8; 20],
            block_gas_limit: 0,
            chain_id: 1,
            base_fee: U256::ZERO,
            difficulty: U256::ZERO,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_context() {
        let ctx = ExecutionContext::default();
        assert_eq!(ctx.gas_limit, 0);
        assert!(ctx.calldata.is_empty());
        assert!(ctx.code.is_empty());
    }
}
