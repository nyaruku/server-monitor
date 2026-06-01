mod headers;
use headers::system_provider::system_provider;
use std::sync::{Arc, Mutex};

fn main() {
    let sys = Arc::new(Mutex::new(sysinfo::System::new_all()));
    system_provider::start(Arc::clone(&sys));

    loop {
        let info = {
            let s = sys.lock().unwrap();
            system_provider::getSysInfo(&s)
        };
        for (i, usage) in info.cpuUsage.iter().enumerate() {
            println!("CPU {} | {:.1}%", i, usage);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // let listener = std::net::TcpListener::bind("127.0.0.1:15080").unwrap();
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     println!("Connection established!");
    // }
}
