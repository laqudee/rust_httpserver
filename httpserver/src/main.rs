mod handler;
mod router;
mod server;

use server::Server;
fn main() {
    let serve = Server::new("localhost:3000");
    serve.run();
}
