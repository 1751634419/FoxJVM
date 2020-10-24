use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use crate::class::simple_loader::class_reader::ClassReader;

pub struct ConstantUTF8Info {
    pub(crate) str: String,
}

impl ConstantInfo for ConstantUTF8Info {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        let size = reader.read_u16();
        // don't reverse the data
        let data = reader.read_data(size as usize);

        self.str = ConstantUTF8Info::decode_mutf8(data);

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ConstantUTF8Info {
    pub fn get_str(&self) -> &String {
        return &self.str;
    }

    fn decode_mutf8(data: Vec<u8>) -> String {
        /*
        the data vector shouldn't be reversed,
        as the following code is paraphrased from Java.
         */
        let utflen = data.len();
        let mut charvec: Vec<u16> = vec![0; utflen];

        let mut c: u16;
        let mut char2: u16;
        let mut char3: u16;
        let mut charvec_count: usize = 0;

        let mut count = 0;
        while count < utflen {
            c = data[count] as u16;
            if c > 127 {
                break
            }
            count += 1;
            charvec[charvec_count] = c;
            charvec_count += 1;
        }

        while count < utflen {
            c = data[count] as u16;
            match c >> 4 {
                0|1|2|3|4|5|6|7 => {
                    count += 1;
                    charvec[charvec_count] = c;
                    charvec_count += 1;
                }
                12|13 => {
                    count += 2;
                    if count > utflen {
                        panic!("partial character at end");
                    }
                    char2 = data[count - 1] as u16;
                    if char2 & 0xC0 != 0x80 {
                        panic!("malformed input")
                    }
                    charvec[charvec_count] = c&0x1F<<6 | char2&0x3F;
                    charvec_count += 1;
                }
                14 => {
                    count += 3;
                    if count > utflen {
                        panic!("malformed input");
                    }
                    char2 = data[count - 2] as u16;
                    char3 = data[count - 1] as u16;
                    if char2&0xC0 != 0x80 || char3&0xC0 != 0x80 {
                        panic!("malformed input")
                    }
                    charvec[charvec_count] = c&0x0F<<12 | char2&0x3F<<6 | char3&0x3F<<0;
                    charvec_count += 1;
                }
                _ => {
                    panic!("malformed input")
                }
            }
        }

        let (left, right) = charvec.split_at(charvec_count);
        return decode_utf16(left.iter().cloned()).
            map(|r| r.unwrap_or(REPLACEMENT_CHARACTER)).
            collect::<String>();
    }
}

pub struct ConstantClassInfo {
    pub(crate) name_index: u16,
}

impl ConstantInfo for ConstantClassInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.name_index = reader.read_u16();

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantStringInfo {
    pub(crate) string_index: u16,
}

impl ConstantInfo for ConstantStringInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.string_index = reader.read_u16();

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantMemberRefInfo {
    class_index: u16,
    name_and_type_index: u16,
}

impl ConstantInfo for ConstantMemberRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.class_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantIntegerInfo {
    pub(crate) val: i32,
}

impl ConstantInfo for ConstantIntegerInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.val = reader.read_u32() as i32;

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantLongInfo {
    pub(crate) val: i64,
}

impl ConstantInfo for ConstantLongInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.val = reader.read_u64() as i64;

        return true;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantDoubleInfo {
    pub(crate) val: f64,
}

impl ConstantInfo for ConstantDoubleInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        let d = reader.read_data(8);
        let mut data:[u8; 8] = [0; 8];
        for i in 0..8 {
            data[i] = d[i];
        }
        self.val = f64::from_be_bytes(data);

