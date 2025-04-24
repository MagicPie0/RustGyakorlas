use std::io::{self};

fn main() {
    println!("Adj meg egy számot: ");
    
    let mut bevitel = String::new();
    io::stdin().read_line(&mut bevitel).expect("Nem sikerült beolvasni!");

    let bevitel = bevitel.trim(); 
 
    if bevitel.is_empty() {
        println!("Nem írtál be semmit. Kérlek próbáld újra.");
        main(); 
        return; 
    }

    let szam: i32 = match bevitel.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Nem számot írtál be!");
            main(); 
            return;
        }
    };

    if szam > 0 {
        println!("A szám pozitív!");
    } else if szam < 0 {
        println!("A szám negatív!");
    } else {
        println!("A szám nulla!");
    }

    if szam % 2 == 0 {
        println!("A szám páros!");
    } else {
        println!("A szám páratlan!");
    }

    println!("Újra akarod játszani (I/N)?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Hiba a beolvasásban!");
    let input = input.trim().to_uppercase(); 

    match input.as_str() {
        "I" => main(), 
        "N" => return, 
        _   => return, 
    }
}
