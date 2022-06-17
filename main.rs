fn get_num_requests() -> usize {
    std::env::var("NUM_REQUESTS")
        .expect("Expected 'NUM_REQUESTS' env variable")
        .parse()
        .expect("Expected 'NUM_REQUESTS' env variable to be integer")
}

const SERVER: &str = "http://localhost:8080";

#[cfg(not(feature = "async"))]
fn main() {
    fn sync_connect_and_read() {
        let _body: String = ureq::get(SERVER).call().unwrap().into_string().unwrap();
    }

    let mut handles = Vec::new();
    for _ in 1..get_num_requests() {
        if cfg!(feature = "threads") {
            let handle = std::thread::spawn(sync_connect_and_read);
            handles.push(handle);
        } else {
            sync_connect_and_read();
        }
    }
    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}

#[cfg(feature = "async")]
#[cfg_attr(feature = "threads", tokio::main)]
#[cfg_attr(not(feature = "threads"), tokio::main(flavor = "current_thread"))]
async fn main() {
    use hyper::client::{connect::Connect, Client};
    use hyper::{body::HttpBody, Uri};

    async fn async_connect_and_read<C, B>(client: Client<C, B>)
    where
        C: Connect + Clone + Send + Sync + 'static,
        B: HttpBody + Send + 'static + std::default::Default,
        B::Data: Send,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let _future = client.get(Uri::from_static(SERVER)).await.unwrap();
    }

    let client = Client::new();

    let mut handles = Vec::new();
    for _ in 1..get_num_requests() {
        handles.push(tokio::spawn(async_connect_and_read(client.clone())));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
