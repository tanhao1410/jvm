#[allow(non_camel_case_types)]
pub struct I_RETURN {}

impl Instruction for I_RETURN {
    fn execute(&mut self, frame: &mut Frame) {
        let t = frame.thread();
        let mut thread = t.write().unwrap();
        let current_frame = thread.pop_frame();
        let invoker_frame = thread.top_frame();
        let ret_val = current_frame.borrow_mut().operand_stack().pop_int();
        invoker_frame.borrow_mut().operand_stack().push_int(ret_val);
    }
}

impl Debug for I_RETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}