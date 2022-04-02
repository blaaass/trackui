use super::plane::Plane;
use std::collections::HashMap;
use yew::{html::ImplicitClone, prelude::*};
use leaflet::LatLng;

#[derive(Clone)]
pub struct Planelist {
    pub planelist: HashMap<String,Plane>,
}

impl ImplicitClone for Planelist {}

impl Planelist {
    pub fn new()-> Self {
        Self {planelist: HashMap::new()}
    }

    pub fn addplane(&mut self, icao:String) {
        self.planelist.insert(icao, Plane::new());
    }

    pub fn get_pos_selection(&self, icao: &String) -> LatLng {
        let pos = &self.planelist.get(icao).unwrap().position;
        return LatLng::new(pos.0.into(), pos.1.into());
    }
}