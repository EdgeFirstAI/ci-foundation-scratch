//! Proof crate for EdgeFirstAI shared CI (EDGEAI-1553).

/// Identity used by the Quick-tier test.
pub fn ping() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_ok() {
        assert_eq!(ping(), "ok");
    }
}
