#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // println!("The third element is {third}");

    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element"),
    // }

    // let v = vec![1, 2, 3, 4, 5];
    // for value in &v {
    //     println!("Value: {value}");
    // }

    // let mut v = vec![1, 2, 3, 4, 5];
    // println!("{:?}", v);

    // for value in &mut v {
    //     *value = *value + 50;
    // }
    // println!("{:?}", v);

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Float(10.12),
    //     SpreadsheetCell::Text(String::from("blue")),
    // ];

    // let s1 = String::from("Hello ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = format!("{}-{}-{}", s1, s2, s3);

    // let s = String::from("abcdefghijkl");
    // println!("{}", s.len());
    // let s = String::from("Здравствуйте");
    // println!("{}", s.len());

    // let s = String::from("ãîǹ");
    // for c in s.chars() {
    //     println!("{c}")
    // }
    // for c in s.bytes() {
    //     println!("{c}")
    // }

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // let team_name = String::from("Blue");

    // println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
