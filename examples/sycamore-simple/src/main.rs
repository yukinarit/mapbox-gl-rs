use mapboxgl::{LngLat, Map, MapOptions};
use sycamore::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;

fn main() {
    window().unwrap().set_onload(Some(
        Closure::<dyn Fn()>::new(move || {
            let _map = Map::new(
                MapOptions::new(std::env!("MAPBOX_TOKEN").into(), "map".into())
                    .center(LngLat::new(-88.6867772119069, 41.989067254515696))
                    .zoom(12.0),
            )
            .unwrap();
        })
        .into_js_value()
        .as_ref()
        .unchecked_ref(),
    ));
    sycamore::render(|cx| {
        view! {
            cx,
            div(id="map", style="width: 100%; height: 100vh")
        }
    })
}
