use std::io;

fn main() {
    println!("Adivinhe o número!");

    println!("Digite seu palpite: ");

    let mut palpite = String::new();

    io::stdin()
        .read_line(&mut palpite)
        .expect("Falha ao let entrada");

    print!("Você disse: {palpite}")
}
