// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut x = 3; // by default variables in rust are immutable
                   // also if no type is given and just assginment like that
                   // compiler can "determine" the needed type just like auto
                   // in C++ 
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
