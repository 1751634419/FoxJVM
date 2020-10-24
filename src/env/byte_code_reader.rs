use std::convert::TryFrom;

pub struct ByteCodeReader {
    code: Vec<u8>,
    pub pc: usize,
}

impl ByteCodeReader {
    pub fn new(code: Vec<u8>) -> ByteCodeReader {
        return ByteCodeReader {
            code,
            pc: 0,
        }
    }

    pub fn read_i32(&mut self) -> i32 {
        let b1 = i32::from(self.read_u8());
        let b2 = i32::from(self.read_u8());
        let b3 = i32::from(self.read_u8());
        let b4 = i32::from(self.read_u8());
        return (b1 << 24) | (b2 << 16) | (b3 << 8) | b4;
    }

    pub fn read_i32s(&mut self, n: usize) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::new();
        for i in 0..n {
            vec.push(self.read_i32());
        }
        return vec;
    }

    pub fn skip_padding(&mut self) {
        while self.pc%4 != 0 {
            self.read_u8();
        }
    }

    pub fn read_i16(&mut self) -> i16 {
        return i16::try_from(self.read_u16()).unwrap();
    }

    pub fn read_u16(&mut self) -> u16 {
        let b1 = u16::from(self.read_u8());
        let b2 = u16::from(self.read_u8());
        return (b1 << 8) | b2;
    }

    pub fn read_i8(&mut self) -> i8 {
        return i8::try_from(self.read_u8()).unwrap();
    }

    pub fn read_u8(&mut self) -> u8 {
        let d = self.code[self.pc];
        self.pc += 1;
        return d;
    }

    pub fn reset(&mut self, code: Vec<u8>, pc: usize) {
        self.code = code;
        self.pc = pc;
    }

    // pub fn read_reversed_data(&mut self, size: usize) -> Vec<u8> {
    //     let data = &self.data[self.pc .. self.pc + size];
    //     self.pc += size;
    //
    //     let mut rev: Vec<u8> = vec!();
    //     for i in 0..data.len() {
    //         rev.push(data[i]);
    //     }
    //     rev.reverse();
    //
    //     return rev;
    // }
    //
    // pub fn read_data(&mut self, size: usize) -> Vec<u8> {
    //     let src_data = &self.data[self.pc .. self.pc + size];
    //     self.pc += size;
    //
    //     return src_data.to_vec();
    // }
}