use std::io::{self};

fn main() {
    println!("Írj be egy számot: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Nem sikerült beolvasni a számot!");

    match osztas(&input) {
        Ok(osztok) => println!("Összes osztó: {:?}", osztok),
        Err(e) => eprintln!("{}", e)
    }
}

fn osztas(input: &str) -> io::Result<Box<Vec<i32>>>{
    let szam: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Nem jó szám!"))
        }
    };

    let mut osztok = Vec::new();

    if szam == 0{
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "A szám nulla!"))
    }
    else if szam < 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Nem lehet negatív a szám!"))
    }
    else{
        for i in 1..=szam{
            if szam % i == 0{
                osztok.push(i);
            }
        }
        return Ok(Box::new(osztok));
    }

}

