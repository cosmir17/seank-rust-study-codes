use std::collections::BTreeMap;
use std::sync::{Mutex, Arc, RwLock};

// Disclaimer: The following code is not my original work. It was sourced from the
// YouTube video at https://youtu.be/Z-2siR9Ki84?si=YDzpkoNi4TZzZ6vW.
// I have manually typed out the code and comments for my own learning and study purposes.
// Upon review, it appears that these examples are not optimized for a multithreaded environment.
// In my personal opinion, using Arc in a single-threaded context may not be the most appropriate choice.
// RC would be the right choice.
// However, it's possible that the author intentionally kept the examples single-threaded due to
// presentation slide space constraints and to convey specific points.
fn main() {
    //Arc<T>
    //Increment reference counter on clone()

    let string_type_1: String = "Hello World".to_owned();
    let string_type_2: String = String::from("Hello World");

    let arc1: Arc<String> = Arc::new(string_type_1);
    let arc2 = Arc::clone(&arc1);

    assert_eq!(arc1, arc2);
    assert!(Arc::strong_count(&arc1) == 2);

    //Mutex<T> example
    //implements mutual exclusion via atomics
    //only one lock can be held at the same time
    let mutex = Mutex::new("Hello World!".to_owned());

    let mut mutex_guard = mutex.lock().unwrap();
    mutex_guard.push_str("from the Mutex");
    drop(mutex_guard); //re-lock the mutex


    //Arc<Mutex<T>>
    //Combine this with Arc<T> to refer to a Mutex from many different places(threads)
    //this pattern means that you can take some kind of value and spread it all over your program
    //put in a different thread, put it in your data structures whatever
    //wherever it is being referenced you can still get immutable access to the underlying value
    //and also if all of those arcs go out of scope (the underlying value, the mutex and
    // whatever is inside the mutex) are going to be cleaned up

    //there is this lock function, and what you get back from it is a mutex guard
    //which is de-referable, so you can still push something in onto it and then when you drop the mutex guard
    //you re-lock the mutex or unlock the mutex,
    //depends on how you think about it whether it should be lock or unlock

    let m1: Arc<Mutex<String>> = Arc::new(Mutex::new("Hello World".to_owned()));
    let m2 = Arc::clone(&m1);

    //thread 1
    let mut m1_guard = m1.lock().unwrap();
    m1_guard.push_str("from the Mutex!");
    drop(m1_guard); //re-lock the mutex

    //thread 2
    let mut m2_guard = m2.lock().unwrap();
    m2_guard.push_str("from the other Mutex!");
    drop(m2_guard); //re-lock the mutex

    //there is an alternative to this the mutex is sort of,
    //mutex is very harsh, it only really allows one access at a time
    //what if it's just all reading, what if we have something that writes occasionally (or rarely)
    //and we have lots of readers
    //underpinning the Rust's borrow rules (multiple read borrows but only one borrow if it's mutable)
    //smart pointer variant of that is RwLock
    //Arc<RwLock<T>>
    //follow the rust borrow rules!
    //allow infinite concurrent readers
    //only allow a single mutator

    //RwLock<T>:
    // RwLock<T> is a reader-writer lock that allows multiple readers to access the data concurrently, but only one writer at a time.
    // It follows Rust's borrowing rules, allowing multiple immutable borrows or a single mutable borrow.
    // In the example, multiple threads (r1, r2, r3) can acquire read locks concurrently using read(), while a single thread (w1)
    // can acquire a write lock using write(). The write lock blocks until all read locks are released.
    let rwlock = RwLock::new("Hello World".to_owned());

    //Thread 1
    let r1 = rwlock.read().unwrap(); //we can read multiple times
    let r2 = rwlock.read().unwrap(); //we can access that memory location read only as times as we want
    let r3 = rwlock.read().unwrap(); //

    //thread 2
    let mut w1 = rwlock.write().unwrap();
    w1.push_str("from the RwLock");
    drop(w1);
    //but when we want to write to it, we have to call
    //the write function, the write function is going to fail or block in this case until no readers exist and no other writer exits
    //similarly, a read function is going to fail if there is currently a writer writing to it.
    //I am not dropping any of my read handles here because of the space

    //what if we want to have some kind of concurrent access and we have some kind of schema, where we can pre-filter
    //our requests
    //well that is absolutely possible
    //no one forces you to use a mutex as your global lock and in fact, you probably shouldn't
    //so what this example is you have a tree or some kind of map and you have two different types of indices
    //one is 'super index', prefix to your index to your index, you sort of prefilter. if you are looking at uuids
    //you could sort of look at just the first n characters and then sort of partition all of that data into subsections
    //and then that actual subsection is locked, so you do a read you do an immutable access
    //this would obviously the Btreemap would still have to be in an Arc, otherwise, you can't refer to it from multiple places.
    //you do a read access to the map, the outer one with your super index and then inner value that you get back
    //that's the mutex you lock that and so you can move the lock to be as close as possible to where you're actually storing
    //your data so that you don't have to needlessly lock and it means that there's something called lock congestion, where if you
    //have lots of different threads that all trying to access the same lock that is going to slow down all of those threads
    //because you're going to spend a lot of time waiting for the lock to be freed etc
    //this is a good pattern if you have some kind of map data structure and want to be clever about your locks
    // can we lock smarter?
    //yes, we can

    // let lock_map = BTreeMap<SuperIndex, Mutex<BTreeMap<Index, Data>>>;
    // let sub_map; &Mutex<_> = lock_map.get(my_super_index).unwrap();
    // let mutex_guard = sub_map.lock().unwrap();
    // let data = mutex_guard.get_mut(my_index).unwrap();
    // data.is_42 = 42;

    // Optimizing Locks with BTreeMap:
    // The commented-out code at the end demonstrates an optimization technique to reduce lock contention.
    // Instead of using a single global lock, the data is partitioned into smaller locks based on a "super index" using a BTreeMap.
    // Each "super index" maps to a Mutex<BTreeMap<Index, Data>>, allowing finer-grained locking.
    // When accessing data, the code first obtains a reference to the inner Mutex using the "super index" and then locks it to access the specific data associated with the "index".
    // This approach reduces lock congestion by distributing the locks across different partitions of the data.
}

