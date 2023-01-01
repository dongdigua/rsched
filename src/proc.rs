use std::thread::{self, JoinHandle};
use std::sync::mpsc::sync_channel;

use std::time::Duration;

use crossbeam_utils::sync::{Parker, Unparker};
use rand::prelude::*;


#[derive (Debug)]
pub struct Proc {
    // proc:  &'static dyn Fn() -> (),
    handle: JoinHandle<()>,
    parker:   Parker,
    unparker: Unparker,
}

impl Proc {
    pub fn new() -> Self {
        let (psender, preceiver) = sync_channel(1); // parker
        let (usender, ureceiver) = sync_channel(1); // unparker
        let handle = thread::spawn(move || {
            let p = Parker::new();
            let u = p.unparker().clone();
            psender.send(p).unwrap();
            usender.send(u).unwrap();
            //todo!("some random procedure");

            Proc::random_thing();
        });
        let parker   = preceiver.recv().unwrap();
        let unparker = ureceiver.recv().unwrap();
        // parker.park();  // immediately stop the thread
        Proc {
            handle,
            parker,
            unparker,
        }
    }

    fn random_thing() {
        let mut rng = thread_rng();
        let n: u64 = rng.gen_range(100..1000); 

        thread::sleep(Duration::from_millis(n)); // just simple sleep, add more things later
    }
}


#[test]
fn try_parker() {
    let p = Parker::new();
    let u = p.unparker().clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        println!("thread");
        u.unpark();
    });

    // println!("thread end");
    // Wakes up when `u.unpark()` provides the token.
    p.park();
    println!("called park");
}
