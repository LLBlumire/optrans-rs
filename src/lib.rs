#![deny(missing_docs)]
//! # OpTrans
//! Generic operational transformations for rust
//!
//! All types that can be operated upon must have a single legal operation, and implement [`Operand`]
//!
//! All operations must implement [`TransformableOperation`]
//!
//! You can use [`Commit`] and [`CommitData`] to add in additional validation and checks

mod commit;
pub use commit::*;

/// Marks data that can be operated upon. Data should only have one valid operation
pub trait Operand {
    /// The operation that will modify this operand, as a type can only implement operand once in your program
    /// there can only be one legal operation for any given type.
    type Operation: TransformableOperation;
}

/// Defines an operation
pub trait TransformableOperation {
    /// The operand that this TransformableOperation will apply to
    type Operand: Operand<Operation = Self>;
    /// Possible failure states of applying this operation
    type ApplicationFailure;
    /// Possible failure to transforming the operation
    type TransformationFailure;
    /// Apply this operation to some data
    fn apply(&self, to: &mut Self::Operand) -> Result<(), Self::ApplicationFailure>;
    /// Transform other to validly mutate data that has been modified by self
    fn transform(&self, other: &mut Self) -> Result<(), Self::TransformationFailure>;
}
