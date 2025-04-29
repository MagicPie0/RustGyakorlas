use std::io::{self};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Nem sikerült beolvasni a számot!");

    let szam: i32 = match input.trim().parse(){
        Ok(n) => n,
        Err(_e) => 0 
    };

    let mut all_szam: Vec<Vec<u8>> = Vec::new();
    
    if szam != 0{
        all_szam = allokalt_szam(szam);
    }

    let flat: Vec<u8> = all_szam.into_iter().flatten().collect();

    let output = String::from_utf8_lossy(&flat);

    println!("{}", output);

    return;
}

fn allokalt_szam(n: i32) -> Vec<Vec<u8>> {
    let block = vec![0u8; (n * 1000) as usize];
    vec![block]
}

