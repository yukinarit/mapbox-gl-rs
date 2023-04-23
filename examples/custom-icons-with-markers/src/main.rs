use futures::channel::oneshot;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use log::*;
use mapboxgl::{
    event, layer, ImageOptions, Layer, LngLat, Map, MapEventListener, MapFactory, MapOptions,
    Marker, MarkerOptions,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

#[hook]
fn use_map() -> Rc<RefCell<Option<MapFactory>>> {
    let map = use_mut_ref(|| Option::<MapFactory>::None);

    {
        let map = map.clone();
        use_effect_with_deps(
            move |_| {
                let mut m = create_map();

                // create a marker
                let document = web_sys::window().unwrap().document().unwrap();
                let element: HtmlElement = document
                    .create_element("div")
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .unwrap();
                element.set_class_name("marker");
                // todo
                let width = 60;
                let height = 60;
                element.set_attribute("style", &format!("background-image: url(https://placekitten.com/g/{}/{}/); width: {}px; height: {}px; background-size: 20%;", width, height, width, height)).unwrap();

                let mut marker_options = MarkerOptions::new();
                marker_options.element = Some(element);
                // todo
                marker_options.draggable = Some(true);
                marker_options.scale = Some(2);
                let lnglat = LngLat::new(-65.017, -16.457);
                let marker = Marker::new(lnglat, marker_options);
                marker.add_to(&m.map);
            },
            (),
        );
    }

    map
}

pub fn create_map() -> MapFactory {
    let token = std::env!("MAPBOX_TOKEN");

    let opts = MapOptions::new(token.into(), "map".into())
        .center(LngLat::new(-65.017, -16.457))
        .zoom(5.0)
        .style("mapbox://styles/mapbox/streets-v12".into());

    mapboxgl::MapFactory::new(opts).unwrap()
}

#[function_component(App)]
fn app() -> Html {
    let _map = use_map();

    html! {
        <div id="map" style="width: 100vw; height: 100vh;"></div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
