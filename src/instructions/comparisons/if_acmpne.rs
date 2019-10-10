#[allow(non_camel_case_types)]
pub struct IF_ACMPNE {
    base: BranchInstruction
}
impl IF_ACMPNE {
    pub fn new() -> Self {
        Self { base: BranchInstruction::new() }
    }
}

impl Instruction for IF_ACMPNE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.base.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let val2 = stack.pop_ref();
        let val1 = stack.pop_ref();
        if val1 != val2 {
            self.base.branch(frame);
        }
    }
}