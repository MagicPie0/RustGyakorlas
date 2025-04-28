use std::io::{self};

fn main() {
    let mut szo  = String::new(); 

    io::stdin().read_line(&mut szo).expect("Nem sikerült beolvasni a szót!");

    maganhangzo_visszaado(szo);
}

fn maganhangzo_visszaado(szo: String ){
    let mut maganhangzok: Vec<char> = Vec::new();

    maganhangzok = szo.chars().filter(|&x| x == 'a' || x == 'e' || x == 'i' || x == 'o' || x == 'u' || x == 'A' || x == 'E' || x == 'I' || x == 'O' || x == 'U').collect();

    for maganhangzo in maganhangzok.iter() {
        println!("{}", maganhangzo);
    }
}
