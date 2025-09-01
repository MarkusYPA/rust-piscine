#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    // max 4000?
    fn from(mut value_in: u32) -> Self {
        let mut result = RomanNumber(Vec::new());

        if value_in == 0 {
            result.0.push(RomanDigit::Nulla);
            return result;
        }

        while value_in > 0 {
            for value_r in [1000, 500, 100, 50, 10, 5, 1] {
                if value_in > value_r - 1 {
                    if value_in / 100 == 9 {
                        result.0.push(RomanDigit::C);
                        result.0.push(RomanDigit::M);
                        value_in -= 900;
                    }

                    if value_in / 100 == 4 {
                        result.0.push(RomanDigit::C);
                        result.0.push(RomanDigit::D);
                        value_in -= 400;
                    }

                    if value_in / 10 == 9 {
                        result.0.push(RomanDigit::X);
                        result.0.push(RomanDigit::C);
                        value_in -= 90;
                    }

                    if value_in / 10 == 4 {
                        result.0.push(RomanDigit::X);
                        result.0.push(RomanDigit::L);
                        value_in -= 40;
                    }

                    if value_in == 9 {
                        result.0.push(RomanDigit::I);
                        result.0.push(RomanDigit::X);
                        value_in -= 9;
                    }

                    if value_in == 4 {
                        result.0.push(RomanDigit::I);
                        result.0.push(RomanDigit::V);
                        value_in -= 4;
                    }

                    let mut units = value_in / value_r;

                    while units > 0 {
                        result.0.push(RomanDigit::from(value_r));
                        units -= 1;
                        value_in -= value_r;
                    }
                }
            }
        }

        result
    }
}

impl From<RomanDigit> for u32 {
    fn from(value: RomanDigit) -> Self {
        match value {
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
            RomanDigit::C => 100,
            RomanDigit::D => 500,
            RomanDigit::M => 1000,
            RomanDigit::Nulla => 0,
        }
    }
}

impl From<RomanNumber> for u32 {
    // max 4000?
    fn from(value_in: RomanNumber) -> Self {
        let mut result = 0;

        for (i, v) in value_in.0.iter().enumerate() {
            let mut prev = 2000;
            if i != 0 {
                prev = u32::from(value_in.0[i-1]);
            }
            
            let current = u32::from(*v);

            if prev < current {
                result += current - prev - prev;
            } else {
                result += current;
            }
        }

        result
    }
}