        return true;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantFloatInfo {
    pub(crate) val: f32,
}

impl ConstantInfo for ConstantFloatInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        let d = reader.read_data(4);
        let mut data:[u8; 4] = [0; 4];
        for i in 0..4 {
            data[i] = d[i];
        }
        self.val = f32::from_be_bytes(data);

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantFieldRefInfo {
    pub(crate) class_index: u16,
    pub(crate) name_and_type_index: u16,
}

impl ConstantInfo for ConstantFieldRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.class_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantMethodRefInfo {
    pub(crate) class_index: u16,
    pub(crate) name_and_type_index: u16,
}

impl ConstantInfo for ConstantMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.class_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantInterfaceMethodRefInfo {
    pub(crate) class_index: u16,
    pub(crate) name_and_type_index: u16,
}

impl ConstantInfo for ConstantInterfaceMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.class_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantNameAndTypeInfo {
    pub(crate) name_index: u16,
    pub(crate) descriptor_index: u16,
}

impl ConstantInfo for ConstantNameAndTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.name_index = reader.read_u16();
        self.descriptor_index = reader.read_u16();

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantMethodHandleInfo {
    pub(crate) reference_kind: u8,
    pub(crate) reference_index: u16,
}

impl ConstantInfo for ConstantMethodHandleInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.reference_kind = reader.read_u8();
        self.reference_index = reader.read_u16();

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantMethodTypeInfo {
    pub(crate) descriptor_index: u16,
}

impl ConstantInfo for ConstantMethodTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.descriptor_index = reader.read_u16();

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantInvokeDynamicInfo {
    pub(crate) bootstrap_method_attr_index: u16,
    pub(crate) name_and_type_index: u16,
}

impl ConstantInfo for ConstantInvokeDynamicInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool {
        self.bootstrap_method_attr_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();

        return false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConstantPool (Vec<Option<Box<dyn ConstantInfo>>>);

pub trait ConstantInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> bool;

    fn as_any(&self) -> &dyn Any;
}

use std::any::{Any};

fn read_constant_info(reader: &mut ClassReader) -> Box<dyn ConstantInfo> {
    let index = reader.read_u8();

    match index {
        1 => return Box::new(ConstantUTF8Info{ str: "".to_string() }),
        3 => return Box::new(ConstantIntegerInfo{ val: 0 }),
        4 => return Box::new(ConstantFloatInfo{val: 0f32}),
        5 => return Box::new(ConstantLongInfo{val: 0}),
        6 => return Box::new(ConstantDoubleInfo{val: 0f64}),
        7 => return Box::new(ConstantClassInfo{ name_index: 0 }),
        8 => return Box::new(ConstantStringInfo{ string_index: 0 }),
        9 => return Box::new(ConstantFieldRefInfo { class_index: 0, name_and_type_index: 0 }),
        10 => return Box::new(ConstantMethodRefInfo{ class_index: 0, name_and_type_index: 0 }),
        11 => return Box::new(ConstantInterfaceMethodRefInfo{ class_index: 0, name_and_type_index: 0 }),
        12 => return Box::new(ConstantNameAndTypeInfo{ name_index: 0, descriptor_index: 0 }),
        15 => return Box::new(ConstantMethodHandleInfo{ reference_kind: 0, reference_index: 0 }),
        16 => return Box::new(ConstantMethodTypeInfo{ descriptor_index: 0 }),
        18 => return Box::new(ConstantInvokeDynamicInfo{ bootstrap_method_attr_index: 0, name_and_type_index: 0 }),
        _ => {
            panic!("Unknown constant info id: {}", index);
        }
    }
}

impl ConstantPool {
    pub fn new(reader: &mut ClassReader) -> ConstantPool {
        let cp_count = reader.read_u16();
        let mut info: Vec<Option<Box<dyn ConstantInfo>>> = vec![];

        let mut i = 1;
        info.push(None); // placeholder for index #0
        while i < cp_count {
            let mut cp = read_constant_info(reader);
            let plus = cp.read_info(reader);

            info.push(Some(cp));
            i += 1;

            if plus {
                i += 1;
                info.push(None);
            }
        }

        return ConstantPool(info);
    }

    pub fn get(&self, n: usize) -> Option<&Option<Box<dyn ConstantInfo>>> {
        let v = self.0.get(n);
        return v;
    }

    pub fn get_utf8(&self, n: usize) -> Option<&ConstantUTF8Info> {
        self.get_any(n).downcast_ref::<ConstantUTF8Info>()
    }

    pub fn get_any(&self, n: usize) -> &dyn Any {
        self.get(n).unwrap().as_ref().unwrap().as_any()
    }
}