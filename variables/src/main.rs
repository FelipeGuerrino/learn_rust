fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const PI: f64 = 3.14;

    let y = 5;

    let y = y + 1; //shadowing

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    let spaces = "    ";
    let spaces = spaces.len(); //shadowing com tipagens diferentes, impossivel com variaveis com a keyword mut
    println!("The number of spaces is: {}", spaces);
}
