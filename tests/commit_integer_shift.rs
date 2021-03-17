use optrans::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Register {
    value: i32,
}
impl Operand for Register {
    type Operation = RegisterShift;
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
    let base = CommitableData::new(Register { value: 5 });

    let op_on_machine_1 = Commit::on(&base, RegisterShift { shift: 4 }); // shift up to 9

    let op_on_machine_2 = Commit::on(&base, RegisterShift { shift: -1 }); // shift down to 4

    {
        // server
        let mut base = base.clone();
        let mut op_on_machine_1 = op_on_machine_1.clone();
        let op_on_machine_2 = op_on_machine_2.clone();

        // machine 2 gets in first
        op_on_machine_2.apply(&mut base).unwrap();
        assert_eq!(base.data, Register { value: 4 });

        // machine 1 comes in, we need to update it
        op_on_machine_2.transform(&mut op_on_machine_1).unwrap();
        // our machine 1 op should now be plus 5, to make the shift to 9 work
        assert_eq!(op_on_machine_1.operation, RegisterShift { shift: 5 });

        op_on_machine_1.apply(&mut base).unwrap();
        assert_eq!(base.data, Register { value: 9 });
    }
    {
        // on machine 1
        let synced_base = base.clone();
        let mut base = synced_base.clone();
        let mut op_on_machine_1 = op_on_machine_1.clone();
        op_on_machine_1.apply(&mut base).unwrap();
        // when we go to sync to the server, we get told that machine 2 actually got in first!
        let op_on_machine_2 = op_on_machine_2.clone();
        // we need to reset our base, the server will provide us a new base in the real world,
        // but here (for our test) it's the old synced_base with transform 2 done on it
        let mut base = synced_base.clone();
        op_on_machine_2.apply(&mut base).unwrap();
        // We need to transform our base with the new update we've been told about
        op_on_machine_2.transform(&mut op_on_machine_1).unwrap();
        op_on_machine_1.apply(&mut base).unwrap();
        // and then we're good to send it off to the server!

        assert_eq!(base.data, Register { value: 9 });
    }
    {
        // on machine 2
        let synced_base = base.clone();
        let mut base = synced_base.clone();
        let op_on_machine_2 = op_on_machine_2.clone();
        op_on_machine_2.apply(&mut base).unwrap();
        // we sync to the server, they say it's all good, a while later they tell us about a new update
        // the server wouldn't tell us about the original operation, but the mutated one, but we'll just
        // do that locally for our test
        let mut op_on_machine_1 = op_on_machine_1.clone();
        op_on_machine_2.transform(&mut op_on_machine_1).unwrap();
        op_on_machine_1.apply(&mut base).unwrap();

        assert_eq!(base.data, Register { value: 9 });
    }
}
