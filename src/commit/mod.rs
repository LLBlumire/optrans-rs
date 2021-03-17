use crate::*;

mod data;
pub use data::*;

mod failure;
pub use failure::*;

/// A commit is a checked operation. The operation can only occur on data that expects
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Commit<O> {
    /// The data issue that the commit expects to run on, randomly determined by teh commitable data
    pub issue: u64,
    /// The base the commit is designed to apply to
    pub base: u128,
    /// The unique identifier of this commit
    pub id: u128,
    /// The operation this commit will run
    pub operation: O,
}

impl<O> Commit<O>
where
    O: TransformableOperation,
{
    /// Constructs a new commit on a given base [`CommitData`]
    pub fn on(base: &CommitableData<O::Operand>, operation: O) -> Commit<O> {
        Commit {
            issue: base.issue,
            base: base.last_commit,
            id: rand::random(),
            operation,
        }
    }
}

impl<O> TransformableOperation for Commit<O>
where
    O: TransformableOperation,
{
    type Operand = CommitableData<O::Operand>;
    type ApplicationFailure = CommitFailure<O::ApplicationFailure>;
    type TransformationFailure = CommitFailure<O::TransformationFailure>;
    fn apply(&self, to: &mut Self::Operand) -> Result<(), Self::ApplicationFailure> {
        if to.issue != self.issue {
            return Err(CommitFailure::InvalidIssue);
        }
        if to.last_commit != self.base {
            return Err(CommitFailure::InvalidBase);
        }
        self.operation.apply(&mut to.data)?;
        to.last_commit = self.id;
        Ok(())
    }
    fn transform(&self, other: &mut Self) -> Result<(), Self::TransformationFailure> {
        if other.issue != self.issue {
            return Err(CommitFailure::InvalidIssue);
        }
        if other.base != self.base {
            return Err(CommitFailure::InvalidBase);
        }
        self.operation.transform(&mut other.operation)?;
        other.base = self.id;
        Ok(())
    }
}
