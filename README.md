<h1 align="center">mapbox-gl-rs</h1>
<p align="center">Unofficial Rust binding for <a href="https://github.com/mapbox/mapbox-gl-js">mapbox-gl-js</a></p>
<p align="center">
  <a href="https://crates.io/crates/mapboxgl">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/mapboxgl.svg">
  </a>
  <a href="https://docs.rs/mapboxgl">
    <img alt="Docs.rs" src="https://img.shields.io/badge/docs.rs-mapboxgl-blue">
  </a>
  <a href="https://github.com/yukinarit/mapbox-gl-rs/actions/workflows/test.yml">
    <img alt="GithubActions" src="https://github.com/yukinarit/mapbox-gl-rs/actions/workflows/test.yml/badge.svg">
  </a>
</p>

<p align="center"><img src="https://raw.githubusercontent.com/yukinarit/mapbox-gl-rs/main/logo.svg" style="width:80px"/></p>

## What's this?

[mapbox-gl-js](https://github.com/mapbox/mapbox-gl-js) is an open source library for rendering a beautiful vector-based maps in web browser, built with Mapbox and OSS community. The goal of this project is to create a rust binding for `mapbox-gl-js` via [WebAssembly](https://webassembly.org/) so that Rustacean can build webapps with beautiful Mapbox maps only in Rust.

**NOTE:** `mapbox-gl-rs` is in super-duper infant stage. Most of the features are WIP. Please bear with that if you're interested! Also, any contributions e.g. bug reports, feature requests, sending a patch are appreciated.

## How does it work?

[wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) provides a Rust library and tool to compile Rust code into WebAssembly, which allows Rust based application e.g. [egui](https://www.egui.rs/#demo) and [RustPython](https://rustpython.github.io/) to run on web browsers. Most of the people don't know that `wasm-bindgen` also allows to invoke JavaScript code from Rust. This project leverages this feature to provide a Rust binding for `mapbox-gl-js`.

<p align="center"><img src="https://raw.githubusercontent.com/yukinarit/mapbox-gl-rs/main/how-it-works.svg"/></p>

## Usage

Add `mapboxgl` to your Cargo.toml
```toml
mapboxgl = "0.1.0"
```

Add the following CSS and JavaScript snippet in html, so that mapboxgl JavaScript module is loaded.
```html
<link href='https://api.mapbox.com/mapbox-gl-js/v2.10.0/mapbox-gl.css' rel='stylesheet' />
<script src="https://api.mapbox.com/mapbox-gl-js/v2.10.0/mapbox-gl.js"></script>
```

Build and run your app. If you are a [Yew](https://github.com/yewstack/yew) user, the following command will build and start a web server automatically
```
trunk serve
```

## Supported features

* Map
    * [x] [Map](https://docs.mapbox.com/mapbox-gl-js/api/map/)
    * [x] [Options](https://docs.mapbox.com/mapbox-gl-js/api/properties/)
* Markers and controls
    * [x] [Marker](https://docs.mapbox.com/mapbox-gl-js/api/markers/#marker)
    * [ ] [AttributionControl](https://docs.mapbox.com/mapbox-gl-js/api/markers/#attributioncontrol)
    * [ ] [FullscreenControl](https://docs.mapbox.com/mapbox-gl-js/api/markers/#fullscreencontrol)
    * [ ] [GeolocateControl](https://docs.mapbox.com/mapbox-gl-js/api/markers/#geolocatecontrol)
    * [ ] [NavigationControl](https://docs.mapbox.com/mapbox-gl-js/api/markers/#navigationcontrol)
    * [x] [Pupup](https://docs.mapbox.com/mapbox-gl-js/api/markers/#popup)
    * [ ] [ScaleControl](https://docs.mapbox.com/mapbox-gl-js/api/markers/#scalecontrol)
* Geography and Geometry
    * [x] [LngLat](https://docs.mapbox.com/mapbox-gl-js/api/geography/#lnglat)
    * [x] [LngLatBounds](https://docs.mapbox.com/mapbox-gl-js/api/geography/#lnglatbounds)
    * [ ] [LngLatBoundsLike](https://docs.mapbox.com/mapbox-gl-js/api/geography/#lnglatboundslike)
    * [ ] [LngLatLike](https://docs.mapbox.com/mapbox-gl-js/api/geography/#lnglatlike)
    * [ ] [MercatorCoordinate](https://docs.mapbox.com/mapbox-gl-js/api/geography/#mercatorcoordinate)
    * [ ] [Point](https://docs.mapbox.com/mapbox-gl-js/api/geography/#point)
    * [ ] [PointLike](https://docs.mapbox.com/mapbox-gl-js/api/geography/#pointlike)
* User interaction handlers
    * [x] [BoxZoomHandler](https://docs.mapbox.com/mapbox-gl-js/api/handlers/#boxzoomhandler)
    * [ ] [DoubleClickZoomHandler](https://docs.mapbox.com/mapbox-gl-js/api/handlers/#doubleclickzoomhandler)
    * [ ] [DragPanHandler](https://docs.mapbox.com/mapbox-gl-js/api/handlers/#dragpanhandler)
    * [ ] [DragRotateHandler](https://docs.mapbox.com/mapbox-gl-js/api/handlers/#dragrotatehandler)
    * [ ] [KeyboardHandler](https://docs.mapbox.com/mapbox-gl-js/api/handlers/#keyboardhandler)
    * [ ] [ScrollZoomHandler](https://docs.mapbox.com/mapbox-gl-js/api/handlers/#scrollzoomhandler)
    * [ ] [TouchPitchHandler](https://docs.mapbox.com/mapbox-gl-js/api/handlers/#touchpitchhandler)
    * [ ] [TouchZoomRotateHandler](https://docs.mapbox.com/mapbox-gl-js/api/handlers/#touchzoomrotatehandler)
* Sources
    * [ ] [CanvasSource](https://docs.mapbox.com/mapbox-gl-js/api/sources/#canvassource)
    * [ ] [CanvasSourceOptions](https://docs.mapbox.com/mapbox-gl-js/api/sources/#canvassourceoptions)
    * [x] [GeoJsonSource](https://docs.mapbox.com/mapbox-gl-js/api/sources/#geojsonsource)
    * [ ] [ImageSource](https://docs.mapbox.com/mapbox-gl-js/api/sources/#imagesource)
    * [ ] [VectorTileSource](https://docs.mapbox.com/mapbox-gl-js/api/sources/#vectortilesource)
    * [ ] [VideoSource](https://docs.mapbox.com/mapbox-gl-js/api/sources/#videosource)
* Events and event types
    * [x] [MapBoxZoomEvent](https://docs.mapbox.com/mapbox-gl-js/api/events/#mapboxzoomevent)
    * [x] [MapDataEvent](https://docs.mapbox.com/mapbox-gl-js/api/events/#mapdataevent)
    * [x] [MapMouseEvent](https://docs.mapbox.com/mapbox-gl-js/api/events/#mapmouseevent)
    * [x] [MapTouchEvent](https://docs.mapbox.com/mapbox-gl-js/api/events/#maptouchevent)
    * [x] [MapWheelEvent](https://docs.mapbox.com/mapbox-gl-js/api/events/#mapwheelevent)

## Examples

[![](https://raw.githubusercontent.com/yukinarit/mapbox-gl-rs/main/set-data.gif)](https://github.com/yukinarit/mapbox-gl-rs/tree/main/examples/set-data)

[![](https://raw.githubusercontent.com/yukinarit/mapbox-gl-rs/main/popup.gif)](https://github.com/yukinarit/mapbox-gl-rs/tree/main/examples/popup)

* [simple](https://github.com/yukinarit/mapbox-gl-rs/tree/main/examples/simple): Hello world
* [on-load](https://github.com/yukinarit/mapbox-gl-rs/tree/main/examples/on-load): Catch the event when the map is loaded
* [popup](https://github.com/yukinarit/mapbox-gl-rs/tree/main/examples/popup): Show popup control
* [geojson-source](https://github.com/yukinarit/mapbox-gl-rs/tree/main/examples/geojson-source): Load GeoJSON source and show lines
* [set-data](https://github.com/yukinarit/mapbox-gl-rs/tree/main/examples/set-data): Update paths in realtime using set-data

## License

This project is licensed under the [MIT license](https://github.com/yukinarit/mapbox-gl-rs/blob/main/LICENSE).
