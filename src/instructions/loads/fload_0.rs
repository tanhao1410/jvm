#[allow(non_camel_case_types)]
pub struct FLOAD_0 {}

impl Instruction for FLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        FLOAD::_fload(frame, 0)
    }
}