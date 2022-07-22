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
    loop_vec();
    vector_eum();
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
    text(String),
}
fn vector_eum() {
    let row = vec![
        SpreadsheeCell::Int(4),
        SpreadsheeCell::Float(3.4),
        SpreadsheeCell::text(String::from("Ndeta")),
    ];
    for sheet in &row {
        println!("Here is the sheet {:?}", sheet);
    }
}
