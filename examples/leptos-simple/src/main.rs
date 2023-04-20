use leptos::*;
use mapboxgl::{LngLat, MapFactory, MapOptions};

fn main() {
    let token = std::env!("MAPBOX_TOKEN");
    mount_to_body(|cx| view! { cx,  <Map/> });
    let _map = MapFactory::new(
        MapOptions::new(token.into(), "map".into())
            .center(LngLat::new(139.7647863, 35.6812373))
            .zoom(15.0),
    )
    .unwrap();
}

#[component]
fn Map(cx: Scope) -> impl IntoView {
    view! {cx, <div id="map" style="width: 100%; height: 100vh"/>}
}
