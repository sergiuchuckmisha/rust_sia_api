//copypaste from geoglobula

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;

use hyper;
use hyper::{Method, Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

use errors::*;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    method: Method,
    path: String,
}

type Handler = fn(&Request) -> Result<Response>;
type RoutingTable = HashMap<Signature, Handler>;

pub struct API {
    // Routes and handlers
    routes: RoutingTable,
}

impl API
{
    // Constructor that specifies path prefix
    pub fn new() -> API {
        let routes = RoutingTable::new();
        API {
            routes: routes,
        }
    }

    // Add new route to interface and assign handler
    pub fn add_route(&mut self, _method: Method, _path: String, handler: Handler) -> Result<()> {
        let signature = Signature{ method: _method, path: _path };
        let route = &self.routes.entry(signature).or_insert( handler );
        
        Ok(())
    }

    // API middleware entrypoint 
    pub fn handle_request(&self, req: &Request) -> Result<Response> {
        let signature = Signature{ method: req.method().clone(), path: String::from(req.path().clone()) };
        //println!("{:?}", signature);
        let response = match self.routes.get( &signature ) {
            None => {
                Response::new()
                    .with_status(StatusCode::NotFound)
            },
            Some(handler) => {
                handler(req)?
            }
        };

        Ok(response)
    }
}

//Convenience macros
