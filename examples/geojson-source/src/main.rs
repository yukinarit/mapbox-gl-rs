use log::*;
use mapboxgl::{
    event, layer, LatLng, Layer, Map, MapEventListner, MapFactory, MapOptions, Popup, PopupOptions,
};
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
    fn on_load(&mut self, map: &Map, _e: event::MapBaseEvent) {
        use geojson::{Feature, GeoJson, Geometry, Value};

        map.add_geojson_source(
            "route",
            Feature {
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
            },
        );

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
        });
    }
}

impl Model {
    pub fn load_map(&mut self) {
        let token = std::option_env!("MAPBOX_TOKEN")
            .unwrap_or("pk.eyJ1IjoieXVraW5hcml0IiwiYSI6ImNsYTdncnVsZDBuYTgzdmxkanhqanZwdnoifQ.m3FLgX5Elx1fUIyyn7dZYg");
        let opts = MapOptions::new(token.into(), "map".into())
            .center(LatLng {
                lat: 37.830348,
                lng: -122.486052,
            })
            .zoom(15.0);
        let mut factory = mapboxgl::MapFactory::new(opts).unwrap();
        factory.set_listener(Listner {});
        self.map = Some(factory);
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
