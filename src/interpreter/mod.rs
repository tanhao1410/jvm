
use crate::rtda::Thread;
use crate::instructions::*;
use std::sync::Arc;
use crate::rtda::heap::Method;

pub fn interpret(method: Arc<Method>) {
    let mut thread = Thread::new();
    Thread::new_frame(thread.clone(), method.clone());

    let mut thread = crate::util::arc_util::as_mut_ref(thread);

    _loop(thread, method.code());
}

fn _loop(thread: &mut Thread, bytecode: Arc<Vec<u8>>) {
    let mut frame = thread.pop_frame();
    let mut reader = BytecodeReader::new(bytecode.as_ref(), 0);

    loop {
        let pc = frame.next_pc();
        thread.set_pc(pc);

        reader.reset(pc);
        let opcode = reader.read_u8();

        let mut inst = new_instruction(opcode);
        inst.fetch_operands(&mut reader);
        frame.set_next_pc(reader.pc());

        // println!("pc:{}, inst:{}", pc, inst);
        inst.execute(&mut frame);

        frame.operand_stack().dbg_print_top();
    }
}