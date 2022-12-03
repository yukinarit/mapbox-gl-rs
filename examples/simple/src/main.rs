use mapboxgl::{event, LngLat, Map, MapEventListener, MapFactory, MapOptions};
use std::borrow::BorrowMut;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

fn use_map() -> Rc<RefCell<Option<MapFactory>>> {
    let map = use_mut_ref(|| Option::<MapFactory>::None);

    {
        let mut map = map.clone();
        use_effect_with_deps(
            move |_| {
                let mut m = create_map();
                || {}
            },
            (),
        );
    }

    map
}

#[function_component(App)]
fn app() -> Html {
    let _map = use_map();

    html! {
      <div id="map" style="width: 100vw; height: 100vh;"></div>
    }
}

pub fn create_map() -> MapFactory {
    let token = std::option_env!("MAPBOX_TOKEN")
        .unwrap_or("pk.eyJ1IjoieXVraW5hcml0IiwiYSI6ImNsYTdncnVsZDBuYTgzdmxkanhqanZwdnoifQ.m3FLgX5Elx1fUIyyn7dZYg");

    let opts = MapOptions::new(token.into(), "map".into())
        .center(LngLat::new(139.7647863, 35.6812373))
        .zoom(15.0);

    mapboxgl::MapFactory::new(opts).unwrap()
}

fn main() {
    yew::start_app::<App>();
}
