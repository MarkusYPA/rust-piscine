use chrono::Utc;

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value.to_owned()),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), // "2022-10-17 12:09:25"
            err: err,
        }
    }
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }

        let mut alphas: bool = false;
        let mut nums: bool = false;
        let mut symbols: bool = false;

        for c in self.password.chars() {
            if c.is_alphabetic() {
                alphas = true;
            };
            if c.is_numeric() {
                nums = true;
            };
            if c.is_ascii_punctuation() {
                symbols = true;
            }
        }

        if alphas && nums && symbols {
            return Ok(());
        } else {
            Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ))
        }
    }

    pub fn new(n: String, pw: String) -> Self {
        Form {
            name: n.to_owned(),
            password: pw.to_owned(),
        }
    }
}
