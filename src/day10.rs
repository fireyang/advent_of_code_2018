mod day10 {
    use std::collections::HashSet;

    #[allow(dead_code)]
    pub fn print_msg(input: &Vec<(i32, i32, i32, i32)>, time: i32) {
        let mut sx = i32::max_value();
        let mut sy = sx;
        let mut ex = 0;
        let mut ey = 0;
        let mut after_map = HashSet::new();
        let after: Vec<(i32, i32)> = input
            .iter()
            .map(|val| {
                let x = val.0 + val.2 * time;
                let y = val.1 + val.3 * time;
                (x, y)
            })
            .collect();
        after.iter().for_each(|(x, y)| {
            let v = (x, y);
            after_map.insert(v);
            if sx > *x {
                sx = *x;
            }
            if sy > *y {
                sy = *y;
            }
            if ex < *x {
                ex = *x;
            }
            if ey < *y {
                ey = *y;
            }
        });
        for i in sy - 1..ey + 2 {
            let mut l = String::new();
            for j in sx - 1..ex + 2 {
                let pos = (&j, &i);
                if after_map.contains(&pos) {
                    l.push('#');
                } else {
                    l.push('.');
                }
            }
            // println!("{:?}",l.iter().collect::<String>());
            println!("{:?}", l);
        }
    }

    #[allow(dead_code)]
    pub fn part1(input: &Vec<(i32, i32, i32, i32)>, start: i32, times: i32, step: usize) -> i32 {
        let mut width = vec![];
        let mut height = vec![];
        let mut min_size = i32::max_value();
        let mut cur_time = 0;
        for time in (start..times).step_by(step) {
            cur_time = time;
            let mut sx = i32::max_value();
            let mut sy = sx;
            let mut ex = 0;
            let mut ey = 0;
            let after: Vec<(i32, i32)> = input
                .iter()
                .map(|val| {
                    let x = val.0 + val.2 * time;
                    let y = val.1 + val.3 * time;
                    (x, y)
                })
                .collect();
            after.iter().for_each(|(x, y)| {
                if sx > *x {
                    sx = *x;
                }
                if sy > *y {
                    sy = *y;
                }
                if ex < *x {
                    ex = *x;
                }
                if ey < *y {
                    ey = *y;
                }
            });
            let size = ex - sx + ey - sy;
            if min_size > size {
                min_size = size;
            } else {
                width.push(ex - sx);
                height.push(ey - sy);
                break;
            }
            // width.push(ex - sx);
            // println!("width, {:?}",ex- sx);
            // height.push(ey - sy);
        }
        println!("size, {:?}", min_size);
        println!("current, {:?}", cur_time);
        println!("width, {:?}", width);
        println!("height, {:?}", height);
        cur_time - 1
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::common;

    fn parse(val: &str) -> i32 {
        val.trim().parse::<i32>().unwrap()
    }

    fn parse_from_str(val: &String) -> Vec<(i32, i32, i32, i32)> {
        val.split('\n')
            .map(|x| x.trim())
            .filter(|&x| x.len() != 0)
            .map(|x| {
                let vec: Vec<&str> = x.split(|c| c == '<' || c == '>' || c == ',').collect();
                // println!("{:?}", vec);
                let x = parse(&vec[1]);
                let y = parse(&vec[2]);
                let vx = parse(&vec[4]);
                let vy = parse(&vec[5]);
                (x, y, vx, vy)
            })
            .collect()
    }

    #[test]
    fn day10_part1() {
        let content = common::read_from_file("./data/day10_part1_test.txt").unwrap();
        let input = parse_from_str(&content);
        // println!("{:?}", input);
        let current = day10::part1(&input, 0, 10, 1);
        day10::print_msg(&input, current);
        // let content2 = common::read_from_file("./data/day10_part1.txt").unwrap();
        // let input2 = parse_from_str(&content2);
        // let current2 = day10::part1(&input2, 10009, 10012, 1);
        // day10::print_msg(&input2, current2);
        // assert_eq!(day7::part1(&points), "CABDFE".to_string());
    }
}
