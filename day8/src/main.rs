use std::{fs::File, io::{BufReader, BufRead}};
fn boardleft(board:&Vec<Vec<i32>>,x:usize,y:usize,tree:i32)->i32{
    let mut rtval=0;
    if x==0 {
        return 0;
    }
    let mut help=x;
    while help>0 {
        rtval+=1;
        if board[y][help-1]>=tree {
            break;
        }
        //println!("{}",help);
        help-=1;
    }
    rtval
}
fn boardright(board:&Vec<Vec<i32>>,x:usize,y:usize,xsize:usize,tree:i32)->i32{
    let mut rtval=0;
    if x==xsize-1 {
        return 0;
    }
    let mut help=x;
    while help<xsize-1 {
        rtval+=1;
        if board[y][help+1]>=tree {
            break;
        }
        help+=1;
    }
    rtval
}
fn boardtop(board:&Vec<Vec<i32>>,x:usize,y:usize,tree:i32)->i32{
    let mut rtval=0;
    if y==0 {
        return 0;
    }
    let mut help=y;
    while help>0 {
        rtval+=1;
        if board[help-1][x]>=tree {
            break;
        }
        help-=1;
    }
    rtval
}
fn boarddown(board:&Vec<Vec<i32>>,x:usize,y:usize,ysize:usize,tree:i32)->i32{
    let mut rtval=0;
    if y==ysize-1 {
        return 0;
    }
    let mut help=y;
    while help<ysize-1 {
        rtval+=1;
        if board[help+1][x]>=tree {
            break;
        }
        help+=1;
    }
    rtval
}
fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let xsize:usize=99;
    let ysize:usize=99;
    let mut points=0;
    let mut points2=0;
    let mut board:Vec<Vec<i32>>=vec![vec![0;xsize];ysize];
    let mut left:Vec<Vec<i32>>=vec![vec![0;xsize];ysize];
    let mut right:Vec<Vec<i32>>=vec![vec![0;xsize];ysize];
    let mut top:Vec<Vec<i32>>=vec![vec![0;xsize];ysize];
    let mut down:Vec<Vec<i32>>=vec![vec![0;xsize];ysize];
    let mut row=0;
    let mut col;
    for lines in reader.lines(){
        col=0;
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        for d in help.chars(){
            board[row][col]=d.to_digit(10).unwrap() as i32;
            left[row][col]=d.to_digit(10).unwrap() as i32;
            right[row][col]=d.to_digit(10).unwrap() as i32;
            top[row][col]=d.to_digit(10).unwrap() as i32;
            down[row][col]=d.to_digit(10).unwrap() as i32;
            col+=1;
        }
        row+=1;

    }
    let mut lefth;
    let mut righth;
    for y in 0..ysize{
        lefth=-1;
        righth=-1;
        for x in 0..xsize{
            left[y][x]=lefth;
            right[y][xsize-1-x]=righth;
            if board[y][x]>lefth{
                lefth=board[y][x];
            }
            if board[y][xsize-1-x]>righth{
                righth=board[y][xsize-1-x];
            }
        }
    }
    let mut toph;
    let mut dowh;
    for x in 0..xsize{
        toph=-1;
        dowh=-1;
        for y in 0..ysize{
            top[y][x]=toph;
            down[ysize-1-y][x]=dowh;
            if board[y][x]>toph{
                toph=board[y][x];
            }
            if board[ysize-1-y][x]>dowh{
                dowh=board[ysize-1-y][x];
            }
        }
    }
    for y in 0..ysize{
        for x in 0..xsize{
            let val=board[y][x];
            //println!("row:{},col:{},left:{},right{},top{},down{}",y,x,left[y][x],right[y][x],top[y][x],down[y][x]);
            if left[y][x]<val || right[y][x]<val 
            || top[y][x]<val || down[y][x]<val {
              //  println!("row:{},col:{}",y,x);
                points+=1;
            }
            let scenic=boardleft(&board, x, y, val)*boardtop(&board, x, y, val)
            *boarddown(&board, x, y, ysize, val)*boardright(&board, x, y, xsize, val);
            if scenic>points2{
                points2=scenic;
            }
        }
    }
    println!("{},{}",points,points2)
}
