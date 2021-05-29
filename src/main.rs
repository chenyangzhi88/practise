use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, usize};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, Condvar};
static NTHREADS: i32 = 3;
static TankSize: usize = 100;
pub enum OperateType{
    Write,
    Read,
}
pub struct Pair {
    key: String,
    value: String
}

pub struct Writer {
    content: Pair,
    lock: Mutex<bool>,
    cvar: Condvar,
    operate_type: OperateType,
}

pub fn Put(key: String, value: String, mut index: BTreeMap<String, String>) {
    index.insert(key, value);
}

pub struct WriteTank(Vec<Writer>);
fn main() {
    //cond var practise----------------------------------------------------
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    //-------------------------------------------------------------

    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let mut cache_index = BTreeMap::new();

    let (tx, rx): (Sender<Writer>, Receiver<Writer>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            let mut i = 0;
            loop {
                let key_i = i.to_string();
                let value_i = String::from("fasdfafasdfasdfasdfassssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssfasdfasdddddddddddddddddddfdfasdfad");
                let pair_i = Pair{ key: key_i, value: value_i};
                let writer_i = Writer { content: pair_i, lock: lock_id, cvar: cvar_id};
                thread_tx.send(writer_i).unwrap();
                i += 1;
                if i > 1_000_000 {

                    break;
                }
            }
            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..3_000_000 {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        if ids.len() > TankSize {
            for it in ids.into_iter() {
               let (key, value) = it.content;
               Put(key, value, cache_index);
            }
        }
        let result = rx.try_recv().unwrap();
        ids.push(result);
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids.len());
}