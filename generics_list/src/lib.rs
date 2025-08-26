#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        match self.head {
            None => self.head = Some(Node { value, next: None }),
            Some(_) => {
                self.head = Some(Node {
                    value,
                    next: Some(Box::new(self.head.take().unwrap())),
                })
            }
        }
    }

    pub fn pop(&mut self) {
        let old_head = self.head.take();

        match old_head {
            None => return,
            Some(n) => match n.next {
                // don't match reference, take ownership
                None => self.head = None,
                Some(boxed_node) => self.head = Some(*boxed_node),
            },
        }
    }

    pub fn len(&self) -> usize {
        match &self.head {
            None => 0,
            Some(n) => {
                let mut count = 0;
                let mut curr = Some(n);
                while let Some(node) = curr {
                    count += 1;
                    curr = node.next.as_deref();    // ??
                }
                count
            }
        }
    }
}

#[cfg(test)]
mod tests;
