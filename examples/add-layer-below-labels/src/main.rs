use leptos::*;
use mapboxgl::FillLayer;
use mapboxgl::{LngLat, Map, MapOptions};
use std::rc::Rc;

pub fn main() {
    leptos::mount_to_body(|| view! { <MapComponent/> });
}

struct MapListener {}

impl mapboxgl::MapEventListener for MapListener {
    fn on_load(&mut self, map: Rc<mapboxgl::Map>, _e: mapboxgl::event::MapBaseEvent) {
        map.add_geojson_source_from_url(
            "urban-areas",
            "https://docs.mapbox.com/mapbox-gl-js/assets/ne_50m_urban_areas.geojson",
        )
        .unwrap();

        let mut first_symbol_layer: Option<mapboxgl::layer::SymbolLayer> = None;

        if let Some(layers) = map.get_style().layers {
            for layer in layers {
                if let mapboxgl::layer::Layer::Symbol(l) = layer {
                    first_symbol_layer = Some(l);
                    break;
                }
            }
        }

        let mut urban_areas_fill = FillLayer::new("urban-areas-fill", "urban-areas");
        urban_areas_fill.paint.fill_color = Some("#f08".into());
        urban_areas_fill.paint.fill_opacity = Some(0.4.into());

        map.add_layer(urban_areas_fill, first_symbol_layer.map(|l| l.inner.id))
            .unwrap();
    }
}

#[component]
fn MapComponent() -> impl IntoView {
    let map_store = create_rw_signal(None);
    let map_ref = create_node_ref::<html::Div>();
    map_ref.on_load(move |m| {
        let _map_el = m.on_mount(move |map| {
            let token = std::env!("MAPBOX_TOKEN");
            let opts = MapOptions::new(token.into(), map.get_attribute("id").unwrap())
                .center(LngLat::new(-88.137343, 35.137451))
                .zoom(3.0)
                .style("mapbox://styles/mapbox/streets-v12".into());
            let map = Map::new(opts).unwrap();
            map.on(MapListener {}).unwrap();
            map_store.set(Some(map));
        });
    });
    view! {<div id="map" style="position: absolute; top: 0; bottom: 0; width: 100%;" node_ref=map_ref/>}
}
