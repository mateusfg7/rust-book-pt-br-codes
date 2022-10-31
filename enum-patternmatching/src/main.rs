#![allow(unused)]

fn main() {
    // let ip_vquatro = VersaoIp::V4;
    // let ip_vseis = VersaoIp::V6;

    // let local = EnderecoIp {
    //     versao: VersaoIp::V4,
    //     endereco: String::from("127.0.0.1"),
    // };
    //
    // let loopback = EnderecoIp {
    //     versao: VersaoIp::V6,
    //     endereco: String::from("::1"),
    // };

    // let local = VersaoIp::V4(String::from("127.0.0.1"));
    // let loopback = VersaoIp::V6(String::from("::1"));

    // let local = VersaoIp::V4(127, 0, 0, 1);
    // let loopback = VersaoIp::V6(String::from("::1"));

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    //
    // if y.is_some() {
    //     let soma = x + y.unwrap();
    //     println!("5+5={soma}");
    // } else {
    //     println!("y is not valid");
    // }

    // println!("----------------------------------------------------------------");
    // println!("Penny: {}", value_in_cents(Moeda::Penny));
    // println!("Nickel: {}", value_in_cents(Moeda::Nickel));
    // println!("Dime: {}", value_in_cents(Moeda::Dime));
    // // println!("Quarter: {}", value_in_cents(Moeda::Quarter));
    // println!(
    //     "Quarter Alaska: {}",
    //     value_in_cents(Moeda::Quarter(UsState::Alaska))
    // );
    // println!("----------------------------------------------------------------");

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     // other => move_player(other),
    //     _ => (),
    // }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // Do the same as code above
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

// fn add_fancy_hat() {
//     println!("Add Fancy Hat");
// }
// fn remove_fancy_hat() {
//     println!("Remove Fancy Hat");
// }
// fn move_player(num_spaces: u8) {
//     println!("Move player {num_spaces}");
// }
// fn reroll() {}

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(n) => Some(n + 1),
//     }
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }
//
// enum Moeda {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
//
// fn value_in_cents(moeda: Moeda) -> u32 {
//     match moeda {
//         Moeda::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Moeda::Nickel => 5,
//         Moeda::Dime => 10,
//         Moeda::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// enum VersaoIp {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum VersaoIp {
//     V4(String),
//     V6(String),
// }

// enum VersaoIp {
//     V4,
//     V6,
// }

// struct EnderecoIp {
//     versao: VersaoIp,
//     endereco: String,
// }
