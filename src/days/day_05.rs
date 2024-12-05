use crate::days::Day as D;

use std::collections::{BTreeSet, BTreeMap};

pub(crate) struct Day {
    input: Vec<String>
}

#[derive(Default)]
struct Rule {
    must_be_before: BTreeSet<i64>,
    must_be_after: BTreeSet<i64>
}

impl Day {
    fn compute_rules(&self, rules: &BTreeMap<i64, Rule>, update: &Vec<i64>) -> BTreeMap<i64, Rule> {

        let mut result = BTreeMap::new();

        let changed = &mut vec!();

        for a in update.iter() {
            result.insert(*a, Rule::default());
            match rules.get(a) {
                Some(rule_a) => {
                    for b in rule_a.must_be_after.iter() {
                        if update.contains(b) && !result.get(a).unwrap().must_be_after.contains(b) {
                            result.get_mut(a).unwrap().must_be_after.insert(*b);
                            changed.push((*a, *b));
                        }
                    }
                }
                _ => {}
            }
        }

        while changed.len() > 0 {
            let mut new_changed = vec!();

            for (a, b) in changed.iter() {
                match rules.get(b) {
                    Some(rule_b) => {
                        for c in rule_b.must_be_after.iter() {
                            if update.contains(c) && !result.get(a).unwrap().must_be_after.contains(c) {
                                result.get_mut(a).unwrap().must_be_after.insert(*c);
                                new_changed.push((*a, *c));
                            }
                        }
                    }
                    _ => {}
                }
            }

            *changed = new_changed;
        }

        result

    }
}


impl D for Day {
    fn new(input: Vec<String>) -> Self {
        Self {
            input
        }
    }

    fn part_one(&self) -> i64 {
        let mut res = 0;

        let mut rules = BTreeMap::new();
        let mut next = false;
        for line in &self.input {
            if line.len() == 0 {
                next = true;
                continue
            }

            if !next {
                let ns = line.split("|").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
                let rule: &mut Rule = rules.entry(ns[0]).or_insert(Rule::default());
                rule.must_be_before.insert(ns[1]);

                let rule = rules.entry(ns[1]).or_insert(Rule::default());
                rule.must_be_after.insert(ns[0]);
                continue
            }

            let mut is_correct = true;
            println!("---");
            println!("{line}");
            let update = line.split(",").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
            let rules_to_apply = self.compute_rules(&rules, &update);
            
            for i in 1..update.len() {
                let u0 = update[i - 1];
                let u1 = update[i];
                println!("");

                if rules_to_apply.get(&u0).unwrap().must_be_after.contains(&u1) {
                    is_correct = false;
                    break
                }
            }
            if is_correct {
                println!("{:?} {}", update, update[update.len() / 2]);
                res += update[update.len() / 2];
            }

        }

        res
    }

    fn part_two(&self) -> i64 {    
        let mut res = 0;

        let mut rules = BTreeMap::new();
        let mut next = false;
        for line in &self.input {
            if line.len() == 0 {
                next = true;
                continue
            }

            if !next {
                let ns = line.split("|").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
                let rule: &mut Rule = rules.entry(ns[0]).or_insert(Rule::default());
                rule.must_be_before.insert(ns[1]);

                let rule = rules.entry(ns[1]).or_insert(Rule::default());
                rule.must_be_after.insert(ns[0]);
                continue
            }

            let mut is_correct = true;
            println!("---");
            println!("{line}");
            let mut update = line.split(",").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
            let rules_to_apply = self.compute_rules(&rules, &update);

            for i in 1..update.len() {
                let u0 = update[i - 1];
                let u1 = update[i];
                println!("");

                if rules_to_apply.get(&u0).unwrap().must_be_after.contains(&u1) {
                    is_correct = false;
                    break
                }
            }
            if !is_correct {

                while !is_correct {
                    is_correct = true;
                    for i in 1..update.len() {
                        let u0 = update[i - 1];
                        let u1 = update[i];
                        println!("");
        
                        if rules_to_apply.get(&u0).unwrap().must_be_after.contains(&u1) {
                            update.swap(i - 1, i);
                            is_correct = false;
                            break
                        }
                    }
                }

                res += update[update.len() / 2];
            }

        }

        res
    }
        
}

crate::test!(
    143,
    "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",

    123,
    "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
);
