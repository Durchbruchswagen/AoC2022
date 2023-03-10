use std::{fs::File, io::{BufReader, BufRead}, vec};
fn flat(cycle:&mut Vec<i32>,board:&mut Vec<Vec<char>>,jets:&Vec<char>,peak:&mut i32,jettime:&mut i32)->(){
    let mut posx:usize=2;
    let mut newpeak=*peak;
    //println!("DIPA {}",*peak);
    let mut posy:usize=((*peak) + 4) as usize;
    let mut newjettime=*jettime;
    loop{
        if jets[newjettime.rem_euclid(jets.len() as i32) as usize]=='<' {
            if posx>0 && board[posy as usize][posx-1 as usize]=='.'{
                posx-=1;
            }
        }
        else{
            if posx+4<7 && board[posy][posx+4]=='.'{
                posx+=1;
            }
        }
        newjettime+=1;
        if posy>0 && board[posy-1][posx]=='.' && board[posy-1][posx+1]=='.' &&
        board[posy-1][posx+2]=='.' && board[posy-1][posx+3]=='.'{
            posy-=1;
        }
        else{
            board[posy][posx]='#';
            board[posy][posx+1]='#';
            board[posy][posx+2]='#';
            board[posy][posx+3]='#';
            if newpeak<posy as i32{
                newpeak=posy as i32;
            }
            break;
        }
    }
    *jettime=newjettime;
    cycle.push(newpeak-*peak);
    *peak=newpeak;
}

fn cross(cycle:&mut Vec<i32>,board:&mut Vec<Vec<char>>,jets:&Vec<char>,peak:&mut i32,jettime:&mut i32)->(){
    let mut posx:usize=3;
    let mut newpeak=*peak;
    let mut posy:usize=*peak as usize +5;
    let mut newjettime=*jettime;
    loop{
        if jets[newjettime.rem_euclid(jets.len() as i32) as usize]=='<' {
            if posx>1 && board[posy as usize][posx-2 as usize]=='.' && board[posy+1 as usize][posx-1 as usize]=='.' && 
            board[posy-1 as usize][posx-1 as usize]=='.'{
                posx-=1;
            }
        }
        else{
            if posx+2<7 && board[posy as usize][posx+2 as usize]=='.' && board[posy+1 as usize][posx-1 as usize]=='.' && 
            board[posy-1 as usize][posx+1 as usize]=='.'{
                posx+=1;
            }
        }
        newjettime+=1;
        if posy>1 && board[posy-2][posx]=='.' && board[posy-1][posx+1]=='.' &&
        board[posy-1][posx-1]=='.'{
            posy-=1;
        }
        else{
            board[posy][posx]='#';
            board[posy][posx+1]='#';
            board[posy][posx-1]='#';
            board[posy+1][posx]='#';
            board[posy-1][posx]='#';
            if newpeak<posy as i32+1{
                newpeak=posy as i32+1;
            }
            break;
        }
    }
    *jettime=newjettime;
    cycle.push(newpeak-*peak);
    *peak=newpeak;
}

fn l(cycle:&mut Vec<i32>,board:&mut Vec<Vec<char>>,jets:&Vec<char>,peak:&mut i32,jettime:&mut i32)->(){
    let mut posx:usize=2;
    let mut newpeak=*peak;
    let mut posy:usize=*peak as usize +4;
    let mut newjettime=*jettime;
    loop{
        if jets[newjettime.rem_euclid(jets.len() as i32) as usize]=='<' {
            if posx>0 && board[posy as usize][posx-1 as usize]=='.' && board[posy+1 as usize][posx+1 as usize]=='.' && 
            board[posy+2 as usize][posx+1 as usize]=='.'{
                posx-=1;
            }
        }
        else{
            if posx+3<7 && board[posy as usize][posx+3 as usize]=='.' && board[posy+1 as usize][posx+3 as usize]=='.' && 
            board[posy+2 as usize][posx+3 as usize]=='.'{
                posx+=1;
            }
        }
        newjettime+=1;
        if posy>0 && board[posy-1][posx]=='.' && board[posy-1][posx+1]=='.' &&
        board[posy-1][posx+2]=='.'{
            posy-=1;
        }
        else{
            board[posy][posx]='#';
            board[posy][posx+1]='#';
            board[posy][posx+2]='#';
            board[posy+1][posx+2]='#';
            board[posy+2][posx+2]='#';
            if newpeak<posy as i32+2{
                newpeak=posy as i32+2;
            }
            break;
        }
    }
    *jettime=newjettime;
    cycle.push(newpeak-*peak);
    *peak=newpeak;
}

