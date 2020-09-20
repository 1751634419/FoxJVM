use crate::class::simple_loader::class_reader::ClassReader;
use crate::class::simple_loader::constant_pool::{ConstantPool, ConstantInfo};
use crate::class::*;
use std::fs::read;

pub trait AttributeInfo {
    fn read_data(&mut self, data: Vec<u8>, constant_pool: &ConstantPool);

    fn get_name(&self) -> &str;
}

pub struct UnknownAttributeInfo {
    data: Vec<u8>,
}

impl AttributeInfo for UnknownAttributeInfo {
    fn read_data(&mut self, data: Vec<u8>, constant_pool: &ConstantPool) {
        self.data = data;
    }

    fn get_name(&self) -> &str {
        return "UnknownAttribute";
    }
}

pub fn read_attribute_info_vec(reader: &mut ClassReader, cp: &ConstantPool) -> Vec<Box<dyn AttributeInfo>> {
    let n = reader.read_u16();
    let mut vec : Vec<Box<dyn AttributeInfo>> = vec![];

    for i in 0..n {
        vec.push(read_attribute_info(reader, cp));
    }

    return vec;
}

pub fn read_attribute_info(reader: &mut ClassReader, cp: &ConstantPool) -> Box<dyn AttributeInfo> {
    let name_index = reader.read_u16() as usize;
    let name = cp.get_utf8(name_index).unwrap().get_str().as_str();

    let data_len = reader.read_u32() as usize;
    let data = reader.read_data(data_len);

    let mut info : Box<dyn AttributeInfo>;
    match name {
        "Code" => info = Box::new(CodeAttribute{
            max_stack: 0,
            max_locals: 0,
            code: vec![],
            exception_table: vec![],
            attributes: vec![]
        }),

        "ConstantValue" => info = Box::new(ConstantValueAttribute { constant_value_index: 0 }),

        "InnerClasses" => info = Box::new(InnerClassesAttribute { classes: vec![] }),

        "Exceptions" => info = Box::new(ExceptionsAttribute { exception_index_table: vec![] }),

        "Signature" => info = Box::new(SignatureAttribute{ signature_index: 0 }),

        "StackMapTable" => info = Box::new(StackMapTableAttribute{ data: vec![] }),

        "LineNumberTable" => info = Box::new(LineNumberTableAttribute{ line_number_entries: vec![] }),
        
        "LocalVariableTable" => info = Box::new(LocalVariableTableAttribute{ local_var_table_entries: vec![] }),

        "SourceFile" => info = Box::new(SourceFileAttribute{ source_file_index: 0 }),

        _ => {
            // todo
            print!("Unknown attribute: {}\n", name);
            info = Box::new(UnknownAttributeInfo{ data: vec![] })
        }
    }
    info.read_data(data, cp);

    return info;
}

/*
Code_attribute {
    *u2 attribute_name_index;
    u4 attribute_length;*

    // read_info:
    u2 max_stack;
    u2 max_locals;
    u4 code_length;
    u1 code[code_length];
    u2 exception_table_length;
    {   u2 start_pc;
        u2 end_pc;
        u2 handler_pc;
        u2 catch_type;
    } exception_table[exception_table_length];
    u2 attributes_count;
    attribute_info attributes[attributes_count];
}
*/

pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<Box<dyn AttributeInfo>>
}

impl CodeAttribute {
    fn read_exception_table(reader: &mut ClassReader) -> Vec<ExceptionTableEntry> {
        let n = reader.read_u16();
        let mut vec: Vec<ExceptionTableEntry> = vec![];

        for i in 0..n {
            vec.push(ExceptionTableEntry{
                start_pc: reader.read_u16(),
                end_pc: reader.read_u16(),
                handler_pc: reader.read_u16(),
                catch_type: reader.read_u16(),
            })
        }

        return vec;
    }
}

impl AttributeInfo for CodeAttribute {
    fn read_data(&mut self, data: Vec<u8>, constant_pool: &ConstantPool) {
        let mut reader = ClassReader::new(data);

        self.max_stack = reader.read_u16();
        self.max_locals = reader.read_u16();
        let code_len = reader.read_u32() as usize;
        self.code = reader.read_data(code_len);
        self.exception_table = CodeAttribute::read_exception_table(&mut reader);
        self.attributes = read_attribute_info_vec(&mut reader, constant_pool);
    }

    fn get_name(&self) -> &str {
        return "Code";
    }
}

/*
ConstantValue_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 constantvalue_index;
}
*/
pub struct ConstantValueAttribute {
    constant_value_index: u16
}

impl AttributeInfo for ConstantValueAttribute {
    fn read_data(&mut self, data: Vec<u8>, constant_pool: &ConstantPool) {
        let mut reader = ClassReader::new(data);
        self.constant_value_index = reader.read_u16();
    }

    fn get_name(&self) -> &str {
        return "ConstantValue";
    }
}

