use std::io;
use std::net;
use std::env;
use std::io::{Read, Write};
use std::mem::drop;

use json_parser_simple;
use EMDisi_lib;
use EMUtil::{is_valid_ipv4_port , search_vec};



extern crate num_cpus;

#[derive(Debug)]
struct Peers {
    stream : net::TcpStream,
    head : String,
    tail : String,
}

static mut CLIENTS : Vec<Peers> = Vec::new();

static mut GLOBAL_VAR_LOCK : bool = false;

static mut NUM_THREADS : usize = 0;



fn start_chain_server(emchain_serv : &String) {
    if !is_valid_ipv4_port(&emchain_serv.trim().to_string()){
        println!("invalid ipv4:port");
        return;
    }
    let listener = net::TcpListener::bind(emchain_serv.trim().to_string()).expect("failed to start the server");
    println!("EMNetChain Server listening on {}" , emchain_serv);

    std::thread::spawn(||{
        matcher();
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                

                unsafe{
                    while NUM_THREADS >= num_cpus::get(){}
                    while GLOBAL_VAR_LOCK{}
                    GLOBAL_VAR_LOCK = true;
                    NUM_THREADS += 1;
                    GLOBAL_VAR_LOCK = false;
                }

                std::thread::spawn( move || {
                    handle_client(stream);
                });
            },
            Err(err) => {
                println!("Error accepting connection: {}", err);
            }
        }
    }


    drop(listener);
}

fn handle_client(mut stream: net::TcpStream) {
    

    let mut buffer = [0; 1024];
    while let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..size]);
        if let Some(index) = message.find('}') {
            let json_str = &message[..=index];
            let json_map = json_parser_simple::json_scan(json_str);
            if let Some(json_parser_simple::JsonValue::String(msg)) = json_map.get("msg") {
                if let Some(json_parser_simple::JsonValue::String(tail)) = json_map.get("tail") {
                    if let Some(json_parser_simple::JsonValue::String(head)) = json_map.get("head") {
                        if msg == "CONNECT"{
                            
                            
                            let peer = Peers {
                                tail: tail.clone(),
                               head: head.clone(),
                                stream: stream.try_clone().expect("Failed to clone stream"),
                            };
                            
                            unsafe{
                                while GLOBAL_VAR_LOCK{}
                                GLOBAL_VAR_LOCK = true;
                                CLIENTS.push(peer);
                                
                                GLOBAL_VAR_LOCK = false;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    unsafe{
        while GLOBAL_VAR_LOCK{}
        GLOBAL_VAR_LOCK = true;
        NUM_THREADS -= 1;
        GLOBAL_VAR_LOCK = false;
    }
}

fn matcher(){
    loop{
        unsafe{
            

            

            while CLIENTS.len() < 2 {}
            
            let mut response = format!(r#"{{"msg":"CONNECT","part":"{}","ip":"{}"}}"#, "head" , CLIENTS[1].tail);
            if let Err(e) = CLIENTS[0].stream.write_all(response.as_bytes()) {
                println!("Failed to write to stream: {}", e);
            }
            response = format!(r#"{{"msg":"CONNECT","part":"{}","ip":"{}"}}"#, "tail" , CLIENTS[0].head);
            if let Err(e) = CLIENTS[1].stream.write_all(response.as_bytes()) {
                println!("Failed to write to stream: {}", e);
            }
            
            while GLOBAL_VAR_LOCK{}
            GLOBAL_VAR_LOCK = true;
            //drop(CLIENTS[0].stream.try_clone().unwrap());
            CLIENTS.remove(0);
            GLOBAL_VAR_LOCK = false;
        }
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut emdisi_serv = String::new();
    let mut emchain_serv = String::new();
    if args.len() <= 1{
        
        println!("insert ipv4:port for emdisi server :");
        io::stdin().read_line(&mut emdisi_serv).expect("Not a valid input");

        
        println!("insert ipv4:port for emnetchain server ");
        io::stdin().read_line(&mut emchain_serv).expect("Not a valid input");

        
    }else{
        let index_emdisi_serv = search_vec(&args , &String::from("-dis-addr"));
        let index_emchain_serv = search_vec(&args , &String::from("-chain-addr"));

        emdisi_serv = args[index_emdisi_serv as usize + 1].clone();
        emchain_serv = args[index_emchain_serv as usize + 1].clone();
    }

    
    std::thread::spawn(move ||{
        EMDisi_lib::on_dis(&emdisi_serv);
    });
    

    start_chain_server(&emchain_serv);   
}
