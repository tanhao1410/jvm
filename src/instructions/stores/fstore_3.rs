#[allow(non_camel_case_types)]
pub struct FSTORE_3 {}

impl Instruction for FSTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        FSTORE::_fstore(frame, 3);
    }
}