use axum::Router;

pub struct Server {
    pub address: String,
    pub router: axum::Router,
}

impl Server {
    pub fn new(a: &String, r: &Router) -> Self {
        Server {
            address: a.to_owned(),
            router: r.to_owned(),
        }
    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind(&self.address).await.unwrap();
        axum::serve(listener, self.router.into_make_service()).await.expect("failed to run server");
    }
}
