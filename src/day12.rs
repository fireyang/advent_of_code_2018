mod day12 {
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn part1(init: &str, temp: &Vec<(&str, &str)>, generation: i32) -> i32 {
        let mut m = HashMap::new();
        for (k, v) in temp.iter() {
            if *v == "#" {
                m.entry(k).or_insert(*v);
            }
        }
        let mut start = init.to_string();
        let mut result = String::new();
        let mut offset = 0i32;
        for _i in 0..generation {
            result.clear();
            let mut s = String::with_capacity(start.len() + 8);
            offset -= 2;
            s.push_str("....");
            s.push_str(start.as_str());
            s.push_str("....");
            for i in 0..s.len() - 4 {
                let cur = s.get(i..i + 5).unwrap();
                if let Some(v) = m.get(&cur) {
                    result.push_str(v);
                } else {
                    result.push('.');
                }
            }
            // let skip = result.chars().skip_while(|&x| x == '.').count();
            let plant_offset = result.find('#').unwrap();
            offset += plant_offset as i32;
            start = result.trim_matches('.').to_string();
            // let num = start.chars().filter(|&x| x =='#').count();
            // println!("{:?},{:?}", i+1, num);
        }
        // println!("{:?}",start);
        let mut val = 0;
        for (i, it) in start.chars().enumerate() {
            if it == '#' {
                val += i as i32 + offset;
            }
        }
        val
    }
}
#[cfg(test)]
mod tests {

    static STR_INPUT: &str = r#"
    initial state: #..#.#..##......###...###

        ...## => #
        ..#.. => #
        .#... => #
        .#.#. => #
        .#.## => #
        .##.. => #
        .#### => #
        #.#.# => #
        #.### => #
        ##.#. => #
        ##.## => #
        ###.. => #
        ###.# => #
        ####. => #
    "#;

    fn parse_from_str(val: &String) -> (&str, Vec<(&str, &str)>) {
        let lines: Vec<&str> = val
            .split('\n')
            .map(|x| x.trim())
            .filter(|&x| x.len() != 0)
            .collect();
        let init = lines[0].trim().split_whitespace().collect::<Vec<&str>>()[2];

        let temp = lines[1..]
            .to_vec()
            .iter()
            .map(|x| x.trim())
            .map(|x| {
                let vec: Vec<&str> = x.split_whitespace().collect();
                (vec[0], vec[2])
            })
            .collect();
        (init, temp)
    }

    use super::*;
    use crate::common;

    #[test]
    fn day11_part1() {
        let input = &STR_INPUT.to_string();
        let (init_state, temp) = parse_from_str(input);
        // println!("{:?}", (init_state, temp.borrow()));
        assert_eq!(day12::part1(init_state, &temp, 20), 325);
        let content = common::read_from_file("./data/day12_part1.txt").unwrap();
        let (init_state2, temp2) = parse_from_str(&content);
        assert_eq!(day12::part1(init_state2, &temp2, 20), 2736);
    }
    #[test]
    fn day11_part2() {
        let content = common::read_from_file("./data/day12_part1.txt").unwrap();
        let (init_state2, temp2) = parse_from_str(&content);
        let pre = day12::part1(init_state2, &temp2, 124) as i64;
        let next = day12::part1(init_state2, &temp2, 125) as i64;
        let sum = pre + (50000000000i64 - 124) * (next - pre);
        assert_eq!(sum, 3150000000905i64);
    }
}
