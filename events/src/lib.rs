use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?}, {}, {})",
            self.position,
            self.size,
            self.content
                .truecolor(self.color.0, self.color.1, self.color.2)
        )
    }
}

use Event::*; // for tests

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(cont) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: cont.to_string(),
            },
            Event::Registration(dur) => {
                let hours = dur.num_hours();
                let minutes = dur.num_minutes() - hours * 60;
                let seconds = dur.num_seconds() - minutes * 60 - hours * 3600;
                let time_left = format!("{}H:{}M:{}S", hours, minutes, seconds);

                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!("You have {} left before the registration ends", time_left),
                }
            }
            Event::Appointment(cont) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: cont.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: String::from("Enjoy your holiday"),
            },
        }
    }
}
#[cfg(test)]
mod tests;
