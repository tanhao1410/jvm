struct F2I {}

impl Instruction for F2I {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let f = stack.pop_float();
        let i = f as i32;
        stack.push_int(i);
    }
}