#[allow(non_camel_case_types)]
pub struct LCONST_1 {}

impl Instruction for LCONST_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_long(1);
    }
}