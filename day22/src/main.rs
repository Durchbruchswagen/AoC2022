use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::cmp;
use std::collections::HashMap;
fn ext(x:i32,y:i32,board:&Vec<Vec<char>>)->bool{
    if x<0 || y<0 || x as usize>=board[0 as usize].len() || y as usize>=board.len(){
        return true;
    }
    if board[y as usize][x as usize]==' '{
        return true;
    }
    false
}
fn ins(x:i32,y:i32,board:&Vec<Vec<char>>)->bool{
    !ext(x, y, board)
}
fn movedir(dir:i32,x:i32,y:i32)->(i32,i32){
    match dir{
        0=>return (x+1,y),
        1=>return (x,y+1),
        2=>return (x-1,y),
        3=>return (x,y-1),
        _=>return (0,0)
    }
}
fn inner_corner(x:i32,y:i32,board:&Vec<Vec<char>>)->(bool,Vec<(i32,i32)>){
    let mut dir=vec![];
    if ins(x+1,y,board) && ins(x, y+1, board) && ext(x+1, y+1, board){
        dir.push((0,1));
    }
    if ins(x+1,y,board) && ins(x, y-1, board) && ext(x+1, y-1, board){
        dir.push((0,3));
    }
    if ins(x-1,y,board) && ins(x, y-1, board) && ext(x-1, y-1, board){
        dir.push((2,3));
    }
    if ins(x-1,y,board) && ins(x, y+1, board) && ext(x-1, y+1, board){
        dir.push((2,1));
    }
    if dir.is_empty(){
        return (false,vec![]);
    }
    else{
        return (true,dir);
    }

}

fn movezip(x:i32,y:i32,dir:i32,board:&Vec<Vec<char>>)->(i32,i32,i32){
    let (nextx,nexty)=movedir(dir, x, y);
    let left=(dir+1).rem_euclid(4);
    let right=(dir-1).rem_euclid(4);
    let (nextxl,nextyl)=movedir(left, x, y);
    let (nextxr,nextyr)=movedir(right, x, y);
    if ext(nextx,nexty,board){
        if ins(nextxl,nextyl,board){
            return (x,y,left);
        }
        if ins(nextxr,nextyr,board){
            return (x,y,right);
        }
    }
    return (nextx,nexty,dir);
}

fn zipcube(x:i32,y:i32,dir0:i32,dir1:i32,board:&Vec<Vec<char>>,mapping:&mut HashMap<(i32,i32,i32),(i32,i32,i32)>){
    //println!("START:x{},y{}",x,y);
    let mut dire0=dir0;
    let mut dire1=dir1;
    let mut prevdir0=dir0;
    let mut prevdir1=dir1;
    let (mut x0,mut y0)=movedir(dire0, x, y);
    let (mut x1, mut y1)=movedir(dire1, x, y);
    while dire0==prevdir0 || dire1==prevdir1{
       // println!("x0:{},y0:{},x1:{},y1:{}",x0,y0,x1,y1);
        prevdir0=dire0;
        prevdir1=dire1;
        let mut dir_outer0=(dire0+1).rem_euclid(4);
        if ins(movedir(dir_outer0, x0, y0).0,movedir(dir_outer0, x0, y0).1,board){
            dir_outer0=(dire0-1).rem_euclid(4);
        }
        let mut dir_outer1=(dire1+1).rem_euclid(4);
        if ins(movedir(dir_outer1, x1, y1).0,movedir(dir_outer1, x1, y1).1,board){
            dir_outer1=(dire1-1).rem_euclid(4);
        }
        let dir_norm0=(dir_outer0+2).rem_euclid(4);
        let dir_norm1=(dir_outer1+2).rem_euclid(4);
        //println!("INSERTING:{},{},{}",x0,y0,dir_outer0);
        mapping.insert((x0,y0,dir_outer0), (x1,y1,dir_norm1));
        //println!("INSERTING:{},{},{}",x1,y1,dir_outer1);
        mapping.insert((x1,y1,dir_outer1), (x0,y0,dir_norm0));
        (x0,y0,dire0)=movezip(x0, y0, dire0, board);
        (x1,y1,dire1)=movezip(x1, y1, dire1, board);
        let (mut check0,_)=inner_corner(x0, y0, board);
        let (mut check1,_)=inner_corner(x1, y1, board);
        if check0 || check1{
            //println!("ENDED BY BREAK");
            break;
        }
    }
    //println!("ENDED BY ROUNDING");
}

