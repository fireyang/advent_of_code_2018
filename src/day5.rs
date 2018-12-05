mod day5 {
    extern crate regex;
    use std::collections::LinkedList;
    // use std::collections::HashMap;
    use std::collections::HashSet;
    use regex::Regex;
    use std::char;

    fn check_react(a: char, b: &char) -> bool {
        // println!("{:?}",(a, b, a.eq_ignore_ascii_case(b), (a != *b)));
        a.eq_ignore_ascii_case(b) && a != *b
    }

    #[allow(dead_code)]
    pub fn part1(input: String) -> i32 {
        // let mut react = true;
        let mut tail = input.chars().collect::<LinkedList<_>>();
        let mut head: LinkedList<char> = LinkedList::new();
        while let Some(unit) = tail.pop_front() {
            if let Some(unit2) = head.back() {
                if check_react(unit, unit2) {
                    head.pop_back();
                } else {
                    head.push_back(unit);
                }
            } else {
                head.push_back(unit);
            }
        }
        head.len() as i32
    }

    pub fn remove_type_unit(input: &String, c: &char) -> String {
        let exp = format!("{}|{}", c, c.to_uppercase());
        let re = Regex::new(exp.as_str()).unwrap();
        let after = re.replace_all(&input, "");
        after.to_string()
    }

    pub fn scan_letter(input: &String) -> HashSet<char> {
        let mut ch_set = HashSet::new();
        input.to_lowercase().chars().for_each(|ch|{
            ch_set.insert(ch);
        });
        ch_set
    }

    #[allow(dead_code)]
    pub fn part2(input: String) -> i32 {
        let input_set = scan_letter(&input);
        let v = input_set.iter().map(|ch|{
            let s = remove_type_unit(&input, ch);
            let o = part1(s);
            // println!("{:?}", (ch, o));
            o
        }).min();
        v.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day5_part1() {
        assert_eq!(day5::part1("dabAcCaCBAcCcaDA".to_string()), 10);
        let content = common::read_from_file("./data/day5_part1.txt").unwrap();
        assert_eq!(day5::part1(content), 11814);
    }

    #[test]
    fn day5_part2() {
        assert_eq!(day5::part2("dabAcCaCBAcCcaDA".to_string()), 4);
        let content = common::read_from_file("./data/day5_part1.txt").unwrap();
        assert_eq!(day5::part2(content), 4282);
    }
}
