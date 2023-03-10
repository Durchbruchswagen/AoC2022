use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::collections::HashSet;

fn floodfill(cubes: &HashSet<(i32,i32,i32)>,startx:i32,starty:i32,startz:i32)->bool{
    let mut stack=vec![];
    let mut visited=HashSet::new();
    let mut x=startx;
    let mut y=starty;
    let mut z=startz;
    stack.push((x,y,z));
    while !stack.is_empty(){
        let pair=stack.pop().unwrap();
        x=pair.0;
        y=pair.1;
        z=pair.2;
        if x>=20 || y>=20 || z>=20 || x<0 || y<0 || z<0{
            //println!("IN VOLUME:{},{},{}",pair.0,pair.1,pair.2);
            return false;
        }
        visited.insert(pair);
        if !cubes.contains(&(x,y,z+1)) && !visited.contains(&(x,y,z+1)){
            stack.push((x,y,z+1));
        }
        if !cubes.contains(&(x,y,z-1)) && !visited.contains(&(x,y,z-1)){
            stack.push((x,y,z-1));
        }
        if !cubes.contains(&(x,y-1,z)) && !visited.contains(&(x,y-1,z)){
            stack.push((x,y-1,z));
        }
        if !cubes.contains(&(x,y+1,z)) && !visited.contains(&(x,y+1,z)){
            stack.push((x,y+1,z));
        }
        if !cubes.contains(&(x-1,y,z)) && !visited.contains(&(x-1,y,z)){
            stack.push((x-1,y,z));
        }
        if !cubes.contains(&(x+1,y,z)) && !visited.contains(&(x+1,y,z)){
            stack.push((x+1,y,z));
        }
        

    }
   // println!("{},{},{}",startx,starty,startz);
    true
}


fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut cubes:HashSet<(i32,i32,i32)>=HashSet::new();
    let mut volume:HashSet<(i32,i32,i32)>=HashSet::new();
    let mut sides=0;
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let help2:Vec<&str>=help.split([',']).collect();
        let x:i32=help2[0].parse().unwrap();
        let y:i32=help2[1].parse().unwrap();
        let z:i32=help2[2].parse().unwrap();
        cubes.insert((x,y,z));
        volume.insert((x-1,y,z));
        volume.insert((x+1,y,z));
        volume.insert((x,y-1,z));
        volume.insert((x,y+1,z));
        volume.insert((x,y,z-1));
        volume.insert((x,y,z+1));
        sides+=6;
        if cubes.contains(&(x,y,z-1)) {
            sides-=2;
        }
        if cubes.contains(&(x,y,z+1)) {
            sides-=2;
        }
        if cubes.contains(&(x,y+1,z)) {
            sides-=2;
        }
        if cubes.contains(&(x,y-1,z)) {
            sides-=2;
        }
        if cubes.contains(&(x-1,y,z)) {
            sides-=2;
        }
        if cubes.contains(&(x+1,y,z)) {
            sides-=2;
        }
    }
    println!("{}",sides);
    for (x,y,z) in &cubes{
        if volume.contains(&(*x,*y,*z)){
            volume.remove(&(*x,*y,*z));
        }
    }
    if(!cubes.contains(&(2,2,2)) && floodfill(&cubes, 2, 2, 2)){
        println!("DUPA");
    }
    for (x,y,z) in &cubes{
        println!("{},{},{}",*x,*y,*z);
        if !cubes.contains(&(2,2,2)) && floodfill(&cubes, 2, 2, 2){
            println!("WYJEBALO SIE");
        }
        if !cubes.contains(&(*x,*y,*z-1)) && floodfill(&cubes,*x, *y, *z-1){
            println!("DUPA1: {},{},{}",*x,*y,*z-1);
            sides-=1;
        }
        if !cubes.contains(&(*x,*y,*z+1)) && floodfill(&cubes, *x, *y, *z+1){
            println!("DUPA2: {},{},{}",*x,*y,*z+1);
            sides-=1;
        }
        if !cubes.contains(&(*x,*y+1,*z)) && floodfill(&cubes, *x, *y+1, *z){
            println!("DUPA3: {},{},{}",*x,*y-1,*z);
            sides-=1;
        }
        if !cubes.contains(&(*x,*y-1,*z)) && floodfill(&cubes, *x, *y-1, *z){
            println!("DUPA4: {},{},{}",*x,*y+1,*z);
            sides-=1;
        }
        if !cubes.contains(&(*x-1,*y,*z)) && floodfill(&cubes, *x-1, *y, *z){
            println!("DUPA5: {},{},{}",*x+1,*y,*z);
            sides-=1;
        }
        if !cubes.contains(&(*x+1,*y,*z)) && floodfill(&cubes, *x+1, *y, *z){
            println!("DUPA6: {},{},{}",*x-1,*y,*z);
            sides-=1;
        }
    }
    
    println!("{}",sides);
}
