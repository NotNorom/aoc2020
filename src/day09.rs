pub fn part_01(data: &str) -> usize {
    let numbers: Vec<_> = data.lines().map(str::parse::<usize>).filter_map(std::result::Result::ok).collect();
    let invalid = numbers.iter().skip(25).zip(numbers.windows(25)).find_map(|(elem, window)| {
        for a in window.into_iter() {
            for b in window.into_iter() {
                if a == b {
                    continue;
                }
                if a + b == *elem {
                    return None;
                }
            }
        }
        Some(*elem)
    });

    invalid.unwrap_or_default()
}

pub fn part_02(data: &str) -> usize {
    let numbers: Vec<_> = data.lines().map(str::parse::<usize>).filter_map(std::result::Result::ok).collect();
    let invalid: usize = 731031916; //solution from previous step

    for i in 2..numbers.len() {
        let res = numbers.windows(i).find_map(|v| {
            if v.iter().sum::<usize>() == invalid {
                let mut w = v.to_owned();
                w.sort();
                return Some(w.first().unwrap() + w.last().unwrap());
            }
            return None;
        });

        if let Some(r) = res {
            return r.into();
        }
    };
    0
}
