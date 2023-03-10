use core::num;
use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::collections::HashMap;
fn solve1(finished:&HashMap<String,i128>,expressions:&HashMap<String,(String,String,String)>,a:String,b:String,c:String,d:String)->i128{
    let v1;
    let v2;
    let mut rtval=0;
    //println!("DUPA:{}",a);
    if finished.contains_key(&b){
        v1=*finished.get(&b).unwrap();
    }
    else{
        //println!("asfs {}",b);
        let help=expressions.get(&b).unwrap();
        v1=solve1(finished, expressions, b, help.0.to_string(), help.1.to_string(), help.2.to_string());
    }
    if finished.contains_key(&d) {
        v2=*finished.get(&d).unwrap();
    }
    else{
        //println!("dasda {}",d);
        let help=expressions.get(&d).unwrap();
        v2=solve1(finished, expressions, d, help.0.to_string(), help.1.to_string(), help.2.to_string());
    }
    match c.as_ref(){
        "+"=>rtval=v1+v2,
        "-"=>rtval=v1-v2,
        "*"=>rtval=v1*v2,
        "/"=>rtval=v1/v2,
        _=>()
    }
    //println!("{} equals {}",a,rtval);
    rtval
}

fn solve2(finished:&mut HashMap<String,i128>,expressions:&HashMap<String,(String,String,String)>,a:String,b:String,c:String,d:String)->(i128,i128){
    let v1;
    let v2;
    let mut rtval=0;
    //println!("DUPA:{}",a);
    if b=="humn"{
        v1=0;
    }
    else if finished.contains_key(&b){
        v1=*finished.get(&b).unwrap();
    }
    else{
        //println!("asfs {}",b);
        let help=expressions.get(&b).unwrap();
        v1=solve2(finished, expressions, b.to_string(), help.0.to_string(), help.1.to_string(), help.2.to_string()).0;
    }
    finished.insert(b, v1);
    if d=="humn"{
        v2=0;
    }
    else if finished.contains_key(&d) {
        v2=*finished.get(&d).unwrap();
    }
    else{
        //println!("dasda {}",d);
        let help=expressions.get(&d).unwrap();
        v2=solve2(finished, expressions, d.to_string(), help.0.to_string(), help.1.to_string(), help.2.to_string()).0;
    }
    finished.insert(d, v2);
    if a=="root"{
        return (v1,v2);
    }
    match c.as_ref(){
        "+"=>rtval=v1+v2,
        "-"=>rtval=v1-v2,
        "*"=>rtval=v1*v2,
        "/"=>rtval=v1/v2,
        _=>()
    }
    //println!("{} equals {}",a,rtval);
    (rtval,0)
}

fn main() {
    let file = File::open("data.txt").unwrap();
    let mut finished = HashMap::new();
    let mut expressions=HashMap::new();
    //let mut order=vec![];
    let reader = BufReader::new(file);
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let monkey=help.split([':',' ']).collect::<Vec<&str>>();
        //println!("{:?}",monkey);
        if monkey.len()==3 {
            finished.insert(monkey[0].to_string(), monkey[2].to_string().parse::<i128>().unwrap());
        }
        else{

            expressions.insert(monkey[0].to_string(),(monkey[2].to_string(),monkey[3].to_string(),monkey[4].to_string()));

        }
    }
    //println!("{:?}",finished);
    //println!("{:?}",expressions);
    let help=expressions.get(&"root".to_string()).unwrap();
    println!("{:?}",help);
    let wynik=solve1(&finished, &expressions, "root".to_string(), help.0.to_string(), help.1.to_string(), help.2.to_string());
    println!("WYNIK:{}",wynik);
    let mut operators=vec![];
    let mut curr="humn".to_string();
    loop{
        if curr=="root"{
            break;
        }
        let mut newcurr="".to_string();
        for (x,(a,b,c)) in &expressions{
            if *a==curr || *c==curr{
                newcurr=x.to_string();
                if *c==curr && b=="-"{
                    operators.push(("--".to_string(),a.to_string()));
                }
                else if *c==curr && b=="/"{
                    operators.push(("//".to_string(),a.to_string()));
                }
                else{
                    if *c==curr{
                        operators.push((b.to_string(),a.to_string()));
                    }
                    else {operators.push((b.to_string(),c.to_string()));
                    }
                }
                break;
            }
        }
        curr=newcurr.to_string();
    }
    let help2=solve2(&mut finished, &expressions, "root".to_string(), help.0.to_string(), help.1.to_string(), help.2.to_string());
    operators.pop();
    println!("{:?}",operators);
    println!("{:?}",help2);
    let mut solveres=help2.1;
    let revoper:Vec<(String,String)>=operators.into_iter().rev().collect();
    for (a,b) in revoper{
        let number=finished.get(&b).unwrap();
        match a.as_ref(){
            "+"=>solveres=solveres-number,
            "-"=>solveres=solveres+number,
            "--"=>solveres=number-solveres,
            "*"=>solveres=solveres/number,
            "/"=>solveres=solveres*number,
            "//"=>solveres=number/solveres,
            _=>()
        }
    }
    println!("{:?}",finished);
    println!("solver:{}",solveres);
}
