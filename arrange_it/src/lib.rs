pub fn arrange_phrase(phrase: &str) -> String {
    let mut nums_and_letters = phrase.split(' ').map(|wd| {
        let mut num = 0;
        let mut letters = String::new();
        for c in wd.chars() {
            if c.is_numeric() {
                num = c.to_string().parse().unwrap()
            } else {
                letters.push(c)
            }
        }
        return (num, letters);
    }).collect::<Vec<(i32, String)>>();

    nums_and_letters.sort();
    nums_and_letters.iter().map(|(_, w)| w.to_owned()).collect::<Vec<String>>().join(" ")
}
