mod day3 {
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[derive(Debug)]
    pub struct Claim {
        pub id: i32,
        pub left: i32,
        pub top: i32,
        pub wide: i32,
        pub tall: i32,
    }

    impl Claim {
        pub fn points(&self) -> Vec<(i32, i32)> {
            let mut list: Vec<(i32, i32)> = vec![];
            for i in self.left..(self.left + self.wide) {
                for j in self.top..(self.top + self.tall) {
                    list.push((i, j));
                }
            }
            list
        }
    }

    #[allow(dead_code)]
    pub fn part1(list: Vec<Box<Claim>>) -> i32 {
        let mut h: HashMap<(i32, i32), i32> = HashMap::new();
        let mut ret: i32 = 0i32;
        list.iter().for_each(|c| {
            for p in c.points().iter() {
                let counter = h.entry(*p).or_insert(0);
                *counter += 1;
            }
        });
        for (_, &val) in h.iter() {
            if val > 1 {
                ret += 1;
            }
        }
        ret
    }

    #[allow(dead_code)]
    pub fn part2(list: Vec<Box<Claim>>) -> i32 {
        let mut h: HashMap<(i32, i32), i32> = HashMap::new();
        let mut  ok_set: HashSet<i32> = HashSet::new();
        for c in list.iter(){
            let mut is_over = false;
            for p in c.points().iter() {
                match h.get(p) {
                    Some(cid) => {
                        // println!("remove,{:?}", (cid, c.id));
                        ok_set.remove(cid);
                        is_over = true;
                        ()
                    },
                    None => {h.insert(*p, c.id);()}
                }
            }
            if !is_over{
                ok_set.insert(c.id);
            }
        }
        *ok_set.iter().nth(0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::day3::Claim;
    use super::*;
    use crate::common;

    fn parse_claim(val: String) -> Box<Claim> {
        let v: Vec<i32> = val[1..]
            .split(|c| c == '@' || c == ',' || c == ':' || c == 'x')
            .map(|x| x.trim())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        // println!("sp:{:?}",v);
        let c = Box::new(Claim {
            id: v[0],
            left: v[1],
            top: v[2],
            wide: v[3],
            tall: v[4],
        });
        c
    }
    //
    fn parse_from_str(val: &str) -> Vec<Box<Claim>> {
        let str_vec: Vec<&str> = val
            .split('\n')
            .map(|x| x.trim())
            .filter(|&x| x.len() != 0)
            .collect();
        let claim_vec: Vec<Box<Claim>> =
            str_vec.iter().map(|&x| parse_claim(x.to_string())).collect();
        claim_vec
    }

    #[test]
    fn day3_part1() {
        let s = r#"
        #1 @ 1,3: 4x4
        #2 @ 3,1: 4x4
        #3 @ 5,5: 2x2
        "#;
        let list: Vec<Box<Claim>> = parse_from_str(s);
        // println!("result,{:?}", list);
        let r = day3::part1(list);
        assert_eq!(r, 4);
        let list2 = common::parse_from_file("./data/day3_part1.txt").unwrap();
        let claim_vec: Vec<Box<Claim>> = list2.iter().map(|x| parse_claim(x.to_string())).collect();
        let r2 = day3::part1(claim_vec);
        assert_eq!(r2, 105071);
    }
    #[test]
    fn day3_part2() {
        let s = r#"
        #1 @ 1,3: 4x4
        #2 @ 3,1: 4x4
        #3 @ 5,5: 2x2
        "#;
        let list: Vec<Box<Claim>> = parse_from_str(s);
        // println!("result,{:?}", list);
        let r = day3::part2(list);
        assert_eq!(r, 3);
        let list2 = common::parse_from_file("./data/day3_part2.txt").unwrap();
        let claim_vec: Vec<Box<Claim>> = list2.iter().map(|x| parse_claim(x.to_string())).collect();
        let r2 = day3::part2(claim_vec);
        assert_eq!(r2, 222);
    }
}
