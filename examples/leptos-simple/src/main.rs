use leptos::{html, prelude::*};

use mapboxgl::{LngLat, Map, MapOptions};

pub fn main() {
    leptos::mount::mount_to_body(|| view! { <MapComponent/> });
}

#[component]
fn MapComponent() -> impl IntoView {
    // Make a signal that can store the map inside the reactive system..
    // The map has to _live_ for it to be able track its handlers and stuff!
    let map_store = RwSignal::new_local(None);

    let map_ref = NodeRef::<html::Div>::new();
    map_ref.on_load(move |div_element| {
        let token = std::env::var("MAPBOX_TOKEN").unwrap_or_else(|_| "your_token_here".to_string());
        let map = Map::new(
            MapOptions::new(token, div_element.get_attribute("id").unwrap())
                .center(LngLat::new(10.7, 59.9))
                .zoom(12.0),
        )
        .unwrap();
        // Persist the map into the reactive system
        map_store.set(Some(map));
    });
    view! {<div id="map" style="position: absolute; top: 0; bottom: 0; width: 100%;" node_ref=map_ref/>}
}
