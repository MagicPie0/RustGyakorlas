use std::io::{self, BufReader, BufWriter, Read, Write};
use std::fs::File;
use std::process;


fn main() {
    println!("Mit szeretnél csinálni? \n Fájlt kiolvasni(1) \n Fájlt írni(2) \n Kilépni(3) ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Nem sikerült a fájlnév beolvasása");

    let szam = input_ellenorzese(&input);

    let mut fajl_nev: String = String::new();

    match szam{
        1 => {
            println!("Add meg a fájl nevét a beolvasáshoz: ");
            io::stdin().read_line(&mut fajl_nev).expect("Nem sikerült a fájlnév beolvasása");
            
            fajl_nev = fajl_nev_ellenorzes(fajl_nev.trim()); 

            match fajl_olvasasa(&fajl_nev) {
                Ok(sorok) => {
                    println!("{:?}", sorok);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
            kilepes_ellenorzes();

        },
        2 => {
            println!("Add meg a fájl nevét: ");
            io::stdin().read_line(&mut fajl_nev).expect("Nem sikerült a fájlnév beolvasása");

            fajl_nev = fajl_nev_ellenorzes(fajl_nev.trim()); 


            match fajl_irasa(&fajl_nev) {
                Ok(siker) =>{
                    println!("{}", siker)
                }
                Err(e) =>{
                    eprintln!("{}", e)
                } 
            }
            kilepes_ellenorzes();
            
        },
        3 => {
            println!("Sikeresen kiléptél!");
            return;
        }
        _ => {
            println!("Érvénytelen választás!");
            kilepes_ellenorzes();

        },
    }

}

fn kilepes_ellenorzes(){
    println!("Szeretnéd folytatni (I/N)?");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Nem sikerült beolvasni a parancsot!");

    let input = input.trim().to_uppercase();


    if input == "I"{
        println!("Sikeresen újra indítottad!");
        main();
    }
    else if input == "N"{
        println!("Kilépés a programból...");

        process::exit(0); 

    }
    else{
        println!("Érvénytelen válasz! Kérlek, válassz I-t vagy N-t.");

        kilepes_ellenorzes();
    }
}

fn fajl_nev_ellenorzes(fajl_nev: &str) -> String{
    if !fajl_nev.is_empty() && !fajl_nev.ends_with(".txt"){
        return format!("{}.txt", fajl_nev)
    }
    fajl_nev.to_string()
}

fn input_ellenorzese(input: &str) -> i32{
    match input.trim().parse::<i32>(){
        Ok(n) => n,
        Err(_) => 0
    }
}

fn fajl_olvasasa(fajl_nev: &str) -> io::Result<Box<Vec<String>>> {
    let fajl = match File::open(fajl_nev.trim()) {
        Ok(n) => n,
        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "A fájl nem létezik"))  
    };

    let mut fajl_tartalma =  BufReader::new(fajl);

    let mut buffer = Vec::new();

    let mut sorok: Vec<String> = Vec::new();

    match fajl_tartalma.read_to_end(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "A fájl üres!"));
            }

            let kontent = String::from_utf8_lossy(&buffer);
            for sor in kontent.lines(){
                if !sor.trim().is_empty(){
                    sorok.push(sor.to_string());
                }
            }

            if sorok.is_empty(){
                return Err(io::Error::new(io::ErrorKind::InvalidData, "A fájl csak üres sorokat tartalmaz!"));
            }

            Ok(Box::new(sorok))

        }
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Hiba történt a fájl olvasásakor: {}", e))),
    }
}

fn fajl_irasa(fajl_nev: &str) -> io::Result<String>{
    let fajl = match File::create(fajl_nev.trim()) {
        Ok(n) => n,
        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "A fájl nem hozta létre!"))  
    };

    let mut buf_writer = BufWriter::new(fajl);

    println!("Írd le amit a fájlba szeretnél írni:");
    let mut fajl_tartalma = String::new();

    io::stdin().read_line(&mut fajl_tartalma).expect("Nem sikerült átadni a fájl tartalmát");

    if fajl_tartalma.is_empty(){
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Nem lehet üres a fájl tartalma!"));
    }

    match buf_writer.write_all(fajl_tartalma.as_bytes()) {
        Ok(_) => return  Ok("Fájl sikeresen mentve".to_string()),
        Err(_e) => return Err(io::Error::new(io::ErrorKind::ReadOnlyFilesystem, "Hiba történt a fájlba írásakor!"))

    }
}