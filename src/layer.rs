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
    Fill(FillLayer),
    Background(BackgroundLayer),
    Line(LineLayer),
    Symbol(SymbolLayer),
    Circle(CircleLayer),
}

pub trait IntoLayer {
    fn into_layer(self) -> Layer;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerBase {
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
}

impl LayerBase {
    pub fn new(id: impl Into<String>, source: impl Into<String>) -> LayerBase {
        LayerBase {
            id: id.into(),
            maxzoom: None,
            minzoom: None,
            source: source.into(),
            filter: None,
            source_layer: None,
            slot: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomLayer {
    #[serde(flatten)]
    pub inner: LayerBase,
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
            inner: LayerBase::new(id, source),
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
pub struct FillLayer {
    #[serde(flatten)]
    pub inner: LayerBase,
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
            inner: LayerBase::new(id, source),
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
pub struct LineLayer {
    #[serde(flatten)]
    pub inner: LayerBase,
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
            inner: LayerBase::new(id, source),
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
    #[serde(flatten)]
    pub inner: LayerBase,
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
            inner: LayerBase::new(id, source),
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
pub struct CircleLayer {
    #[serde(flatten)]
    pub inner: LayerBase,
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
            inner: LayerBase::new(id, source),
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
