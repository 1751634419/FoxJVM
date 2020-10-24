use crate::env::inst::Instruction;
use crate::env::basic_env_elements::Frame;
use crate::env::byte_code_reader::ByteCodeReader;

fn iload(frame: &mut Frame, index: u8) {
    let val = frame.local_vars.get_i32(index as usize);
    frame.operand_stack.push_i32(val.unwrap());
}

pub struct ILoad {
    index: u8,
}

impl Instruction for ILoad {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        iload(frame, self.index);
    }
}

pub struct ILoad_0 {
}

impl Instruction for ILoad_0 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        iload(frame, 0);
    }
}

pub struct ILoad_1 {
}

impl Instruction for ILoad_1 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        iload(frame, 1);
    }
}

pub struct ILoad_2 {
}

impl Instruction for ILoad_2 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        iload(frame, 2);
    }
}

pub struct ILoad_3 {
}

impl Instruction for ILoad_3 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        iload(frame, 3);
    }
}

fn lload(frame: &mut Frame, index: u8) {
    let val = frame.local_vars.get_i64(index as usize);
    frame.operand_stack.push_i64(val.unwrap());
}

pub struct LLoad {
    index: u8,
}

impl Instruction for LLoad {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        lload(frame, self.index);
    }
}

pub struct LLoad_0 {
}

impl Instruction for LLoad_0 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        lload(frame, 0);
    }
}

pub struct LLoad_1 {
}

impl Instruction for LLoad_1 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        lload(frame, 1);
    }
}

pub struct LLoad_2 {
}

impl Instruction for LLoad_2 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        lload(frame, 2);
    }
}

pub struct LLoad_3 {
}

impl Instruction for LLoad_3 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        lload(frame, 3);
    }
}

fn fload(frame: &mut Frame, index: u8) {
    let val = frame.local_vars.get_f32(index as usize);
    frame.operand_stack.push_f32(val.unwrap());
}

pub struct FLoad {
    index: u8,
}

impl Instruction for FLoad {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        fload(frame, self.index);
    }
}

pub struct FLoad_0 {
}

impl Instruction for FLoad_0 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        fload(frame, 0);
    }
}

pub struct FLoad_1 {
}

impl Instruction for FLoad_1 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        fload(frame, 1);
    }
}

pub struct FLoad_2 {
}

impl Instruction for FLoad_2 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        fload(frame, 2);
    }
}

pub struct FLoad_3 {
}

impl Instruction for FLoad_3 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        fload(frame, 3);
    }
}

fn dload(frame: &mut Frame, index: u8) {
    let val = frame.local_vars.get_f64(index as usize);
    frame.operand_stack.push_f64(val.unwrap());
}

pub struct DLoad {
    index: u8,
}

impl Instruction for DLoad {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        dload(frame, self.index);
    }
}

pub struct DLoad_0 {
}

impl Instruction for DLoad_0 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        dload(frame, 0);
    }
}

pub struct DLoad_1 {
}

impl Instruction for DLoad_1 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        dload(frame, 1);
    }
}

pub struct DLoad_2 {
}

impl Instruction for DLoad_2 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        dload(frame, 2);
    }
}

pub struct DLoad_3 {
}

impl Instruction for DLoad_3 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        dload(frame, 3);
    }
}

fn aload(frame: &mut Frame, index: u8) {
    let val = frame.local_vars.get_ref(index as usize);
    frame.operand_stack.push_ref(val.unwrap());
}

pub struct ALoad {
    index: u8,
}

impl Instruction for ALoad {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        aload(frame, self.index);
    }
}

pub struct ALoad_0 {
}

impl Instruction for ALoad_0 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        aload(frame, 0);
    }
}

pub struct ALoad_1 {
}

impl Instruction for ALoad_1 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        aload(frame, 1);
    }
}

pub struct ALoad_2 {
}

impl Instruction for ALoad_2 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        aload(frame, 2);
    }
}

pub struct ALoad_3 {
}

impl Instruction for ALoad_3 {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        aload(frame, 3);
    }
}