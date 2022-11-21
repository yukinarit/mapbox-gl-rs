use futures::channel::oneshot;
use log::*;
use mapboxgl::{event, LngLat, Map, MapEventListner, MapFactory, MapOptions, Popup, PopupOptions};
use std::borrow::BorrowMut;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

struct Listner {
    tx: Option<oneshot::Sender<()>>,
}

impl MapEventListner for Listner {
    fn on_load(&mut self, _map: &Map, _e: event::MapBaseEvent) {
        self.tx.take().unwrap().send(()).unwrap();
    }

    fn on_click(&mut self, map: &Map, e: event::MapMouseEvent) {
        let latlng = LngLat::new(e.lng_lat.lng, e.lng_lat.lat);

        let popup = Popup::new(latlng, PopupOptions::new());
        popup.set_html("<h1>Hello</h1>");
        popup.add_to(map);
        debug!("clicked {:?}", e);
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
                m.set_listener(Listner { tx: Some(tx) });

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
        .center(LngLat::new(139.7647863, 35.6812373))
        .zoom(15.0);

    mapboxgl::MapFactory::new(opts).unwrap()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
