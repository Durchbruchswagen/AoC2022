use std::{fs::File, io::{BufReader, BufRead}, vec};
fn FromSNFU(number:&str)->i128{
    let mut retval=0;
    let mut foo=number.to_string();
    foo=foo.chars().rev().collect();
    let mut mult=1;
    for c in foo.chars(){
        match c{
            '0'=>retval+=0,
            '1'=>retval+=mult,
            '2'=>retval+=2*mult,
            '='=>retval+=-2*mult,
            '-'=>retval-=mult,
            _=>()
        }
        mult*=5
    }
    retval
}
fn ToSNFU(number:i128)->String{
    let mut help=Vec::new();
    let mut numb=number;
    let mut ind=0;
    let mut off=0;
    help.push(0);
    let mut breakflag=false;
    loop{
        numb+=off;
        if numb<5{
            breakflag=true;
        }
        off=0;
        let rem=numb%5;
        numb=numb/5;
        if help.len()<=ind{
            help.push(0);
        }
        match rem{
            0=>help[ind]+=0,
            1=>help[ind]+=1,
            2=>help[ind]+=2,
            3=>help[ind]+=-2,
            4=>help[ind]+=-1,
            _=>()
        }
        if rem==3 || rem==4{
            off=1;
        }
        ind+=1;
        if breakflag{
            if off>0{
                help.push(1);
            }
            break;
        }
    }
    let mut rtval:String=help.into_iter().map(|x| match x {
        0=>'0',
        1=>'1',
        2=>'2',
        -1=>'-',
        -2=>'=',
        _=>'?'
    }).collect();
    rtval=rtval.chars().rev().collect();
    rtval
}
fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut wynik=0;
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        wynik+=FromSNFU(&help);
    }
    let napis=ToSNFU(wynik);
    println!("{}",napis);
}
