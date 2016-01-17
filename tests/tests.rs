extern crate posix_ipc as ipc;

use ipc::signals;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn raise_and_catch_with_closure() {
    let caught: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let caught2 = caught.clone();
    let f = move |_| {
        *caught2.lock().unwrap() = true;
    };
    unsafe {
        signals::Signal::Usr1.handle(Box::new(f)).unwrap();
    }
    signals::Signal::Usr1.raise().unwrap();
    assert!(*caught.lock().unwrap());
}

#[test]
fn raise_and_catch_with_func() {
    static mut caught: bool = false;
    {
        fn f(_: signals::Signal) {unsafe { caught = true }}
        unsafe {
            signals::Signal::Usr1.handle(Box::new(f)).unwrap();
        }
    }
    signals::Signal::Usr1.raise().unwrap();
    thread::sleep(Duration::from_millis(500)); // this is really racy :(
    assert!(unsafe { caught });
}

