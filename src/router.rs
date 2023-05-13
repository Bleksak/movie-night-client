use std::collections::HashMap;
use std::hash::Hash;

use crate::route::{self, Route};

type RouteCtor = fn() -> Box<dyn Route>;

#[derive(Default)]
pub struct Router<RouteKey = route::RouteKey> {
    routes: HashMap<RouteKey, (RouteCtor, Box<dyn Route>)>,
    active: Option<RouteKey>
}

impl<RouteKey: Hash + Eq + PartialEq> Router<RouteKey> {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            active: None,
        }
    }
    
    pub fn navigate(&mut self, key: &RouteKey) {
        
    }
    
    pub fn add(mut self, name: RouteKey, handler: RouteCtor) -> Self {
        self.routes.insert(name, (handler, handler()));
        self
    }
    
    pub fn get(&self, name: &RouteKey) -> &(RouteCtor, Box<dyn Route>) {
        self.routes.get(name).unwrap()
    }
    
    pub fn get_mut(&mut self, name: &RouteKey) -> &mut Box<dyn Route> {
        &mut self.routes.get_mut(name).unwrap().1
    }
}
