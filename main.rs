use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    //Explicitly declaring the type
    let v: Vec<i32> = Vec::new();
    //letting rust infer the type
    let t = vec![1, 3, 54, 5];
    let third: &i32 = &t[2];
    println!("The third element is {}", third);
    match t.get(2) {
        Some(d) => println!("The third element is {}", d),
        None => println!("The is no third element "),
    }
    let mut x = Vec::new();
    x.push(6);
    x.push(7);
    x.push(8);
    x.push(9);
    {
        let scope_v = vec![1, 2, 4, 5, 67, 8, 9];
        //Do stuff with vec
    } //vec goes out of scope and is freed here

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 6);
    scores.insert(String::from("Red"), 4);
    for (key, value) in &scores {
        println!("{}'s score is {}", key, value);
    }

    loop_vec();
    vector_eum();
    iter_string();
    overwriting_valuie();
    update_value_based();
}
fn loop_vec() {
    //Immutable
    let v = vec![1, 2, 3, 4, 5, 6, 7, 5];
    for c in &v {
        println!("{}", c);
    }
    //mutable
    let mut v_1 = vec![2, 35, 6, 7, 7];
    for f in &mut v_1 {
        *f += 10;
        println!("{}", f);
    }
}

#[derive(Debug)]
enum SpreadsheeCell {
    Int(i32),
    Float(f32),
    Text(String),
}
fn vector_eum() {
    let row = vec![
        SpreadsheeCell::Int(4),
        SpreadsheeCell::Float(3.4),
        SpreadsheeCell::Text(String::from("Ndeta")),
    ];
    for sheet in &row {
        println!("Here is the sheet {:?}", sheet);
    }
}
fn iter_string() {
    let t = "mamamaia";
    for c in t.chars() {
        println!("{}", c);
    }
}

fn vect_tuple_hashmap() {
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_score = vec![10, 50];
    // We can use <_,_> because Rust will infer the type as Strini32
    let mut scores_hashmap: HashMap<_, _> =
        teams.into_iter().zip(initial_score.into_iter()).collect();
}
fn ownershp_in_hashmap() {
    let field_name = String::from("favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
}

fn overwriting_valuie() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 6);
    scores.insert(String::from("Blue"), 4);
    println!("{:?}", scores);
}

fn update_value_based() {
    let text = "hello there YouTube";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
