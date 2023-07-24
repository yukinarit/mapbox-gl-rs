//! This module is all about id returned from mapbox-gl-rs.
//! mapbox-gl-js provides a convenient interface to (de)register a resource.

/// ID for an event listener.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MapListenerId(pub uuid::Uuid);

/// ID for a marker.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MarkerId(pub uuid::Uuid);

/// ID for a callback.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CallbackId(pub uuid::Uuid);
