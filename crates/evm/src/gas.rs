use crate::error::EvmError;

#[derive(Debug, Clone)]
pub struct Gas {
    limit: u64,
    used: u64,
}

impl Gas {
    pub fn new(limit: u64) -> Self {
        Self { limit, used: 0 }
    }

    pub fn consume(&mut self, cost: u64) -> Result<(), EvmError> {
        if self.used + cost > self.limit {
            return Err(EvmError::OutOfGas);
        }
        self.used += cost;
        Ok(())
    }

    pub fn remaining(&self) -> u64 {
        self.limit - self.used
    }

    pub fn used(&self) -> u64 {
        self.used
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_gas_tracker() {
        let gas = Gas::new(1000);
        assert_eq!(gas.remaining(), 1000);
        assert_eq!(gas.used(), 0);
    }

    #[test]
    fn consume_gas() {
        let mut gas = Gas::new(100);
        assert!(gas.consume(30).is_ok());
        assert_eq!(gas.remaining(), 70);
        assert_eq!(gas.used(), 30);
    }

    #[test]
    fn consume_exact_limit() {
        let mut gas = Gas::new(10);
        assert!(gas.consume(10).is_ok());
        assert_eq!(gas.remaining(), 0);
    }

    #[test]
    fn consume_over_limit_fails() {
        let mut gas = Gas::new(10);
        assert!(gas.consume(11).is_err());
        assert_eq!(gas.remaining(), 10);
    }
}
