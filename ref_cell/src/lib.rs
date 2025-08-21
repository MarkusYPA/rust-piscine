use std::{cell::RefCell, rc::Rc};

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    value: RefCell<usize>,
    max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Self {
            messages: Default::default(),
            value: Default::default(),
            max,
        }
    }

    pub fn set_value(&self, thing: &Rc<usize>) {
        let count: usize = Rc::strong_count(thing);

        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
            return;
        }

        let pct = ((count * 100) as f64 / self.max as f64) as usize;
        if pct > 70 {
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                pct
            ));
        }

        self.value.replace(count);
    }

    pub fn peek(&self, thing: &Rc<usize>) {
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            ((Rc::strong_count(thing) * 100) as f64 / self.max as f64) as usize
        ));
    }
}

#[cfg(test)]
mod tests;
