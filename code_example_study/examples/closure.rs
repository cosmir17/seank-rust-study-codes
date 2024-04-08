// #[derive(Debug)]
// #[allow(dead_code)]
// struct City {
//     city: String,
//     population: u64
// }
//
// fn sort_pop(city: &mut Vec<City>) {
//     city.sort_by_key(pop_helper)
// }
//
// fn pop_helper(pop: &City) -> u64 {
//     pop.population
// }

// pub fn closure_fn_course() {
//     let a = City{city: String::from("A"), population: 100};
//     let b = City{city: String::from("Y"), population: 33};
//     let c = City{city: String::from("W"), population: 22};
//     let d = City{city: String::from("S"), population: 255};
//
//     let mut vec: Vec<City> = Vec::new();
//     vec.push(a);
//     vec.push(b);
//     vec.push(c);
//     vec.push(d);
//
//     sort_pop(&mut vec);
//     println!("{:?}", vec);
//
//     let add = |x: i32| -> i32 {x + 1};
//     let add_v2 = |x| x+1;
//     add_v2(1);
// }

// pub fn fn_closure<F>(value: i32, closure: F) -> i32
//     where
//         F: Fn(i32) -> i32,
// {
//     closure(value)
// }
//
// pub fn fnmut_closure<F>(value: i32, mut closure: F)
//     where
//         F: FnMut(i32) -> (),
// {
//     closure(value);
// }

pub fn process_file<F>(file_path: &str, processor: F)
    where
        F: FnOnce(&str) -> Result<(), std::io::Error>,
{
    match std::fs::read_to_string(file_path) {
        Ok(contents) => {
            if let Err(e) = processor(&contents) {
                eprintln!("Error processing file: {}", e);
            }
        }
        Err(e) => {
            println!("Error here 1");
            eprintln!("Error reading file: {}", e);
        }
    }
}

fn main() {

}