mod day8 {

    #[derive(Debug)]
    struct Node {
        pub children: Vec<Box<Node>>,
        pub metas: Vec<i32>,
    }

    impl Node {
        pub fn total(&self) -> i32 {
            let child = self.children.iter().map(|c| c.total()).sum::<i32>();
            self.metas.iter().sum::<i32>() + child
        }
    }

    fn create(num: i32, inputs: &mut Vec<i32>) -> Vec<Box<Node>> {
        (0..num)
            .into_iter()
            .map(|_| {
                let child_count = inputs.remove(0);
                let meta_count = inputs.remove(0) as usize;
                let children = create(child_count, inputs);
                let metas: Vec<i32> = inputs.drain(0..meta_count).collect();
                Box::new(Node {
                    children: children,
                    metas: metas,
                })
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn part1(mut inputs: Vec<i32>) -> i32 {
        let tree = create(1, &mut inputs);
        //println!("{:?}", tree[0].total());
        tree[0].total()
    }
}
#[cfg(test)]
mod tests {

    // use std::collections::HashMap;

    use super::*;
    use crate::common;

    static STR_INPUT: &str = r#"
    2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
    "#;

    fn parse_from_str(val: &String) -> Vec<i32> {
        let v: Vec<Vec<i32>> = val
            .split('\n')
            .map(|x| x.to_string())
            .map(|x| {
                x.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect()
            })
            // .flatten()
            .collect();
        // println!("vec,{:?}", v);
        v.into_iter().flatten().collect()
    }

    #[test]
    fn day8_part1() {
        let content = &STR_INPUT.to_string();
        let input = parse_from_str(&content);
        assert_eq!(day8::part1(input), 138);
        let content2 = common::read_from_file("./data/day8_part1.txt").unwrap();
        let points2 = parse_from_str(&content2);
        assert_eq!(day8::part1(points2), 41028);
    }
}
