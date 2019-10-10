#[allow(non_camel_case_types)]
pub struct DUP {}

impl Instruction for DUP {
    /*
    bottom -> top
    [...][c][b][a]
                 \_
                   |
                   V
    [...][c][b][a][a]
    */
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let slot = stack.pop_slot();
        stack.push_slot(slot);
        stack.push_slot(slot);
    }
}