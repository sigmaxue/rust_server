//use std::ffi::CString;
//use std::os::raw::c_char;
use tiny_http::{Server, Response};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

extern {
    fn LedInit();
    fn LedTask();
    fn SetLedColor(r:u8, g:u8);
}

#[derive(Debug)]
struct PiMessage {
    msg_type: u32,
    action: u32,
}

fn main()  {
    let (tx, rx): (mpsc::Sender<PiMessage>, mpsc::Receiver<PiMessage>) = mpsc::channel();

    let handle = thread::spawn(move|| {
        unsafe{LedInit()}
        let mut led_enable = 0;
        loop {
            if let Ok(received) = rx.try_recv() {
                println!("Got: {:?}", received);
                if received.msg_type == 1 {
                    led_enable = received.action; 
                }   
            }
            
            thread::sleep(Duration::from_millis(1000));

            if led_enable > 0 {
                unsafe{LedTask()}   
            }
        }
    });

    let server = Server::http("0.0.0.0:8000").unwrap();
    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                 request.method(),
                 request.url(),
                 request.headers()
                );

        let mut pi = PiMessage{msg_type: 1, action: 0};
        if request.url().contains("open") {
            pi = PiMessage{msg_type: 1, action: 1};
            tx.send(pi).unwrap();
        } else if request.url().contains("close") {
            pi = PiMessage{msg_type: 1, action: 0};
            tx.send(pi).unwrap();
        } 
        
        let response = Response::from_string("hello world");
        request.respond(response);
    }
    handle.join().unwrap();
}
