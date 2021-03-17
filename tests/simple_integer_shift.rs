use optrans::*;

#[derive(Debug, PartialEq)]
pub struct Register {
    value: i32,
}
impl Operand for Register {
    type Operation = RegisterShift;
}

#[derive(Debug, PartialEq)]
pub struct RegisterShift {
    shift: i32,
}

impl TransformableOperation for RegisterShift {
    type Operand = Register;
    type ApplicationFailure = ();
    type TransformationFailure = ();
    fn apply(&self, to: &mut Self::Operand) -> Result<(), Self::ApplicationFailure> {
        to.value += self.shift;
        Ok(())
    }
    fn transform(&self, other: &mut Self) -> Result<(), Self::TransformationFailure> {
        other.shift -= self.shift;
        Ok(())
    }
}

#[test]
fn integer_shift_test() {
    let mut base = Register { value: 5 };

    let mut op_on_machine_1 = RegisterShift { shift: 4 }; // shift up to 9

    let op_on_machine_2 = RegisterShift { shift: -1 }; // shift down to 4

    // machine 2 gets in first
    op_on_machine_2.apply(&mut base).unwrap();
    assert_eq!(base, Register { value: 4 });

    // machine 1 comes in, we need to update it
    op_on_machine_2.transform(&mut op_on_machine_1).unwrap();
    // our machine 1 op should now be plus 5, to make the shift to 9 work
    assert_eq!(op_on_machine_1, RegisterShift { shift: 5 });

    op_on_machine_1.apply(&mut base).unwrap();
    assert_eq!(base, Register { value: 9 });
}
