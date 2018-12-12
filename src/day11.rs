mod day11 {
    use std::collections::HashSet;
    // use std::num::Float;

    #[allow(dead_code)]
    pub fn power(cell: (i32, i32), sn: i32) -> i32 {
        let rack_id = cell.0 + 10;
        let power_start = rack_id * cell.1;
        let r = (power_start + sn) * rack_id;
        // println!("{:?}", r);
        // println!("{:?}", r / 100 - r / 1000 * 10 - 5);
        r / 100 - r / 1000 * 10 - 5
    }

    #[allow(dead_code)]
    pub fn find_sn(sn: i32, size: i32) -> (i32, i32, i32){
        let mut lagest_power = 0;
        let mut x = 0;
        let mut y = 0;
        for i in 0..300-size{
            for j in 0..300-size{
                let power = squqre((i,j),sn,size);
                if lagest_power < power{
                    lagest_power = power;
                    x = i;
                    y = j;
                }
            }
        }
        (lagest_power, x, y)
    }

    pub fn squqre(cell: (i32, i32), sn: i32, size: i32) -> i32{
        let mut total_power = 0;
        for i in cell.0..cell.0+size{
            for j in cell.1..cell.1+size{
                let v = power((i,j),sn);
                total_power += v;
            }
        }
        total_power
    }

    pub fn part2(sn: i32) -> (i32,i32,i32){
        let mut max_total = 0;
        let mut x =0;
        let mut y = 0;
        let mut c_size = 0;
        for size in 1..300{
            for i in 0..300-size{
                for j in 0..300-size{
                    let total = squqre((i,j), sn, size);
                    // if total > 0{
                        println!("total:{:?}", (sn, i,j,size, total));
                    // }
                    if max_total < total{
                        // println!("total:{:?}", (sn, i,j,size, total));
                        // println!("total:{:?}", total);
                        max_total = total;
                        x = i;
                        y =j;
                        c_size = size;
                    }else{
                        // return (x,y,c_size);
                    }
                }
            }
        }
        (x,y,c_size)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::common;

    static STR_INPUT: &str = r#"
    -2  -4   4   4   4
    -4   4   4   4  -5
     4   3   3   4  -4
      1   1   2   4  -3
      -1   0   2  -5  -2
    "#;

    static STR_INPUT2: &str = r#"
    -3   4   2   2   2
    -4   4   3   3   4
    -5   3   3   4  -4
     4   3   3   4  -3
      3   3   3  -5  -1
    "#;

    #[test]
    fn day11_part1() {

        //base
        assert_eq!(day11::power((3, 5), 8), 4);
        assert_eq!(day11::power((122, 79), 57), -5);
        assert_eq!(day11::power((217, 196), 39), 0);
        assert_eq!(day11::power((101, 153), 71), 4);
        // part1
        //
        assert_eq!(day11::find_sn(18, 3), (29, 33,45));
        assert_eq!(day11::find_sn(42, 3), (30, 21,61));
        assert_eq!(day11::find_sn(8561, 3), (30,21,37));
    }

    #[test]
    fn day11_part2() {
        assert_eq!(day11::part2(8561), (0,0,0));
    }
}
