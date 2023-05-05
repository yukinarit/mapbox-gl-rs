use futures::channel::oneshot;
use log::info;
use mapboxgl::marker::{MarkerBundle, MarkerEventListener};
use mapboxgl::{
    event, LngLat, Map, MapEventListener, MapFactory, MapOptions, Marker, MarkerOptions,
};
use std::{cell::RefCell, rc::Rc};
use web_sys::{Document, Element};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

struct MarkerListener {}

struct Listener {
    tx: Option<oneshot::Sender<()>>,
}

impl MapEventListener for Listener {
    fn on_load(&mut self, _map: Rc<Map>, _e: event::MapBaseEvent) {
        self.tx.take().unwrap().send(()).unwrap();
    }
}

impl MarkerEventListener for MarkerListener {
    fn on_dragend(&mut self, m: Rc<Marker>, _e: mapboxgl::event::DragEvent) {
        let document: Document = web_sys::window().unwrap().document().unwrap();
        let coordinates: Element = document.get_element_by_id("coordinates").unwrap();
        coordinates
            .set_attribute("style", "display: block;")
            .unwrap();
        let lnglat = m.get_lnglat();
        coordinates.set_inner_html(&format!(
            "Longitude: {}<br/>Latitude: {}",
            lnglat.lng(),
            lnglat.lat()
        ));
    }
}

#[hook]
fn use_map() -> Rc<RefCell<Option<MapFactory>>> {
    let map = use_mut_ref(|| Option::<MapFactory>::None);

    {
        let map = map.clone();
        use_effect_with_deps(
            move |_| {
                let mut m = create_map();
                let (tx, rx) = oneshot::channel();
                m.set_listener(Listener { tx: Some(tx) });

                // add marker
                let mut marker_options = MarkerOptions::new();
                marker_options.draggable = Some(true);
                let marker = Marker::new(LngLat::new(0.0, 0.0), marker_options);
                let mut mb = MarkerBundle::new(marker.into());
                mb.set_listener(MarkerListener {});
                let _id = m.add_marker(mb);

                wasm_bindgen_futures::spawn_local(async move {
                    rx.await.unwrap();
                    if let Ok(mut map) = map.try_borrow_mut() {
                        info!("map loaded");
                        map.replace(m);
                    } else {
                        log::error!("Failed to create Map");
                    }
                });
                || {}
            },
            (),
        );
    }
    map
}

pub fn create_map() -> MapFactory {
    let token = std::env!("MAPBOX_TOKEN");

    let opts = MapOptions::new(token.into(), "map".into())
        .center(LngLat::new(0.0, 0.0))
        .zoom(2.0)
        .style("mapbox://styles/mapbox/streets-v12".into());

    mapboxgl::MapFactory::new(opts).unwrap()
}

#[function_component(App)]
fn app() -> Html {
    let _map = use_map();
    html! {
        <>
            <div id="map" style="width: 100vw; height: 100vh;"></div>
            <pre id="coordinates" class="coordinates"></pre>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
