//! # Math
//! simple math functions

use std::cmp::Reverse;
use std::collections::BinaryHeap;

///gcd of two u32s
pub fn gcd(mut a:u32,mut b:u32)->u32{
    loop{
        (a,b)=(if a>b{a}else{b}, if a>b{b}else{a});
        a-=b;
        if a==0{break b;}
    }
}

///gcd of a vector of u32
/// ## Returns Option\<u32\>
/// <hr>
/// <b>None</b>: Empty vector
/// 
/// <b>Some(u32)</b>: Nonempty vector
pub fn gcd_vec(collection:& Vec<u32>)->Option<u32>
{
    if collection.len()==0{return None;}
    let mut tmp=*collection.get(0).expect("");
    for &n in collection.iter(){
        tmp=gcd(tmp, n);
        if tmp==1{break;}
    }
    Some(tmp)
}

///Calculates the frobenius number of a vector of u32
pub fn frobenius_number(collection: &Vec<u32>) -> Option<u32> {
    if collection.is_empty() {
        return None;
    }
    let minimum = *collection.iter().min().unwrap();
    if minimum <= 1 {
        return None; 
    }
    let mut unique_coins: Vec<u32> = collection.iter()
        .filter(|&&x| x != minimum)
        .cloned()
        .collect();
    unique_coins.sort_unstable();
    unique_coins.dedup();

    let min_usize = minimum as usize;

    let mut dist = vec![u64::MAX; min_usize];
    

    let mut heap = BinaryHeap::new();
    dist[0] = 0;
    heap.push(Reverse((0_u64, 0_usize)));

    while let Some(Reverse((current_val, remainder))) = heap.pop() {
        if current_val > dist[remainder] {
            continue;
        }

        for &coin in &unique_coins {
            let next_remainder = (remainder + (coin as usize) % min_usize) % min_usize;
            let next_val = current_val + coin as u64;

            if next_val < dist[next_remainder] {
                dist[next_remainder] = next_val;
                heap.push(Reverse((next_val, next_remainder)));
            }
        }
    }

    let max_shortest_path = dist.into_iter().max()?;
    if max_shortest_path == u64::MAX {
        return None;
    }

    Some((max_shortest_path - minimum as u64) as u32)
}