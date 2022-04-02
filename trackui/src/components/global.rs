use crate::components::control::{Control};
use crate::components::map_component::{MapComponent, Point};
use yew::prelude::*;
use zmq::{Context, Message, Error};

use crate::object::plane::Plane;
use crate::object::planelist::Planelist;
use std::collections::hash_map::Entry::{Occupied, Vacant};


//set up of the main component which hold the other ones--------------------------------------
pub enum Msg {
    SelectPlane(Plane),
    UpdatePlane(Vec<String>),
}

pub struct Model {
    planes: Planelist,
    sicao: String,  //icao of the plane followed
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let planes = Planelist::new();
        let sicao = "".to_owned();
        Self {planes, sicao, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {

        let mut flag = false;

        match msg {
            
            //msg to select a plane to follow (from control component)
            Msg::SelectPlane(plane) => {
                
            }
            //msg to update the component
            Msg::UpdatePlane(data) => {
                let plane = match self.planes.planelist.entry(data[0].clone()) {
                    Vacant(entry) => entry.insert(Plane::new()),
                    Occupied(entry) => entry.into_mut(),
                };

                match data[1].as_str() {
                    "P" => plane.update_position(&data),
                    "S" => plane.update_speed(&data),
                    "C" => plane.update_callsign(&data),
                    "D" => plane.update_db(&data),
                    _ => (),
                };
                flag = true;
            }
        }
        flag
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    //view of the selected city
    fn view(&self) -> Html {
        let cb = self.link.callback(|name| Msg::SelectPlane(name));
        let update_cb = self.link.callback(|data| Msg::UpdatePlane(data));


        


        html! {
            <>
                <MapComponent planes=&self.planes plane_selected=self.sicao.clone()/>
                <Control select_plane=cb planes=&self.planes/>
            </>
        }
    }
}