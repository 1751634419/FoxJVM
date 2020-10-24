use crate::env::byte_code_reader::ByteCodeReader;
use crate::env::basic_env_elements::Frame;

pub trait Instruction {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader);

    fn execute(&mut self, frame: &mut Frame);
}