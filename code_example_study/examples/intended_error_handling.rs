use std::num::ParseIntError;

fn main()  -> Result<(), ParseIntError> {
    // let number_to_be_panic = "gsdgsdg".parse::<i32>().unwrap(); //error happens and panic
    let number_not_to_be_panic = "eege".parse::<i32>()?; //this doesn't panic
    println!("{:?}", number_not_to_be_panic);
    // println!("{:?}", number_to_be_panic)
    Ok(())
}