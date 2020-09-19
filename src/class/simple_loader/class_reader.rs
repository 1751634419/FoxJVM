use crate::class::class::Class;
use crate::class::simple_loader::constant_pool::ConstantPool;

pub struct ClassFile {
    // magic
    pub(crate) minor_version: u16,
    pub(crate) major_version: u16,
    pub(crate) constant_pool: ConstantPool,
    pub(crate) access_flags: u16,
    pub(crate) this_class: u16,
    pub(crate) super_class: u16,
    pub(crate) interfaces: Vec<u16>,
    // fields
    // methods
    // attributes
}

impl ClassFile {
    pub fn new(reader: &mut ClassReader) -> ClassFile {
        let magic_number = reader.read_u32();
        if magic_number != 0xCAFEBABE {
            panic!("The magic number doesn't conform to the regulation.");
        }
        let minor_version = reader.read_u16();
        let major_version = reader.read_u16();
        let constant_pool = ConstantPool::new(reader);
        let access_flags = reader.read_u16();
        let this_class = reader.read_u16();
        let super_class = reader.read_u16();
        let interfaces = reader.read_u16s();


        return ClassFile {
            minor_version: minor_version,
            major_version: major_version,
            constant_pool: constant_pool,
            access_flags: access_flags,
            this_class: this_class,
            super_class: super_class,
            interfaces: interfaces,

        }
    }
}

pub struct ClassReader {
    data: Vec<u8>,
    point: usize,
}

// todo rewrite the implementation with safer code
impl ClassReader {
    pub fn new(data: Vec<u8>) -> ClassReader {
        return ClassReader {
            data,
            point: 0,
        }
    }

    pub fn read_u64(&mut self) -> u64 {
        let data = self.read_reversed_data(8);

        let ptr :*const u8 = data.as_ptr();
        let ptr :*const u64 = ptr as *const u64;
        let s = unsafe{ *ptr};
        return s;
    }

    pub fn read_u16(&mut self) -> u16 {
        let data = self.read_reversed_data(2);

        let ptr :*const u8 = data.as_ptr();
        let ptr :*const u16 = ptr as *const u16;
        let s = unsafe{ *ptr};
        return s;
    }

    pub fn read_u32(&mut self) -> u32 {
        let data = self.read_reversed_data(4);

        let ptr :*const u8 = data.as_ptr();
        let ptr :*const u32 = ptr as *const u32;
        let s = unsafe{ *ptr};
        return s;
    }

    pub fn read_u16s(&mut self) -> Vec<u16> {
        let n = self.read_u16();
        let mut s : Vec<u16> = vec!();
        for i in 0..n {
            s.push(self.read_u16());
        }
        return s;
    }

    pub fn read_u8(&mut self) -> u8 {
        let d = self.data[self.point];
        self.point += 1;
        return d;
    }

    pub fn read_reversed_data(&mut self, size: usize) -> Vec<u8> {
        let data = &self.data[self.point .. self.point + size];
        self.point += size;

        let mut rev: Vec<u8> = vec!();
        for i in 0..data.len() {
            rev.push(data[i]);
        }
        rev.reverse();

        return rev;
    }

    pub fn read_data(&mut self, size: usize) -> Vec<u8> {
        let src_data = &self.data[self.point .. self.point + size];
        self.point += size;

        return src_data.to_vec();
    }
}