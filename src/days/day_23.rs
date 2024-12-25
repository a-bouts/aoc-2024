use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;

use crate::days::Day as D;

pub(crate) struct Day {
    input: Vec<String>
}

impl D for Day {
    fn new(input: Vec<String>) -> Self {
        Self {
            input
        }
    }

    fn part_one(&self) -> Result<i64> {
        
        let mut links = BTreeMap::new();
        let mut computers = BTreeSet::new();
        for line in &self.input {
            let cs = line.split("-").map(|c| c.to_string()).collect::<Vec<_>>();
            links.entry(cs[0].clone()).or_insert(vec![]).push(cs[1].clone());
            links.entry(cs[1].clone()).or_insert(vec![]).push(cs[0].clone());
            computers.insert(cs[0].clone());
            computers.insert(cs[1].clone());
        }

        let mut res = 0;

        
        let cs = computers.clone();
        for c in cs.iter() {
            if !computers.contains(c) {
                continue
            }

            let mut groups: Vec<Vec<String>> = vec![];

            for l in links.get(c).unwrap() {
                if !computers.contains(l) {
                    continue
                }

                let third = links.get(l).unwrap().iter().filter(|l| *l != c && links.get(c).unwrap().contains(l)).map(|c| c.clone()).collect::<Vec<_>>();

                for t in third.into_iter() {
                    if !computers.contains(&t) {
                        continue
                    }

                    if groups.iter().any(|g| g.contains(l) && g.contains(&t)) {
                        continue;
                    }

                    groups.push(vec![l.clone(), t.clone()]);

                    let mut group = BTreeSet::from([c.clone()]);

                    computers.remove(c);

                    group.insert(l.clone());
                    //computers.remove(l);
                    group.insert(t);
                    //computers.remove(third[0]);
                    
                    println!("{:?}", group);
                    if group.iter().any(|c| c.starts_with("t")) {
                        res += 1;
                    }
                }
            }
        }

        Ok(res)
    }

    fn part_two(&self) -> Result<i64> {

        fn is_group(cs: &Vec<String>, links: &BTreeMap<String, Vec<String>>) -> bool {
            for c in cs {
                for l in cs {
                    if l != c && !links.get(c).unwrap().contains(&l) {
                        return false;
                    }
                }
            }

            true
        }

        fn best_of(cs: Vec<String>, links: &BTreeMap<String, Vec<String>>, groups: &mut Vec<String>, max: &mut usize) -> Vec<String> {
            
            let join = cs.join(",");

            if groups.contains(&join) {
                return cs;
            }
            
            if is_group(&cs, links) {
                if *max < cs.len() {
                    *max = cs.len();
                }
                groups.push(join);
                return cs
            }

            if cs.len() == *max {
                return vec![];
            }

            let mut best = vec![];
            for i in 0..cs.len() {
                let mut cs = cs.clone();
                cs.remove(i);
                cs.sort();
                let b = best_of(cs, links, groups, max);
                if b.len() > best.len() {
                    best = b;
                }
            }

            best
        }
        
        let mut links = BTreeMap::new();
        let mut computers = BTreeSet::new();
        for line in &self.input {
            let cs = line.split("-").map(|c| c.to_string()).collect::<Vec<_>>();
            links.entry(cs[0].clone()).or_insert(vec![]).push(cs[1].clone());
            links.entry(cs[1].clone()).or_insert(vec![]).push(cs[0].clone());
            computers.insert(cs[0].clone());
            computers.insert(cs[1].clone());
        }

        let mut res = 0;


        let cache = &mut vec![];
        let max = &mut 0;

        for (c, cs) in links.iter() {
            let mut cs = cs.clone();
            cs.push(c.clone());
            cs.sort();
            let group = best_of(cs, &links, cache, max);
            if group.len() == *max {
                println!("group {:?}", group.join(","));
            }
        }

        Ok(res)
    }
        
}

crate::test!(
    7,
    "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",

    0,
    "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
);
