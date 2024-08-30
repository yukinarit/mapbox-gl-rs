use futures::channel::oneshot;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use log::*;
use mapboxgl::layer::SymbolLayer;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

use mapboxgl::{event, ImageOptions, LngLat, Map, MapEventListener, MapOptions};

struct Listener {
    tx: Option<oneshot::Sender<()>>,
}

impl MapEventListener for Listener {
    fn on_load(&mut self, map: Rc<Map>, _e: event::MapBaseEvent) {
        self.tx.take().unwrap().send(()).unwrap();

        let map2 = map.clone();
        map.load_image(
            "https://docs.mapbox.com/mapbox-gl-js/assets/cat.png",
            move |res| {
                if let Ok(image) = res.map_err(|e| warn!("{e}")) {
                    info!("image loaded");
                    web_sys::console::info_1(&image.inner);

                    map2.add_image("cat", image, ImageOptions::default())
                        .unwrap();
                    map2.add_geojson_source(
                        "point",
                        GeoJson::FeatureCollection(FeatureCollection {
                            bbox: None,
                            foreign_members: None,
                            features: vec![Feature {
                                bbox: None,
                                geometry: Some(Geometry::new(Value::Point(vec![
                                    -77.4144, 25.0759,
                                ]))),
                                id: None,
                                properties: None,
                                foreign_members: None,
                            }],
                        }),
                    )
                    .unwrap();

                    let mut sl = SymbolLayer::new("points", "point");
                    sl.layout.icon_image = Some("cat".into());
                    sl.layout.icon_size = Some(0.25.into());
                    map2.add_layer(sl, None).unwrap();
                }
            },
        );
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
      <div id="map" style="width: 100vw; height: 100vh;"></div>
    }
}

pub fn create_map() -> Rc<Map> {
    let token = std::env!("MAPBOX_TOKEN");

    let opts = MapOptions::new(token.into(), "map".into())
        .center(LngLat::new(-77.432, 25.0306))
        .zoom(10.0)
        .style_ref("mapbox://styles/mapbox/dark-v11".into());

    Map::new(opts).unwrap()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
