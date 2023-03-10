use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::collections::BinaryHeap;


fn iswall(x:i32,y:i32,board:&Vec<Vec<char>>)->bool{
    if y<0 || y>=(board.len() as i32) || x<0 || x>=(board[0].len() as i32){
        return true;
    } 
    board[y as usize][x as usize]=='#'
}

fn isafe(x:i32,y:i32,board:&Vec<Vec<char>>)->bool{
    !(board[y as usize][x as usize]=='>' || board[y as usize][x as usize]=='<' || board[y as usize][x as usize]=='^' || board[y as usize][x as usize]=='v')
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State{
    x:i32,
    y:i32,
    cycle:i32,
    currtime:i32,
}
impl Ord for State {
    fn cmp(&self, other:&Self)->Ordering{
        other.currtime.cmp(&self.currtime)
        .then_with(|| self.x.cmp(&other.x))
        .then_with(|| self.y.cmp(&other.y))
        .then_with(|| self.cycle.cmp(&other.cycle))
    }
}

impl PartialOrd for State{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortestpath(startx:i32,starty:i32,finishx:i32,finishy:i32,cyclesnum:i32,cycles:&Vec<Vec<Vec<char>>>,starttime:i32)->i32{
    let mut besttimes=HashMap::new();
    let mut queue=BinaryHeap::new();
    besttimes.insert((startx,starty,starttime%cyclesnum), starttime);
    queue.push(State{
        x:startx,
        y:starty,
        cycle:starttime%cyclesnum,
        currtime:starttime,
    });
    while let Some(State { x:nx, y:ny, cycle:nc, currtime:nt })=queue.pop(){
        if nt>besttimes[&(nx,ny,nc)]{
            continue;
        }
        //println!("x:{},y:{}",nx,ny);
        for (ax,ay) in [(0,1),(0,-1),(1,0),(-1,0)]{
            let bx=nx+ax;
            let by=ny+ay;
            if (bx,by)==(finishx,finishy){
                let endtime=nt+1;
                return endtime;
            }
            if iswall(bx, by, &cycles[nc as usize]){
               // println!("WALL");
                continue;
            }
            //println!("NEXT STEP X:{},Y:{}",bx,bx);
            for bt in nt+1..nt+cyclesnum+1{
                //println!("x:{},y:{}",bx,by);
                if isafe(bx, by, &cycles[(bt%cyclesnum) as usize]){
                   // println!("SAFE FOR:{}",bt%cyclesnum);
                    if bt < *besttimes.get(&(bx,by,bt%cyclesnum)).unwrap_or(&i32::MAX){
                        besttimes.insert((bx,by,bt%cyclesnum), bt);
                        queue.push(State{
                            x:bx,
                            y:by,
                            cycle:bt%cyclesnum,
                            currtime:bt,
                        });
                    }
                    break;
                }
                if !isafe(nx, ny, &cycles[(bt%cyclesnum) as usize]){
                    break;
                }
            }
        }
    }
    -1
}


fn nextphase(board:&Vec<Vec<char>>,blizzard:&HashSet<(usize,usize,i32)>)->(Vec<Vec<char>>,HashSet<(usize,usize,i32)>){
    let mut newblizzard=HashSet::new();
    let mut newboard=vec![vec!['.';board[0].len()];board.len()];
    let height=board.len();
    let width=board[0].len();
    for (x,y,dir) in blizzard{
        let mut nx=0;
        let mut ny=0;
        match dir{
            0=>(nx,ny)=(*x+1,*y),
            1=>(nx,ny)=(*x,*y+1),
            2=>(nx,ny)=(*x-1,*y),
            3=>(nx,ny)=(*x,*y-1),
            _=>()
        }
        if nx==width-1{
            newblizzard.insert((1,ny,*dir));
        }
        else if nx==0{
            newblizzard.insert((width-2,ny,*dir));
        }
        else if ny==height-1{
            newblizzard.insert((nx,1,*dir));
        }
        else if ny==0{
            newblizzard.insert((nx,height-2,*dir));
        }
        else{
            newblizzard.insert((nx,ny,*dir));
        }
    }
    for y in 0..board.len(){
        for x in 0..board[0].len(){
            if board[y][x]=='#'{
                newboard[y][x]='#';
            }
            else{
                if newblizzard.contains(&(x,y,0)){
                    newboard[y][x]='>';
                }
                else if newblizzard.contains(&(x,y,1)){
                    newboard[y][x]='v';
                }
                else if newblizzard.contains(&(x,y,2)){
                    newboard[y][x]='<';
                }
                else if newblizzard.contains(&(x,y,3)){
                    newboard[y][x]='^';
                }
                else{
                    newboard[y][x]='.';
                }
            }
        }
    }
    (newboard,newblizzard)
}

fn cachecycle(board:&Vec<Vec<char>>,blizzard:&HashSet<(usize,usize,i32)>,cyclesnum:i32)->Vec<Vec<Vec<char>>>{
    let mut cycles=Vec::new();
    let mut newboard=board.to_vec();
    let mut newset=blizzard.clone();
    cycles.push(board.to_vec());
    for _ in 1..cyclesnum{
        let help=nextphase(&newboard, &newset);
        newboard=help.0;
        newset=help.1;
        cycles.push(newboard.to_vec());
    }
    cycles
}

fn lcm(first: i32, second: i32) -> i32 {
    first * second / gcd(first, second)
}

fn gcd(first: i32, second: i32) -> i32 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut width=0;
    let mut height=0;
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        width=help.len();
        height+=1;
    }
    println!("{},{}",width,height);
    let mut board=vec![vec!['.';width];height];
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut y=0;
    let mut blizzard=HashSet::new();
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let mut x=0;
        for c in help.chars(){
            match c {
                '>'=>blizzard.insert((x,y,0)),
                'v'=>blizzard.insert((x,y,1)),
                '<'=>blizzard.insert((x,y,2)),
                '^'=>blizzard.insert((x,y,3)),
                _=>false
            };
            board[y][x]=c;
            x+=1;
        }
        y+=1;
    }
    let cyclelen=lcm(width as i32 -2,height as i32 -2);
    let cycles=cachecycle(&board, &blizzard, cyclelen);
    // for board in &cycles{
    //     println!("NEW BOARD");
    //     for row in board{
    //         println!("{:?}",row);
    //     }
    // }
    println!("{},{}",board[0][1],board[height-1][width-2]);
    let wynik1=shortestpath(1, 0, (width-2) as i32, (height-1) as i32, cyclelen, &cycles, 0);
    println!("{}",wynik1);
    let wynik2=shortestpath((width-2) as i32, (height-1) as i32,1,0, cyclelen, &cycles, wynik1);
    println!("{}",wynik2);
    let wynik3=shortestpath(1, 0, (width-2) as i32, (height-1) as i32, cyclelen, &cycles, wynik2);
    println!("{}",wynik3);

}
