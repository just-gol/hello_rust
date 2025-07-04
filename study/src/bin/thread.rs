use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    thread_8();
}

fn thread_1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("子线程:{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("主线程:{}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn thread_2() {
    let handle = thread::spawn(|| {
        println!("child thrad is runing...");
        for i in 1..5 {
            println!("count:{}", i)
        }
    });
    handle.join().unwrap();
    println!("main thread");
}

fn thread_3() {
    let mut handles = vec![];
    for i in 0..3 {
        let handle = thread::spawn(move || {
            println!("thread {} is working", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("all thread completed")
}

fn thread_4() {
    let string = String::from("thread");
    thread::spawn(move || {
        println!("string:{}", string);
    });
}

fn thread_5() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("value");
        tx.send(msg).unwrap();
    });
    let rece = rx.recv().unwrap();
    println!("rece:{}", rece);
}

fn thread_6() {
    let (tx, rx) = mpsc::channel();
    for i in 1..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            let msg = format!("thread:{}", i);
            tx.send(msg).unwrap();
        });
    }

    for _ in 1..3 {
        let msg = rx.recv().unwrap();
        println!("msg:{}", msg);
    }
}

fn thread_7() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("receive {:?}", rx.try_recv());
}

fn thread_8() {
    let (tx, rx) = mpsc::sync_channel::<i32>(2); // 缓冲区大小 = 2

    tx.send(1).unwrap(); // ✅ 不阻塞
    tx.send(2).unwrap(); // ✅ 不阻塞
    println!("Sent two messages.");

    // 第三条消息将会阻塞，直到有接收者从缓冲区取走消息
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        println!("Receiving: {}", rx.recv().unwrap());
        println!("Receiving: {}", rx.recv().unwrap());
        println!("Receiving: {}", rx.recv().unwrap());
    });

    tx.send(3).unwrap(); // ⏳ 阻塞直到 rx.recv()
    println!("Sent third message.");
}
