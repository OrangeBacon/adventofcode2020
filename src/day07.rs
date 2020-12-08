use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Bag {
    name: String,
    contains: Vec<(u32, Rc<RefCell<Bag>>)>,
}

const TARGET: &str = "shiny gold";

pub fn day07(input: String) {
    let mut bags: HashMap<String, Rc<RefCell<Bag>>> = HashMap::new();

    let input: Vec<Vec<_>> = input
        .lines()
        .map(|x| x.split("contain").collect())
        .collect();

    let remove_bag_suffix = Regex::new(r" (bag|bags)\.*$").unwrap();

    for line in input {
        let name = remove_bag_suffix.replace(line[0].trim(), "");
        let contains: Vec<_> = line[1]
            .split(",")
            .map(|x| remove_bag_suffix.replace(x.trim(), ""))
            .collect();
        let contains = contains
            .iter()
            .map(|x| x.split_once(" ").unwrap())
            .map(|(a,b)| {
                if a == "no" {
                    None
                } else {
                    Some((a.parse::<u32>().unwrap(), b))
                }
            })
            .filter(|&x| x != None)
            .map(|x| x.unwrap())
            .map(|(num, x)| {
                let bag = if bags.contains_key(&String::from(x)) {
                    bags.get(&String::from(x)).unwrap().clone()
                } else {
                    bags.insert(
                        String::from(x),
                        Rc::new(RefCell::new(Bag {
                            name: String::from(x),
                            contains: vec![],
                        })),
                    );
                    bags.get(&String::from(x)).unwrap().clone()
                };
                (num, bag)
            })
            .collect::<Vec<_>>();

        let bag = if bags.contains_key(&String::from(name.clone())) {
            bags.get(&String::from(name)).unwrap().clone()
        } else {
            bags.insert(
                String::from(name.clone()),
                Rc::new(RefCell::new(Bag {
                    name: String::from(name.clone()),
                    contains: vec![],
                })),
            );
            bags.get(&String::from(name)).unwrap().clone()
        };

        bag.borrow_mut().contains = contains;
    }

    fn find_recurse(bag: Rc<RefCell<Bag>>) -> bool {
        bag.borrow().contains.iter().fold(false, |acc, val| {
            acc || (val.1.borrow().name == TARGET)
                || (find_recurse(Rc::new(RefCell::new(val.1.borrow().clone()))))
        })
    }

    let res = bags.iter().fold(0, |acc, (_, bag)| {
        if find_recurse(bag.clone()) {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", res);

    fn count_recurse(bag: Rc<RefCell<Bag>>) -> u32 {
        bag.borrow().contains.iter().fold(1, |acc, val| {
            acc + val.0 * count_recurse(Rc::new(RefCell::new(val.1.borrow().clone())))
        })
    }

    let res = Rc::new(RefCell::new(
        bags.get(TARGET).unwrap().borrow().clone(),
    ));

    println!("{}", count_recurse(res) - 1);
}
