const G: f32 = -9.8;    // Disgusting not to use 9.81

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        Self {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = Self;

    // Advance time by 1, calculate new position and y velocity
    fn next(&mut self) -> Option<Self::Item> {        
        self.time += 1.0;

        // No acceleration component in x velocity
        let new_pos_x = self.init_position.x + self.init_velocity.x * self.time;
        // Acceleration by gravity in y
        let new_pos_y = self.init_position.y
            + self.init_velocity.y * self.time
            + 0.5 * G * self.time * self.time;
        // Only y velocity changes
        let new_vel_y = self.init_velocity.y + G * self.time;

        // Round and assign values 
        self.actual_position = Object {
            x: round_one(new_pos_x),
            y: round_one(new_pos_y),
        };
        self.actual_velocity = Object {
            x: round_one(self.init_velocity.x),
            y: round_one(new_vel_y),
        };

        // No next when hitting the ground
        if self.actual_position.y < 0.0 {
            return None;
        }

        Some(self.clone())
    }
}

fn round_one(n: f32) -> f32 {
    (n*10.0).round() / 10.0
}

#[cfg(test)]
mod tests;
