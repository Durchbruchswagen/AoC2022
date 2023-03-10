use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut points=0;
    let mut points2=0;
    let mut nread=0;
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    for lines in reader.lines(){
        let mut help="".to_string();
        let mut books = HashSet::new();
        match lines {
            Ok(x)=>help=x.to_string(),
            _=>()
        }
        let tokens=help.chars().collect::<Vec<char>>();
        if(nread==0){
            for x in tokens{
                set1.insert(x);
            }
            nread=1
        }
        else if(nread==1){
            for x in tokens{
                set2.insert(x);
            }
            nread=2
        }
        else if(nread==2){
            for x in tokens{
                if (set1.contains(&x) && set2.contains(&x)){
                    let valhelp=x as u32;
                    if(valhelp>=97){points+=valhelp-96}
                    else{points+=valhelp-65+27}
                break;
                }
            };
            set1=HashSet::new();
            set2=HashSet::new();
            nread=0
        }
        else{
            let len=tokens.len();
        for i in 0..(len/2){
            books.insert(tokens[i]);
        }
        for i in (len/2)..len{
            if books.contains(&tokens[i]){
                let valhelp=tokens[i] as u32;
                if(valhelp>=97){points+=valhelp-96}
                else{points+=valhelp-65+27}
                break;
            }
        }
        }
    }
    println!("{},{}",points,points2);
}
