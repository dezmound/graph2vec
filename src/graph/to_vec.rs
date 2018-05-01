use std::fmt;
pub trait ToVec {
    fn to_vec(&self) -> Vec<f32>;
}
impl fmt::Debug for ToVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ToVec {{ }} ")
    }
}