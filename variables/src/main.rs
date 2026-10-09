fn main() {
    let x = 5;

    // TODO: Why is same-scope shadowing at all possible?
    // It seems a bit antithetical to Rust immutability/borrowing.
    //
    // An [example from the Rust book](https://rust-book.cs.brown.edu/ch03-01-variables-and-mutability.html#shadowing):
    // let spaces = "   ";
    // let spaces = spaces.len();
    //
    // Whereas the following is a compiler error due to not being able to change the data-type.
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // NOTE: Presumably explicit casting + a new symbol-name is necessary for this.
    // And then there's no point in making spaces be mutable since you'll have "let space_count"
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner-scope is: {x}");
    }
    println!("The value of x is: {x}");
}
