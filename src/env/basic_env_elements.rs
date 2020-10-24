use std::ops::Deref;
use std::any::Any;
use crate::env::object::Object;

pub struct Frame {
    pub local_vars: LocalVars,
    pub operand_stack: OperandStack,
}

impl Frame {
    pub fn new(max_locals: usize, max_stack: usize) -> Frame {
        return Frame {
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_stack),
        }
    }
}

pub struct Slot {
    pub val_num: i32,
    pub val_obj: Option<Object>
}

pub struct LocalVars {
    vec: Vec<Slot>
}

impl LocalVars {
    pub fn new(size: usize) -> LocalVars {
        let mut vec = Vec::new();
        for i in 0..size {
            vec.push(Slot { val_num: 0, val_obj: None });
        }

        return LocalVars {
            vec,
        }
    }

    pub fn set_i64(&mut self, n: usize, v: i64) {
        let bytes:[u8; 8] = v.to_be_bytes();
        let a: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let b: [u8; 4] = [bytes[4], bytes[5], bytes[6], bytes[7]];
        self.set_i32(n, i32::from_be_bytes(a));
        self.set_i32(n + 1, i32::from_be_bytes(b));
    }

    pub fn set_i32(&mut self, n: usize, v: i32) {
        let mut slot = self.vec.get_mut(n).unwrap();
        slot.val_num = v;
    }

    pub fn get_i32(&self, n: usize) -> Option<i32> {
        if let Some(v) = self.vec.get(n) {
            let val = v.val_num;
            return Some(val);
        } else {
            return None;
        }
    }

    pub fn get_i64(&self, n: usize) -> Option<i64> {
        if let Some(va) = self.vec.get(n) {
            if let Some(vb) = self.vec.get(n + 1) {
                let ab = va.val_num.to_be_bytes();
                let bb = vb.val_num.to_be_bytes();
                let bytes = [ab[0], ab[1], ab[2], ab[3], bb[0], bb[1], bb[2], bb[3]];
                return Some(i64::from_be_bytes(bytes));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn set_f32(&mut self, n: usize, v: f32) {
        self.set_i32(n, i32::from_be_bytes(v.to_be_bytes()));
    }

    pub fn get_f32(&mut self, n: usize) -> Option<f32> {
        if let Some(v) = self.get_i32(n) {
            return Some(f32::from_be_bytes(v.to_be_bytes()));
        } else {
            return None;
        }
    }

    pub fn set_f64(&mut self, n: usize, v: f64) {
        self.set_i64(n, i64::from_be_bytes(v.to_be_bytes()));
    }

    pub fn get_f64(&mut self, n: usize) -> Option<f64> {
        if let Some(v) = self.get_i64(n) {
            return Some(f64::from_be_bytes(v.to_be_bytes()));
        } else {
            return None;
        }
    }

    pub fn set_obj(&mut self, n: usize, v: Object) {
        let mut slot = self.vec.get_mut(n).unwrap();
        slot.val_obj = Option::Some(v);
    }

    pub fn get_obj(&self, n: usize) -> &Option<Object> {
        if let Some(v) = self.vec.get(n) {
            return &v.val_obj;
        } else {
            return &None;
        }
    }
}

pub struct OperandStack {
    local_vars: LocalVars,
    index: usize,
}

impl OperandStack {
    pub fn new(size: usize) -> OperandStack {
        return OperandStack {
            local_vars: LocalVars::new(size),
            index: 0
        }
    }

    pub fn push_i32(&mut self, v: i32) {
        self.local_vars.set_i32(self.index, v);
        self.index += 1;
    }

    pub fn pop_i32(&mut self) -> Option<i32> {
        self.index -= 1;
        return self.local_vars.get_i32(self.index);
    }

    pub fn push_i64(&mut self, v: i64) {
        self.local_vars.set_i64(self.index, v);
        self.index += 2;
    }

    pub fn pop_i64(&mut self) -> Option<i64> {
        self.index -= 2;
        return self.local_vars.get_i64(self.index);
    }

    pub fn push_f32(&mut self, v: f32) {
        self.local_vars.set_f32(self.index, v);
        self.index += 1;
    }

    pub fn pop_f32(&mut self) -> Option<f32> {
        self.index -= 1;
        return self.local_vars.get_f32(self.index);
    }

    pub fn push_f64(&mut self, v: f64) {
        self.local_vars.set_f64(self.index, v);
        self.index += 2;
    }

    pub fn pop_f64(&mut self) -> Option<f64> {
        self.index -= 2;
        return self.local_vars.get_f64(self.index);
    }

    pub fn push_obj(&mut self, obj: Object) {
        self.local_vars.set_obj(self.index, obj);
        self.index -= 1;
    }

    pub fn pop_obj(&mut self) -> &Option<Object> {
        self.index -= 1;
        return self.local_vars.get_obj(self.index);
    }
}

pub struct Stack {
    max_size: usize,
    vec: Vec<Box<Frame>>
}

impl Stack {
    pub fn new(max_size: usize) -> Stack {
        return Stack {
            max_size,
            vec: vec![]
        }
    }

    pub fn push(&mut self, frame: Box<Frame>) {
        if self.vec.len() == self.max_size {
            panic!("The stack is overflowed.");
        }

        self.vec.push(frame);
    }

    pub fn pop(&mut self) -> Option<Box<Frame>> {
        self.vec.pop()
    }

    pub fn get_current_frame(&mut self) -> Option<&Box<Frame>> {
        self.vec.get(self.vec.len() - 1)
    }
}

pub struct Thread {
    pub (crate) pc: u32,
    pub (crate) stack: Stack
}