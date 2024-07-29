use futures::channel::oneshot;
use log::*;
use mapboxgl::{event, LngLat, Map, MapEventListener, MapOptions, QueryFeatureOptions};
use serde::Serialize;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

struct Listener {
    tx: Option<oneshot::Sender<()>>,
}

impl MapEventListener for Listener {
    fn on_load(&mut self, _map: Rc<Map>, _e: event::MapBaseEvent) {
        self.tx.take().unwrap().send(()).unwrap();
    }

    fn on_mousemove(&mut self, map: Rc<Map>, e: event::MapMouseEvent) {
        let features = map
            .query_rendered_features(
                Some(e.point),
                QueryFeatureOptions {
                    ..Default::default()
                },
            )
            .unwrap();

        #[derive(Serialize)]
        struct DisplayFeature {
            r#type: &'static str,
            id: Option<geojson::feature::Id>,
            properties: Option<geojson::JsonObject>,
        }

        let display_features: Vec<_> = features
            .into_iter()
            .map(|f| DisplayFeature {
                r#type: "Feature",
                id: f.id,
                properties: f.properties,
            })
            .collect();

        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("features")
            .expect("Element \"features\" not found")
            .set_inner_html(&serde_json::to_string_pretty(&display_features).unwrap())
    }
}

#[hook]
fn use_map() -> Rc<RefCell<Option<Rc<Map>>>> {
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
    let _map = use_map();

    html! {
      <div>
        <div id="map" style="width: 100vw; height: 100vh;"></div>
        <pre id="features"></pre>
      </div>
    }
}

pub fn create_map() -> Rc<Map> {
    let token = std::env!("MAPBOX_TOKEN");

    let opts = MapOptions::new(token.into(), "map".into())
        .center(LngLat::new(-97.33366638422012, 37.69990857165871))
        .style_ref("mapbox://styles/mapbox/streets-v12".into())
        .zoom(3.0);

    Map::new(opts).unwrap()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
