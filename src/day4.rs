mod day4 {
    extern crate chrono;
    use chrono::prelude::*;
    use std::collections::HashMap;
    // use chrono::{Utc};

    #[derive(Debug)]
    pub struct Guard {
        pub id: i32,
        pub fall_sleep: i32,
        pub sleeps: Vec<i32>,
    }

    impl Guard {
        pub fn sleep_count(&self) -> i32 {
            self.sleeps.iter().sum()
        }
    }

    pub fn parse_input(str_vec: Vec<String>) -> Vec<(DateTime<Utc>, String)> {
        let mut vec2: Vec<(DateTime<Utc>, String)> = str_vec
            .iter()
            .map(|x| {
                let v: Vec<&str> = x[1..].splitn(2, ']').collect();
                let dt = Utc.datetime_from_str(v[0], "%Y-%m-%d %H:%M").unwrap();
                (dt, v[1].to_string())
            })
            .collect();
        vec2.sort_unstable_by(|(a1, _), (b1, _)| a1.cmp(b1));
        vec2
    }

    #[allow(dead_code)]
    pub fn part1(str_vec: Vec<String>) -> i32 {
        let mut user_map: HashMap<i32, Box<Guard>> = HashMap::new();
        let vec2: Vec<(DateTime<Utc>, String)> = parse_input(str_vec);
        let mut current_id = 0i32;
        vec2.iter().for_each(|(dt, x)| {
            if x.contains("Guard") {
                let split_vec: Vec<&str> = x.split(|c| c == '#' || c == ' ').collect();
                let id: i32 = split_vec.get(3).unwrap().parse().unwrap();
                current_id = id;
                user_map.entry(current_id).or_insert(Box::new(Guard {
                    id: current_id,
                    fall_sleep: 0,
                    sleeps: vec![0; 60],
                }));
            } else {
                if let Some(current) = user_map.get_mut(&current_id) {
                    let tm: i32 = dt.minute() as i32;
                    if x.contains("falls asleep") {
                        current.fall_sleep = tm;
                    } else if x.contains("wakes up") {
                        for i in current.fall_sleep..tm {
                            current.sleeps[i as usize] += 1i32;
                        }
                    }
                }
            }
        });
        let mut max = 0i32;
        let mut id = 0i32;
        for (_, g) in user_map.iter() {
            let c = g.sleep_count();
            if max < c {
                id = g.id;
                max = c;
            }
        }
        if let Some(g) = user_map.get(&id) {
            max = 0;
            let mut idx = 0i32;
            for (i, val) in g.sleeps.iter().enumerate() {
                if max < *val {
                    max = *val;
                    idx = i as i32;
                }
            }
            // println!("val:{:?}", (id, idx));
            return id * idx;
        }
        0i32
    }
    #[allow(dead_code)]
    pub fn part2(str_vec: Vec<String>) -> i32 {
        let mut user_map: HashMap<i32, Box<Guard>> = HashMap::new();
        let vec2: Vec<(DateTime<Utc>, String)> = parse_input(str_vec);
        let mut current_id = 0i32;
        vec2.iter().for_each(|(dt, x)| {
            if x.contains("Guard") {
                let split_vec: Vec<&str> = x.split(|c| c == '#' || c == ' ').collect();
                let id: i32 = split_vec.get(3).unwrap().parse().unwrap();
                current_id = id;
                user_map.entry(current_id).or_insert(Box::new(Guard {
                    id: current_id,
                    fall_sleep: 0,
                    sleeps: vec![0; 60],
                }));
            } else {
                if let Some(current) = user_map.get_mut(&current_id) {
                    let tm: i32 = dt.minute() as i32;
                    if x.contains("falls asleep") {
                        current.fall_sleep = tm;
                    } else if x.contains("wakes up") {
                        for i in current.fall_sleep..tm {
                            current.sleeps[i as usize] += 1i32;
                        }
                    }
                }
            }
        });
        let mut max = 0i32;
        let mut id = 0i32;
        let mut idx = 0i32;
        for (_, g) in user_map.iter() {
            for (i, val) in g.sleeps.iter().enumerate() {
                if max < *val {
                    id = g.id;
                    max = *val;
                    idx = i as i32;
                }
            }
        }
        // println!("max,{:?}", (id, idx, max ));
        id * idx
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    fn parse_from_str(val: &str) -> Vec<String> {
        let str_vec: Vec<String> = val
            .split('\n')
            .map(|x| x.trim())
            .filter(|&x| x.len() != 0)
            .map(|x| x.to_string())
            .collect();
        str_vec
    }

    static STR_INPUT: &str = r#"
        [1518-11-01 00:00] Guard #10 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-04 00:46] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-05 00:55] wakes up
    "#;

    #[test]
    fn day4_part1() {
        let vec = parse_from_str(STR_INPUT);
        let r1 = day4::part1(vec);
        assert_eq!(r1, 240);
        let list2 = common::parse_from_file("./data/day4_part1.txt").unwrap();
        let r2 = day4::part1(list2);
        assert_eq!(r2, 36898);
    }

    #[test]
    fn day4_part2() {
        let vec = parse_from_str(STR_INPUT);
        let r1 = day4::part2(vec);
        assert_eq!(r1, 4455);
        let list2 = common::parse_from_file("./data/day4_part1.txt").unwrap();
        let r2 = day4::part2(list2);
        assert_eq!(r2, 80711);
    }
}
