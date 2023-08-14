
use std::error::Error;

use sockets_api::server::mux;


fn main() -> Result<(), Box<dyn Error>> {
    let mut server = mux::IoMuxServer::new();
    server.launch();
    Ok(())
}
