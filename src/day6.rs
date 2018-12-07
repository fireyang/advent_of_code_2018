mod day6 {
    use std::collections::HashMap;
    // use std::collections::HashSet;

    #[derive(Debug)]
    pub struct Coordinate {
        pub id: usize,
        pub x: i32,
        pub y: i32,
    }

    fn get_cycle_points(step: i32) -> Vec<(i32, i32)> {
        let mut vec = vec![];
        for i in -step..step + 1 {
            for j in -step..step + 1 {
                if i.abs() + j.abs() <= step {
                    vec.push((i, j));
                }
            }
        }
        vec
    }

    #[allow(dead_code)]
    fn show_area(map: &HashMap<(i32, i32), (usize, i32)>, size: i32) {
        for i in 0..size {
            let mut s = vec![];
            for j in 0..size {
                let pos = (j as i32, i as i32);
                let id3 = match map.get(&pos) {
                    Some((id, _)) => *id,
                    _ => 0usize,
                };
                s.push(id3);
            }
            println!("map, {:?}", s);
        }
    }

    #[allow(dead_code)]
    pub fn part1(points: Vec<(i32, i32)>) -> i32 {
        let coordinates : Vec<Coordinate>= points
            .iter()
            .enumerate()
            .map(|(id, (x, y))| Coordinate {
                id: id + 1,
                x: *x,
                y: *y,
            })
            .collect();
        let mut map = HashMap::new();
        let mut count_map = HashMap::new();
        let mut infinites =  vec![];
        for c in coordinates.iter() {
            let step = coordinates
                .iter()
                .map(|c2| (c.x - c2.x).abs() + (c.y - c2.y).abs())
                .max().unwrap();
            let vec = get_cycle_points(step);

            // println!("step, {:?}", (c, step));
            let mut count = 0;
            let mut is_infinite = false;
            for (x, y) in vec.iter() {
                let px = *x + c.x;
                let py = *y + c.y;
                // if px < 0 || py < 0 {
                    // is_infinite = true;
                    // continue;
                // }
                let pos2 = (px, py);
                let dis1 = x.abs() + y.abs();
                let mut can_insert = true;
                for c2 in coordinates.iter() {
                    if c.id == c2.id {
                        continue;
                    }
                    let dis2 = (px - c2.x).abs() + (py - c2.y).abs();
                    // println!("i4:{:?}", (c.id,pos2, c2, dis2, dis1));
                    if dis2 <= dis1 {
                        can_insert = false;
                        break;
                    }
                }

                if can_insert{
                    if dis1 == step{
                        is_infinite = true;
                    }
                    if px < 0 || py < 0 {
                        continue;
                    }
                    count += 1;
                    map.insert(pos2, (c.id, dis1));
                }
            }
            if is_infinite{
                infinites.push(c.id);
            }else{
                count_map.insert(c.id, count);
            }
        }
        // println!("max2, {:?}", infinites);
        // show_area(&map, 11);
        // let mut r_map = HashMap::new();
        // for (_, (id, _)) in map.iter() {
        //     let count = r_map.entry(id).or_insert(0);
        //     *count += 1;
        // }
        let max = count_map.iter().map(|(_, v)| v).max().unwrap();
        println!("max, {:?}", count_map);
        *max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;
    static STR_INPUT: &str = r#"
    1, 1
    1, 6
    8, 3
    3, 4
    5, 5
    8, 9
    "#;

    fn parse_from_str(val: String) -> Vec<(i32, i32)> {
        let points = val
            .split('\n')
            .map(|x| x.trim())
            .filter(|&x| x.len() != 0)
            .map(|x| {
                let o: Vec<i32> = x
                    .split(",")
                    .map(|x| x.trim())
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                (o[0], o[1])
            })
            .collect();
        points
    }

    #[test]
    fn day6_part1() {
        let points = parse_from_str(STR_INPUT.to_string());
        assert_eq!(day6::part1(points), 17);
        // println!("{:?}", v);
        let content = common::read_from_file("./data/day6_part1.txt").unwrap();
        let points2 = parse_from_str(content);
        assert_eq!(day6::part1(points2), 3647);
    }

}
