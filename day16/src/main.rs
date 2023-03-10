use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::path::Path;
use itertools::Itertools;
use std::collections::HashSet;
use petgraph::{Graph, data::Build};
use petgraph::graph::NodeIndex; // graph not adj
use petgraph::stable_graph::StableGraph;
use petgraph::algo::floyd_warshall;
use petgraph::{algo, prelude::*}; 
use std::collections::HashMap;

fn sol1(state:i32,vecperm:&Vec<String>,time:i32,
    flowrates:&HashMap<String,i32>,nodesmap:&HashMap<String,NodeIndex>,current:String,res:&HashMap<(NodeIndex,NodeIndex),i32>)->i32{
    let mut best=0;
    let mut rtval=0;
    let mut newtime=time;
    if(current!="AA"){
        newtime-=1;
    }
    if newtime<=0{
        return best;
    }
    rtval=newtime*flowrates.get(&current).unwrap();
    for i in 0..vecperm.len(){
        let node1=*nodesmap.get(&current).unwrap();
        let node2=*nodesmap.get(&vecperm[i]).unwrap();
        let dist=res.get(&(node1,node2)).unwrap();
        if (state>>i)&1==0 && newtime-dist>0{
            let newstate=state|(1<<i);
            let recval=sol1(newstate,vecperm,newtime-dist,flowrates,nodesmap,vecperm[i].to_string(),res);
            if best<recval{
                best=recval;
            }
        }
    }
    return best+rtval;
}

fn sol2(state:i32,vecperm:&Vec<String>,time:i32,timeeleph:i32,
    flowrates:&HashMap<String,i32>,nodesmap:&HashMap<String,NodeIndex>,current:String,currenteleph:String,res:&HashMap<(NodeIndex,NodeIndex),i32>)->i32{
    println!("state:{},time1:{},time2:{}",state,time,timeeleph);
    let mut best=0;
    let mut rtval=0;
    let mut newtime=time;
    let mut rcval=0;
    let mut neweletime=timeeleph;
    if(current!="AA"){
        newtime-=1;
        neweletime-=1;
    }
    if newtime<=0 && neweletime<=0{
        return best;
    }
    if neweletime>0 && newtime<=0{
        return sol1(state, vecperm, neweletime+1, flowrates, nodesmap, currenteleph, res);        
    }
    if neweletime<=0 && newtime>0{
        return sol1(state, vecperm, newtime+1, flowrates, nodesmap, current, res);
    }
    rtval=newtime*flowrates.get(&current).unwrap()+neweletime*flowrates.get(&currenteleph).unwrap();
    for i in 0..15{
        let node1=*nodesmap.get(&current).unwrap();
        let node2=*nodesmap.get(&vecperm[i]).unwrap();
        let dist=res.get(&(node1,node2)).unwrap();
        if (state>>i)&1==0{
            for j in 0..15{
                let node3=*nodesmap.get(&currenteleph).unwrap();
                let node4=*nodesmap.get(&vecperm[j]).unwrap();
                let edist=res.get(&(node3,node4)).unwrap();
                let helpstate=state|(1<<i);
                if (helpstate>>j)&1==0{
                    let newstate=state|(1<<i)|(1<<j);
                    rcval=sol2(newstate, vecperm, newtime-dist, neweletime-edist, flowrates, 
                        nodesmap, vecperm[i].to_string(), vecperm[j].to_string(), res);
                    if best<rcval{
                        best=rcval;
                    }
                }
            }
        }
    }
    return best+rtval;
}
fn main() {
    let mut g=Graph::<(),i32,Directed>::new();
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut permvec=vec![];
    let mut flowrates=HashMap::new();
    let mut nodesmap=HashMap::new();
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let help2:Vec<&str>=help.split([' ',';',',','=']).collect();
        //println!("{:?}",help2);
        let nodestart=help2[1].to_string();
        if help2[5]!="0" {
            permvec.push(nodestart.to_string());
        }
        let mut node;
        if !nodesmap.contains_key(&nodestart){
            node=g.add_node(());
            nodesmap.insert(nodestart.to_string(), node);
        }
        let flow:i32=help2[5].parse().unwrap();
        flowrates.insert(nodestart.to_string(), flow);
        let mut i=11;
        while i<help2.len(){
            let helpnode=help2[i].to_string();
            if !nodesmap.contains_key(&helpnode){
                node=g.add_node(());
                nodesmap.insert(helpnode.to_string(), node);
            }
            g.add_edge(*nodesmap.get(&nodestart).unwrap(), *nodesmap.get(&helpnode).unwrap(), 1);
            i+=2;
        }
    }
    //println!("{:?}",g);
    //println!("{:?}",nodesmap);
    let res=floyd_warshall(&g, |_| 1).unwrap();
    //println!("{:?}",res);
    // let mut wynik=sol1(0, &permvec, 30, &flowrates, &nodesmap, "AA".to_string(), &res);
    // println!("{}",wynik);
    let mut wynik2=0;
    let mut secik=HashSet::new();
    for i in 0..(1<<permvec.len()){
        let mut v1=vec![];
        let mut v0=vec![];
        println!("{}",i);
        let mut n1=0;
        let mut n2=0;
        for j in 0..permvec.len(){
            if(i>>j)&1==1{
                v0.push(permvec[j].to_string());
                n1=n1|(1<<j);
            }
            else{
                v1.push(permvec[j].to_string());
                n2=n2|(1<<j);
            }
        }
        let mut w1=0;
        let mut w2=0;
        if !secik.contains(&(n1,n2)) && !secik.contains(&(n2,n1)){
            w1=sol1(0, &v1, 26, &flowrates, &nodesmap, "AA".to_string(), &res);
            w2=sol1(0, &v0, 26, &flowrates, &nodesmap, "AA".to_string(), &res);
            secik.insert((n1,n2));
        }
            if w1+w2>wynik2{
            wynik2=w1+w2;
        }
    }
    println!("WYNIK:{}",wynik2);
}