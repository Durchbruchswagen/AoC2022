use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::cmp;

fn main(){
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut xmin=-1;
    let mut xmax=-1;
    let mut ymax=0;
    for a in reader.lines(){
        let mut helps="".to_string();
        match a{
            Ok(line)=>helps=line.to_string(),
            _=>()
        }
        let help1:Vec<&str>=helps.split_whitespace().collect();
        for x in help1{
            let help2:Vec<&str>=x.split(",").collect();
            println!("{:?}",help2);
            let xcord:i32=help2[0].parse().unwrap();
            let ycord:i32=help2[1].parse().unwrap();
            if xcord<xmin || xmin==-1{
                xmin=xcord;
            }
            if xcord>xmax || xmax==-1{
                xmax=xcord;
            }
            if ycord>ymax{
                ymax=ycord;
            }
        }
    }
    
    println!("xmax:{},xmin:{},ymax:{}",xmax,xmin,ymax);
    let offset=(2*ymax)-1;
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut board=vec![vec!['.';(xmax-xmin+1+offset+offset) as usize];(ymax+3) as usize];

    for a in reader.lines(){
        let mut helps="".to_string();
        match a{
            Ok(line)=>helps=line.to_string(),
            _=>()
        }
        let help1:Vec<&str>=helps.split_whitespace().collect();
        let mut skip=1;
        let mut p1:Vec<&str>=vec![];
        let mut p2:Vec<&str>=vec![];
        for path in help1{
            if(skip==1){
                skip=0;
                p1=path.split(",").collect();
                continue;
            }
            p2=path.split(",").collect();
            let mut x1:i32=p1[0].parse().unwrap();
            let y1:i32=p1[1].parse().unwrap();
            let mut x2:i32=p2[0].parse().unwrap();
            let y2:i32=p2[1].parse().unwrap();
            x1-=xmin;
            x2-=xmin;
            x1+=offset;
            x2+=offset;
            if x1 != x2{
                let mut from=cmp::min(x1,x2);
                let mut to=cmp::max(x1,x2);
                while(from<=to){
                    board[y1 as usize][from as usize]='#';
                    from+=1;
                }
            }
            else if y1 != y2{
                let mut from=cmp::min(y1,y2);
                let mut to=cmp::max(y1,y2);
                while(from<=to){
                    board[from as usize][x1 as usize]='#';
                    from+=1;
                }
            }
            p1=p2;
        }
    }
    for i in 0..board[(ymax+2) as usize].len(){
        board[(ymax+2) as usize][i]='#';
    }
    let mut wynik=0;
    let mut endflag=0;
    loop {
        if(endflag>0){
            break;
        }
        let mut posx=500-xmin+offset;
        let mut posy=0;
        loop{
            // if(posx==0 || posx==(xmax-xmin+offset) || posy==ymax){
            //     endflag=1;
            //     break;
            // }
            if(posy<(ymax+2) && board[(posy+1) as usize][posx as usize]=='.'){
                posy+=1;
            }
            else if posy<(ymax+2) && posx>0 && board[(posy+1) as usize][(posx-1) as usize]=='.' {
                posy+=1;
                posx-=1;
            }
            else if posy<(ymax+2) && posx<(xmax-xmin+offset+offset) && board[(posy+1) as usize][(posx+1) as usize]=='.'{
                posy+=1;
                posx+=1;
            }
            else{
                board[posy as usize][posx as usize]='o';
                wynik+=1;
                if posy==0 && posx==500-xmin+offset{
                    endflag=1;
                }
                break;
            }
        }
    }
    // for x in board{
    //     println!("{:?}",x);
    // }
    println!("{}",wynik);
}