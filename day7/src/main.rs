use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::mem::needs_drop;
use std::{usize, i128};
#[derive(Debug,Clone)]
struct Node{
    data:i128,
    name:String,
    children: Vec<Node>,
}
impl Node{
    fn new(data: i128,name: String)-> Node{
        Node{data:data,name:name,children:vec![]}
    }
    fn add_child(self,child: Node)-> Node{
        let mut newvec=self.children.clone();
        newvec.push(child);
        Node { data: self.data, name: self.name, children: newvec }
    }
    fn update_data(self,data:i128)->Node{
        Node {data: data,name: self.name,children: self.children,}
    }
    fn zipper(self)->Zipper{
        Zipper { node:self, parent: None, index: 0 }
    }
}
#[derive(Debug)]
struct Zipper{
    node: Node,
    parent: Option<Box<Zipper>>,
    index:usize,
}
impl Zipper{
    fn child(mut self,index:usize)->Zipper{
        let child=self.node.children.swap_remove(index);
        Zipper { node: child, parent: Some(Box::new(self)), index: index}
    }
    fn parent(self)->Zipper{
        let Zipper{node,parent,index}=self;
        let Zipper{
            node: mut pparent,
            parent: parent_parent,
            index: pparent_index,
        }=*parent.unwrap();
        pparent.children.push(node);
        let len = pparent.children.len();
        pparent.children.swap(index,len-1);
        Zipper { node: pparent, parent: parent_parent, index: pparent_index }
    }
    fn update(self,data:i128)->Zipper{
        Zipper { node: self.node.update_data(data), parent: self.parent, index: self.index }
    }
    fn add_child(self,child:Node)->Zipper{
        Zipper { node:self.node.add_child(child), parent: self.parent, index: self.index }
    }
}
struct Solution{
    zip:Zipper,
    correct:Vec<i128>
}

impl Solution {
    fn new(zip:Zipper)->Solution{
        Solution { zip: zip, correct: vec![]}
    }
    fn child(self,index:usize)->Solution{
        Solution { zip: self.zip.child(index), correct: self.correct }
    }
    fn parent(self)->Solution{
        Solution { zip: self.zip.parent(), correct: self.correct }
    }
    fn add(self,value:i128)->Solution{
        let mut newvec=self.correct;
        newvec.push(value);
        Solution { zip:self.zip, correct: newvec }
    }
}

struct Solution2{
    zip:Zipper,
    correct:Vec<(i128,String)>
}

impl Solution2{
    fn new(zip:Zipper)->Solution2{
        Solution2{ zip: zip, correct: vec![]}
    }
    fn child(self,index:usize)->Solution2{
        Solution2{ zip: self.zip.child(index), correct: self.correct }
    }
    fn parent(self)->Solution2{
        Solution2{ zip: self.zip.parent(), correct: self.correct }
    }
    fn add(self,name:String,value:i128)->Solution2{
        let mut newvec=self.correct;
        newvec.push((value,name.to_string()));
        Solution2 { zip:self.zip, correct: newvec }
    }
}

fn compute_size(zipper:Zipper)->Zipper{
    let max=zipper.node.children.len();
    let mut newzipper=zipper;
    let mut newdata=0;
    if max==0{
        if newzipper.parent.is_none(){
            return newzipper;
        }
        else{
            return newzipper.parent();
        }
    }
    for i in 0..max{
        newzipper=compute_size(newzipper.child(i));
        newdata+=newzipper.node.children[i].data;
    }
    newzipper=newzipper.update(newdata);
    if newzipper.parent.is_none(){
        newzipper
    }
    else{
        newzipper.parent()
    }
}

fn solve(sol:Solution)->Solution{
    let max=sol.zip.node.children.len();
    if max==0{
        if sol.zip.parent.is_none(){
            return sol
        }
        else{
            return sol.parent()
        }
    }
    let mut newsol=sol;
    let dataval=newsol.zip.node.data;
    if dataval<=100000{
        newsol=newsol.add(dataval);
    }
    for i in 0..max{
        newsol=solve(newsol.child(i));
    }
    if newsol.zip.parent.is_none(){
        newsol
    }
    else{
        newsol.parent()
    }
}
    fn solve2(sol:Solution2)->Solution2{
        let max=sol.zip.node.children.len();
        if max==0{
            if sol.zip.parent.is_none(){
                return sol
            }
            else{
                return sol.parent()
            }
        }
        let mut newsol=sol;
        let dataval=newsol.zip.node.data;
        let name=newsol.zip.node.name.to_string();
        newsol=newsol.add(name,dataval);
        for i in 0..max{
            newsol=solve2(newsol.child(i));
        }
        if newsol.zip.parent.is_none(){
            newsol
        }
        else{
            newsol.parent()
        }

}
fn main() {
    let filesys=Node::new(0, "/".to_string());
    let mut zipper=filesys.zipper();
    let file = File::open("dane.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut help="".to_string();
        let mut isc=false;
        let mut isdir=false;
        let mut isfile=false;
        let mut cd=false;
        let mut dest="".to_string();
        let mut data=0;
        match line {
            Ok(x)=>help=x,
            _=>()
        }
        for i in help.split_whitespace(){
            if i=="$".to_string(){
                isc=true;
            }
            else if i.parse::<i128>().is_ok(){
                data=i.parse::<i128>().unwrap();
                isfile=true;
            }
            else if i=="dir"{
                isdir=true;
            }
            else if isc && i=="ls"{
                continue;
            }
            else if isc && i=="cd"{
                cd=true;
            }
            else if isc && cd{
                dest=i.to_string();
            }
            else if isdir || isfile{
                dest=i.to_string();
            }
        }
        if cd {
            if dest=="/"{
                continue;
            }
            else if dest==".." {
                zipper=zipper.parent();
            }
            else{
                let maxlen=zipper.node.children.len();
                for i in 0..maxlen{
                    if dest==zipper.node.children[i].name{
                        zipper=zipper.child(i);
                        break;
                    }
                }
            }
        }
        else if isdir{
            zipper=zipper.add_child(Node::new(0,dest));
        }
        else if isfile{
            zipper=zipper.add_child(Node::new(data, dest));
        }
    }
    while !zipper.parent.is_none(){
        zipper=zipper.parent();
    }
    zipper=compute_size(zipper);
    let mut sol=Solution2::new(zipper);
    sol=solve2(sol);
    let max=70000000;
    let needed=30000000;
    let unusedspace=max-sol.zip.node.data;
    let sumvec=sol.correct;
    let mut name="/".to_string();
    let mut bestval=sol.zip.node.data;
    for x in sumvec{
        if x.0+unusedspace>=needed{
            if x.0<bestval{
                bestval=x.0;
                name=x.1;
            }
        }
    }
    println!("Directory:{},size:{}",name,bestval);
}
