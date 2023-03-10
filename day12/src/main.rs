use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::path::Path;


use petgraph::{Graph, data::Build};
use petgraph::graph::NodeIndex; // graph not adj
use petgraph::stable_graph::StableGraph;
use petgraph::algo::dijkstra;
use petgraph::{algo, prelude::*}; 


fn main() {
    let mut g=Graph::<(),i32,Directed>::new();
    let mut starpos=Vec::new();
    let mut start=g.add_node(());
    g.remove_node(start);
    let mut end=g.add_node(());
    g.remove_node(start);
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut tab:Vec<Vec<(NodeIndex,i32)>>=Vec::new();
    let mut y=0;
    for lines in reader.lines(){
        tab.push(Vec::new());
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        for a in help.chars(){
            match a{
                'S'=>{
                    println!("Znaleziono S");
                    start=g.add_node(());
                    tab[y].push((start,'a' as i32));
                    starpos.push(start);
                }
                'a'=>{
                    println!("Znaleziono S");
                    let newa=g.add_node(());
                    tab[y].push((newa,'a' as i32));
                    starpos.push(newa);
                }
                'E'=>{
                    println!("Znaleziono Z");
                    end=g.add_node(());
                    tab[y].push((end,'z' as i32));
                }
                _=>{
                    tab[y].push((g.add_node(()), a as i32));
                }
            }
        }
    y+=1;
    }
    let ymax=tab.len();
    let xmax=tab[0].len();
    for y in 0..ymax{
        for x in 0..xmax{
            if y>0 && (tab[y][x].1-tab[y-1][x].1>=-1){
                g.add_edge(tab[y][x].0,tab[y-1][x].0,1);
            }
            if x>0 && (tab[y][x].1-tab[y][x-1].1>=-1){
                g.add_edge(tab[y][x].0,tab[y][x-1].0,1);
            }
            if y+1<ymax && (tab[y][x].1-tab[y+1][x].1>=-1){
                g.add_edge(tab[y][x].0,tab[y+1][x].0,1);
            }
            if x+1<xmax && (tab[y][x].1-tab[y][x+1].1>=-1){
                g.add_edge(tab[y][x].0,tab[y][x+1].0,1);
            }
        }
    }
    //println!("{:?}",g);
    //println!("Star:{:?},End:{:?}",start,end);
    let path=dijkstra(&g, start, Some(end), |_| 1);
    //println!("{:?}",path);
    println!("{:?}",path.get(&end));
    let mut best:i32=*path.get(&end).unwrap();
    for a in starpos{
        let newpath=dijkstra(&g, a, Some(end), |_| 1);
        if(newpath.get(&end).is_some() && *newpath.get(&end).unwrap()<best){
            best=*newpath.get(&end).unwrap();
        }
    }
    println!("BEST:{}",best);
}