mod day9 {
    use std::collections::HashMap;
    // use std::collections::HashSet;
    use std::rc::Rc;
    use std::rc::Weak;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct Node{
        pub val: i32,
        pub is_root: bool,
        pub pre: Weak<RefCell<Box<Node>>>,
        pub next: Weak<RefCell<Box<Node>>>,
    }

    impl Node{
        pub fn new(val:&i32) -> RefCell<Box<Node>>{
            RefCell::new(Box::new(Node{
                val: *val,
                is_root: false,
                pre: Weak::new(),
                next: Weak::new(),
            }))
        }

        pub fn next(&self) -> Rc<RefCell<Box<Node>>>{
            self.next.upgrade().unwrap()
        }

        pub fn pre(&self) -> Rc<RefCell<Box<Node>>>{
            self.pre.upgrade().unwrap()
        }

        pub fn back(&self, step: usize) -> Rc<RefCell<Box<Node>>>{
            let p = self.pre.upgrade().unwrap();
            if step == 1{
                return p;
            }
            p.clone().borrow().back(step - 1)
        }

        #[allow(dead_code)]
        pub fn val(&self) -> i32{
            self.val
        }
        // pub fn pre(&self) -> Rc<RefCell<Box<Node>>>{
        //     self.pre.upgrade().unwrap()
        // }

        #[allow(dead_code)]
        pub fn values(&self) -> Vec<i32>{
            let mut vec = vec!();
            vec.push(self.val);
            let mut child = self.next();
            // println!("{:?}", child);
            // println!("{:?}",  Rc::ptr_eq(&child, &self.pre()));
            while !Rc::ptr_eq(&child, &self.pre()){
                {
                    let cb = child.borrow();
                    let v = cb.val;
                    vec.push(v);
                }
                child = child.clone().borrow().next();
            }
            vec
        }
    }


    #[allow(dead_code)]
    pub fn part1(player_count: i32, multiple: i32, points: i32 ) -> i64 {
        let mut players = HashMap::new();
        let mut marbles = Vec::with_capacity((points + 1) as usize);
        marbles.push(0);
        let mut current = 0;
        let mut current_player = 0;
        for i in 1..points+1{
            if  i % multiple != 0{
                current += 2;
                let len = marbles.len();
                if current > len{
                    current = current -len;
                }
                marbles.insert(current, i);
            }else{
                let score = players.entry(current_player).or_insert(0i64);
                // println!("turn:{:?}", i);
                let next = get_pos(current, marbles.len(), -7);
                let remove_score = marbles.remove(next);
                *score += i as i64 + remove_score as i64;
                // println!("turn2:{:?}", (i, current, remove_score,  current_player, score));
                println!("{:?}", (current_player, i, remove_score));
                current = next;
            }
            if i % 10000 ==0 || i > 7100000{
                println!("current,{:?}", (i, marbles.len()));
            }
            current_player +=1;
            current_player = current_player % player_count;
        }
        let max_source = players.iter()
            .map(|(_,v)|v)
            .max().unwrap();
        // println!("players:{:?}", max_source);
        *max_source
    }

    #[allow(dead_code)]
    pub fn part2(player_count: i32, multiple: i32, points: i32 ) -> i64 {
        let mut m = HashMap::new();
        let root = Rc::new(Node::new(&0));
        {
            let mut root_mut = root.borrow_mut();
            root_mut.is_root = true;
            // println!("aaaa");
            root_mut.pre = Rc::downgrade(&root);
            root_mut.next = Rc::downgrade(&root);
        }
        let len = points+1;
        let mut current = root.clone();
        let mut players = HashMap::new();
        let mut current_player = 0;
        for i in 1..len{
            if i % multiple == 0{
                let p = current.borrow().back(7);
                let pb = p.borrow();
                pb.pre().borrow_mut().next = pb.next.clone();
                pb.next().borrow_mut().pre = pb.pre.clone();
                let v = pb.val;
                m.remove(&v);
                let score = players.entry(current_player).or_insert(0i64);
                *score += i as i64 + v as i64;
                // println!("{:?}", (current_player, i, v));
                current = pb.next();
            }else{
                let n = current.borrow().next();
                let n2 = n.borrow().next();
                current = Rc::new(Node::new(&i));
                m.entry(i).or_insert(current.clone());
                {
                    let mut new_node_mut = current.borrow_mut();
                    new_node_mut.next = Rc::downgrade(&n2.clone());
                    new_node_mut.pre = Rc::downgrade(&n.clone());
                }
                n.borrow_mut().next = Rc::downgrade(&current);
                n2.borrow_mut().pre = Rc::downgrade(&current);
            }
            // let r = root.borrow().values();
            // println!("{:?}", r);
            // println!("{:?}", i);
            current_player +=1;
            current_player = current_player % player_count;
        }
        let max_source = players.iter()
            .map(|(_,v)|v)
            .max().unwrap();
        // println!("players:{:?}", max_source);
        // println!("players:{:?}", players);
        *max_source
        // 0
    }

    fn get_pos(current: usize, len: usize, val: i32) -> usize{
        if len == 0{
            return 0;
        }else{
            let mut next = current as i32 + val;
            while next<0{
                next += len as i32;
            }
            return next as usize % len;
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    // use crate::common;

    #[test]
    fn day9_part1() {
        // assert_eq!(day9::part2(9, 23, 25), 32);
        // assert_eq!(day9::part2(10, 23, 1618), 8317);
        // assert_eq!(day9::part1(13, 23, 7999), 146373);
        // assert_eq!(day9::part1(17, 23, 1104), 2764);
        // assert_eq!(day9::part2(21, 23, 6111), 54718);
        // assert_eq!(day9::part1(30, 23, 5807), 37305);
        // assert_eq!(day9::part1(9, 23, 300), 32);

        // assert_eq!(day9::part2(468, 23, 71010), 374287);
        // assert_eq!(day9::part1(468, 23, 71010 * 2), 1380094);
        // 2991367
        // 5205125
        assert_eq!(day9::part2(468, 23, 71010 * 100), 1380094);
    }
}
