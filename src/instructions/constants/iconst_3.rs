struct ICOUNT_3 {}

impl Instruction for ICOUNT_3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(3);
    }
}