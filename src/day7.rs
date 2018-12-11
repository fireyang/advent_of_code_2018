mod day7 {
    use std::collections::HashMap;
    // use std::collections::HashSet;
    lazy_static! {
        static ref HASHMAP: HashMap<char, i32> = {
            let mut m = HashMap::new();
            "abcdefghijklmnopqrstuvwxyz"
                .to_uppercase()
                .chars()
                .enumerate()
                .for_each(|(idx, ch)| {
                    m.insert(ch, (idx + 1) as i32);
                });
            m
        };
    }

    #[allow(dead_code)]
    pub fn part1(input: &Vec<(&str, &str)>) -> String {
        let mut sort_map = HashMap::new();
        let mut deep_map = HashMap::new();
        for (p, n) in input.iter() {
            let k = sort_map.entry(p).or_insert(Vec::new());
            k.push(n);
            let count = deep_map.entry(n).or_insert(0);
            *count += 1;
        }
        // println!("deep_map,{:?}", deep_map);
        let mut quene = vec![];
        let mut r = vec![];
        for (k, _) in sort_map.iter() {
            match deep_map.get(k) {
                None => {
                    quene.push(k);
                }
                _ => (),
            }
        }
        while quene.len() > 0 {
            // println!("quene,{:?}", quene);
            quene.sort();
            let head = quene.remove(0);
            r.push(head);
            if let Some(next_vec) = sort_map.get(head) {
                for v in next_vec.iter() {
                    match deep_map.get_mut(v) {
                        Some(1) => {
                            quene.push(v);
                        }
                        Some(c) => {
                            *c -= 1;
                        }
                        None => (),
                    }
                }
            }
        }
        // let (root, _) = deep_map.iter().find(|(_,v)| **v == 0).unwrap();
        // println!("root,{:?}", root);
        // println!("root,{:?}", r);
        r.iter().map(|x| x.to_string()).collect::<String>()
    }
    #[allow(dead_code)]
    pub fn part2(input: &Vec<(&str, &str)>, worker_count: usize, base: i32) -> i32 {
        let mut sort_map = HashMap::new();
        let mut deep_map = HashMap::new();
        for (p, n) in input.iter() {
            let p1 = HASHMAP[&p.chars().next().unwrap()];
            let n1 = HASHMAP[&n.chars().next().unwrap()];
            let k = sort_map.entry(p1).or_insert(Vec::new());
            k.push(n1);
            let count = deep_map.entry(n1).or_insert(0);
            *count += 1;
        }
        // println!("deep_map,{:?}", deep_map);
        let mut quene = vec![];
        // let mut r = vec![];
        for (k, _) in sort_map.iter() {
            match deep_map.get(k) {
                None => {
                    quene.push((*k, *k + base));
                }
                _ => (),
            }
        }
        let mut coust = 0;
        while quene.len() > 0 {
            // println!("quene,{:?}", quene);
            coust += 1;

            let mut add_vec = vec![];
            quene = quene
                .iter()
                .enumerate()
                .map(|(i, (head, life))| {
                    if i < worker_count {
                        return (*head, *life - 1);
                    } else {
                        return (*head, *life);
                    }
                })
                .filter(|(head, life)| {
                    // println!("add_vec, {:?}", (head, life));
                    if *life == 0 {
                        add_vec.push(*head);
                    }
                    *life != 0
                })
                .collect();
            for head in add_vec.iter() {
                // println!("add_vec, {:?}", add_vec);
                if let Some(next_vec) = sort_map.get(&head) {
                    for v in next_vec.iter() {
                        match deep_map.get_mut(v) {
                            Some(1) => {
                                quene.push((*v, *v + base));
                            }
                            Some(c) => {
                                *c -= 1;
                            }
                            None => (),
                        }
                    }
                }
            }
        }
        println!("count,{:?}", coust);
        coust
    }
}
#[cfg(test)]
mod tests {

    // use std::collections::HashMap;

    use super::*;
    use crate::common;
    static STR_INPUT: &str = r#"
    Step C must be finished before step A can begin.
    Step C must be finished before step F can begin.
    Step A must be finished before step B can begin.
    Step A must be finished before step D can begin.
    Step B must be finished before step E can begin.
    Step D must be finished before step E can begin.
    Step F must be finished before step E can begin.
    "#;

    fn parse_from_str(val: &String) -> Vec<(&str, &str)> {
        val.split('\n')
            .map(|x| x.trim())
            .filter(|&x| x.len() != 0)
            .map(|x| {
                let vec: Vec<&str> = x.split_whitespace().collect();
                (vec[1], vec[7])
            })
            .collect()
    }

    #[test]
    fn day7_part1() {
        let input = &STR_INPUT.to_string();
        let points = parse_from_str(input);
        assert_eq!(day7::part1(&points), "CABDFE".to_string());
        let content = common::read_from_file("./data/day7_part1.txt").unwrap();
        let points2 = parse_from_str(&content);
        assert_eq!(
            day7::part1(&points2),
            "HEGMPOAWBFCDITVXYZRKUQNSLJ".to_string()
        );
    }

    #[test]
    fn day7_part2() {
        let input = &STR_INPUT.to_string();
        let points = parse_from_str(input);
        assert_eq!(day7::part2(&points, 2, 0), 15);
        let content = common::read_from_file("./data/day7_part2.txt").unwrap();
        let points2 = parse_from_str(&content);
        assert_eq!(day7::part2(&points2, 5, 60), 1226);
    }
}