//Answers from A.I.
//In the original code I attached to this thread in the first message here,
//
// Explain to me what '.to_owned()', LockResult from 'mutex.lock().unwrap();', 'Mutex Guard' and '.push_str' method do.
//
// Certainly! Let's go through each of the mentioned concepts and methods in the context of the original code:
//
// .to_owned():
// The .to_owned() method is defined on borrowed string slices (&str) and creates an owned String from the borrowed slice.
// In the code, "Hello World".to_owned() creates a new String instance with the content "Hello World" and transfers ownership of the string to the new instance.
// This is useful when you need an owned String rather than a borrowed &str, particularly when storing the string in a data structure that requires owned values, such as Mutex<String> or Arc<String>.
// LockResult from mutex.lock().unwrap():
// mutex.lock() is a method call on a Mutex<T> instance that attempts to acquire a lock on the mutex.
// The lock() method returns a LockResult<MutexGuard<T>>, which is a Result type indicating the success or failure of acquiring the lock.
// If the lock is successfully acquired, the LockResult will be an Ok variant containing a MutexGuard<T>.
// If the lock acquisition fails (e.g., due to poisoning), the LockResult will be an Err variant.
// In the code, unwrap() is called on the LockResult to extract the MutexGuard<T> if the lock acquisition is successful, or to panic if it fails.
// MutexGuard:
// MutexGuard is a type that represents a lock guard for a mutex.
// When you call lock() on a Mutex<T> and it succeeds, you get a MutexGuard<T> as a result.
// The MutexGuard acts as a proof of ownership of the lock and provides exclusive access to the data wrapped inside the mutex.
// While the MutexGuard is held, no other thread can acquire the lock on the same mutex, ensuring mutual exclusion.
// The MutexGuard automatically releases the lock when it goes out of scope or when it is manually dropped using drop(mutex_guard).
// .push_str() method:
// The .push_str() method is defined on the String type and appends a string slice (&str) to the end of the String.
// In the code, mutex_guard.push_str("from the Mutex") appends the string slice "from the Mutex" to the String wrapped inside the Mutex.
// This operation modifies the String in-place, increasing its length and updating its content.
// Since mutex_guard is a MutexGuard<String>, it provides mutable access to the underlying String, allowing the .push_str() method to be called.
// These concepts and methods work together to ensure safe and exclusive access to shared mutable data across multiple threads. The Mutex provides the synchronization mechanism, the MutexGuard represents the ownership of the lock and provides access to the data, and methods like .to_owned() and .push_str() are used to manipulate the data safely within the lock's scope.
//
// It's important to note that the use of unwrap() in the code assumes that the lock acquisition always succeeds. In real-world scenarios, it's recommended to handle the potential failure cases gracefully instead of using unwrap() directly.
//
// Copy
//
//
// S
// Tell me why 'let string_type_1: String = "Hello World".to_owned();' was used rather than 'let string_type_2: String = String::from("Hello World");'
//
// In Rust, both "Hello World".to_owned() and String::from("Hello World") achieve the same result of creating an owned String from a string literal. However, there are some differences in terms of performance and semantics.
//
// "Hello World".to_owned():
// When you call to_owned() on a string literal, it creates a new String instance and copies the content of the string literal into the newly allocated String.
// The to_owned() method is defined on the ToOwned trait, which is implemented for str. It provides a generic way to convert a borrowed value to an owned value.
// Under the hood, to_owned() typically calls String::from(self) to perform the conversion.
// The to_owned() method is more generic and can be used on any type that implements the ToOwned trait, not just str.
// String::from("Hello World"):
// String::from() is a static method defined on the String type itself. It takes a string slice (&str) as an argument and creates a new String instance with the content of the slice.
// String::from() is specifically designed to create a String from a string slice, making the intention clear.
// It is a more direct and idiomatic way to create a String from a string literal or a string slice.
// In terms of performance, String::from() is generally optimized for creating String instances from string slices.
// In the code you provided, "Hello World".to_owned() is used to create an owned String from the string literal "Hello World". While it achieves the desired result, using String::from("Hello World") would be more idiomatic and potentially slightly more efficient.
//
// Here's an equivalent version using String::from():
//
// rust
//
//
// Copy code
// let string_type_2: String = String::from("Hello World");
// Both "Hello World".to_owned() and String::from("Hello World") will create a new String instance with the content "Hello World", but String::from() is more commonly used and is considered the idiomatic way to create a String from a string literal or slice.
//
// In general, it's recommended to use String::from() when you specifically want to create a String from a string slice, as it clearly conveys the intention and is optimized for that purpose. The to_owned() method is more generic and can be used when you have a borrowed value of any type that implements the ToOwned trait and you want to convert it to an owned value.
//
// Copy
//
//
// Claude does not have the ability to run the code it generates yet.
// S
// In Rust, I see codes like the following 'mutex.lock().unwrap()'. It can panic and cause runtime error. Wouldn't it cause a production issue?
//
// Yes, using unwrap() directly on the result of mutex.lock() can indeed cause a panic and lead to runtime errors if the lock acquisition fails. In production code, it is generally recommended to handle the potential failure cases gracefully instead of using unwrap() directly.
//
// The lock() method on a Mutex returns a LockResult<MutexGuard<T>>, which is a Result type indicating the success or failure of acquiring the lock. If the lock acquisition succeeds, it returns an Ok variant containing the MutexGuard. However, if the lock acquisition fails (e.g., due to poisoning), it returns an Err variant.
//
// When you call unwrap() on the LockResult, it will panic if the Result is an Err variant. In a production environment, a panic can lead to unexpected termination of the program and potential issues.
//
// To handle the lock acquisition result properly and avoid panics, you have a few options:
//
// Use match to handle the Result:
// rust
//
//
// Copy code
// match mutex.lock() {
//     Ok(guard) => {
//         // Use the MutexGuard
//     }
//     Err(e) => {
//         // Handle the error case
//         eprintln!("Failed to acquire lock: {:?}", e);
//         // Take appropriate action, such as returning an error or using a fallback mechanism
//     }
// }
// Use if let with Ok to handle the success case and else for the error case:
// rust
//
//
// Copy code
// if let Ok(guard) = mutex.lock() {
//     // Use the MutexGuard
// } else {
//     // Handle the error case
//     eprintln!("Failed to acquire lock");
//     // Take appropriate action
// }
// Use the ? operator to propagate the error:
// rust
//
//
// Copy code
// fn do_something_with_mutex(mutex: &Mutex<T>) -> Result<(), Error> {
//     let guard = mutex.lock()?;
//     // Use the MutexGuard
//     Ok(())
// }
// In this case, the ? operator will propagate the error if the lock acquisition fails, and the function will return an Err variant. The caller of the function can then handle the error appropriately.
// By handling the lock acquisition result explicitly and avoiding the use of unwrap(), you can gracefully deal with potential failures and prevent runtime panics in production code.
//
// It's important to consider the specific requirements and error handling strategy of your application when dealing with lock acquisition results. In some cases, panicking might be acceptable if the lock acquisition failure is considered a fatal error. However, in most production scenarios, it's recommended to handle the errors gracefully, log relevant information, and take appropriate actions based on the specific context and requirements of your application.