use std::{fs::File, io::{BufReader, BufRead}};
fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut resultvec=Vec::new();
    for lines in reader.lines(){
        let mut ind=0;
        let mut wynik=0;
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let mut helps="".to_string();
        for c in help.chars(){
            //println!("{}",c);
            //println!("PACKET:{}",helps);
            wynik+=1;
            //println!("Wynik:{}",wynik);
            //println!("");
            for cc in helps.chars(){
                if c==cc{
                    break;
                }
                ind+=1;
            }
            if ind!=helps.len(){
                helps=helps[(ind+1)..helps.len()].to_string();
            }
            if ind==13{
                resultvec.push(wynik);
                break;
            }
            ind=0;
            helps.push(c);
        }
    }
    println!("{:?}",resultvec);
}
