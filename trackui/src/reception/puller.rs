#![allow(dead_code)]
use zmq::{Socket};

pub fn listening(sock: &Socket) {
    loop {
        let flag = 0;
        let message = match sock.recv_string(flag).unwrap() {
            Ok(msg) => msg,
            _ => panic!("Erreur de réception"),
        };

        analysis(message);
    }
}

fn analysis(msg : String) {
    let mut buffer: String = "".into();
    let mut data: Vec<String> = vec![];

    //cuting the differents parts of the module
    for i in 0..msg.len() {
        if &msg[i..i+1]=="|" {
            data.push(buffer);
            buffer = "".into();
        }
        else {
            buffer.push_str(&msg[i..i+1]);
        }
    };

    match data[0].as_str() {
        "P" => update_position(&data),
        "S" => update_speed(&data),
        "C" => update_callsign(&data),
        "D" => update_db(&data),
        _ => (),
    };
}

fn update_position(data: &Vec<String>) {

}

fn update_speed(data: &Vec<String>) {
    
}

fn update_callsign(data: &Vec<String>) {
    
}

fn update_db(data: &Vec<String>) {
    
}