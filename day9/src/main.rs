use std::{fs::File, io::{BufReader, BufRead}, vec};
use std::collections::HashSet;
fn Goup(wynik:&mut i32,hx:&mut i32,hy:&mut i32,tx:&mut i32,ty:&mut i32,haset:&mut HashSet<(i32,i32)>)->(){
    if hx==tx && hy==ty{
        *hy+=1;
    }
    else if hy==ty {
        *hy+=1;
    }
    else if hx==tx {
        if hy<ty {
            *hy+=1;
        }
        else if hy>ty {
            *hy+=1;
            *ty+=1;
        }
    }
    else if hy!=ty && hx!=tx{
        if hy<ty {
            *hy+=1;
        }
        else if hy>ty {
            *hy+=1;
            *ty+=1;
            if(hx<tx){
                *tx-=1;
            }
            else if hx>tx {
                *tx+=1;
            }
        }
    }
    if !haset.contains(&(*tx,*ty)){
        haset.insert((*tx,*ty));
        *wynik+=1;
    }
}
fn Godown(wynik:&mut i32,hx:&mut i32,hy:&mut i32,tx:&mut i32,ty:&mut i32,haset:&mut HashSet<(i32,i32)>)->(){
    if hx==tx && hy==ty{
        *hy-=1;
    }
    else if hy==ty {
        *hy-=1;
    }
    else if hx==tx {
        if hy>ty {
            *hy-=1;
        }
        else if hy<ty {
            *hy-=1;
            *ty-=1;
        }
    }
    else if hy!=ty && hx!=tx{
        if hy>ty {
            *hy-=1;
        }
        else if hy<ty {
            *hy-=1;
            *ty-=1;
            if hx<tx {
                *tx-=1;
            }
            else if hx>tx {
                *tx+=1;
            }
        }
    }
    if !haset.contains(&(*tx,*ty)){
        haset.insert((*tx,*ty));
        *wynik+=1;
    }
}
fn Goleft(wynik:&mut i32,hx:&mut i32,hy:&mut i32,tx:&mut i32,ty:&mut i32,haset:&mut HashSet<(i32,i32)>)->(){
    if hx==tx && hy==ty{
        *hx-=1;
    }
    else if hx==tx {
        *hx-=1;
    }
    else if hy==ty {
        if hx>tx {
            *hx-=1;
        }
        else if hx<tx {
            *hx-=1;
            *tx-=1;
        }
    }
    else if hy!=ty && hx!=tx{
        if hx>tx {
            *hx-=1;
        }
        else if hx<tx {
            *hx-=1;
            *tx-=1;
            if hy<ty {
                *ty-=1;
            }
            else if hy>ty {
                *ty+=1;
            }
        }
    }
    if !haset.contains(&(*tx,*ty)){
        haset.insert((*tx,*ty));
        *wynik+=1;
    }
}
fn Goright(wynik:&mut i32,hx:&mut i32,hy:&mut i32,tx:&mut i32,ty:&mut i32,haset:&mut HashSet<(i32,i32)>)->(){
    if hx==tx && hy==ty{
        *hx+=1;
    }
    else if hx==tx {
        *hx+=1;
    }
    else if hy==ty {
        if hx<tx {
            *hx+=1;
        }
        else if hx>tx {
            *hx+=1;
            *tx+=1;
        }
    }
    else if hy!=ty && hx!=tx{
        if hx<tx {
            *hx+=1;
        }
        else if hx>tx {
            *hx+=1;
            *tx+=1;
            if hy<ty {
                *ty-=1;
            }
            else if hy>ty {
                *ty+=1;
            }
        }
    }
    if !haset.contains(&(*tx,*ty)){
        haset.insert((*tx,*ty));
        *wynik+=1;
    }
}

