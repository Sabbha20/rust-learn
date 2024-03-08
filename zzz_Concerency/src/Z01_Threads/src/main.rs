use std::thread;
use std::thread::sleep;
use std::time::Duration;

// fn main() {
//     for i in 0..=10{
//         thread::spawn(move || {
//             sleep(Duration::from_millis(1000));
//             println!("new thread: {}", i);
//         });
//     }
//     println!("main thread");
// }

// Since in the above code, the main fuinction was executing before any threads was running
// so we will make the main wait for other threads to finish

fn main() {
    let mut threads = vec![];
    for i in 0..=10{
        let th = thread::spawn(move || {
            sleep(Duration::from_millis(1000));
            println!("new thread: {}", i);
        });
        threads.push(th);
    }

    for t in threads{
        t.join();
    }
    println!("main thread");
}
