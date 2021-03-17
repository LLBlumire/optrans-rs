#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
/// Commit failure extends a user failure type with
pub enum CommitFailure<F> {
    /// A user defined operation failure
    OperationFailure(F),
    /// A failure caused by issues not matching
    InvalidIssue,
    /// A failure caused by bases not matching
    InvalidBase,
}
impl<F> From<F> for CommitFailure<F> {
    fn from(from: F) -> CommitFailure<F> {
        CommitFailure::OperationFailure(from)
    }
}
