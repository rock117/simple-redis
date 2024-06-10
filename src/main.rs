mod command;
mod datatype;
mod error;
mod executor;
mod resp;
#[tokio::main]
async fn main() {
    println!("redis server started");
}
