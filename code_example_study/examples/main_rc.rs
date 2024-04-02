//so the box and regular rust variables, the memory that variable refers to is going
//to be cleaned up as soon as it goes out of the scope.
//what if you have a situation where you want to have multiple references of some memory but
//you're not really sure about the order in which those references are going to go out
//of the scope

//you want the memory to stay around until the last reference goes out of scope

//then use RC<T>, reference counting, it's going to count the number of references to that memory
//and it's going to keep that memory alive until the last reference goes out of scope

//one good example is :
//we have some strucuture and we want a reference to that structure to be stored in multiple data structures
//and we're not really sure which data structure goes out of scope or what's going to be done with that data
//structure but we do know that we want that memory to be deallocated if there's no references to it remaining


use std::rc::Rc;

#[derive(Debug)]
struct Truck {
    capacity: i32
}

fn main() {
    //we want to create three different types of trucks
    let (truck_a, truck_b, truck_c) = (Rc::new(Truck { capacity: 1}),
         Rc::new(Truck { capacity: 2}),
         Rc::new(Truck {capacity: 3}),
    );


    // let facility_one = vec![truck_a, truck_b]; //truck_a and truck_b are Truck type not RC
    // we can use borrow to make it compile but the problem is
    // the main function has to maintain the ownership of truck_b, then truck_b would get de-allocated when the
    // main function is done even if we stopped needing truck_b long before that
    // so we solve this dilemma with Rc
    let facility_one = vec![truck_a, Rc::clone(&truck_b)]; //this clone doesn't clone the truck_b
    // data itself, just creating a new reference
    //The second element of the first vector points to the same memory as the first element of the second vector
    // we can be confident with this setup that
    // 1. once all the vectors that contain truck_b are de-allocated
    // 2. OR all the truck_bs and all the vectors are removed for whatever reasons,
    // truck_b memory will be deallocated so
    // this is very memory efficient in that sense
    // so the main function doesn't need to maintain ownership of truck_b

    let facility_two = vec![truck_b, truck_c];

    println!("Facility one {:?}", facility_one);
    println!("Facility two {:?}", facility_two);

    //we passed the clone of truck_b to facility one
    //we passed the ownership of the actual truck_b rc to facilty two
    //because we are using rc, it doesn't matter which one we de-allocated first

    //we can de-allocate facility two and i am gonna use mem drop to do that

    std::mem::drop(facility_two);
    println!("Facility one after drop : {:?}", facility_one);
    //so facility one still has a valid reference to truck_b after dropping facility two?
    //the reason that is because the reference count rc didn't go zero
    //so the memory wasn't de-allocated

    //__________
    //one useful function when dealing with Rcs called strong count :
    //that gives the exact count references that rc has at the moment
    //to show how that works,
}
