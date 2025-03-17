
pub trait StateSet {
    const NAME: &str;
    const SIZE: usize;
    const MAX_DEPTH: u8;
    fn solved() -> Self;
    fn from_hash(hash: usize) -> Self;
    fn get_hash(&self) -> usize;
    fn get_sym_hashes(&self) -> Vec<usize>;
    fn get_next_state_sets(&self) -> Vec<Self> where Self: Sized;
}