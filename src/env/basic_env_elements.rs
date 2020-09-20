use std::ops::Deref;

pub struct Frame {
    lower: Option<Box<Frame>>,

}

pub struct Stack {
    max_size: u32,
    curr_size: u32,
    top: Option<Box<Frame>>
}

impl Stack {
    pub fn new(max_size: u32) -> Stack {
        return Stack {
            max_size,
            curr_size: 0,
            top: None
        }
    }

    pub fn push(&mut self, frame: &mut Frame) {
        if self.curr_size == self.max_size {
            panic!("The stack is overflowed.");
        }


        if let Some(v) = self.top {
            frame.lower = self.top;
            self.top = Option::Some(Box::new(frame));
        }

        self.curr_size += 1;
    }

    pub fn pop(&mut self) -> Option<*mut Frame> {
        if let Some(v) = self.top {
            self.top = v.lower;
            v.lower = None;
            return Option::Some(*mut Frame);
        }

        return None;
    }
}

pub struct Thread {
    pc: u32,

}