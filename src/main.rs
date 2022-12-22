mod dashboard;
mod http;
mod timer;

#[tokio::main]
async fn main() {
    // Start the HTTP server
    tokio::spawn(async move {
        http::start().await;
    });

    // Start the GUI
    dashboard::start();
}
