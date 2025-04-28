use std::io::{self};

fn main() {
    let szo1 = String::new();
    let szo2 = String::new();

    io::stdin().read_line(&mut szo1).expect("Hiba");
    io::stdin().read_line(&mut szo2).expect("Hiba");

    match szo_osszehasonlitas(&szo1, &szo2){
        Ok(szo) => println!("{}", szo),
        Err(e) => println!("Hiba: {}", e),
    }
}

fn szo_osszehasonlitas(szo1: &str, szo2: &str) -> io::Result<String>{
    if szo1.len() > szo2.len(){
        return Ok(szo1.to_string());
    }
    else if szo1.len() < szo2.len(){
        return Ok(szo2.to_string());
    }
    else{
        return Ok("".to_string());
    }
}