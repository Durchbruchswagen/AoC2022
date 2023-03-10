use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
fn findmove(elfes:&HashSet<(i32,i32)>,x:i32,y:i32,off:i32)->(bool,i32,i32){
    let checkorder=off.rem_euclid(4);
    if !elfes.contains(&(x-1,y-1)) && !elfes.contains(&(x,y-1)) && !elfes.contains(&(x+1,y-1)) && !elfes.contains(&(x+1,y)) &&
    !elfes.contains(&(x+1,y+1)) && !elfes.contains(&(x,y+1)) && !elfes.contains(&(x-1,y+1)) && !elfes.contains(&(x-1,y)){
        return (false,0,0);
    }
    for i in 0..4{
        if (checkorder+i).rem_euclid(4)==0{
            if !elfes.contains(&(x-1,y-1)) && !elfes.contains(&(x,y-1)) && !elfes.contains(&(x+1,y-1)){
                return (true,x,y-1);
            }
        }

        else if (checkorder+i).rem_euclid(4)==1{
            if !elfes.contains(&(x-1,y+1)) && !elfes.contains(&(x,y+1)) && !elfes.contains(&(x+1,y+1)){
                return (true,x,y+1);
            }
        }

        else if (checkorder+i).rem_euclid(4)==2{
            if !elfes.contains(&(x-1,y+1)) && !elfes.contains(&(x-1,y)) && !elfes.contains(&(x-1,y-1)){
                return (true,x-1,y);
            }
        }

        else if (checkorder+i).rem_euclid(4)==3{
            if !elfes.contains(&(x+1,y+1)) && !elfes.contains(&(x+1,y)) && !elfes.contains(&(x+1,y-1)){
                return (true,x+1,y);
            }
        }
    }
    (false,0,0)
}




fn solve1(elfes:&mut HashSet<(i32,i32)>)->i32{
    for i in 0..10{
        let mut dontmove=HashSet::new();
        let mut newmoves=HashMap::new();
        let mut allmoves=HashSet::new();
        for (x,y) in elfes.iter(){
            let (check,nx,ny)=findmove(elfes, *x, *y, i);
            if !check {
                continue;
            }
            newmoves.insert((*x,*y), (nx,ny));
            if allmoves.contains(&(nx,ny)){
                dontmove.insert((nx,ny));
            }
            allmoves.insert((nx,ny));
        }
        let mut newset=vec![];
        for (x,y) in elfes.iter(){
            if newmoves.contains_key(&(*x,*y)){
                let (nx,ny)=*newmoves.get(&(*x,*y)).unwrap();
                if dontmove.contains(&(nx,ny)){
                    newset.push((*x,*y));
                }
                else{
                    newset.push((nx,ny));
                }
            }
            else{
                newset.push((*x,*y));
            }
        }
        elfes.clear();
        for (x,y) in newset{
            elfes.insert((x,y));
        }
    }
    let mut max_x=0;
    let mut min_x=0;
    let mut max_y=0;
    let mut min_y=0;
    let mut setvals=true;
    for (x,y) in elfes.iter(){
        if setvals{
            max_x=*x;
            min_x=*x;
            max_y=*y;
            min_y=*y;
            setvals=false;
        }
        else{
            max_x=cmp::max(max_x, *x);
            min_x=cmp::min(min_x, *x);
            max_y=cmp::max(max_y, *y);
            min_y=cmp::min(min_y,*y);
        }
    }
    println!("left:{},right:{},up:{},down:{},elfes:{}",min_x,max_x,max_y,min_y,elfes.len());
    //println!("{:?}",elfes);
    let rtval=(max_x-min_x+1)*(max_y-min_y+1)-elfes.len() as i32;
    rtval
}

fn solve2(elfes:&mut HashSet<(i32,i32)>)->i32{
    let mut rounds=0;
    loop{
        let mut dontmove=HashSet::new();
        let mut newmoves=HashMap::new();
        let mut allmoves=HashSet::new();
        for (x,y) in elfes.iter(){
            let (check,nx,ny)=findmove(elfes, *x, *y, rounds);
            if !check {
                continue;
            }
            newmoves.insert((*x,*y), (nx,ny));
            if allmoves.contains(&(nx,ny)){
                dontmove.insert((nx,ny));
            }
            allmoves.insert((nx,ny));
        }
        let mut moves=0;
        let mut newset=vec![];
        for (x,y) in elfes.iter(){
            if newmoves.contains_key(&(*x,*y)){
                let (nx,ny)=*newmoves.get(&(*x,*y)).unwrap();
                if dontmove.contains(&(nx,ny)){
                    newset.push((*x,*y));
                }
                else{
                    newset.push((nx,ny));
                    moves+=1;
                }
            }
            else{
                newset.push((*x,*y));
            }
        }
        elfes.clear();
        for (x,y) in newset{
            elfes.insert((x,y));
        }
        if moves==0{
            break;
        }
        rounds+=1;
    }

    rounds
}
fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut y=0;
    let mut elfes=HashSet::new();
    let mut elfnum=0;
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let mut x=0;
        for c in help.chars(){
            match c{
                '#'=>elfes.insert((x,y)),
                _=>false
            };
            if c=='#'{
                elfnum+=1;
            }
            x+=1;
        }
        y+=1;
    }
    println!("{}",elfnum);
    let wynik=solve2(&mut elfes);
    println!("{}",wynik);
}
