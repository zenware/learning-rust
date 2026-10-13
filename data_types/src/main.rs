// `isize` and `usize` are Architecture dependent, but when would I pick them over specifying?
fn main() {
    let dec_int = 98_222;
    let hex_int = 0xff;
    let oct_int = 0o77;
    let bin_int = 0b1111_0000;
    let byte = b'A'; // u8 only
    println!("{dec_int} {hex_int} {oct_int} {bin_int} {byte}");

    let x = 2.0; // Defaults to f64
    let y: f32 = 3.0; // Both follow IEEE-754 Floating Point

    // Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    let (a, b, d) = tup2; // Pattern Matching/Destructuring to pull values out of tuple.
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let mut tup3: (i32, i32) = (1, 2);
    tup3.0 = 0
    tup3.1 += 5;

    let arr = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    // TODO: Can you put computation in the array initializer? e.g. "rand()"?
    // Or if you're trying to be really stupid "sleep()"?
    let arr3 = ["Initial Value"; 5]; // Will initialize the array to 5 elements of "Initial Value"
    let first = arr3[0];
    let second = arr3[1];

}
