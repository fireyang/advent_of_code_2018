mod day2 {
    // use std::collections::HashSet;
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        let mut twice = 0i32;
        let mut three = 0i32;
        let mut twice_add = 0i32;
        let mut three_add = 0i32;
        let mut h: HashMap<char, i32> = HashMap::new();
        vec.iter().for_each(|x| {
            for ch in x.chars() {
                let counter = h.entry(ch).or_insert(0);
                *counter += 1;
            }
            twice_add = 0;
            three_add = 0;
            for (_, num) in &h {
                match num {
                    2 => twice_add = 1,
                    3 => three_add = 1,
                    _ => (),
                }
            }
            twice += twice_add;
            three += three_add;
            h.clear();
        });
        twice * three
    }

    #[allow(dead_code)]
    pub fn part2(vec: Vec<String>) -> String {
        let mut max = 0usize;
        let mut r_str: String = String::new();
        for (i, x) in vec.iter().enumerate() {
            for y in vec[i + 1..].iter() {
                let mut vec2: Vec<char> = vec![];
                for (s1, s2) in x.chars().zip(y.chars()) {
                    if s1 == s2 {
                        vec2.push(s1);
                    }
                }
                if max < vec2.len() {
                    max = vec2.len();
                    // r_str = vec2.iter().collect::<String>();
                    r_str = vec2.iter().collect();
                }
            }
        }
        r_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    fn parse_from_str(val: &str) -> Vec<String> {
        val.split_whitespace()
            .map(|v| v.trim().to_string())
            .collect()
    }

    #[test]
    fn day2_part1() {
        let s = r#"
        abcdef
        bababc
        abbcde
        abcccd
        aabcdd
        abcdee
        ababab
            "#;
        let list: Vec<String> = parse_from_str(s);
        let r = day2::part1(list);
        // println!("test,{:?}", list);
        assert_eq!(r, 12);
        let list2 = common::parse_from_file("./data/day2_part1.txt");
        let r2 = day2::part1(list2.unwrap());
        assert_eq!(r2, 8610);
    }

    #[test]
    fn day2_part2() {
        let s = r#"
        abcde
        fghij
        klmno
        pqrst
        fguij
        axcye
        wvxyz
        "#;
        let list: Vec<String> = parse_from_str(s);
        let r = day2::part2(list);
        assert_eq!(r, "fgij");
        let list2 = common::parse_from_file("./data/day2_part2.txt");
        let r2 = day2::part2(list2.unwrap());
        assert_eq!(r2, "iosnxmfkpabcjpdywvrtahluy");
    }
}
