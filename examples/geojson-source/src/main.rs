use futures::channel::oneshot;
use log::*;
use mapboxgl::{event, layer, Layer, LngLat, Map, MapEventListener, MapFactory, MapOptions};
use std::borrow::BorrowMut;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

struct Listener {
    tx: Option<oneshot::Sender<()>>,
}

impl MapEventListener for Listener {
    fn on_load(&mut self, map: &Map, _e: event::MapBaseEvent) {
        self.tx.take().unwrap().send(()).unwrap();

        use geojson::{Feature, GeoJson, Geometry, Value};

        map.add_geojson_source(
            "route",
            GeoJson::Feature(Feature {
                bbox: None,
                geometry: Some(Geometry::new(Value::LineString(vec![
                    vec![-122.483696, 37.833818],
                    vec![-122.483482, 37.833174],
                    vec![-122.483396, 37.8327],
                    vec![-122.483568, 37.832056],
                    vec![-122.48404, 37.831141],
                    vec![-122.48404, 37.830497],
                    vec![-122.483482, 37.82992],
                    vec![-122.483568, 37.829548],
                    vec![-122.48507, 37.829446],
                    vec![-122.4861, 37.828802],
                    vec![-122.486958, 37.82931],
                    vec![-122.487001, 37.830802],
                    vec![-122.487516, 37.831683],
                    vec![-122.488031, 37.832158],
                    vec![-122.488889, 37.832971],
                    vec![-122.489876, 37.832632],
                    vec![-122.490434, 37.832937],
                    vec![-122.49125, 37.832429],
                    vec![-122.491636, 37.832564],
                    vec![-122.492237, 37.833378],
                    vec![-122.493782, 37.833683],
                ]))),
                id: None,
                properties: None,
                foreign_members: None,
            }),
        )
        .unwrap();

        map.add_layer(&Layer {
            id: "route".into(),
            r#type: "line".into(),
            source: "route".into(),
            layout: Some(layer::Layout {
                line_join: "round".into(),
                line_cap: "round".into(),
            }),
            paint: Some(layer::Paint {
                line_color: "#888".into(),
                line_width: 8,
            }),
        })
        .unwrap();
    }
}

fn use_map() -> Rc<RefCell<Option<MapFactory>>> {
    let map = use_mut_ref(|| Option::<MapFactory>::None);

    {
        let mut map = map.clone();
        use_effect_with_deps(
            move |_| {
                let mut m = create_map();

                let (tx, rx) = oneshot::channel();
                m.set_listener(Listener { tx: Some(tx) });

                wasm_bindgen_futures::spawn_local(async move {
                    rx.await.unwrap();
                    info!("map loaded");
                    map.borrow_mut().replace(Some(m));
                });

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
        .center(LngLat::new(-122.486052, 37.830348))
        .zoom(15.0);

    mapboxgl::MapFactory::new(opts).unwrap()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
