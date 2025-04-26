use std::io::{self};

fn main() {
    println!("Adj meg egy szót: ");

    let mut szo = String::new();
    let _ = io::stdin().read_line(&mut szo);

    let forditott_szo = szo_forditasa(&szo);

    match forditott_szo{
        Ok(forditott_szo) => println!("{}", forditott_szo),
        Err(_) => println!("Nem adtál meg értéket!")
    }
}

fn szo_forditasa(szo: &str) -> io::Result<String>{
    if !szo.is_empty(){
        Ok(szo.chars().rev().collect::<String>())
    }
    else{
        Err(io::Error::new(io::ErrorKind::InvalidInput, ""))
    }


}