fn Gov2(dir:&str,wynik:&mut i32,vecx:&mut Vec<i32>,vecy:&mut Vec<i32>,haset:&mut HashSet<(i32,i32)>)->(){
    match dir {
        "U"=>vecy[0]+=1,
        "D"=>vecy[0]-=1,
        "L"=>vecx[0]-=1,
        "R"=>vecx[0]+=1,
        _=>()
    }
    for i in 0..9{
        if vecy[i]>=vecy[i+1]-1 && vecy[i]<=vecy[i+1]+1 && vecx[i]>=vecx[i+1]-1 && vecx[i]<=vecx[i+1]+1{
            continue;
        }
        else if vecy[i]>vecy[i+1] && (vecx[i]>=vecx[i+1]-1 && vecx[i]<=vecx[i+1]){
            vecy[i+1]+=1;
            if vecx[i]>vecx[i+1] {
                vecx[i+1]+=1;
            }
            else if vecx[i]<vecx[i+1] {
                vecx[i+1]-=1;
            }

        }
        else if vecy[i]<vecy[i+1] && (vecx[i]>=vecx[i+1]-1 && vecx[i]<=vecx[i+1]) {
            vecy[i+1]-=1;
            if vecx[i]>vecx[i+1] {
                vecx[i+1]+=1;
            }
            else if vecx[i]<vecx[i+1] {
                vecx[i+1]-=1;
            }

        }
        else if vecx[i]>vecx[i+1] && (vecy[i]>=vecy[i+1]-1 && vecy[i]<=vecy[i+1]) {
            vecx[i+1]+=1;
            if vecy[i]>vecy[i+1] {
                vecy[i+1]+=1;
            }
            else if vecy[i]<vecy[i+1] {
                vecy[i+1]-=1;
            }

        }
        else if vecx[i]<vecx[i+1] && (vecy[i]>=vecy[i+1]-1 && vecy[i]<=vecy[i+1]) {
            vecx[i+1]-=1;
            if vecy[i]>vecy[i+1] {
                vecy[i+1]+=1;
            }
            else if vecy[i]<vecy[i+1] {
                vecy[i+1]-=1;
            }

        }
        else{
            if vecx[i]<vecx[i+1]{
                if vecy[i]<vecy[i+1]{
                    vecx[i+1]-=1;
                    vecy[i+1]-=1;
                }
                else if vecy[i]>vecy[i+1]{
                    vecx[i+1]-=1;
                    vecy[i+1]+=1;
                }
            }
            if vecx[i]>vecx[i+1]{
                if vecy[i]<vecy[i+1]{
                    vecx[i+1]+=1;
                    vecy[i+1]-=1;
                }
                else if vecy[i]>vecy[i+1]{
                    vecx[i+1]+=1;
                    vecy[i+1]+=1;
                }
            }
        }
    }
    if !haset.contains(&(vecx[9],vecy[9])){
        haset.insert((vecx[9],vecy[9]));
        *wynik+=1;
    }
}
fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut secik=HashSet::new();
    let mut secik2:HashSet<(i32,i32)>=HashSet::new();
    let mut hx=0;
    let mut hy=0;
    let mut tx=0;
    let mut ty=0;
    let mut wynik=0;
    let mut wynik2=0;
    let mut vecx=vec![0;10];
    let mut vecy=vec![0;10];
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        let tokens=help.split_whitespace().collect::<Vec<&str>>();
        for _ in 0..tokens[1].parse::<i32>().unwrap(){
            match tokens[0]{
                "R"=>Goright(&mut wynik, &mut hx, &mut hy, &mut tx, &mut ty, &mut secik),
                "U"=>Goup(&mut wynik, &mut hx, &mut hy, &mut tx, &mut ty, &mut secik),
                "L"=>Goleft(&mut wynik, &mut hx, &mut hy, &mut tx, &mut ty, &mut secik),
                "D"=>Godown(&mut wynik, &mut hx, &mut hy, &mut tx, &mut ty, &mut secik),
                _=>()
            }
            Gov2(tokens[0], &mut wynik2, &mut vecx, &mut vecy, &mut secik2);
        }
    }
    println!("{},{}",wynik,wynik2);
}
