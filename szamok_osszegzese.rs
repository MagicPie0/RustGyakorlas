use std::fs::File;
use std::io::{self, Read, Write};
use rand::Rng;

fn main() {
    if let Err(e) = fajlba_iras() {
        eprintln!("Hiba történt a fájl írásakor: {}", e);
    }

    match fajlbol_olvasas(){
        Ok(szamok) => {
            println!("A fájl tartalma: {:?}", szamok);

            let osszeg = szamok_osszegzese(&szamok);
            println!("A számok összege: {}", osszeg);
        },
        Err(e) => {
            eprintln!("Hiba történt a fájlból olvasáskor: {}", e);
        }
    }
}

fn fajlba_iras() -> io::Result<()> {
    let mut file = File::create("szamok.txt")?;  

    let mut random_szam = rand::rng();


    for _ in 0..10 {
        let szam: i32 = random_szam.random_range(1..101);
        write!(file, "{},", szam)?;  
    }

    println!("A fájlba írás sikeres volt!");
    Ok(())
}


fn fajlbol_olvasas() -> io::Result<Vec<i32>> {
    let mut file = File::open("szamok.txt")?; 

    let mut fajl_tartalma = String::new();
    file.read_to_string(&mut fajl_tartalma)?;  

    let szamok: Vec<i32> = fajl_tartalma
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    Ok(szamok)
}

fn szamok_osszegzese(szamok: &[i32]) -> i32{
    szamok.iter().sum()
}