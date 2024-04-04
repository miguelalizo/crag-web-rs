use std::net::ToSocketAddrs;

mod threadpool;
mod server;

fn main() -> std::io::Result<()> {
    // validate addr
    let addr = "127.0.0.1:8010";
    let socket_addr = match addr.to_socket_addrs() {
        Ok(addr_iter) => addr_iter,
        Err(_) => panic!("could not resolve socket address")
    }
        .next()
        .unwrap();

    // Create server
    let pool_size = 4;
    let srvr = server::Server::new(socket_addr, pool_size)
        .expect("Unable to create Server");
   
    // run Server 
    srvr.run();

    Ok(())

}

