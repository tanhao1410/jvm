#[allow(non_camel_case_types)]
pub struct FSTORE_0 {}

impl Instruction for FSTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        FSTORE::_fstore(frame, 0);
    }
}