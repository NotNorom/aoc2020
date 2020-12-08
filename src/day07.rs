use std::collections::{HashMap, HashSet};

pub fn part_01(data: &str) -> usize {
    let bag_children: HashMap<_, _> = data
        .lines()
        .map(parse_line)
        .map(|b| (b.name, b.bags))
        .collect();
    
    let mut bag_parents = HashMap::new();
    bag_children.into_iter().for_each(|(key, value)| {
        if let Some(v) = value {
            for bag in v {
                let entry = bag_parents.entry(bag.1).or_insert(Vec::<String>::new());
                (*entry).push(key.clone());
            }
        };
    });

    let mut final_bags = vec![];
    get_parents(&"shiny gold".to_string(), &bag_parents, &mut final_bags);
    final_bags.iter().collect::<HashSet<_>>().len()
}

fn get_parents(from: &String, data: &HashMap<String, Vec<String>>, result: &mut Vec<String>) {
    if let Some(parents) = data.get(from) {
        result.append(&mut parents.clone());
        parents.iter().for_each(|v| get_parents(v, data, result));
    }
}

// shiny gold

#[derive(Debug)]
struct BagType {
    pub name: String,
    pub bags: Option<Vec<(usize, String)>>,
}

fn parse_line(line: &str) -> BagType {
    let a: Vec<&str> = line.split_terminator(" bags contain ").collect();
    let name = a[0].to_owned();
    if a[1] == "no other bags." {
        return BagType {name: String::from(name), bags: None}
    } 
    let bags: Vec<_> = a[1]
        .replace("bags", "")
        .replace("bag", "")
        .replace(".", "")
        .split_terminator(",")
        .map(|v| {
            let mut split = v.trim().splitn(2, " ");
            let number = split.nth(0).unwrap().parse::<usize>().unwrap();
            let other_bags = match split.nth(0) {
                Some(s) => s.to_string(),
                None => String::new(),
            };
            //(split.nth(0).unwrap().parse::<usize>().unwrap(), String::from(split.nth(1).unwrap()))
            (number, other_bags)
        })
        //.inspect(|v| println!("{:?}", v))
        .collect();
    BagType {name: String::from(name), bags: Some(bags)}
}



// part 2

pub fn part_02(data: &str) -> usize {
    let bag_children: HashMap<String, Option<Vec<(usize, String)>>> = data
        .lines()
        .map(parse_line)
        .map(|b| (b.name, b.bags))
        .collect();
    
    let mut bag_parents = HashMap::new();
    bag_children.clone().into_iter().for_each(|(key, value)| {
        if let Some(v) = value {
            for bag in v {
                let entry = bag_parents.entry(bag.1).or_insert(Vec::<String>::new());
                (*entry).push(key.clone());
            }
        };
    });

    let mut final_bags = vec![];
    get_children(&"shiny gold".to_string(), &bag_children, &mut final_bags);
    final_bags.len()
}

fn get_children(from: &String, data: &HashMap<String, Option<Vec<(usize, String)>>>, result: &mut Vec<String>) {
    let children = match data.get(from) {
        Some(v) => v,
        _ => return,
    };
    if let None = children {
        return;
    }
    children.clone().unwrap().iter().for_each(|(amount,name)| {
        for _ in 0..*amount {
            result.push(name.clone());
            get_children(name, data, result);
        }
    });
}