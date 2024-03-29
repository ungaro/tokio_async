use mini_redis::client;
use bytes::Bytes;


#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    }
}


#[tokio::main]
async fn main() {


    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();


    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    
        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            use Command::*;
    
            match cmd {
                Get { key } => {
                    client.get(&key).await;
                }
                Set { key, val } => {
                    client.set(&key, val).await;
                }
            }
        }
    });

    // Establish a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    let tx2 = tx.clone();

    // Spawn two tasks, one gets a key, the other sets a key
    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "foo".to_string(),
        };
    
        tx.send(cmd).await.unwrap();
    });
    
    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
        };
    
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}