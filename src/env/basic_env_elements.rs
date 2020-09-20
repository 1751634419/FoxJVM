use std::ops::Deref;

pub struct Frame<'env> {
    lower: Option<&'env mut Frame<'env>>,

}

// impl Frame {
//     fn set_lower(&mut self, frame: Option<* mut Frame>) {
//         self.lower = frame;
//     }
//
//     fn get_lower(&mut self) -> Option<* mut Frame> {
//         self.lower
//     }
// }

pub struct Stack<'env> {
    max_size: u32,
    curr_size: u32,
    top: Option<&'env mut Frame<'env>>
}

use std::mem::replace;
impl<'env> Stack<'env> {
    pub fn new(max_size: u32) -> Stack<'env> {
        return Stack {
            max_size,
            curr_size: 0,
            top: None
        }
    }

    pub fn push(&mut self, frame: &'env mut Frame<'env>) {
        if self.curr_size == self.max_size {
            panic!("The stack is overflowed.");
        }

        if let Some(v) = self.top {
            frame.lower = self.top;
            self.top = Option::Some(frame);
        }


        self.curr_size += 1;
    }

    pub fn pop(&mut self) -> Option<&mut Frame<'env>> {
        if let Some(mut v) = self.top {
            // unsafe {
            //     self.top = (*v).lower;
            //     (*v).lower = None;
            // }
            self.top = v.lower;
            v.lower = None;
            return Option::Some(v);
        }

        return None;
    }
}

pub struct Thread<'env> {
    pc: u32,
    stack: &'env Stack<'env>
}