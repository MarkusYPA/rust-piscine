/* mod err;
use err::*;

use json;
use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {

        // Return ReadErr if reading fails, otherwise continue
        let content = fs::read_to_string(path).map_err(|e| ReadErr {
            child_err: Box::new(e),
        })?; // ? used to produce a value instead of Result (Error dealt with in map_err)

        let parsed = json::parse(&content).map_err(|e| ParseErr::Malformed(Box::new(e)))?;

        if parsed["tasks"].is_empty() {
            // Infer what to convert to?
            // "this conversion is whatever the implementation of From<T> for U chooses to do"
            return Err(ParseErr::Empty.into());
        }

        let title = parsed["title"]
            .as_str()
            .ok_or("Missing or invalid 'title'")?
            .to_string();

        let tasks_array = parsed["tasks"]
            .members()
            .map(|t| {
                Ok(Task {
                    id: t["id"].as_u32().ok_or("Invalid id")?,
                    description: t["description"]
                        .as_str()
                        .ok_or("Invalid description")?
                        .to_string(),
                    level: t["level"].as_u32().ok_or("Invalid level")?,
                })
            })
            .collect::<Result<Vec<Task>, Box<dyn Error>>>()?;

        Ok(TodoList {
            title,
            tasks: tasks_array,
        })
    }
}


#[cfg(test)]
mod tests; */

mod err;
pub use err::{ParseErr, ReadErr};

use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let contents = fs::read_to_string(path).map_err(|e| ReadErr {
            child_err: Box::new(e),
        })?;

        let contents = json::parse(&contents).map_err(|e| ParseErr::Malformed(Box::new(e)))?;
        if contents["tasks"].is_empty() {
            return Err(ParseErr::Empty.into());
        }

        Ok(Self {
            title: contents["title"].as_str().unwrap().to_owned(),
            tasks: contents["tasks"]
                .members()
                .map(|m| Task {
                    id: m["id"].as_u32().unwrap(),
                    description: m["description"].as_str().unwrap().to_owned(),
                    level: m["level"].as_u32().unwrap(),
                })
                .collect(),
        })
    }
}