/*
InnerClasses_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 number_of_classes;
    {   u2 inner_class_info_index;
        u2 outer_class_info_index;
        u2 inner_name_index;
        u2 inner_class_access_flags;
    } classes[number_of_classes];
}
*/
pub struct InnerClassesEntry {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

pub struct InnerClassesAttribute {
    classes: Vec<InnerClassesEntry>
}

impl AttributeInfo for InnerClassesAttribute {
    fn read_data(&mut self, data: Vec<u8>, constant_pool: &ConstantPool) {
        let mut reader = ClassReader::new(data);
        let n = reader.read_u16();
        let mut vec: Vec<InnerClassesEntry> = vec![];

        for i in 0..n {
            vec.push(InnerClassesEntry{
                inner_class_info_index: reader.read_u16(),
                outer_class_info_index: reader.read_u16(),
                inner_name_index: reader.read_u16(),
                inner_class_access_flags: reader.read_u16()
            });
        }

        self.classes = vec;
    }

    fn get_name(&self) -> &str {
        return "InnerClasses"
    }
}

/*
Exceptions_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 number_of_exceptions;
    u2 exception_index_table[number_of_exceptions];
}
*/
pub struct ExceptionsAttribute {
    exception_index_table: Vec<u16>
}

impl AttributeInfo for ExceptionsAttribute {
    fn read_data(&mut self, data: Vec<u8>, constant_pool: &ConstantPool) {
        let mut reader = ClassReader::new(data);
        self.exception_index_table = reader.read_u16s();
    }

    fn get_name(&self) -> &str {
        return "Exceptions";
    }
}

/*
Signature_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 signature_index;
}
*/
pub struct SignatureAttribute {
    signature_index: u16
}

impl AttributeInfo for SignatureAttribute {
    fn read_data(&mut self, data: Vec<u8>, constant_pool: &ConstantPool) {
        let mut reader = ClassReader::new(data);
        self.signature_index = reader.read_u16();
    }

    fn get_name(&self) -> &str {
        return "Signature";
    }
}

/* StackMapTable_attribute {
 u2 attribute_name_index;
 u4 attribute_length;
 u2 number_of_entries;
 stack_map_frame entries[number_of_entries];
}
 */
// TODO FUTURE
pub struct StackMapTableAttribute {
    data: Vec<u8>
}

impl AttributeInfo for StackMapTableAttribute {
    fn read_data(&mut self, data: Vec<u8>, constant_pool: &ConstantPool) {
        self.data = data;
    }

    fn get_name(&self) -> &str {
        return "StackMapTable";
    }
}

/*
LineNumberTable_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 line_number_table_length;
    {   u2 start_pc;
        u2 line_number;
    } line_number_table[line_number_table_length];
}
*/
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16
}

pub struct LineNumberTableAttribute {
    line_number_entries: Vec<LineNumberTableEntry>
}

impl AttributeInfo for LineNumberTableAttribute {
    fn read_data(&mut self, data: Vec<u8>, constant_pool: &ConstantPool) {
        let mut reader = ClassReader::new(data);
        let n = reader.read_u16() as usize;
        let mut vec: Vec<LineNumberTableEntry> = vec![];

        for i in 0..n {
            vec.push(LineNumberTableEntry {
                start_pc: reader.read_u16(),
                line_number: reader.read_u16()
            })
        }

        self.line_number_entries = vec;
    }

    fn get_name(&self) -> &str {
        return "LineNumberTable";
    }
}

/*
LocalVariableTable_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 local_variable_table_length;
    {   u2 start_pc;
        u2 length;
        u2 name_index;
        u2 descriptor_index;
        u2 index;
    } local_variable_table[local_variable_table_length];
}
*/
pub struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16
}

pub struct LocalVariableTableAttribute {
    local_var_table_entries: Vec<LocalVariableTableEntry>
}

impl AttributeInfo for LocalVariableTableAttribute {
    fn read_data(&mut self, data: Vec<u8>, constant_pool: &ConstantPool) {
        let mut reader = ClassReader::new(data);
        let n = reader.read_u16() as usize;
        let mut vec: Vec<LocalVariableTableEntry> = vec![];

        for i in 0..n {
            vec.push(LocalVariableTableEntry {
                start_pc: reader.read_u16(),
                length: reader.read_u16(),
                name_index: reader.read_u16(),
                descriptor_index: reader.read_u16(),
                index: reader.read_u16()
            });
        }

        self.local_var_table_entries = vec;
    }

    fn get_name(&self) -> &str {
        return "LocalVariableTable";
    }
}

/*
SourceFile_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 sourcefile_index;
}
*/
pub struct SourceFileAttribute {
    source_file_index: u16
}

impl AttributeInfo for SourceFileAttribute {
    fn read_data(&mut self, data: Vec<u8>, constant_pool: &ConstantPool) {
        let mut reader = ClassReader::new(data);

        self.source_file_index = reader.read_u16();
    }

    fn get_name(&self) -> &str {
        return "SourceFile"
    }
}