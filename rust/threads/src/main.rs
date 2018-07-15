use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        (1..10)
            .for_each(|i| {
                println!("number {} from spawned thread", i);
                thread::sleep(Duration::from_millis(1));
            });
    });

    handle
        .join()
        .unwrap();

    (1..5)
        .for_each(|i| {
            println!("number {} from main thread", i);
            thread::sleep(Duration::from_millis(1));
        });

    let v = vec![1, 2, 3];
    let new_handle = thread::spawn(move || {
        println!("vector {:?}", v);
    });

    new_handle
        .join()
        .unwrap();
}