fn creatcube(board:&Vec<Vec<char>>)->HashMap<(i32,i32,i32),(i32,i32,i32)>{
    let mut mapping=HashMap::new();
    for y in 0..board.len(){
        for x in 0..board[0].len(){
            let cx=x as i32;
            let cy=y as i32;
            let (check,dirs)=inner_corner(cx, cy, board);
            if check{
               // println!("{},{},{:?}",cx,cy,dirs);
                for (dir0,dir1) in dirs{
                    zipcube(cx, cy, dir0, dir1, board, &mut mapping);
                }
            }
        }
    }
    mapping
}
fn main() {
    let mut upperbound = HashMap::new();
    let mut lowerbound = HashMap::new();
    let mut leftbound=HashMap::new();
    let mut rightbound=HashMap::new();
    let file = File::open("data1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut cols=0;
    let mut rows=0;
    for lines in reader.lines(){
        let mut help="".to_string();
        rows+=1;
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let help2:Vec<char>=help.chars().collect();
        cols=cmp::max(cols, help2.len() as i32);
    }
    println!("{},{}",cols,rows);
    let mut board=vec![vec![' ';cols as usize];rows as usize];
    let file = File::open("data1.txt").unwrap();
    let reader = BufReader::new(file);
    let maxcol=cols;
    let maxrow=rows;
    let mut start=0;
    let mut row=0;
    let mut findstart=true;
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let help2:Vec<char>=help.chars().collect();
        let mut ind=0;
        for x in help2{
            if findstart && x=='.'{
                start=ind;
                findstart=false;
            }
            if x!=' '{
                if upperbound.contains_key(&ind){
                    let help=*upperbound.get(&ind).unwrap();
                    upperbound.insert(ind, cmp::min(help,row));
                }
                else{
                    upperbound.insert(ind, row);
                }

                if lowerbound.contains_key(&ind){
                    let help=*lowerbound.get(&ind).unwrap();
                    lowerbound.insert(ind, cmp::max(help,row));
                }
                else{
                    lowerbound.insert(ind, row);
                }

                if leftbound.contains_key(&row){
                    let help=*leftbound.get(&row).unwrap();
                    leftbound.insert(row, cmp::min(help,ind));
                }
                else{
                    leftbound.insert(row, ind);
                }

                if rightbound.contains_key(&row){
                    let help=*rightbound.get(&row).unwrap();
                    rightbound.insert(row, cmp::max(help,ind));
                }
                else{
                    rightbound.insert(row, ind);
                }
            }
            board[row][ind]=x;
            ind+=1;
        }
        row+=1;
    }
    // for x in &board{
    //     println!("{:?}",*x);
    // }
    //println!("START:{}",start);
    let file = File::open("data2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut tokens=Vec::new();
    let mut num="".to_string();
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let help2:Vec<char>=help.chars().collect();
        for x in help2{
            if x.is_digit(10){
                num+=&x.to_string();
            }
            else{
                if num!=""{
                    tokens.push(num.to_string());
                    num="".to_string()
                }
                tokens.push(x.to_string());
            }
        }
    }
    if num!="" {
        tokens.push(num);
    }
    //println!("{:?}",tokens);
    // println!("UPPER:{:?}",upperbound);
    // println!("LOWER:{:?}",lowerbound);
    // println!("LEFT:{:?}",leftbound);
    // println!("RIGHT:{:?}",rightbound);
    let mut dir:i32=0;
    let mut currx=start;
    let mut currow=0;
    for x in &tokens{
        if x=="R"{
            dir=(dir+1).rem_euclid(4);
        }
        else if x=="L"{
            dir=(dir-1).rem_euclid(4);
        }
        else{
            let help=x.parse::<i32>().unwrap();
            for _ in 0..help{
                // match dir{
                //     0=>board[currow][currx]='>',
                //     1=>board[currow][currx]='v',
                //     2=>board[currow][currx]='<',
                //     3=>board[currow][currx]='>',
                //     _=>()
                // }
                if dir==0{
                    if currx+1==maxcol as usize || board[currow][currx+1]==' '{
                        let newx=*leftbound.get(&currow).unwrap();
                        if board[currow][newx]=='#'{
                            break;
                        }
                        currx=newx;
                    }
                    else{
                        if board[currow][currx+1]=='#'{
                            break;
                        }
                        currx+=1;
                    }
                }
                if dir==1{
                    if currow+1== maxrow as usize || board[currow+1][currx]==' '{
                        let newy=*upperbound.get(&currx).unwrap();
                        if board[newy][currx]=='#'{
                            break;
                        }
                        currow=newy;
                    }
                    else{
                        if board[currow+1][currx]=='#'{
                            break;
                        }
                        currow+=1;
                    }
                }
                if dir==2{
                    if currx==0 as usize || board[currow][currx-1]==' '{
                        let newx=*rightbound.get(&currow).unwrap();
                        if board[currow][newx]=='#'{
                            break;
                        }
                        currx=newx;
                    }
                    else{
                        if board[currow][currx-1]=='#'{
                            break;
                        }
                        currx-=1;
                    }
                }
                if dir==3{
                    if currow== 0 as usize || board[currow-1][currx]==' '{
                        let newy=*lowerbound.get(&currx).unwrap();
                        if board[newy][currx]=='#'{
                            break;
                        }
                        currow=newy;
                    }
                    else{
                        if board[currow-1][currx]=='#'{
                            break;
                        }
                        currow-=1;
                    }
                }
            }
        }
    }
    currx+=1;
    currow+=1;
    println!("x:{},y:{},dir:{},wynik:{}",currx,currow,dir,1000*currow as i32+4*currx as i32+dir);
    // for x in &board{
    //     println!("{}",x.into_iter().map(|x| x.to_string()).collect::<String>());
    // }
    // PART 2
    let mut test=board.to_vec();
    let mapping=creatcube(&board);
    for y in 0..test.len(){
        for x in 0..test[0].len(){
            let cx=x as i32;
            let cy=y as i32;
            if mapping.contains_key(&(cx,cy,0)) || mapping.contains_key(&(cx,cy,1))  || mapping.contains_key(&(cx,cy,2)) || mapping.contains_key(&(cx,cy,3)){
                test[y][x]='M';
            } 
        }
    }
    // for x in &test{
    //     println!("{}",x.into_iter().map(|x| x.to_string()).collect::<String>());
    // }
    println!("{}",start);
    let mut x=start as i32;
    let mut y=0;
    let mut dir:i32=0;
    for t in &tokens{
        if t=="L"{
            dir=(dir-1).rem_euclid(4);
        }
        else if t=="R"{
            dir=(dir+1).rem_euclid(4);
        }
        else{
            let number=t.parse::<i32>().unwrap();
            for _ in 0..number{
                let (mut nextx,mut nexty)=movedir(dir, x, y);
                let mut newdir=dir;
                if ext(nextx,nexty,&board){
                    //println!("CHECKING FOR:{},{},{}",x,y,dir);
                    (nextx,nexty,newdir)=*mapping.get(&(x,y,dir)).unwrap();
                    if board[nexty as usize][nextx as usize]=='#' {
                        break;
                    }
                    else{
                        x=nextx;
                        y=nexty;
                        dir=newdir;
                    }
                }
                else{
                    if board[nexty as usize][nextx as usize]=='#'{
                        break;
                    }
                    else{
                        x=nextx;
                        y=nexty;
                        dir=newdir;
                    } 
                }
            }
        }
    }
    x+=1;
    y+=1;
    println!("x:{},y:{},dir:{},wynik:{}",x,y,dir,1000*y as i32+4*x as i32+dir);
}
