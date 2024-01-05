use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero_to_production_in_rust::configuration::get_configuration;
use zero_to_production_in_rust::startup::run;
use zero_to_production_in_rust::telemetry::{get_subscriber, init_subscriber};

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
//     init_subscriber(subscriber);
//
//     // Panic if we can't read configuration
//     let configuration = get_configuration().expect("Failed to read configuration.");
//     let connection_pool = PgPoolOptions::new()
//         .acquire_timeout(std::time::Duration::from_secs(2))
//         .connect_lazy(configuration.database.connection_string().expose_secret())
//         .expect("Failed to connect to Postgres.");
//
//     // We have removed the hard-coded `8000` - it's now coming from our settings!
//     let address = format!(
//         "{}:{}",
//         configuration.application.host, configuration.application.port
//     );
//     let listener = TcpListener::bind(address)?;
//     run(listener, connection_pool)?.await?;
//     Ok(())
// }

// use std::sync::{Arc, Mutex};
// use std::{thread, time};
// fn main() {
//     let arc_mutex = Arc::new(Mutex::new(0));
//
//     let mut handle_of_threads = Vec::new();
//
//     for i in 0..10 {
//         let arc_clone_mutex = Arc::clone(&arc_mutex);
//         let handle_of_thread = thread::spawn(move || {
//             let mut num = arc_clone_mutex.lock().unwrap();
//             thread::sleep(time::Duration::from_secs(1));
//             *num += i;
//         });
//         handle_of_threads.push(handle_of_thread);
//     }
//
//     for i in handle_of_threads {
//         i.join().unwrap();
//     }
//
//     println!("main run");
//
//     let get = *arc_mutex.lock().unwrap();
//
//     println!("main run");
//
//     println!("Result: {}", get);
// }
use std::time::Duration;
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
    thread,
};
use zero_to_production_in_rust::thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let threadPool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        threadPool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(50));
            ("HTTP/1.1 200 OK", "hello sleep")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404"),
    };

    let contents = filename;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
