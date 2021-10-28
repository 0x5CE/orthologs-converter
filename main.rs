use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{self, prelude::*, BufReader};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./main filename");
        return Ok(())
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut e_count = HashMap::new();
    let mut names = HashMap::new();

    // step 1: parse the file & gather the data in hashmap
    //
    for line in reader.lines() {
        let ln = Box::new(line?);
        let split = ln.split("\t").collect::<Vec<&str>>();
        let l = split[0].split("|") .collect::<Vec<&str>>()[0];
        let r = split[1].split("|") .collect::<Vec<&str>>()[0];

        // fighting the borrow checker
        let ll = String::from(l).clone();
        let lhash = calculate_hash(&ll);
        let rr = String::from(r).clone();
        let rhash = calculate_hash(&rr);

        names.insert(lhash, ll);
        names.insert(rhash, rr);

        match e_count.get(&(lhash, rhash)) {
            Some(&n) => e_count.insert((lhash, rhash), n+1),
            _ => e_count.insert((lhash, rhash), 1),
        };
    }
    
    //step 2: step the result as a matrix
    //
    
    // printing row
    for (_,y) in names.iter() {
        print!("\t{}", y);
    }
    println!("\n");

    for (x1,y) in names.iter() { 
        print!("{}\t", y);
        for (x2,_) in names.iter() {
            match e_count.get(&(*x1, *x2)) {
                Some(&n) => print!("{}\t", n),
                _ => print!("-\t"),
            };
        };
        println!("");
    };
    Ok(())
}
