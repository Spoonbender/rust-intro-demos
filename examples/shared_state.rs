use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let mut results = Vec::new();
    thread::spawn(move || {
        for num in 1..=10 {
            let square = num * num;
            let result = format!("{} X {} = {}", num, num, square);
            results.push(result);
        }
    });
}








fn main2() {
    let results = Arc::new(Mutex::new(Vec::new()));

    let mutex = results.clone();
    let handle1 = thread::spawn(move || {
        for num in 1..=10 {
            let square = num * num;
            let result = format!("{} X {} = {}", num, num, square);
            {
                let mut guard = mutex.lock().unwrap(); // take lock
                let shared_state = &mut *guard;
                shared_state.push(result);
                thread::sleep(Duration::from_millis(100));
            }   // lock released as `guard`'s scope ends
        }
    });

    let mutex = results.clone();
    let handle2 = thread::spawn(move || {
        for num in 11..=20 {
            let square = num * num;
            let result = format!("{} X {} = {}", num, num, square);
            {
                let mut guard = mutex.lock().unwrap(); // take lock
                let shared_state = &mut *guard;
                shared_state.push(result);
                thread::sleep(Duration::from_millis(100));
            }   // lock released as `guard`'s scope ends
        }
    });
   
    handle1.join().unwrap();
    handle2.join().unwrap();

    let results = results.lock().unwrap();
    println!("Results: {:?}", results);
}
