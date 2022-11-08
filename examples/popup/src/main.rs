use log::*;
use mapboxgl::{event, LatLng, Map, MapEventListner, MapFactory, MapOptions, Popup, PopupOptions};
use yew::prelude::*;

enum Msg {}

struct Model {
    map: Option<MapFactory>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { map: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div id="map" style="width: 100vw; height: 100vh;"></div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render && self.map.is_none() {
            self.load_map();
        }
    }
}

struct Listner {}

impl MapEventListner for Listner {
    fn on_click(&mut self, map: &Map, e: event::MapMouseEvent) {
        let latlng = LatLng {
            lat: e.lng_lat.lat,
            lng: e.lng_lat.lng,
        };

        let popup = Popup::new(latlng, PopupOptions::new());
        popup.set_html("<h1>Hello</h1>");
        popup.add_to(map);
        debug!("clicked {:?}", e);
    }
}

impl Model {
    pub fn load_map(&mut self) {
        let token = std::option_env!("MAPBOX_TOKEN")
            .unwrap_or("pk.eyJ1IjoieXVraW5hcml0IiwiYSI6ImNsYTdncnVsZDBuYTgzdmxkanhqanZwdnoifQ.m3FLgX5Elx1fUIyyn7dZYg");
        let opts = MapOptions::new(token.into(), "map".into())
            .center(LatLng {
                lat: 35.6812373,
                lng: 139.7647863,
            })
            .zoom(12.0);
        let mut factory = mapboxgl::MapFactory::new(opts).unwrap();
        factory.set_listener(Listner {});
        self.map = Some(factory);
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
