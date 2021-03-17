mod commit;
pub use commit::*;

/// Marks data that can be operated upon. Data should only have one valid operation
pub trait Operand {
    type Operation: TransformableOperation;
}

/// Defines an operation
pub trait TransformableOperation {
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
