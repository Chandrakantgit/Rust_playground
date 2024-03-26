fn main(){
    println!("Hello World!"); // println is a macro here,its not a fucntion.Macors in Rust allows extensive metaprogramming
    let x = 5;
    println!("{}",x+5+5);

        // As can named arguments.
        println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}