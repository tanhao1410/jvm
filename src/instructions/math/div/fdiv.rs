#[allow(non_camel_case_types)]
pub struct FDIV {}

impl Instruction for FDIV {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        stack.push_float(v1 / v2);
    }
}

impl Debug for FDIV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}