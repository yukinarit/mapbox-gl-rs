use futures::channel::oneshot;
use gloo::timers::future::TimeoutFuture;
use log::*;
use mapboxgl::{event, layer, Layer, LngLat, Map, MapEventListener, MapFactory, MapOptions};
use std::{cell::RefCell, ops::Deref, rc::Rc};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref, use_state, UseStateHandle};

/// Custom hook to update state on every interval.
#[hook]
fn use_interval(milli: u32) -> UseStateHandle<usize> {
    let second = use_state(|| 0usize);
    {
        let second = second.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let mut sec = 0;
                    loop {
                        TimeoutFuture::new(milli).await;
                        sec += 1;
                        second.set(sec);
                    }
                });
                || ()
            },
            (),
        );
    }

    second
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

                wasm_bindgen_futures::spawn_local(async move {
                    rx.await.unwrap();
                    info!("map loaded");
                    if let Ok(mut map) = map.try_borrow_mut() {
                        map.replace(m);
                    } else {
                        error!("Failed to create Map");
                    }
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
    let route = use_state(|| Option::<geojson::FeatureCollection>::None);
    let map = use_map();

    let second = use_interval(10);

    {
        use_effect_with_deps(
            move |(route, second)| {
                if let (Some(json), Some(map)) = (route.deref(), map.borrow().as_ref()) {
                    if let Some(source) = map.map.get_geojson_source("trace") {
                        if let Some(path) = subpath(json, *second.deref()) {
                            if let geojson::Value::LineString(coordinates) =
                                &path.features[0].geometry.as_ref().unwrap().value
                            {
                                let latlng = LngLat::new(
                                    coordinates.last().unwrap()[0],
                                    coordinates.last().unwrap()[1],
                                );
                                info!("latlng = {:?}", latlng);
                                update_data(source, path);
                                map.map.pan_to(latlng);
                            }
                        }
                    } else {
                        let path = subpath(json, *second.deref()).unwrap();
                        add_data(map, path).unwrap();
                    }
                }
                || ()
            },
            (route.clone(), second),
        );
    }

    {
        let route = route.clone();
        use_effect_with_deps(
            |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let json = fetch().await.unwrap();
                    info!("route geojson was loaded: {:?}", json);
                    route.set(Some(json));
                });
                || ()
            },
            (),
        );
    }

    html! {
      <div id="map" style="width: 100vw; height: 100vh;"></div>
    }
}

async fn fetch() -> anyhow::Result<geojson::FeatureCollection> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let req = Request::new_with_str_and_init(
        "https://docs.mapbox.com/mapbox-gl-js/assets/hike.geojson",
        &opts,
    )
    .unwrap();
    let window = web_sys::window().unwrap();
    let resp: Response = JsFuture::from(window.fetch_with_request(&req))
        .await
        .unwrap()
        .dyn_into()
        .unwrap();

    web_sys::console::log_1(&resp);

    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    Ok(serde_wasm_bindgen::from_value(json).unwrap())
}

struct Listener {
    tx: Option<oneshot::Sender<()>>,
}

impl MapEventListener for Listener {
    fn on_load(&mut self, _map: &Map, _e: event::MapBaseEvent) {
        self.tx.take().unwrap().send(()).unwrap();
    }
}

pub fn create_map() -> MapFactory {
    let token = std::env!("MAPBOX_TOKEN");

    let opts = MapOptions::new(token.into(), "map".into())
        .center(LngLat::new(-122.019807, 45.632433))
        .zoom(15.0);
    mapboxgl::MapFactory::new(opts).unwrap()
}

fn add_data(f: &MapFactory, json: geojson::FeatureCollection) -> anyhow::Result<()> {
    f.map
        .add_geojson_source("trace", geojson::GeoJson::FeatureCollection(json))?;

    f.map.add_layer(&Layer {
        id: "trace".into(),
        r#type: "line".into(),
        source: "trace".into(),
        layout: Some(layer::Layout {
            line_join: "round".into(),
            line_cap: "round".into(),
        }),
        paint: Some(layer::Paint {
            line_color: "yellow".into(),
            line_width: 8,
        }),
    })?;

    Ok(())
}

fn update_data(mut source: mapboxgl::GeoJsonSource, json: geojson::FeatureCollection) {
    if let Err(e) = source.set_data(geojson::GeoJson::FeatureCollection(json)) {
        error!("Failed to update data: {:?}", e);
    }
}

fn subpath(json: &geojson::FeatureCollection, n: usize) -> Option<geojson::FeatureCollection> {
    use geojson::{Geometry, Value};

    let mut path = json.clone();
    if let Value::LineString(coordinates) = &json.features[0].geometry.as_ref().unwrap().value {
        if n >= coordinates.len() {
            return None;
        }

        path.features[0].geometry = Some(Geometry::new(geojson::Value::LineString(
            coordinates[0..n].into(),
        )));

        Some(path)
    } else {
        None
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
