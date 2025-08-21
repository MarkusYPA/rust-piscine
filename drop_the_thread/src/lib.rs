use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,         // number of dropped threads
    pub states: RefCell<Vec<bool>>, // true for dropped, false for undropped threads. index == id
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            drops: Default::default(),
            states: Default::default(),
        }
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
        let th = Thread::new(self.thread_len(), c, self);
        self.states.borrow_mut().push(false);
        (th.pid, th)
        // Our custom drop() gets called for Thread here, immediately when out of scope
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn drop_thread(&self, id: usize) {
        if self.states.borrow()[id] {
            panic!("{} is already dropped", id);
        }
        self.states.borrow_mut()[id] = true;
        self.drops.replace(self.drops.get() + 1);
    }
}

#[derive(Debug, Clone)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        Self {
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        // possible panic happens in drop_thread
    }
}

// Reason for custom Drop implementation is to keep ThreadPool up to date? 
impl Drop for Thread<'_> {
    fn drop(&mut self) {
        // add_drop() is a previous name for drop_thread()
        self.parent.drop_thread(self.pid);
    }
}

#[cfg(test)]
mod tests;
