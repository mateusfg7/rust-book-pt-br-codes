fn main() {
    // let user1 = User {
    //     email: String::from("alguem@exemplo.com"),
    //     username: String::from("algum_nome123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // println!("User1 Email: {}", user1.email);

    // let mut user2 = User {
    //     email: String::from("outro.alguem@exemplo.com"),
    //     username: String::from("algum_456465"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // println!("User2 Email: {}", user2.email);

    // user2.email = String::from("novo.email@exemplo.com");
    // println!("User2 Email: {}", user2.email);

    // let user2 = User {
    //     email: String::from("alguem@exemplo.com"),
    //     username: String::from("algum_456465"),
    //     ..user1
    // };
    // println!("{}", user2.sign_in_count);

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);

    /* CALCULA ÁREA DO QUADRADO */

    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        length: 40,
        width: 10,
    };
    let rect3 = Rectangle {
        length: 45,
        width: 60,
    };

    let square = Rectangle::square(100);

    println!("react1 is {:?}", rect1);
    println!("react1 is {:#?}", rect1);
    println!("--------------------------------------------------------");

    println!("The area of rectangle is {} square pixels", rect1.area());
    println!("--------------------------------------------------------");

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("--------------------------------------------------------");

    println!(
        "The area of square {}x{} is {}",
        square.width,
        square.length,
        square.area()
    );
    println!("--------------------------------------------------------");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        // if self.length > rect.length && self.width > rect.width {
        //     return true;
        // }
        // false

        self.length > rect.length && self.width > rect.width
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
