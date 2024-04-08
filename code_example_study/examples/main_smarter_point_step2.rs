// struct Truck {
//     next_truck: Option<Truck>
// }

//rust can't do recursive type in Struct without box

//error[E0072]: recursive type `Truck` has infinite size
//  --> code_example_study/examples/main_smarter_point_step2.rs:1:1
//   |
// 1 | struct Truck {
//   | ^^^^^^^^^^^^
// 2 |     next_truck: Option<Truck>
//   |                        ----- recursive without indirection
//   |
// help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
//   |
// 2 |     next_truck: Option<Box<Truck>>
//   |                        ++++     +

struct Truck {
    next_truck: Option<Box<Truck>>
}

fn main () {

}
