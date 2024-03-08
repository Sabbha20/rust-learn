use std::sync::mpsc;
use std::thread;
use std::time::Duration;


const NUM_THREADS:usize = 20;
fn start_threads(d:usize, tx:mpsc::Sender<usize>){
    thread::spawn(move || {
        println!("Setting timer: {}", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("Sending msg: {}", d);
        tx.send(d).unwrap();
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || { tx.send(42).unwrap() });
    println!("received msg: {}", rx.recv().unwrap() );

    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_THREADS{
        start_threads(i, tx.clone());
    }
    for j in rx.iter().take(NUM_THREADS){
        println!("Received msg: {}", j);
    }


}
