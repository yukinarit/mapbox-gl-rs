use futures::channel::oneshot;
use log::info;
use mapboxgl::marker::{MarkerEventListener, MarkerFactory};
use mapboxgl::{
    event, LngLat, Map, MapEventListener, MapFactory, MapOptions, Marker, MarkerOptions,
};
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

struct MarkerListener {}

struct Listener {
    tx: Option<oneshot::Sender<()>>,
    marker: Option<MarkerFactory>,
}

impl MapEventListener for Listener {
    fn on_load(&mut self, map: Rc<Map>, _e: event::MapBaseEvent) {
        self.tx.take().unwrap().send(()).unwrap();

        info!("map loaded");
        let mut marker_options = MarkerOptions::new();
        marker_options.draggable = Some(true);
        let marker = Marker::new(LngLat::new(0.0, 0.0), marker_options);
        let mut marker_factory = MarkerFactory::new(marker.into());
        marker_factory.set_listener(MarkerListener {});
        marker_factory.marker.add_to(&map);
        info!("add marker");
        self.marker.replace(marker_factory);
    }
}

impl MarkerEventListener for MarkerListener {
    fn on_drag(&mut self, _m: Rc<Marker>, _e: event::DragEvent) {
        info!("drag");
    }

    fn on_dragstart(&mut self, _m: Rc<Marker>, _e: event::MapBaseEvent) {
        info!("dragstart");
    }

    fn on_dragend(&mut self, _m: Rc<Marker>, _e: mapboxgl::event::DragEvent) {
        info!("dragend");
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
                m.set_listener(Listener {
                    tx: Some(tx),
                    marker: None,
                });

                wasm_bindgen_futures::spawn_local(async move {
                    rx.await.unwrap();
                    if let Ok(mut map) = map.try_borrow_mut() {
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
        <div id="map" style="width: 100vw; height: 100vh;"></div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
