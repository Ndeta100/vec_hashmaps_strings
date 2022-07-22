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
}
