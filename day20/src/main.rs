use std::{fs::File, io::{BufReader, BufRead}, vec};
fn main() {
    let file = File::open("data.txt").unwrap();
    let mut vectorek:Vec<(i128,i32)>=vec![];
    let reader = BufReader::new(file);
    let mut indhelp=0;
    for lines in reader.lines(){
        let mut help="".to_string();
        match lines{
            Ok(a)=>help=a.to_string(),
            _=>()
        }
        vectorek.push((help.parse::<i128>().unwrap()*811589153,indhelp));
        indhelp+=1;
    }
    let mut mixer=vectorek.to_vec();
    let cycles=10;
    for _ in 0..cycles{
    for i in 0..vectorek.len(){
        //println!("{:?}",mixer);
        let mut ind=0;
        for y in 0..mixer.len(){
            if mixer[y].1==i as i32{
                ind=y;
                break;
            }
        }
        let x=mixer[ind].0;
        let help=mixer.len() as i32 -1;
        let newind=(ind as i128 + x).rem_euclid(help as i128);
        mixer.remove(ind);
        mixer.insert(newind as usize, (x,i as i32));
    }
}
   // println!("{:?}",mixer);
    let mut zeroind=0;
    for i in 0..mixer.len(){
        if mixer[i].0==0{
            zeroind=i;
            break;
        }
    }
    let mut wynik=0;
    let ind1=(zeroind as i32+1000).rem_euclid(mixer.len() as i32);
    let ind2=(zeroind as i32+2000).rem_euclid(mixer.len() as i32);
    let ind3=(zeroind as i32+3000).rem_euclid(mixer.len() as i32);
    wynik+=mixer[ind1 as usize].0+mixer[ind3 as usize].0+mixer[ind2 as usize].0;
    println!("{}",wynik);
}
