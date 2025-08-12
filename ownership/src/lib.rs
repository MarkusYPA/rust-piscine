pub fn first_subword(s: String) -> String {
    for (i, c) in s.chars().enumerate() {
        if i != 0 {
            if c == ' ' || c == '_' || c.is_uppercase(){
                return s[..i].to_owned();
            }
        }
    }
    s.to_owned()
}