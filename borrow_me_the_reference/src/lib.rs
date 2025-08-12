enum Operation {
    Add,
    Subtract,
}

pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::new();
    let mut skip = 0;

    for c in s.chars() {
        if c == '-' && res.len() != 0 {
            res.pop();
            continue;
        }

        if c == '+' {
            skip += 1;
            continue;
        }

        if skip > 0 {
            skip -= 1;
            continue;
        }

        res.push(c);
    }

    *s = res;
}

pub fn do_operations(v: &mut [String]) {
    let mut results = Vec::new();

    for s in v.iter() {
        let mut nums: Vec<i32> = vec![];
        let mut op: Option<Operation> = None;
        let mut num = "".to_owned();

        for c in s.chars() {
            if c != '+' && c != '-' {
                num.push(c);
            } else {
                nums.push(num.parse().unwrap());
                if c == '+' {
                    op = Some(Operation::Add);
                    num.clear();
                }
                if c == '-' {
                    op = Some(Operation::Subtract);
                    num.clear();
                }
            }
        }

        nums.push(num.parse().unwrap());

        let res = match op {
            Some(Operation::Add) => (nums[0] + nums[1]).to_string(),
            Some(Operation::Subtract) => (nums[0] - nums[1]).to_string(),
            None => s.clone(),
        };

        results.push(res);
    }

    for (i, res) in results.into_iter().enumerate() {
        v[i] = res;
    }
}