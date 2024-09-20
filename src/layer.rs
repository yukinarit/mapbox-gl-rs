use std::fmt;

use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsValue};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expression<T> {
    Enum(T),
    String(String),
    Number(f64),
    Bool(bool),
    List(Vec<Expression<T>>),
}

pub trait EnumMarker {}
impl<T: EnumMarker> From<T> for Expression<T> {
    fn from(item: T) -> Self {
        Expression::Enum(item)
    }
}
impl<T> From<&str> for Expression<T> {
    fn from(value: &str) -> Self {
        Expression::String(value.into())
    }
}

impl<T> From<String> for Expression<T> {
    fn from(value: String) -> Self {
        Expression::String(value)
    }
}

impl<T> From<f64> for Expression<T> {
    fn from(value: f64) -> Self {
        Expression::Number(value)
    }
}

impl<T> From<bool> for Expression<T> {
    fn from(value: bool) -> Self {
        Expression::Bool(value)
    }
}

impl<T> From<Vec<&str>> for Expression<T> {
    fn from(value: Vec<&str>) -> Self {
        Expression::List(value.into_iter().map(|s| s.into()).collect())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Visibility {
    #[default]
    Visible,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLayer {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Layer {
    Custom(CustomLayer),
    Background(BackgroundLayer),
    Fill(FillLayer),
    Line(LineLayer),
    Symbol(SymbolLayer),
    Raster(RasterLayer),
    RasterParticle(RasterParticleLayer),
    Circle(CircleLayer),
    FillExtrusion(FillExtrusionLayer),
    Heatmap(HeatmapLayer),
    Hillshade(HillshadeLayer),
    Sky(SkyLayer),
    Model(ModelLayer),
}

pub trait IntoLayer {
    fn into_layer(self) -> Layer;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_mode: Option<String>,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub on_add: js_sys::Function,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub on_remove: js_sys::Function,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub prerender: js_sys::Function,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub render: js_sys::Function,
}

impl IntoLayer for CustomLayer {
    fn into_layer(self) -> Layer {
        Layer::Custom(self)
    }
}

fn make_wasm_closure(some_fn: impl Fn(JsValue, JsValue) + 'static) -> js_sys::Function {
    Closure::<dyn Fn(JsValue, JsValue)>::new(some_fn)
        .into_js_value()
        .into()
}

impl CustomLayer {
    pub fn new<F>(id: impl Into<String>, source: impl Into<String>, render: F) -> CustomLayer
    where
        F: 'static + Fn(JsValue, JsValue),
    {
        let render = Closure::<dyn Fn(JsValue, JsValue)>::new(render);
        CustomLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            rendering_mode: None,
            // Use no-ops for the optional functions
            on_add: make_wasm_closure(|_1, _2| {}),
            on_remove: make_wasm_closure(|_1, _2| {}),
            prerender: make_wasm_closure(|_1, _2| {}),
            render: render.into_js_value().into(),
        }
    }
    pub fn set_on_add(&mut self, on_add_fn: impl Fn(JsValue, JsValue) + 'static) {
        self.on_add = make_wasm_closure(on_add_fn);
    }
    pub fn set_on_remove(&mut self, on_remove_fn: impl Fn(JsValue, JsValue) + 'static) {
        self.on_remove = make_wasm_closure(on_remove_fn);
    }
    pub fn set_render(&mut self, render_fn: impl Fn(JsValue, JsValue) + 'static) {
        self.render = make_wasm_closure(render_fn);
    }
    pub fn set_prerender(&mut self, prerender_fn: impl Fn(JsValue, JsValue) + 'static) {
        self.prerender = make_wasm_closure(prerender_fn);
    }
}

impl<T: fmt::Debug> fmt::Display for Expression<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::String(s) => write!(f, "\"{}\"", s),
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Bool(b) => write!(f, "{}", b),
            Expression::List(list) => {
                write!(f, "[")?;
                for (index, item) in list.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Expression::Enum(e) => write!(f, "{:?}", e),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundLayer {
    pub id: String,
    #[serde(default)]
    pub layout: BackgroundLayout,
    #[serde(default)]
    pub paint: BackgroundPaint,
}

impl IntoLayer for BackgroundLayer {
    fn into_layer(self) -> Layer {
        Layer::Background(self)
    }
}

impl BackgroundLayer {
    pub fn new(id: impl Into<String>) -> BackgroundLayer {
        BackgroundLayer {
            id: id.into(),
            layout: BackgroundLayout::default(),
            paint: BackgroundPaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct BackgroundPaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_emissive_strength: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_pattern: Option<Expression<()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct BackgroundLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(default)]
    pub layout: FillLayout,
    #[serde(default)]
    pub paint: FillPaint,
}

impl IntoLayer for FillLayer {
    fn into_layer(self) -> Layer {
        Layer::Fill(self)
    }
}

impl FillLayer {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> FillLayer {
        FillLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            layout: FillLayout::default(),
            paint: FillPaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum TranslateAnchor {
    #[default]
    Map,
    Viewport,
}
impl EnumMarker for TranslateAnchor {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct FillPaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_antialias: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_emissive_strength: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_outline_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_pattern: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_translate: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_translate_anchor: Option<Expression<TranslateAnchor>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct FillLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_sort_key: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(default)]
    pub layout: LineLayout,
    #[serde(default)]
    pub paint: LinePaint,
}

impl IntoLayer for LineLayer {
    fn into_layer(self) -> Layer {
        Layer::Line(self)
    }
}

impl LineLayer {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> LineLayer {
        LineLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            layout: LineLayout::default(),
            paint: LinePaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum LineCap {
    #[default]
    Butt,
    Round,
    Square,
}
impl EnumMarker for LineCap {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum LineJoin {
    #[default]
    Miter,
    Bevel,
    Round,
    None,
}
impl EnumMarker for LineJoin {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct LinePaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_blur: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_dasharray: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_emissive_strength: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_gap_width: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_gradient: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_offset: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_pattern: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_translate: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_translate_anchor: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_trim_offset: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<Expression<()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct LineLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_cap: Option<Expression<LineCap>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_join: Option<Expression<LineJoin>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_miter_limit: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_round_limit: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_sort_key: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(default)]
    pub layout: SymbolLayout,
    #[serde(default)]
    pub paint: SymbolPaint,
}

impl IntoLayer for SymbolLayer {
    fn into_layer(self) -> Layer {
        Layer::Symbol(self)
    }
}

impl SymbolLayer {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> SymbolLayer {
        SymbolLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            layout: SymbolLayout::default(),
            paint: SymbolPaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Anchor {
    #[default]
    Center,
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
impl EnumMarker for Anchor {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Alignment {
    #[default]
    Auto,
    Map,
    Viewport,
}
impl EnumMarker for Alignment {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum IconTextFit {
    #[default]
    None,
    Width,
    Height,
    Both,
}
impl EnumMarker for IconTextFit {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SymbolPlacement {
    #[default]
    Point,
    Line,
    LineCenter,
}
impl EnumMarker for SymbolPlacement {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SymbolZOrder {
    #[default]
    Auto,
    ViewportY,
    Source,
}
impl EnumMarker for SymbolZOrder {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum TextJustify {
    #[default]
    Center,
    Auto,
    Left,
    Right,
}
impl EnumMarker for TextJustify {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum TextTransform {
    #[default]
    None,
    Uppercase,
    Lowercase,
}
impl EnumMarker for TextTransform {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SymbolPaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color_brightness_max: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color_brightness_min: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color_contrast: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color_saturation: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emissive_strength: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_halo_blur: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_halo_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_halo_width: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_image_cross_fade: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_translate: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_translate_anchor: Option<Expression<TranslateAnchor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_emissive_strength: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_halo_blur: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_halo_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_halo_width: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_translate: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_translate_anchor: Option<Expression<TranslateAnchor>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SymbolLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_allow_overlap: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_anchor: Option<Expression<Anchor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_ignore_placement: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_image: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_keep_upright: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_offset: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_optional: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_padding: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_pitch_alignment: Option<Expression<Alignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_rotate: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_rotation_alignment: Option<Expression<Alignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_text_fit: Option<Expression<IconTextFit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_text_fit_padding: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_avoid_edges: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_placement: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_sort_key: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_spacing: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_z_elevate: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_z_order: Option<Expression<SymbolZOrder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_allow_overlap: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_anchor: Option<Expression<Anchor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_field: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_font: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_ignore_placement: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_justify: Option<Expression<TextJustify>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_keep_upright: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_letter_spacing: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_line_height: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_max_angle: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_max_width: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_offset: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_optional: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_padding: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_pitch_alignment: Option<Expression<Alignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_radial_offset: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_rotate: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_rotation_alignment: Option<Expression<Alignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_size: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_transform: Option<Expression<TextTransform>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_variable_anchor: Option<Expression<Anchor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_writing_mode: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RasterLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(default)]
    pub layout: RasterLayout,
    #[serde(default)]
    pub paint: RasterPaint,
}

impl IntoLayer for RasterLayer {
    fn into_layer(self) -> Layer {
        Layer::Raster(self)
    }
}

impl RasterLayer {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> RasterLayer {
        RasterLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            layout: RasterLayout::default(),
            paint: RasterPaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RasterPaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_array_band: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_brightness_max: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_brightness_min: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_color_mix: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_color_range: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_contrast: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_elevation: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_emissive_strength: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_fade_duration: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_hue_rotate: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_resampling: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_saturation: Option<Expression<()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RasterLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RasterParticleLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(default)]
    pub layout: RasterParticleLayout,
    #[serde(default)]
    pub paint: RasterParticlePaint,
}

impl IntoLayer for RasterParticleLayer {
    fn into_layer(self) -> Layer {
        Layer::RasterParticle(self)
    }
}

impl RasterParticleLayer {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> RasterParticleLayer {
        RasterParticleLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            layout: RasterParticleLayout::default(),
            paint: RasterParticlePaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RasterParticlePaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_particle_array_band: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_particle_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_particle_count: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_particle_fade_opacity_factor: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_particle_max_speed: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_particle_reset_rate_factor: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster_particle_speed_factor: Option<Expression<()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RasterParticleLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CircleLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(default)]
    pub layout: CircleLayout,
    #[serde(default)]
    pub paint: CirclePaint,
}

impl IntoLayer for CircleLayer {
    fn into_layer(self) -> Layer {
        Layer::Circle(self)
    }
}

impl CircleLayer {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> CircleLayer {
        CircleLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            layout: CircleLayout::default(),
            paint: CirclePaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum CircleAlignment {
    #[default]
    Viewport,
    Map,
}
impl EnumMarker for CircleAlignment {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum CircleScale {
    #[default]
    Map,
    Viewport,
}
impl EnumMarker for CircleScale {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct CirclePaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_blur: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_emissive_strength: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_pitch_alignment: Option<Expression<CircleAlignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_pitch_scale: Option<Expression<CircleScale>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_radius: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_stroke_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_stroke_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_stroke_width: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_translate: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_translate_anchor: Option<Expression<TranslateAnchor>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct CircleLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_sort_key: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillExtrusionLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(default)]
    pub layout: FillExtrusionLayout,
    #[serde(default)]
    pub paint: FillExtrusionPaint,
}

impl IntoLayer for FillExtrusionLayer {
    fn into_layer(self) -> Layer {
        Layer::FillExtrusion(self)
    }
}

impl FillExtrusionLayer {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> FillExtrusionLayer {
        FillExtrusionLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            layout: FillExtrusionLayout::default(),
            paint: FillExtrusionPaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct FillExtrusionPaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_ambient_occlusion_ground_attenuation: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_ambient_occlusion_ground_radius: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_ambient_occlusion_wall_radius: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_base: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_cutoff_fade_range: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_emissive_strength: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_flood_light_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_flood_light_ground_attenuation: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_flood_light_ground_radius: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_flood_light_intensity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_flood_light_wall_radius: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_height: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_pattern: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_rounded_roof: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_translate: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_translate_anchor: Option<Expression<TranslateAnchor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_vertical_gradient: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_extrusion_vertical_scale: Option<Expression<()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct FillExtrusionLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeatmapLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(default)]
    pub layout: HeatmapLayout,
    #[serde(default)]
    pub paint: HeatmapPaint,
}

impl IntoLayer for HeatmapLayer {
    fn into_layer(self) -> Layer {
        Layer::Heatmap(self)
    }
}

impl HeatmapLayer {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> HeatmapLayer {
        HeatmapLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            layout: HeatmapLayout::default(),
            paint: HeatmapPaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct HeatmapPaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heatmap_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heatmap_intensity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heatmap_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heatmap_radius: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heatmap_weight: Option<Expression<()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct HeatmapLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HillshadeLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(default)]
    pub layout: HillshadeLayout,
    #[serde(default)]
    pub paint: HillshadePaint,
}

impl IntoLayer for HillshadeLayer {
    fn into_layer(self) -> Layer {
        Layer::Hillshade(self)
    }
}

impl HillshadeLayer {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> HillshadeLayer {
        HillshadeLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            layout: HillshadeLayout::default(),
            paint: HillshadePaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct HillshadePaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hillshade_accent_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hillshade_emissive_strength: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hillshade_exaggeration: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hillshade_highlight_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hillshade_illumination_anchor: Option<Expression<TranslateAnchor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hillshade_illumination_direction: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hillshade_shadow_color: Option<Expression<()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct HillshadeLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkyLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(default)]
    pub layout: SkyLayout,
    #[serde(default)]
    pub paint: SkyPaint,
}

impl IntoLayer for SkyLayer {
    fn into_layer(self) -> Layer {
        Layer::Sky(self)
    }
}

impl SkyLayer {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> SkyLayer {
        SkyLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            layout: SkyLayout::default(),
            paint: SkyPaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SkyType {
    #[default]
    Gradient,
    Atmosphere,
}
impl EnumMarker for SkyType {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SkyPaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky_atmosphere_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky_atmosphere_halo_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky_atmosphere_sun: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky_atmosphere_sun_intensity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky_gradient: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky_gradient_center: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky_gradient_radius: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky_type: Option<Expression<SkyType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SkyLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelLayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(default)]
    pub layout: ModelLayout,
    #[serde(default)]
    pub paint: ModelPaint,
}

impl IntoLayer for ModelLayer {
    fn into_layer(self) -> Layer {
        Layer::Model(self)
    }
}

impl ModelLayer {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> ModelLayer {
        ModelLayer {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
            layout: ModelLayout::default(),
            paint: ModelPaint::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ModelType {
    #[default]
    Common3d,
    LocationIndicator,
}
impl EnumMarker for ModelType {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ModelPaint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_ambient_occlusion_intensity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_cast_shadows: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_color: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_color_mix_intensity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_cutoff_fade_range: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_emissive_strength: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_height_based_emissive_strength_multiplier: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_opacity: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_receive_shadows: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_rotation: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_roughness: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_scale: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_translation: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<Expression<ModelType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ModelLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<Expression<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}
