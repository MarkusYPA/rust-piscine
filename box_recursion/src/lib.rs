#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "ceo" => Self::CEO,
            "manager" => Self::Manager,
            _ => Self::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>; // Not: Box<Option<Worker>>

#[derive(Debug, PartialEq)]
pub struct Worker {
    pub role: Role, // Role
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self {
            grade: Default::default(), // None
        }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let wrkr = Worker {
            role: Role::from(role),
            name: name.to_owned(),
            next: self.grade.take(), // Takes the value out of Option
        };
        self.grade = Some(Box::new(wrkr));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if self.grade == None {
            return None;
        }

        let removed_name = self.grade.as_ref().unwrap().name.to_owned();
        self.grade = self.grade.as_mut().unwrap().next.take();  // as_mut and take: avoid problems with shared references

        Some(removed_name)
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        match &self.grade {
            Some(bw) => Some((bw.name.clone(), bw.role)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;
