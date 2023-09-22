use std::{future::Future, pin::Pin};
use hyper::{Client, client::HttpConnector, Request};
use tower::ServiceExt;


struct MyClient<'a>(&'a Client<HttpConnector>);

impl<'a> MyClient<'a> {
    async fn send(&mut self) {
        (&mut self.0).oneshot(Request::default()).await;
       // ^^^ Remove this line and foo() compiles
    }
}

fn foo() -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        let http_client = Client::new();
        let mut my_client = MyClient(&http_client);
        my_client.send().await;
    })
    // ^^^ error: implementation of `Service` is not general enough
    // note: `Service<hyper::Request<Body>>` would have to be implemented for the type `&'0 Client<HttpConnector>`, for any lifetime `'0`...
    // note: ...but `Service<hyper::Request<Body>>` is actually implemented for the type `&'1 Client<HttpConnector>`, for some specific lifetime `'1`
}