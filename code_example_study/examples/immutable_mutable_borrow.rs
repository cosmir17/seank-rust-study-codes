fn main() {
    // let mut x = 5;
    //
    // let y = &x;     // Immutable borrow
    // let z = &mut x; // Mutable borrow
    //
    // println!("y: {}, z: {}", y, z); //can't use them together


    //This works
    //
    // let mut x = 5;
    // {
    //     let y = &x;     // Immutable borrow
    //     println!("y: {}", y);
    //
    // }
    // let z = &mut x; // Mutable borrow
    // println!("z: {}", z);


    //can use Immutable and mutable references separately according to the burrow rule
    let mut x = 5;

    let z = &mut x; // Mutable borrow
    println!("z: {}", z);

    let y = &x;     // Immutable borrow
    println!("y: {}", y);

    // println!("z: {}", z); this can't be done here as it breaches the Rust's burrow rule

}