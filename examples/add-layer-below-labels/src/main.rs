use leptos::{logging::log, *};
use mapboxgl::FillLayer;
use mapboxgl::{LngLat, Map, MapOptions};
use std::rc::Rc;

pub fn main() {
    leptos::mount_to_body(|| view! { <MapComponent/> });
}

struct MapListener {}

impl mapboxgl::MapEventListener for MapListener {
    fn on_load(&mut self, map: Rc<mapboxgl::Map>, _e: mapboxgl::event::MapBaseEvent) {
        log!("Running on load!");
        map.add_geojson_source_from_url(
            "urban-areas",
            "https://docs.mapbox.com/mapbox-gl-js/assets/ne_50m_urban_areas.geojson",
        )
        .unwrap();

        let mut first_symbol_layer: Option<mapboxgl::layer::SymbolLayer> = None;

        for layer in map.get_style().layers {
            if let mapboxgl::layer::Layer::Symbol(l) = layer {
                first_symbol_layer = Some(l);
                break;
            }
        }

        let mut urban_areas_fill = FillLayer::new("urban-areas-fill", "urban-areas");
        urban_areas_fill.paint.fill_color = Some("#f08".into());
        urban_areas_fill.paint.fill_opacity = Some(0.4.into());

        map.add_layer(urban_areas_fill, first_symbol_layer.map(|l| l.id))
            .unwrap();
    }
}

#[component]
fn MapComponent() -> impl IntoView {
    let map_store = create_rw_signal(None);
    let map_ref = create_node_ref::<html::Div>();
    map_ref.on_load(move |m| {
        let _map_el = m.on_mount(move |map| {
            let token = std::env::var("MAPBOX_TOKEN").unwrap_or_else(|_| "your_token_here".to_string());
            let opts = MapOptions::new(token, map.get_attribute("id").unwrap())
                .center(LngLat::new(-88.137343, 35.137451))
                .zoom(5.0)
                .style_ref("mapbox://styles/mapbox/standard".into());
            let map = Map::new(opts).unwrap();
            map.on(MapListener {}).unwrap();
            map_store.set(Some(map));
        });
    });

    // 'top' slot is meant for use with symbols, see: https://docs.mapbox.com/mapbox-gl-js/example/geojson-layer-in-slot/
    let values = ["bottom", "middle"];
    view! {
        <>
            <div id="map" style="position: absolute; top: 0; bottom: 0; width: 100%;" node_ref=map_ref/>
            <div id="menu" style="position: absolute; background: #efefef; padding: 10px; font-family: 'Open Sans', sans-serif;">
                {values.into_iter()
                    .map(|n| view! {
                        <label for=n>"move layer to: "{n}</label>
                        <input id=n type="radio" name="rtoggle" value=n on:change={move |_e| map_store.get_untracked().unwrap().set_slot("urban-areas-fill", n).unwrap()} />
                    })
                    .collect::<Vec<_>>()}
            </div>
        </>
    }
}
