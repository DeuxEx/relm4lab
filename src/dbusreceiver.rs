use zbus::{interface, connection};
use std::error::Error;

// 1. Define the structure that will hold your application state
struct DataReceiver;

// 2. Export this structure as a D-Bus Interface
#[interface(name = "com.example.DataReceiver")]
impl DataReceiver {
    // This is the API method App B will call
    async fn send_packet(&self, text: String) -> zbus::fdo::Result<()> {
        println!("Received text packet via D-Bus: {}", text);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let receiver = DataReceiver;

    // 3. Connect to the user's Session Bus and claim a unique name
    let _conn = connection::Builder::session()?
        .name("com.example.AppA")? // This replaces the IP address/Port
        .serve_at("/com/example/AppA", receiver)? // The object path
        .build()
        .await?;

    println!("D-Bus service 'com.example.AppA' is running...");
    
    // Keep the service alive indefinitely
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
