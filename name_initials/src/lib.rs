pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter().map(|full_name| {
        full_name
            .split(' ')
            .map(|name| {
                let mut s = String::new();
                if let Some(c) = name.chars().next() {
                    s.push(c);
                    s.push('.');
                }
                s
            })
            .collect::<Vec<_>>()
            .join(" ")
    }).collect()
}
