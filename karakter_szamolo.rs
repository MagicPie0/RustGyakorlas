use std::io::{self};

fn main() {
    let beker = bekeres().expect("Nem sikerült betenni a változóba a függvény visszatérési értékét!");

    match ellenorzes(&beker) {
        Ok(szam) => println!("{}", szam),
        Err(e) => eprintln!("Hiba: {}", e),
    }
}

fn bekeres() -> io::Result<String> {
    println!("Kérlek írj be egy szót: ");

    let mut szo = String::new();
    io::stdin().read_line(&mut szo).expect("Hiba volt a bevitelkor");

    Ok(szo)
}

fn ellenorzes(szo: &str) -> io::Result<i32> {
    if !szo.trim().is_empty() {
        let szamolhato_szo = szo.replace(char::is_whitespace, "");
        let szam = szamolhato_szo.chars().count() as i32;
        Ok(szam)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Üres szöveget adtál meg"))
    }
}
