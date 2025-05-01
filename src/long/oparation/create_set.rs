use super::Uvec;

pub fn create_set(n: u64) -> Uvec {
    Uvec {
        unit: vec![n]
    }
}