use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world!");
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut points=0;
    let mut points2=0;
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines {
            Ok(x)=>help=x.to_string(),
            _=>()
        }
        let tokens=help.split(' ').collect::<Vec<&str>>();
        let pair=(tokens[0],tokens[1]);
        match pair{
            ("A","Y")=>points+=8,
            ("A","X")=>points+=4,
            ("A","Z")=>points+=3,
            ("B","Y")=>points+=5,
            ("B","X")=>points+=1,
            ("B","Z")=>points+=9,
            ("C","Y")=>points+=2,
            ("C","X")=>points+=7,
            ("C","Z")=>points+=6,
            _=>()
        }
        match pair{
            ("A","Y")=>points2+=4,
            ("A","X")=>points2+=3,
            ("A","Z")=>points2+=8,
            ("B","Y")=>points2+=5,
            ("B","X")=>points2+=1,
            ("B","Z")=>points2+=9,
            ("C","Y")=>points2+=6,
            ("C","X")=>points2+=2,
            ("C","Z")=>points2+=7,
            _=>()
        }
    }
    println!("{},{}",points,points2);
}
