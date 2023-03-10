use std::{fs::File, io::{BufReader, BufRead}};
fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut points=0;
    let mut points2=0;
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let help2:Vec<&str>=help.split(&['-',',']).collect();
        let fsta:i32=help2[0].parse().unwrap();
        let fstb=help2[1].parse().unwrap();
        let ssta=help2[2].parse().unwrap();
        let sstb=help2[3].parse().unwrap();
        if (fsta<=ssta && fstb>=ssta && fsta<=sstb && fstb>=sstb) || (ssta<=fsta && sstb>=fsta && ssta<=fstb && sstb>=fstb){
            //println!("{}-{} and {}-{}",fsta,fstb,ssta,sstb);
            points+=1;
        }
        if(ssta>=fsta && ssta<=fstb) || (sstb>=fsta && sstb<=fstb) || (fsta>=ssta && fsta<=sstb) || (fstb>=ssta && fstb <=sstb){
            points2+=1;
        }
    }
    println!("{},{}",points,points2);
}
