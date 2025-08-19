
use super::*;
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
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(
            name.to_owned(),
            Member {
                role: Role::Associate,
                age: age,
            },
        );
    }

    // Weaker mob loses youngest member(s) and possibly all wealth and cities
    pub fn attack(&mut self, target: &mut Mob) {
        // get stronger and weaker mob
        let (weaker_mob, stronger_mob) = if self.power_combat_score() <= target.power_combat_score()
        {
            (self, target)
        } else {
            (target, self)
        };

        // keep the weaker mob members that aren't youngest
        let lowest_age = weaker_mob.members.values().map(|m| m.age).min().unwrap();
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

    fn power_combat_score(&self) -> i32 {
        self.members
            .values()
            .map(|m| match m.role {
                Role::Associate => 1,
                Role::Soldier => 2,
                Role::Caporegime => 3,
                Role::Underboss => 4,
            })
            .sum::<i32>()
    }
}
