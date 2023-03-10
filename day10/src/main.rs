use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::collections::HashSet;
fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut secik=HashSet::new();
    let mut secik2=HashSet::new();
    let mut obraz=vec![vec!['.';40 as usize];7 as usize];
    let mut wynik=0;
    let mut row=0;
    let mut col=0;
    secik.insert(20);
    secik.insert(60);
    secik.insert(100);
    secik.insert(140);
    secik.insert(180);
    secik.insert(220);
    secik2.insert(40);
    secik2.insert(80);
    secik2.insert(120);
    secik2.insert(160);
    secik2.insert(200);
    secik2.insert(240);
    let mut cycles:i32=0;
    let mut reg=1;
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let tokens=help.split_whitespace().collect::<Vec<&str>>();
        if tokens[0]=="noop"{
            if cycles.rem_euclid(40)==col+2 || cycles.rem_euclid(40)==col ||cycles.rem_euclid(40)==col+1 {
                println!("cykl:{},pozycja sprite x:{}y:{},{}",cycles,col,row,obraz[row].iter().collect::<String>());
                obraz[row][cycles.rem_euclid(40) as usize]='#';
            }
            cycles+=1;
            if secik.contains(&cycles) {
                wynik+=cycles*reg;
            }
            if secik2.contains(&cycles){
                row+=1;
            }
            
        }
        else if tokens[0]=="addx"{
            if cycles.rem_euclid(40)==col+2 || cycles.rem_euclid(40)==col ||cycles.rem_euclid(40)==col+1{
                println!("cykl:{},pozycja sprite x:{}y:{},{}",cycles,col,row,obraz[row].iter().collect::<String>());
                obraz[row][cycles.rem_euclid(40) as usize]='#';
            }
            if secik2.contains(&(cycles+1)) {
                row+=1;
            }

            if (cycles+1).rem_euclid(40)==col+2 || (cycles+1).rem_euclid(40)==col ||(cycles+1).rem_euclid(40)==col+1 {
                println!("cykl:{},pozycja sprite x:{}y:{},{}",cycles,col,row,obraz[row].iter().collect::<String>());
                obraz[row][(cycles+1).rem_euclid(40) as usize]='#';
            }

            if secik2.contains(&(cycles+2)) {
                row+=1;
            }

            cycles+=2;
            if secik.contains(&cycles){
                wynik+=reg*cycles;
            }
            else if secik.contains(&(cycles-1)){
                wynik+=reg*(cycles-1)
            }
            reg+=tokens[1].parse::<i32>().unwrap();
            col+=tokens[1].parse::<i32>().unwrap();
        }
    }
    println!("{}",wynik);
    for i in 0..6{
        let line1:String=obraz[i].iter().collect::<String>();
        println!("{}",line1);
    }
}
