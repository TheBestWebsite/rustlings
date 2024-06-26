// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let number = 3;
    {let number: &str = "3";
    println!("Spell a Number : {}", number);}
    println!("Number plus two is : {}", number + 2);
    println!("Hello, world!");
}
