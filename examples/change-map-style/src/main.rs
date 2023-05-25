use futures::channel::oneshot;
use log::info;
use mapboxgl::{event, LngLat, Map, MapEventListener, MapFactory, MapOptions, StyleOptions};
use std::{cell::RefCell, rc::Rc};
use web_sys::HtmlInputElement;
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
        .center(LngLat::new(-2.81361, 36.77271))
        .zoom(13.0)
        .style("mapbox://styles/mapbox/satellite-streets-v12".into());
    mapboxgl::MapFactory::new(opts).unwrap()
}

#[function_component(App)]
fn app() -> Html {
    let map = use_map();
    let on_click = {
        Callback::from(move |e: Event| {
            let value = e.target_dyn_into::<HtmlInputElement>().unwrap().value();
            let style = format!("mapbox://styles/mapbox/{}", value);
            map.borrow_mut()
                .as_ref()
                .unwrap()
                .map
                .set_style(style, StyleOptions::new());
        })
    };

    html! {
        <>
            <div id="map" style="width: 100vw; height: 100vh;"></div>
            <div id="menu">
                <input id="satellite-streets-v12" type="radio" name="rtoggle" value="satellite-streets-v12" onchange={on_click.clone()} checked=true />
                <label for="satellite-streets-v12">{"satellite streets"}</label>
                <input id="light-v11" type="radio" name="rtoggle" value="light-v11" onchange={on_click.clone()} />
                <label for="light-v11">{"light"}</label>
                <input id="dark-v11" type="radio" name="rtoggle" value="dark-v11" onchange={on_click.clone()} />
                <label for="dark-v11">{"dark"}</label>
                <input id="streets-v12" type="radio" name="rtoggle" value="streets-v12" onchange={on_click.clone()} />
                <label for="streets-v12">{"streets"}</label>
                <input id="outdoors-v12" type="radio" name="rtoggle" value="outdoors-v12" onchange={on_click.clone()} />
                <label for="outdoors-v12">{"outdoors"}</label>
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
