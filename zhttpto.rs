//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;

use extra::uv;
use extra::{net_ip, net_tcp};
use std::{str, os, io, path};

static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";

static mut visitor_count: uint = 0;

fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{
    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());
                println(fmt!("Received connection from: %s", peer_addr));
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                    Ok(bytes) => unsafe {
                        let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));
                        println(fmt!("Request received:\n%s", request_str));
                        visitor_count += 1;

                        let mut ls = ~[];
                        for request_str.line_iter().advance | l | { ls.push(l); }
                        let first_line = ls[0];

                        let mut ws = ~[];
                        for first_line.split_iter(' ').advance | w | { ws.push(w); }
                        let mut full_path = ~"";
                        let mut root = false;
                        if ws.len() == 3 {
                            let getExists = ws[0]=="GET";
                            let httpExists = ws[2].starts_with("HTTP/1.1");
                            if (getExists && httpExists) {
                                println(os::getcwd().to_str() + ws[1]);
                                full_path = os::getcwd().to_str() + ws[1];
                                if ws[1] == "/" {
                                    println("at root");
                                    root = true;
                                }
                            }
                        }

                        let read_result = io::file_reader(~path::Path(full_path));
                        let mut alt_resp = ~"";
                        
                        if !root {
                            if read_result.is_ok() {
                                let file = read_result.unwrap();
                                let line_arr = file.read_lines();
                                println(line_arr.to_str());
                                
                                let mut i = 0;
                                let len_lines = line_arr.len();
                                println("len lines: " + len_lines.to_str());
                                while i<len_lines {
                                    println(i.to_str()+": "+line_arr[i]);
                                    alt_resp = alt_resp + line_arr[i] + "\r\n";
                                    i += 1;
                                }
                            }
                        }
                        
                        println("alt_resp: " + alt_resp);
                        
                        if alt_resp==~"" {
                            let response: ~str = ~
                                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                <doctype !html><html><head><title>Hello, Rust!</title>
                                <style>body { background-color: #111; color: #FFEEAA }
                                h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                                </style></head>
                                <body>
                                <h1>Greetings, Rusty!</h1><center>"
                                + visitor_count.to_str() +
                                "</center></body></html>\r\n";

                            net_tcp::write(&sock, response.as_bytes_with_null_consume());
                        } else {
                            let response: ~str = ~
                                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n"
                                + alt_resp;
                            net_tcp::write(&sock, response.as_bytes_with_null_consume());
                        }
                    },
                };
            }
        }
    };
}

fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}
