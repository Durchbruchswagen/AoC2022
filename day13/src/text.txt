use std::{fs::File, io::{BufReader, BufRead}, vec};
fn comparefun(p1:Vec<String>,p2:Vec<String>)->bool{
    let mut tab1=p1.to_vec();
    let mut tab2=p2.to_vec();
    let mut max1=tab1.len();
    let mut max2=tab2.len();
    let mut i1=0;
    let mut i2=0;
    let mut depth1=0;
    let mut depth2=0;
    while i1<max1 && i2<max2{
        if tab1[i1]=="[" && tab2[i2]=="["{
            i1+=1;
            i2+=1;
            depth1+=1;
            depth2+=1;
        }
        else if tab1[i1].parse::<i32>().is_ok() && tab2[i2].parse::<i32>().is_ok(){
            if tab1[i1].parse::<i32>().unwrap()<tab2[i2].parse::<i32>().unwrap(){
                
                return true;
            }
            if tab1[i1].parse::<i32>().unwrap()>tab2[i2].parse::<i32>().unwrap(){
                break;
            }
            if tab1[i1].parse::<i32>().unwrap()==tab2[i2].parse::<i32>().unwrap(){
                i1+=1;
                i2+=1;
            }
        }
        else if tab1[i1]=="]" && tab2[i2]=="]"{
            i1+=1;
            i2+=1;
            depth1-=1;
            depth2-=1;
        }
        else if tab1[i1]=="[" && tab2[i2].parse::<i32>().is_ok(){
            tab2.insert(i2+1, "]".to_string());
            i1+=1;
            max2=tab2.len();
            depth2+=1;
        }
        else if tab2[i2]=="[" && tab1[i1].parse::<i32>().is_ok(){
            tab1.insert(i1+1, "]".to_string());
            max1=tab1.len();
            i2+=1;
            depth1+=1;
        }
        else if tab1[i1]=="]" && tab2[i2]!="]"{
            return true
        }
        else if tab2[i2]=="]" && tab1[i1]!="]"{
            break;
        }
    }
    if i1!=max1 && i2==max2{
        return true;
    }
    else if i1==max1 && i2==max2{
        return true;
    }
    return false
}
fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut p1="".to_string();
    let mut p2="".to_string();
    let mut ispair=0;
    let mut skip=0;
    let mut pairn=1;
    let mut wynik=0;
    let mut wynik1=1;
    let mut wynik2=2;
    let frstpacket=vec!["[".to_string(),"[".to_string(),"2".to_string(),"]".to_string(),"]".to_string()];
    let scndpacket=vec!["[".to_string(),"[".to_string(),"6".to_string(),"]".to_string(),"]".to_string()];
    let mut tab1=vec![];
    let mut tab2=vec![];
    let mut packets:Vec<Vec<String>>=vec![];
    for lines in reader.lines(){
        if skip==1{
            skip=0;
            pairn+=1;
            tab1=vec![];
            tab2=vec![];
            continue;
        }
        if ispair==0{
            ispair=1;
            match lines{
                Ok(a)=>p1=a.to_string(),
                _=>()
            }
            let mut numberhelp="".to_string();
            for x in p1.chars(){
                if x=='[' || x==']'{
                    if x==']' && numberhelp!="".to_string(){
                        tab1.push(numberhelp);
                        numberhelp="".to_string();
                    }
                    tab1.push(x.to_string());
                }
                else if x.is_digit(10){
                    numberhelp+=&x.to_string();
                }
                else if x==','{
                    if numberhelp!="".to_string(){
                        tab1.push(numberhelp);
                    }
                    numberhelp="".to_string();
                }
            }
            packets.push(tab1.to_vec());
        }
        else if ispair==1{
            ispair=0;
            skip=1;
            match lines{
                Ok(a)=>p2=a.to_string(),
                _=>()
            }
            let mut numberhelp="".to_string();
            for x in p2.chars(){
                if x=='[' || x==']'{
                    if x==']' && numberhelp!="".to_string(){
                        tab2.push(numberhelp);
                        numberhelp="".to_string();
                    }
                    tab2.push(x.to_string());
                }
                else if x.is_digit(10){
                    numberhelp+=&x.to_string();
                }
                else if x==','{
                    if numberhelp!="".to_string() {
                        tab2.push(numberhelp);
                    }
                    numberhelp="".to_string();
                }
            }
            packets.push(tab2.to_vec());
        }  
    }
    for x in packets{
        if comparefun(x.to_vec(),frstpacket.to_vec()){
            wynik1+=1;
        }
        if comparefun(x.to_vec(),scndpacket.to_vec()){
            wynik2+=1;
        } 
    }
    println!("DASDAS");
    println!("{},{},{}",wynik1,wynik2,wynik1*wynik2);
}
