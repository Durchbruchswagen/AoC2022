use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world!");
    let file = File::open("data.txt").unwrap();
    let mut vec=Vec::new();
    let reader = BufReader::new(file);
    let mut actval=0;
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines {
            Ok(x)=>help=x.to_string(),
            _=>()
        }
        if(help==""){
            vec.push(actval);
            actval=0;
        }
        else{
            actval+=help.parse::<i32>().unwrap();
        }
    }
    vec.sort_by(|a,b| b.cmp(a));
    println!("{},{},{},{}",vec[0],vec[1],vec[2],vec[0]+vec[1]+vec[2]);
}
