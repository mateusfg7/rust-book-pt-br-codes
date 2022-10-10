extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 11);

    println!("O número secreto é {numero_secreto}");

    println!("Digite seu palpite: ");

    let mut palpite = String::new();

    io::stdin()
        .read_line(&mut palpite)
        .expect("Falha ao let entrada");

    print!("Você disse: {palpite}")
}
