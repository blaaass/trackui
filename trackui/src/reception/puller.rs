#![allow(dead_code)]
use zmq::{Socket};
use crate::object::{planelist, plane};
use yew::{html::ImplicitClone, prelude::*};
use crate::components::global::{Model, Msg};




pub fn listening(sock: &Socket, ctx: &yew::html::Scope<Model>) {
    loop {
        let flag = 0;
        let message = match sock.recv_string(flag).unwrap() {
            Ok(msg) => msg,
            _ => panic!("Erreur de réception"),
        };

        analysis(message, ctx);
    }
}

fn analysis(msg : String, ctx: &yew::html::Scope<Model>) {
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

    ctx.send_message(Msg::UpdatePlane(data));
}