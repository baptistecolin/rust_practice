fn main() {
    let num: u32 = "42".parse().expect("Not a number !");
    println!("Parsed as {}", num);

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
}
