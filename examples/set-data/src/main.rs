use anyhow::Context;
use futures::channel::oneshot;
use gloo::timers::future::TimeoutFuture;
use log::*;
use mapboxgl::layer::{LineCap, LineJoin, LineLayer};
use mapboxgl::{event, LngLat, Map, MapEventListener, MapOptions};
use std::{cell::RefCell, ops::Deref, rc::Rc};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref, use_state, UseStateHandle};

type IntervalState = UseStateHandle<usize>;

type RouteState = UseStateHandle<Option<geojson::FeatureCollection>>;

type MapRef = Rc<RefCell<Option<Rc<Map>>>>;

/// Custom hook to update state on every interval.
#[hook]
fn use_interval(milli: u32) -> IntervalState {
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
fn use_map() -> MapRef {
    let map = use_mut_ref(|| Option::<Rc<Map>>::None);

    {
        let map = map.clone();
        use_effect_with_deps(
            move |_| {
                let m = create_map();

                let (tx, rx) = oneshot::channel();
                let _ = m.on(Listener { tx: Some(tx) }).unwrap();

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
                if let Err(e) = update(map, route, second) {
                    warn!("{e:#?}");
                }
                || ()
            },
            (route.clone(), second),
        );
    }

    {
        use_effect_with_deps(
            |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let json = fetch().await.unwrap();
                    info!("route geojson was loaded: {json:?}");
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
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
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

    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    Ok(serde_wasm_bindgen::from_value(json).unwrap())
}

struct Listener {
    tx: Option<oneshot::Sender<()>>,
}

impl MapEventListener for Listener {
    fn on_load(&mut self, _map: Rc<Map>, _e: event::MapBaseEvent) {
        self.tx.take().unwrap().send(()).unwrap();
    }
}

pub fn create_map() -> Rc<Map> {
    let token = std::env::var("MAPBOX_TOKEN").unwrap_or_else(|_| "your_token_here".to_string());

    let opts = MapOptions::new(token, "map".into())
        .center(LngLat::new(-122.019807, 45.632433))
        .zoom(15.0);
    mapboxgl::Map::new(opts).unwrap()
}

fn update(map: MapRef, route: &RouteState, second: &IntervalState) -> anyhow::Result<()> {
    let json = route.as_ref().context("Route is not loaded yet")?;
    let borrowed = map.borrow();
    let map = borrowed.as_ref().context("Map is not loaded yet")?;

    if let Some(source) = map.get_geojson_source("trace") {
        // Subsequent update
        let path = subpath(json, *second.deref()).context("Invalid path")?;
        if let geojson::Value::LineString(coordinates) =
            &path.features[0].geometry.as_ref().unwrap().value
        {
            let latlng = LngLat::new(
                coordinates.last().unwrap()[0],
                coordinates.last().unwrap()[1],
            );
            info!("latlng = {latlng:?}");
            update_data(source, path);
            map.pan_to(latlng);
        }
    } else {
        // First update
        let path = subpath(json, *second.deref()).unwrap();
        add_data(map, path).unwrap();
    }

    Ok(())
}

fn add_data(map: &Map, json: geojson::FeatureCollection) -> anyhow::Result<()> {
    map.add_geojson_source("trace", geojson::GeoJson::FeatureCollection(json))?;

    let mut ll = LineLayer::new("trace", "trace");
    ll.paint.line_color = Some("yellow".into());
    ll.paint.line_width = Some(8.0.into());
    ll.layout.line_join = Some(LineJoin::Round.into());
    ll.layout.line_cap = Some(LineCap::Round.into());

    map.add_layer(ll, None)?;
    Ok(())
}

fn update_data(mut source: mapboxgl::GeoJsonSource, json: geojson::FeatureCollection) {
    if let Err(e) = source.set_data(geojson::GeoJson::FeatureCollection(json)) {
        error!("Failed to update data: {e:?}");
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
