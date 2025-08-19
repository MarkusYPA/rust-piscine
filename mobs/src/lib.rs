mod boss;
mod member;

pub use boss::*;
pub use member::*;
pub use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    // Add member of rank Associate
    pub fn recruit(&mut self, name_age: (&str, u32)) {
        self.members.insert(
            name_age.0.to_owned(),
            Member {
                role: Role::Associate,
                age: name_age.1,
            },
        );
    }

    // Weaker mob loses youngest member(s) and possibly all wealth and cities
    pub fn attack(&mut self, target: &mut Mob) {
        // see which mob is weaker
        let own_strength = power_combat_score(&self);
        let target_strength = power_combat_score(target);

        let (weaker_mob, stronger_mob) = if own_strength <= target_strength {
            (self, target)
        } else {
            (target, self)
        };

        // find lowest member age in weaker mob
        let lowest_age = weaker_mob
            .members
            .values()
            .min_by(|a, b| a.age.cmp(&b.age))
            .unwrap()
            .age;

        // keep the members that aren't youngest
        weaker_mob
            .members
            .retain(|_, member| member.age != lowest_age);

        // if weaker mob loses all members, transfer cities and wealth
        if weaker_mob.members.len() == 0 {
            stronger_mob.wealth += weaker_mob.wealth;
            weaker_mob.wealth = 0;

            for c in weaker_mob.cities.iter() {
                stronger_mob.cities.insert(c.to_owned());
            }
            weaker_mob.cities.clear();
        }
    }

    // Transfer target_amount or all target_mob.wealth to own wealth
    pub fn steal(&mut self, target_mob: &mut Mob, target_amount: u64) {
        if target_mob.wealth < target_amount {
            self.wealth += target_mob.wealth;
            target_mob.wealth = 0;
        } else {
            self.wealth += target_amount;
            target_mob.wealth -= target_amount;
        }
    }

    // Add available city to own
    pub fn conquer_city(&mut self, mobs: &[&Mob], city_name: String) {
        if !mobs.iter().any(|m| m.cities.contains(&city_name)) {
            self.cities.insert(city_name.clone());
        }
    }
}

fn power_combat_score(mob: &Mob) -> i32 {
    mob.members
        .values()
        .map(|m| match m.role {
            Role::Associate => 1,
            Role::Soldier => 2,
            Role::Caporegime => 3,
            Role::Underboss => 4,
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests;
