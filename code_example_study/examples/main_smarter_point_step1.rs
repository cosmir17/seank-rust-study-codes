mod main_box;
mod main_rc;

trait Vehicle {
    fn drive(&self);
}

struct Truck;

impl Vehicle for Truck {
    fn drive(&self) {
        println!("Truck is driving")
    }
}

fn main() {
    // let t: dyn Vehicle;
    // let a = Truck;
    // t = a;

    // let t: dyn Vehicle;
    // t = Truck;
    //this doesn't work because something to store
    // on the stack the rust compiler needs to know the size of the memory at compilation time
    // in this case, anything can implement Vehicle and each of the implementation of Vehicle
    // could be a different size so the compiler has no way to knowing or computing the exact
    // size needed in the stack for that memory

    //the only solution here is to store in the heap, which unfortunately has a much slower access time
    //so you wouldn't want to actually store things on the heap if you don't need to

    //but in this case, we do need to and the way we do that is by using Box<dyn Vehicle>
    //then we do
    let t: Box<dyn Vehicle>;
    t = Box::new(Truck);
    t.drive();

    //the other use case for box is a recursive types.
    //what if we want to be able to represent a list of Trucks so we have like a line of trucks waiting
    // in at a stoplight or something
    // we can add field in the Truck struct,
}
