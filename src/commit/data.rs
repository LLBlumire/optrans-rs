use crate::*;

/// Commitable data is data which can receive checked commits
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CommitableData<T> {
    /// The internal data which commits are applied to
    pub data: T,
    /// The issue of the data commits must match
    pub issue: u64,
    /// The id of the last commit applied
    pub last_commit: u128,
}

impl<T> CommitableData<T> {
    /// Creates a new instance of commitable data, with randomly initialised `issue` and `last_commit`.
    pub fn new(data: T) -> CommitableData<T> {
        CommitableData {
            data,
            issue: rand::random(),
            last_commit: rand::random(),
        }
    }
}

impl<T> Operand for CommitableData<T>
where
    T: Operand,
{
    type Operation = Commit<T::Operation>;
}
