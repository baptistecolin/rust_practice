fn main() {
    // about parsing (here what's important is that the type of num had to be specified beforehand)
    let num: u32 = "42".parse().expect("Not a number !");
    println!("Parsed as {}", num);

    // about scalar types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    let a: u32 = 2;
    let b: u32 = 3;

    let z = x/y;
    println!("Le résultat est {}", z);
    let z = a/b;
    println!("Le résultat est {}", z);
    let z = b/a;
    println!("Le résultat est {}", z);

    // Those below wouldn't work because of incompatibility between floats and ints
    //let z = x/b;
    //println!("Le résultat est {}", z);
    //let z = a/y;
    //println!("Le résultat est {}", z);


    // about operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("sum : {}", sum);
    println!("difference : {}", difference);
    println!("product : {}", product);
    println!("quotient : {}", quotient);
    println!("remainder : {}", remainder);

    // about booleans
    //
    let t = true;
    let f: bool = false;
    let or = t || f;
    let and = t && f;

    println!("or : {}", or);
    println!("and : {}", and);
}
