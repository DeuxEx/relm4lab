#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use zbus::{proxy, Connection};
use std::time::Duration;
use tokio::time::sleep;

// 1. Define a proxy that matches App A's setup
#[proxy(
    interface = "com.example.DataReceiver",
    default_service = "com.example.AppA",
    default_path = "/com/example/AppA"
)]
trait DataReceiverProxy {
    // This signature matches the method in App A
    async fn send_packet(&self, text: &str) -> zbus::Result<()>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 2. Connect to the identical Session Bus
    let connection = Connection::session().await?;
    
    // 3. Create the proxy client
    let proxy = DataReceiverProxyProxy::new(&connection).await?;

    let mut counter = 1;
    loop {
        let message = format!("Packet #{} from App B", counter);
        
        // 4. Call the remote D-Bus API directly like a local function
        match proxy.send_packet(&message).await {
            Ok(_) => println!("Successfully sent: {}", message),
            Err(e) => eprintln!("D-Bus communication error: {}", e),
        }

        counter += 1;
        sleep(Duration::from_secs(1)).await;
    }
}
