use std::{fs::File, io::{BufReader, BufRead}};
fn main() {
    let file = File::open("data1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut points=0;
    let mut points2=0;
    let mut crates=vec![vec![];9];
    let mut crates2=vec![vec![];9];
    for lines in reader.lines(){
        let mut ind:usize=1;
        let mut stack:usize=0;
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let help2:Vec<char>=help.chars().collect();
        while (stack<9){
            if(help2[ind]!=' '){
                crates[stack].push(help2[ind]);
                crates2[stack].push(help2[ind]);
                
            }
            ind+=4;
            stack+=1;

        }
    }
    for x in 0..9{
        crates[x].reverse();
        crates2[x].reverse();
    }
    let file = File::open("data2.txt").unwrap();
    let reader = BufReader::new(file);
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let help2:Vec<&str>=help.split_whitespace().collect();
        let mut vechelp=Vec::new();
        let n1=help2[1].parse::<usize>().unwrap();
        let n2=help2[3].parse::<usize>().unwrap();
        let n3=help2[5].parse::<usize>().unwrap();
        for _ in 0..n1{
            let x=crates[n2-1].pop().unwrap();
            let y=crates2[n2-1].pop().unwrap();
            crates[n3-1].push(x);
            vechelp.push(y);
            }
        for _ in 0..n1{
            let x=vechelp.pop().unwrap();
            crates2[n3-1].push(x);
            }
    }
    for x in 0..9{
     let help=crates[x].len();
     if(help>0){
        println!("{}",crates[x][help-1]);
     }   
    }
    println!("DRUGA CZĘŚĆ");
    for x in 0..9{
        let help=crates2[x].len();
        if(help>0){
           println!("{}",crates2[x][help-1]);
        }   
       }
}
