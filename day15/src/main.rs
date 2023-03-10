use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
fn ifintersect(px:i128,py:i128,sx:i128,sy:i128,cabdist:i128)->bool{
    let dist=(py-sy).abs()+(px-sx).abs();
    return dist<=cabdist;
}
fn main(){
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut rowten:HashSet<i128> = HashSet::new();
    let mut scaners = HashMap::new();
    let mut beacons=HashSet::new();
    let sourcerow=10;
    let maxrow=4000000;
    for a in reader.lines(){
        let mut helps="".to_string();
        match a{
            Ok(line)=>helps=line.to_string(),
            _=>()
        }
        let help1:Vec<&str>=helps.split([' ','=',':',',']).collect();
        let sx:i128=help1[3].parse().unwrap();
        let sy:i128=help1[6].parse().unwrap();
        let bx:i128=help1[13].parse().unwrap();
        let by:i128=help1[16].parse().unwrap();
        if by==sourcerow {
            beacons.insert(bx);
        }
        let cabdist=(by-sy).abs()+(bx-sx).abs();
        scaners.insert((sx,sy), cabdist);
        println!("{:?}",helps);
        println!("{}",cabdist);
        // if((sy-sourcerow).abs()<=cabdist){
        //     rowten.insert(sx);
        //     for i in 0..(cabdist-(sy-sourcerow).abs()){
        //         rowten.insert(sx+i+1);
        //         rowten.insert(sx-i-1);
        //     }
        // }
    }
    let mut wynik=0;
    // for x in rowten{
    //     if(!beacons.contains(&x)){
    //         //println!("{}",x);
    //         wynik+=1;
    //     }
    // }
    let mut wx=0;
    let mut wy=0;
    let mut breakflag=0;
    for ((ssx,ssy),cabdist) in &scaners{
        let mut px=*ssx;
        let mut py=ssy+cabdist+1;
        for _ in 0..(cabdist+1){
            if(px<0 || px>maxrow || py<0 || py>maxrow){
                py-=1;
                px+=1;
                continue;
            }
            let mut okflag=1;
            for ((sx,sy),cabdist) in &scaners{
                if(*ssx==*sx && *ssy==*sy){
                    continue;
                }
                if ifintersect(px, py, *sx, *sy, *cabdist){
                    okflag=0;
                    break;
                }
            }
            if okflag==1 {
                wx=px;
                wy=py;
                breakflag=1;
                break;
            }
            py-=1;
            px+=1;
        }
    px=*ssx+cabdist+1;
    py=*ssy;
    if breakflag==1{
        break;
    }
    for _ in 0..(cabdist+1){
        if(px<0 || px>maxrow || py<0 || py>maxrow){
            py-=1;
            px-=1;
            continue;
        }
        let mut okflag=1;
        for ((sx,sy),cabdist) in &scaners{
            if(*ssx==*sx && *ssy==*sy){
                continue;
            }
            if ifintersect(px, py, *sx, *sy, *cabdist){
                okflag=0;
                break;
            }
        }
        if okflag==1 {
            wx=px;
            wy=py;
            breakflag=1;
            break;
        }
        py-=1;
        px-=1;
    }
    px=*ssx;
    py=*ssy-cabdist-1;
    if breakflag==1{
        break;
    }
    for _ in 0..(cabdist+1){
        if(px<0 || px>maxrow || py<0 || py>maxrow){
            py+=1;
            px-=1;
            continue;
        }
        let mut okflag=1;
        for ((sx,sy),cabdist) in &scaners{
            if(*ssx==*sx && *ssy==*sy){
                continue;
            }
            if ifintersect(px, py, *sx, *sy, *cabdist){
                okflag=0;
                break;
            }
        }
        if okflag==1 {
            wx=px;
            wy=py;
            breakflag=1;
            break;
        }
        py+=1;
        px-=1;
    }
    px=*ssx-cabdist-1;
    py=*ssy;
    if breakflag==1{
        break;
    }
    for _ in 0..(cabdist+1){
        if(px<0 || px>maxrow || py<0 || py>maxrow){
            py+=1;
            px+=1;
            continue;
        }
        let mut okflag=1;
        for ((sx,sy),cabdist) in &scaners{
            if(*ssx==*sx && *ssy==*sy){
                continue;
            }
            if ifintersect(px, py, *sx, *sy, *cabdist){
                okflag=0;
                break;
            }
        }
        if okflag==1 {
            wx=px;
            wy=py;
            breakflag=1;
            break;
        }
        py+=1;
        px+=1;
    }
}
println!("{},{}",wx,wy);
}