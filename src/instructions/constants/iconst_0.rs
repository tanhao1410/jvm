#[allow(non_camel_case_types)]
pub struct ICONST_0 {}

impl Instruction for ICONST_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(0);
    }
}