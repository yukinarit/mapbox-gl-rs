use leptos::{logging::log, *};
use mapboxgl::{CustomLayer, LngLat, Map, MapOptions};
use std::rc::Rc;

use std::f64::consts::PI as PIf64;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram, WebGlShader};

pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <MapComponent/> });
}

#[derive(Clone, Debug, Default)]
struct Listener {
    cl: CustomLayer,
}

impl mapboxgl::MapEventListener for Listener {
    fn on_load(&mut self, map: Rc<mapboxgl::Map>, e: mapboxgl::event::MapBaseEvent) {
        log!("loaaaaddededededde {:?} ", e);
        map.add_layer(self.cl.clone(), None).unwrap();
    }
}

#[component]
fn MapComponent() -> impl IntoView {
    let triangle = vec![[25.004, 60.239], [13.403, 52.562], [30.498, 50.541]];
    let vertex_data: Vec<f32> = triangle
        .iter()
        .flat_map(|p| {
            let merc_pos = lonlat_to_mercator(p[0], p[1]);
            [merc_pos.0 as f32, merc_pos.1 as f32]
        })
        .collect();
    let gl_data: RwSignal<Option<(WebGlProgram, WebGlBuffer)>> = create_rw_signal(None);

    let map_store = create_rw_signal(None);
    let map_ref = create_node_ref::<html::Div>();
    map_ref.on_load(move |m| {
        let _map_el = m.on_mount(move |map| {
            let token = std::env!("MAPBOX_TOKEN");
            let map = Map::new(
                MapOptions::new(token.into(), map.get_attribute("id").unwrap())
                    .center(LngLat::new(19.7, 59.9))
                    .zoom(4.0),
            )
            .unwrap();

            let on_add = move |_map, raw_gl: JsValue| {
                let gl: GL = raw_gl.dyn_into().unwrap();

                let vertex_source = "
                uniform mat4 u_matrix;
                attribute vec2 a_pos;
                void main() {
                    gl_Position = u_matrix * vec4(a_pos, 0.0, 1.0);
                }";

                let fragment_source = "
                void main() {
                    gl_FragColor = vec4(1.0, 0.0, 0.0, 0.5);
                }";

                let main_program = create_program(&gl, vertex_source, fragment_source).unwrap();
                gl.use_program(Some(&main_program));
                let vertex_buffer = gl.create_buffer().unwrap();

                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                gl_data.set(Some((main_program, vertex_buffer)));

                gl.buffer_data_with_array_buffer_view(
                    GL::ARRAY_BUFFER,
                    &web_sys::js_sys::Float32Array::from(vertex_data.as_slice()),
                    GL::STATIC_DRAW,
                );
            };
            let render = move |raw_gl: JsValue, matrix: JsValue| {
                let gl: GL = raw_gl.dyn_into().unwrap();
                if let Some((main_program, main_buffer)) = gl_data.get_untracked() {
                    gl.use_program(Some(&main_program));
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&main_buffer));
                    let pos_atrib_loc = gl.get_attrib_location(&main_program, "a_pos") as u32;
                    gl.enable_vertex_attrib_array(pos_atrib_loc);
                    gl.vertex_attrib_pointer_with_i32(pos_atrib_loc, 2, GL::FLOAT, false, 0, 0);

                    let mut mat: [f64; 16] = [0.0; 16];
                    web_sys::js_sys::Float64Array::from(matrix).copy_to(&mut mat);
                    let f32_mat: Vec<f32> = mat.iter().map(|v| *v as f32).collect();

                    gl.uniform_matrix4fv_with_f32_array(
                        gl.get_uniform_location(&main_program, "u_matrix").as_ref(),
                        false,
                        f32_mat.as_slice(),
                    );
                    gl.draw_arrays(GL::TRIANGLES, 0, 3);
                }
            };
            let mut cl = CustomLayer::new("triangle", "", render);
            cl.set_on_add(on_add);
            map.on(Listener { cl }).unwrap();
            map_store.set(Some(map));
        });
    });
    view! {<div id="map" style="position: absolute; top: 0; bottom: 0; width: 100%;" node_ref=map_ref/>}
}

fn create_program(gl: &GL, vert_shader: &str, frag_shader: &str) -> Result<WebGlProgram, String> {
    let vert_shader = compile_shader(gl, GL::VERTEX_SHADER, &vert_shader)?;
    let frag_shader = compile_shader(gl, GL::FRAGMENT_SHADER, &frag_shader)?;
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);
    if !gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))?;
    }
    Ok(program)
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    match gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        true => Ok(shader),
        false => Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"))),
    }
}

fn lonlat_to_mercator(longitude: f64, latitude: f64) -> (f64, f64) {
    let lat_rad = latitude * PIf64 / 180.0;
    let merc_n = (PIf64 / 4.0 + lat_rad / 2.0).tan().ln();
    let y = 0.5 - merc_n / (2.0 * PIf64);
    ((longitude + 180.0) / 360.0, y)
}
