pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut all_initials: Vec<String> = Vec::new();

    for n in names {
        let mut these_initials = String::new();
        n.split(" ").for_each(|w| {
            if let Some(c) = w.chars().next() {
                these_initials.push(c);
                these_initials += ". ";
            }
        });
        all_initials.push(these_initials.trim().to_string());
    };

    all_initials
}
