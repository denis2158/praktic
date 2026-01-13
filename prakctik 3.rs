fn main() {
    let x: i32 = 10;
    let y: i32 = 5; // перенесли y в зовнішню область видимості
    
    {
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    
    println!("Outer scope value of x is {} and value of y is {}", x, y);
}
