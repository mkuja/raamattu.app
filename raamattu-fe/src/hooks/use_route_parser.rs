use log::warn;

use crate::Route;

pub fn use_route_parser(route: &str) -> Route {
    let s = route.to_string();
    let parts = s.split("/");
    let parts: Vec<&str> = parts.skip(3).take_while(|p| p.len() > 0).collect();
    warn!("{:?}", parts);
    Route::Root
}
