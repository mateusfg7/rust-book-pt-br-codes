extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 11);

    println!("O número secreto é {numero_secreto}");

    loop {
        println!("Digite seu palpite: ");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao let entrada");

        // `trim()` will remove any whitespace before and after, or \n (newline), entered by enter key.
        // `parse()` will convert a string to a variety of number types
        let palpite: u32 = palpite
            .trim()
            .parse()
            .expect("Por favor, digite um número!");

        println!("Você disse: {palpite}");

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => println!("Você acertou!"),
        }
    }
}
