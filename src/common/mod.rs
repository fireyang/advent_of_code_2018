use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn parse_from_file(path: &str) -> Result<Vec<String>, &'static str> {
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
