use futures::channel::oneshot;
use js_sys::Math;
use log::*;
use mapboxgl::{event, AnimationOptions, CameraOptions, LngLat, Map, MapEventListener, MapOptions};
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
    let map = use_map();

    html! {
      <div>
        <div id="map" style="width: 100vw; height: 100vh;"></div>
        <button id="fly" onclick={Callback::from(move |_| {
            (*map.try_borrow().unwrap()).as_ref().unwrap().fly_to(CameraOptions {
                center: LngLat::new((Math::random()-0.5)*360.0, (Math::random()-0.5)*100.0),
                ..Default::default()
            }, AnimationOptions{essential: Some(true),..Default::default()});
        })}>{ "Fly" }</button>
      <br/>
      </div>
    }
}

pub fn create_map() -> Rc<Map> {
    let token = std::env::var("MAPBOX_TOKEN").unwrap_or_else(|_| "your_token_here".to_string());

    let opts = MapOptions::new(token, "map".into())
        .style_ref("mapbox://styles/mapbox/streets-v12".into())
        .center(LngLat::new(-74.5, 40.0))
        .zoom(4.0);

    Map::new(opts).unwrap()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
