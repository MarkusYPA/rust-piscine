#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = (); // from_str requires an error type, probably String

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "a" => Ok(Antigen::A),
            "b" => Ok(Antigen::B),
            "ab" => Ok(Antigen::AB),
            "o" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "-" => Ok(RhFactor::Negative),
            "+" => Ok(RhFactor::Positive),
            _ => Err(()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.antigen == other.antigen {
            return self.rh_factor.cmp(&other.rh_factor); // enums can derive Ord
        }
        return self.antigen.cmp(&other.antigen);
    }
}

impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antig: &str;
        let rh: RhFactor;
        if let Some(s) = s.strip_suffix('+') {
            rh = RhFactor::Positive;
            antig = s;
        } else if let Some(s) = s.strip_suffix('-') {
            rh = RhFactor::Negative;
            antig = s;
        } else {
            return Err(());
        }

        if let Ok(ag) = Antigen::from_str(antig) {
            Ok(BloodType {
                antigen: ag,
                rh_factor: rh,
            })
        } else {
            return Err(());
        }
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.antigen, self.rh_factor)
    }
}

impl BloodType {
    /*
    Blood Types 	Donate Blood to 	Receive Blood From
    A+ 	            A+, AB+ 	        A+, A-, O+, O-
    O+ 	            O+, A+, B+, AB+ 	O+, O-
    B+ 	            B+, AB+ 	        B+, B-, O+, O-
    AB+             AB+ 	            Everyone
    A- 	            A+, A-, AB+, AB- 	A-, O-
    O- 	            Everyone 	        O-
    B- 	            B+, B-, AB+, AB- 	B-, O-
    AB-             AB+, AB- 	        AB-, A-, B-, O-
    */

    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor == RhFactor::Positive || other.rh_factor == RhFactor::Negative {
            // self positive or other negative
            match self.antigen {
                Antigen::A => return other.antigen == Antigen::A || other.antigen == Antigen::O,
                Antigen::O => return other.antigen == Antigen::O,
                Antigen::B => return other.antigen == Antigen::B || other.antigen == Antigen::O,
                Antigen::AB => return true,
            }
        }

        // self negative, other positive
        false
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut dons = Vec::new();

        for rh in [RhFactor::Negative, RhFactor::Positive] {
            for ag in [Antigen::A, Antigen::AB, Antigen::B, Antigen::O] {
                let bt = BloodType {
                    antigen: ag,
                    rh_factor: rh.clone(),
                };
                if self.can_receive_from(&bt) {
                    dons.push(bt);
                }
            }
        }

        dons
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recs = Vec::new();

        for rh in [RhFactor::Negative, RhFactor::Positive] {
            for ag in [Antigen::A, Antigen::AB, Antigen::B, Antigen::O] {
                let bt = BloodType {
                    antigen: ag,
                    rh_factor: rh.clone(),
                };
                if bt.can_receive_from(self) {
                    recs.push(bt);
                }
            }
        }

        recs
    }
}

#[cfg(test)]
mod tests;
