use leaflet::{LatLng, Map, TileLayer, Marker, Icon};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::{html::ImplicitClone, prelude::*};
use yew::{
    utils::document,
    web_sys::{Element, HtmlElement, Node},
    Html,
};

//-----------------------------------------------------
use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};
use crate::object::planelist::Planelist;
use crate::object::plane::Plane;

//set atenna postion [SHOULD BE AN OPTION AT THE INITIALISATION OF THE PRG]
const ANTENNA: (f64,f64) = (48.6339, 2.44417);


pub enum Msg {}

pub struct MapComponent {
    map: Map,
    selection: String,  //icao of the selected plane
    container: HtmlElement,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(pub f64, pub f64);




#[derive(Properties, Clone)]
pub struct Props {
    pub planes: Planelist,
    pub plane_selected: String, //icao of the selected plane
}


//map component ----------------------------------------------

impl MapComponent {
    fn render_map(&self) -> Html {
        let node: &Node = &self.container.clone().into();
        Html::VRef(node.clone())
    }
}

impl Component for MapComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(mut props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("map");
        let leaflet_map = Map::new_with_element(&container, &JsValue::NULL);
        let mut splane = props.planes.planelist.get_mut(&props.plane_selected).unwrap();
        let screen = Self {
            map: leaflet_map,
            container,
            selection: "".into(),
        };
        for aircraft in props.planes.planelist {
            screen.add_plane(aircraft.1);
        };
        screen
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.map.setView(&LatLng::new(ANTENNA.0, ANTENNA.1), 11.0);    //center the map on the antenna for the first render
            add_tile_layer(&self.map);
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.selection == props.plane_selected {
            false
        } else {
            self.selection = props.plane_selected;
            self.map.setView(&props.planes.get_pos_selection(&self.selection), 11.0);
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
            <img class="product_card_image" src="plane.png"/>
            <div class="map-container component-container">
                {self.render_map()}
            </div>
            </>
        }
    }
}

fn add_tile_layer(map: &Map) {
    TileLayer::new(
        "https://tiles.stadiamaps.com/tiles/alidade_smooth_dark/{z}/{x}/{y}{r}.png",
        &JsValue::NULL,
    )
    .addTo(map);
}


#[derive(Serialize, Deserialize)]
struct IconOptions {
    iconUrl: String,
    iconSize: [u16;2],
}



impl MapComponent {

    //need to add the trajectory of the selected plane
    pub fn add_plane(&self, plane: Plane) -> ShouldRender {
        //paramètrage du marqueur de l'avion
        let mark = Marker::new(&LatLng::new(plane.position.0.into(),plane.position.1.into()), &JsValue::NULL);
        let i = Icon::new( &JsValue::from_serde(&IconOptions {
            iconUrl: "ressources/plane.png".into(),
            iconSize: [50,50],
        })
        .expect("Unable to serialize icon options"),
    );
        mark.setIcon(&i);

        mark.addTo(&self.map);
        return true;
    }
}

