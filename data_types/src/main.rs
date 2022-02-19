fn main() {
    // let guess: u32 = "42".parse().expect("Not a number!")

    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tuple;

    println!("The value of y is {}", y);

    let first = tuple.0 //acessando index da tuple

    let array = [1, 2, 3, 4, 5];

    let array: [i32; 5] = [1, 2, 3, 4, 5];

    let array = [3; 5]; //=> [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];

    

}
