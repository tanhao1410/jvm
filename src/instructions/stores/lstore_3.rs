#[allow(non_camel_case_types)]
pub struct LSTORE_3 {}

impl Instruction for LSTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        LSTORE::_lstore(frame, 3);
    }
}