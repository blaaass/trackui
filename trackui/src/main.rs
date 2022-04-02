use crate::components::control::{Control};
use crate::components::map_component::{MapComponent, Point};
use yew::prelude::*;
use zmq::{Context, Message, Error};

use crate::object::plane::Plane;
use crate::object::planelist::Planelist;
use std::collections::hash_map::Entry::{Occupied, Vacant};
pub mod components;
pub mod reception;
pub mod object;




fn main() {

//YEW----------------------------------------------------------
    yew::initialize();
    let document = yew::utils::document();
    let app = document.query_selector("#yew").unwrap().unwrap();

    let a  = yew::App::<components::global::Model>::new().mount(app);

    yew::run_loop();

    //ZeroMQ--------------------------------------------------------
    let ctx = Context::new();
    let addr = "tcp://127.0.0.1:1234";
    let sock = ctx.socket(zmq::PULL).unwrap();
    sock.bind(addr).unwrap();


    reception::puller::listening(&sock, &a);
}
