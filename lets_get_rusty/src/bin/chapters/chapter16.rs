pub mod fearless_concurrency {

    use std::{thread, time::Duration};
    pub fn fearless_concurrency() {
        thread::spawn(|| {
            for i in 1..30 {
                println!("Hi Spawned thread {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("Hi Main thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    } // in above code, the spawn thread loop only runs for 5 times

    //NOTE: when the main thread ends, the Spawned thread stops executing no matter what
    //There is a way to prevent this behaviour:
    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("Hi Spawned Thread {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // handle.join().unwrap(); // If we do handle join here, the main thread will block and
        // first print the result of handle and then go ahead

        for i in 1..5 {
            println!("Hi Main thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap(); // blocks the current thread until the result comes from handle
    }
}

pub mod message_passing {
    use std::sync::mpsc;
    use std::thread;
    fn main() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let msg = String::from("hi");
            tx.send(msg).unwrap();
        });
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
}

//Rc is not thread safe, so we use Arc (Atomic rc)
pub mod sharing_state {
    use std::sync::{Arc, Mutex};
    use std::thread;
    fn main() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap(); //lock blocks the current thread
            *num = 6; // since num is a reference ,we deference it to get the inner value , int32
                      // here, and change its value to 6
        }
    }

    fn sharing_state() {
        let counter = Arc::new(Mutex::new(0)); // counter is immutable
        let mut handles = vec![];
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