fn tall(cycle:&mut Vec<i32>,board:&mut Vec<Vec<char>>,jets:&Vec<char>,peak:&mut i32,jettime:&mut i32)->(){
    let mut posx:usize=2;
    let mut newpeak=*peak;
    let mut posy:usize=*peak as usize +4;
    let mut newjettime=*jettime;
    loop{
        if jets[newjettime.rem_euclid(jets.len() as i32) as usize]=='<' {
            if posx>0 && board[posy as usize][posx-1 as usize]=='.' && board[posy+1 as usize][posx-1 as usize]=='.' && 
            board[posy+2 as usize][posx-1 as usize]=='.' && board[posy+3 as usize][posx-1 as usize]=='.'{
                posx-=1;
            }
        }
        else{
            if posx+1<7 && board[posy as usize][posx+1 as usize]=='.' && board[posy+1 as usize][posx+1 as usize]=='.' && 
            board[posy+2 as usize][posx+1 as usize]=='.' && board[posy+3 as usize][posx+1 as usize]=='.'{
                posx+=1;
            }
        }
        newjettime+=1;
        if posy>0 && board[posy-1][posx]=='.'{
            posy-=1;
        }
        else{
            board[posy][posx]='#';
            board[posy+1][posx]='#';
            board[posy+2][posx]='#';
            board[posy+3][posx]='#';
            if newpeak<posy as i32 +3{
                newpeak=posy as i32 +3;
            }
            break;
        }
    }
    *jettime=newjettime;
    cycle.push(newpeak-*peak);
    *peak=newpeak;
}

fn square(cycle:&mut Vec<i32>,board:&mut Vec<Vec<char>>,jets:&Vec<char>,peak:&mut i32,jettime:&mut i32)->(){
    let mut posx:usize=2;
    let mut newpeak=*peak;
    let mut posy:usize=*peak as usize +4;
    let mut newjettime=*jettime;
    loop{
        if jets[newjettime.rem_euclid(jets.len() as i32) as usize]=='<' {
            if posx>0 && board[posy as usize][posx-1 as usize]=='.' && board[posy+1 as usize][posx-1 as usize]=='.'{
                posx-=1;
            }
        }
        else{
            if posx+2<7 && board[posy as usize][posx+2 as usize]=='.' && board[posy+1 as usize][posx+2 as usize]=='.'{
                posx+=1;
            }
        }
        newjettime+=1;
        if posy>0 && board[posy-1][posx]=='.' && board[posy-1][posx+1]=='.'{
            posy-=1;
        }
        else{
            board[posy][posx]='#';
            board[posy][posx+1]='#';
            board[posy+1][posx]='#';
            board[posy+1][posx+1]='#';
            if newpeak<posy as i32 +1{
                newpeak=posy as i32 +1;
            }
            break;
        }
    }
    *jettime=newjettime;
    cycle.push(newpeak-*peak);
    *peak=newpeak;

}

fn main() {
    //5265
    let mut board:Vec<Vec<char>>=vec![vec!['.';7];5265];
    let mut jets:Vec<char>=vec![];
    let mut cycle:Vec<i32>=vec![];
    let mut peak=-1;
    let mut jettime=0;
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        jets=help.chars().collect()
    }
    let mut rocks:i32=0;
    while(rocks<2022){
        match rocks.rem_euclid(5){
            0=>flat(&mut cycle,&mut board, &jets, &mut peak, &mut jettime),
            1=>cross(&mut cycle,&mut board, &jets, &mut peak, &mut jettime),
            2=>l(&mut cycle,&mut board, &jets, &mut peak, &mut jettime),
            3=>tall(&mut cycle,&mut board, &jets, &mut peak, &mut jettime),
            4=>square(&mut cycle,&mut board, &jets, &mut peak, &mut jettime),
            _=>()
        }
        //println!("{}",peak);
        rocks+=1;
    }
    println!("{},qt:{}",rocks,peak);
    let mut rocks2:i64=0;
    let mut peak2:i128=0;
    let file = File::open("cycle.txt").unwrap();
    let reader = BufReader::new(file);
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        for x in help.split_whitespace(){
            peak2+=x.parse::<i128>().unwrap();
            rocks2+=1;
        }
    }
    let file = File::open("cycle1.txt").unwrap();
    let reader = BufReader::new(file);
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        cycle=help.split_whitespace().map(|x| x.parse().unwrap()).collect();
    }
    println!("rocks:{},peaks:{}",rocks2,peak2);
    let mut maks:i128=0;
    for x in &cycle{
        maks+=*x as i128;
    }
    let mut maksrock:i128=1000000000000;
    maksrock-=rocks2 as i128;
    println!("maksrock:{},maks:{},full:{}",maksrock,maks,(maksrock/(cycle.len() as i128)));
    peak2+=maks*(maksrock/(cycle.len() as i128));
    maksrock=maksrock%(cycle.len() as i128);
    for i in 0..maksrock{
        peak2+=cycle[i as usize] as i128;
    }
    println!("rocks:{},peaks:{}",rocks2,peak2);
}