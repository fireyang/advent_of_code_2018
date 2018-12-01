
mod day1 {
    use std::collections::HashSet;

    #[allow(dead_code)]
    pub fn puzzle1(vec: Vec<String>) -> i32 {
        let v: Vec<i32> = vec
            .iter()
            .map(|value| value.parse::<i32>().unwrap())
            // .sum();
            .collect();
        // println!("out:{:?}", v);
        let result = v.iter().sum();

        return result;
    }

    #[allow(dead_code)]
    pub fn puzzle2(vec: Vec<String>) -> i32 {
        let mut current:i32 = 0;
        let vec2 : Vec<i32>= vec.iter().map(|value|value.parse::<i32>().unwrap()).collect();
        let mut frequency_set: HashSet<i32> = HashSet::new();
        while !frequency_set.contains(&current){
            for x in vec2.iter() {
                frequency_set.insert(current);
                current += x;
                // println!("current,add,{:?},{:?}", x,current);
                if frequency_set.contains(&current){
                    break;
                }
            };
        }
        return current;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    // use std::path::Path;

    fn parse_from_str(val: &str) -> Vec<String> {
        val.split(",").map(|v| v.trim().to_string()).collect()
    }

    fn parse_from_file(path: &str) -> Result<Vec<String>, &'static str> {
        let f = match File::open(path) {
            Ok(f) => f,
            _ => return Err("file not exist"),
        };
        let file = BufReader::new(&f);
        let mut lines = vec![];
        for line in file.lines() {
            let l = line.unwrap();
            lines.push(l);
            // println!("{}", l);
        }
        // println!("{:?}", lines);
        return Ok(lines);
    }

    fn check_str1(val: &str) -> i32 {
        let vec1 = parse_from_str(val);
        day1::puzzle1(vec1)
    }

    fn check_file1(path: &str) -> i32 {
        let r = match parse_from_file(path) {
            Ok(vec) => day1::puzzle1(vec),
            _ => 0,
        };
        return r;
    }

    fn check_str2(val: &str) -> i32 {
        let vec1 = parse_from_str(val);
        day1::puzzle2(vec1)
    }

    fn check_file2(path: &str) -> i32 {
        let r = match parse_from_file(path) {
            Ok(vec) => day1::puzzle2(vec),
            _ => 0,
        };
        return r;
    }

    #[test]
    fn day1_part1() {
        assert_eq!(check_str1("+1, +1, +1"), 3);
        assert_eq!(check_str1("+1, +1, -2"), 0);
        assert_eq!(check_str1("-1, -2, -3"), -6);
        let r = check_file1("./data/day1_part1.txt");
        assert_eq!(r, 477);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(check_str2("+1, -1"), 0);
        assert_eq!(check_str2("+3, +3, +4, -2, -4"), 10);
        assert_eq!(check_str2("-6, +3, +8, +5, -6"), 5);
        assert_eq!(check_str2("+7, +7, -2, -7, -4"), 14);
        let r = check_file2("./data/day1_part2.txt");
        assert_eq!(r, 390);
    }
}
