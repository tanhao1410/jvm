struct ALOAD_1 {}

impl Instruction for ALOAD_1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        ALOAD::_aload(frame, 1)
    }
}