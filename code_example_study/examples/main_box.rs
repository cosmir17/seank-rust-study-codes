trait Vehicle {
    fn drive(&self);
}

struct Truck {
    next_truck: Box<Option<Self>>,
}

impl Vehicle for Truck {
    fn drive(&self) {
        println!("Truck is driving")
    }
}

fn main() {
    //the other use case for box is a recursive types.
    //what if we want to be able to represent a list of Trucks so we have like a line of trucks waiting
    // in at a stoplight or something
    // we can add field in the Truck struct,

    // struct Truck {
    //     next_truck: Option<Truck>
    // }

    //we have an error here,
    //recursive without indirection, recursive type 'Truck' has infinite size

    //so we use Box
    // struct Truck {
    //     next_truck: Box<Option<Truck>>
    // }

    //two use cases for Box
    // 1. when we have a variable with a trait type that can't be computed at compile time, there we can use box
    // 2. recursive data type, for example, we have a struct and one of the fields has a recursive type
    // (that it's enclosed in so with Box and regular rust variables the memory)
}
