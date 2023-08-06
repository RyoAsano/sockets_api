
use std::error::Error;

use sockets_api::server::mux;
use sockets_api::server::mux::IoMuxServerSvc;

fn main() -> Result<(), Box<dyn Error>> {
    let mut server = mux::IoMuxServer::new();
    server.listen();
    server.launch()?;
    Ok(())
}
