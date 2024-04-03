use std::sync::{Mutex, Arc};

fn main() {
    //arc<T>
    //Increment reference counter on clone()
    let arc1 = Arc::new("Hello World".to_owned());
    let arc2 = Arc::clone(&arc1);

    assert_eq!(arc1, arc2);
    assert!(Arc::strong_count(&arc1) == 2);

    //mutex example
    //implements mutual exclusion via atomics
    //only one lock can be held at the same time
    let mutex = Mutex::new("Hello World!".to_owned());

    let mut mutex_guard = mutex.lock().unwrap();
    mutex_guard.push_str("from the Mutex");
    drop(mutex_guard); //re-lock the mutex



}