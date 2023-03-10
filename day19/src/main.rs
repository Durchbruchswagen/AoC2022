use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::collections::HashMap;
use std::cmp;
fn solve(time:i32,orerobcost:i32,clayrobcost:i32,obsidianrobcost:(i32,i32),geoderobcost:(i32,i32),
maksore:i32,maksclay:i32,maksobs:i32,currorerob:i32,currclayrob:i32,currobsrob:i32,currgeoderob:i32,
currore:i32,currclay:i32,currobs:i32,delayed:i32)->i32{
    let mut recval;
    let mut retval=0;
    let newtime=time-1;
    let mut bestval=0;
    let newore=currore+currorerob;
    let newclay=currclay+currclayrob;
    let newobs=currobs+currobsrob;
    let mut newdelay=delayed;
    if currore>=orerobcost {
        newdelay=newdelay|(1<<0);
    }
    if currore>=clayrobcost {
        newdelay=newdelay|(1<<1);
    }
    if currore>=obsidianrobcost.0 && currclay>=obsidianrobcost.1 {
        newdelay=newdelay|(1<<2);
    }
    if currore>=geoderobcost.0 && currobs>=geoderobcost.1 {
        newdelay=newdelay|(1<<3);
    }
    retval+=currgeoderob;
    if time==0{
        return 0
    }
    recval=solve(newtime,orerobcost,clayrobcost,obsidianrobcost,geoderobcost,maksore,maksclay,maksobs,currorerob,currclayrob,currobsrob,currgeoderob,newore,newclay,newobs,newdelay);
    
    bestval=cmp::max(bestval, recval);
    if (newtime*currgeoderob)+time*(time-1)/2 >= bestval  && currore>=geoderobcost.0 && currobs>=geoderobcost.1 && (delayed>>3)&1==0{
        recval=solve(newtime, orerobcost, clayrobcost, obsidianrobcost, geoderobcost, maksore, maksclay, maksobs, currorerob, currclayrob, currobsrob, currgeoderob+1, newore-geoderobcost.0, newclay, newobs-geoderobcost.1,0);

        bestval=cmp::max(bestval, recval);
    }
    if (newtime*currgeoderob)+time*(time-1)/2 >= bestval  && currore>=obsidianrobcost.0 && currclay>=obsidianrobcost.1 && currobsrob<maksobs && (delayed>>2)&1==0{
        recval=solve(newtime, orerobcost, clayrobcost, obsidianrobcost, geoderobcost, maksore, maksclay, maksobs, currorerob, currclayrob, currobsrob+1, currgeoderob, newore-obsidianrobcost.0, newclay-obsidianrobcost.1, newobs,0);
        
        bestval=cmp::max(bestval, recval);
    }

    if (newtime*currgeoderob)+time*(time-1)/2 >= bestval && currore>=clayrobcost && currclayrob<maksclay && (delayed>>1)&1==0{
        recval=solve(newtime, orerobcost, clayrobcost, obsidianrobcost, geoderobcost, maksore, maksclay, maksobs, currorerob, currclayrob+1, currobsrob, currgeoderob, newore-clayrobcost, newclay, newobs,0);
        
        bestval=cmp::max(bestval, recval);
    }
    if (newtime*currgeoderob)+time*(time-1)/2 >= bestval && currore>=orerobcost && currorerob<maksore && (delayed>>0)&1==0{
        
        recval=solve(newtime, orerobcost, clayrobcost, obsidianrobcost, geoderobcost, maksore, maksclay, maksobs, currorerob+1, currclayrob, currobsrob, currgeoderob, newore-orerobcost, newclay, newobs,0);
        
        bestval=cmp::max(bestval, recval);
    }
    bestval+retval
}
fn main() {
    let file = File::open("data.txt").unwrap();
    let mut rtval=1;
    let reader = BufReader::new(file);
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let help2:Vec<&str>=help.split_whitespace().collect();
        //println!("{:?}",help2);
        let multiply=help2[1].to_string().parse::<i32>().unwrap();
        let orerobcost=help2[6].to_string().parse::<i32>().unwrap();
        let clayrobcost=help2[12].to_string().parse::<i32>().unwrap();
        let obsidianrobcost=(help2[18].to_string().parse::<i32>().unwrap(),help2[21].to_string().parse::<i32>().unwrap());
        let geoderobcost=(help2[27].to_string().parse::<i32>().unwrap(),help2[30].to_string().parse::<i32>().unwrap());
        let maksore=cmp::max(orerobcost, cmp::max(clayrobcost, cmp::max(obsidianrobcost.0, geoderobcost.0)));
        let maksclay=obsidianrobcost.1;
        let maksobs=geoderobcost.1;
        //println!("{},{},{},{:?},{:?},{},{},{}",multiply,orerobcost,clayrobcost,obsidianrobcost,geoderobcost,maksore,maksclay,maksobs);
        let wynik=solve(32, orerobcost, clayrobcost, obsidianrobcost, geoderobcost, maksore, maksclay, maksobs, 1, 0, 0, 0, 0, 0, 0,0);
        println!("{}",wynik);
        rtval*=wynik;
    }
    println!("WYNIK: {}",rtval);
}
