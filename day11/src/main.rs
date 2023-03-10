use std::{fs::File, io::{BufReader, BufRead}, vec};
#[derive(Debug,Clone)]
struct Monkey{
    items:Vec<i128>,
    operator:String,
    number:String,
    truem:i128,
    falsem:i128,
    test:i128,
    inspect:i128
}
impl Monkey{
    fn new(operator:&str,number:&str,truem:i128,falsem:i128,test:i128)->Monkey{
        Monkey{
            items:vec![],
            operator:operator.to_string(),
            number:number.to_string(),
            truem:truem,
            falsem:falsem,
            test:test,
            inspect:0
        }
    }
}
fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut monkeynumb=0;
    let mut monkeys:Vec<Monkey>=vec![];
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let tokens=help.split_whitespace().collect::<Vec<&str>>();
        let mut cont=0;
        let mut vechelp:Vec<i128>=vec![];
        let mut operator="".to_string();
        let mut number="".to_string();
        let mut truem=0;
        let mut falsem=0;
        let mut test=0;
        for i in 0..tokens.len(){
            if cont>0 {
                cont-=1;
                continue;
            }
            if tokens[i]=="items" {
                let mut help=0;
                let mut ii=i+1;
                loop {
                    if tokens[ii].parse::<i128>().is_ok(){
                        vechelp.push(tokens[ii].parse::<i128>().unwrap());
                        help+=1;
                        ii+=1;
                    }
                    else{
                        cont=help;
                        break;
                    }
                }
            }
            else if tokens[i]=="Operation" {
                operator=tokens[i+1].to_string();
                number=tokens[i+2].to_string();
                cont=2;
            }
            else if tokens[i]=="Test"{
                test=tokens[i+1].parse::<i128>().unwrap();
                cont=1;
            }
            else if tokens[i]=="true"{
                truem=tokens[i+1].parse::<i128>().unwrap();
                cont=1;
            }
            else if tokens[i]=="false"{
                falsem=tokens[i+1].parse::<i128>().unwrap();
                cont=1;
            }
    }
    monkeys.push(Monkey::new(&operator,&number, truem, falsem, test));
    for i in 0..vechelp.len(){
        monkeys[monkeynumb].items.push(vechelp[i]);
    }
    println!("{:?}",monkeys[monkeynumb]);
    monkeynumb+=1;
}
for _ in 0..10000{
    for i in 0..monkeynumb{
        let mut truerem=vec![];
        let mut falserem=vec![];
        for x in 0..monkeys[i].items.len(){
            monkeys[i].inspect+=1;
            let mut helpnumb=0;
            if monkeys[i].number=="old"{
                helpnumb=monkeys[i].items[x];
            }
            else{
                helpnumb=monkeys[i].number.parse::<i128>().unwrap();
            }
            match monkeys[i].operator.as_str(){
                "+"=>monkeys[i].items[x]+=helpnumb,
                "*"=>monkeys[i].items[x]*=helpnumb,
                _=>()
            }
            if monkeys[i].items[x]%monkeys[i].test==0{
                monkeys[i].items[x]=monkeys[i].items[x].rem_euclid(9699690);
                truerem.push(x);
            }
            else{
                monkeys[i].items[x]=monkeys[i].items[x].rem_euclid(9699690);
                falserem.push(x);
            }
        }
        for x in truerem{
            let index=monkeys[i].truem as usize;
            let valadd=monkeys[i].items[x];
            monkeys[index].items.push(valadd);
        }
        for x in falserem{
            let index=monkeys[i].falsem as usize;
            let valadd=monkeys[i].items[x];
            monkeys[index].items.push(valadd);
        }
        monkeys[i].items=vec![];
    }
}
for i in 0..monkeynumb{
    let help=monkeys[i].inspect;
    println!("{}",help);
}
}