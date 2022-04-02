#![allow(dead_code)]
#![allow(unused_variables)]
use super::planelist::Planelist;
use yew::{html::ImplicitClone, prelude::*};


#[derive(Clone, Debug)]
pub struct Plane {
    //definition of characteristic attributes of the plane
    //icao adress is stocked by the hasmap
    pub complement: String,                         //complementary information about the plane (from a database) [type string temporaire]
    pub callsign: String,                           //callsign of the flight  
    pub position: (f32,f32),                        //actual position of the plane (longitude, latitude)
    pub altitude: u32,                              //altitude of the plane
    pub speed: (f32, String, f32, String),                        //speed, track angle, vertical speed, speed type
    pub wake_vortex_cat: String,
    
    //past data
    pub position_history: Vec<(f32, f32, u32)>,     //historical of all past position
    pub speed_history: Vec<(f32,String, f32,String)>,   //historical of all past speed
}

impl ImplicitClone for Plane {}

//methods implementation
impl Plane {

    pub fn new() -> Self {
        let n = Self {
            complement: "".to_owned(),
            callsign: "".to_owned(),
            position: (0.,0.),
            altitude: (0),
            speed: (0.0, String::from("N/A"), 0.0, String::from("")),
            wake_vortex_cat: "".to_owned(),
            
            position_history: vec![],
            speed_history: vec![],
        };
        return n;
    }
}

impl Plane {
    pub fn update_position(&mut self, data: &Vec<String>) -> () {
        self.position = (data[2].parse::<f32>().unwrap(),data[3].parse::<f32>().unwrap())
    }

    pub fn update_speed(&mut self, data: &Vec<String>) {
        //TODO
    }
    
    pub fn update_callsign(&mut self, data: &Vec<String>) {
        //TODO
    }
    
    pub fn update_db(&mut self, data: &Vec<String>) {
        //TODO
    }
}