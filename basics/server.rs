extern mod std;
use std::net::tcp;
use std::net::ip;
use std::uv;

fn main() {
    tcp::listen(ip::v4::parse_addr("127.0.0.1"), 8080, 100, uv::global_loop::get(),
            |_comm_chan|{
                error!("Server is listening");
            },
            |new_client, _comm_chan|{
              error!("New client");
             //task::spawn{ ||
                let result = tcp::accept(new_client);
                if result.is_ok(){
                    error!("Accepted!");
                    let socket = result::unwrap(move result);
                    error!("Unwrapped");
                    // Now do stuff with this socket
                    let data = socket.read(8080); // XXX: This blocks
                    io::println(fmt!("%?", data));
                }else{
                    error!("Not accepted!");
                }
              //}; 
            });
}

