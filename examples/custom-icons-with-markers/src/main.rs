use geojson::GeoJson;
use mapboxgl::{LngLat, MapFactory, MapOptions, Marker, MarkerOptions};
use std::str::FromStr;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

#[hook]
fn use_map(geojson: GeoJson) -> Rc<RefCell<Option<MapFactory>>> {
    let map = use_mut_ref(|| Option::<MapFactory>::None);

    {
        let _map = map.clone();
        use_effect_with_deps(
            move |_| {
                let m = create_map();

                // create a marker element for each feature
                let geo_value = geojson.to_json_value();
                let document = web_sys::window().unwrap().document().unwrap();
                if let Some(features) = geo_value["features"].as_array() {
                    for feature in features {
                        let element: HtmlElement = document
                            .create_element("div")
                            .unwrap()
                            .dyn_into::<HtmlElement>()
                            .unwrap();
                        element.set_class_name("marker");

                        // get properties
                        let properties = feature["properties"].clone();
                        let width = properties["iconSize"][0].as_f64().unwrap();
                        let heigth = properties["iconSize"][1].as_f64().unwrap();
                        element.set_attribute("style", &format!("background-image: url(https://placekitten.com/g/{}/{}/); width: {}px; height: {}px; background-size: 100%;", width, heigth, width, heigth)).unwrap();
                        let handler = wasm_bindgen::prelude::Closure::wrap(Box::new(move || {
                            let message = properties["message"].as_str().unwrap();
                            web_sys::window()
                                .unwrap()
                                .alert_with_message(message)
                                .unwrap();
                        })
                            as Box<dyn FnMut()>);
                        element
                            .add_event_listener_with_callback(
                                "click",
                                handler.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                        handler.forget();

                        let mut marker_options = MarkerOptions::new();
                        marker_options.element = Some(element);
                        // get geometry
                        let point = feature["geometry"]["coordinates"].as_array().unwrap();
                        let lnglat =
                            LngLat::new(point[0].as_f64().unwrap(), point[1].as_f64().unwrap());
                        let marker = Marker::new(lnglat, marker_options);
                        marker.add_to(&m.map);
                    }
                }
            },
            (),
        );
    }

    map
}

pub fn create_map() -> MapFactory {
    let token = std::env!("MAPBOX_TOKEN");

    let opts = MapOptions::new(token.into(), "map".into())
        .center(LngLat::new(-65.017, -16.457))
        .zoom(5.0)
        .style("mapbox://styles/mapbox/streets-v12".into());

    mapboxgl::MapFactory::new(opts).unwrap()
}

#[function_component(App)]
fn app() -> Html {
    let geojson_str = r#"{
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "properties": {
                    "message": "Foo",
                    "iconSize": [60, 60]
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [-66.324462, -16.024695]
                }
            },
            {
                "type": "Feature",
                "properties": {
                    "message": "Bar",
                    "iconSize": [50, 50]
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [-61.21582, -15.971891]
                }
            },
            {
                "type": "Feature",
                "properties": {
                    "message": "Baz",
                    "iconSize": [40, 40]
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [-63.292236, -18.281518]
                }
            }
        ]
    }"#;

    let geojson = GeoJson::from_str(geojson_str).unwrap();
    let _map = use_map(geojson);

    html! {
        <div id="map" style="width: 100vw; height: 100vh;"></div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
