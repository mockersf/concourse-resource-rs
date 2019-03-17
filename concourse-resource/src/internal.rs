//! Internal types used to wrap inputs and outputs. They should not be built
//! directly but are used by macros

use serde::{Deserialize, Serialize};

/// Simple Key-Value struct as needed by Concourse for metadata
#[allow(missing_debug_implementations)]
#[derive(Serialize)]
pub struct KV {
    /// The name of this metadata
    pub name: String,
    /// The value of this metadata
    pub value: String,
}

/// Output of the "in" step of the resource
#[allow(missing_debug_implementations)]
#[derive(Serialize)]
pub struct InOutputKV<V> {
    /// The fetched version.
    pub version: V,
    /// A list of key-value pairs. This data is intended for public consumption and will make
    /// it upstream, intended to be shown on the build's page.
    pub metadata: Option<Vec<KV>>,
}

/// Output of the "out" step of the resource
#[allow(missing_debug_implementations)]
#[derive(Serialize)]
pub struct OutOutputKV<V> {
    /// The resulting version.
    pub version: V,
    /// A list of key-value pairs. This data is intended for public consumption and will make
    /// it upstream, intended to be shown on the build's page.
    pub metadata: Option<Vec<KV>>,
}

/// Input of the "check" step of the resource
#[allow(missing_debug_implementations)]
#[derive(Deserialize)]
pub struct CheckInput<S, V> {
    /// Resource configuration, from the `source` field
    pub source: Option<S>,
    /// Latest version retrieved, or `None` on first check
    pub version: Option<V>,
}

/// Input of the "in" step of the resource
#[allow(missing_debug_implementations)]
#[derive(Deserialize)]
pub struct InInput<S, V, P> {
    /// Resource configuration, from the `source` field
    pub source: Option<S>,
    /// Version to retrieve
    pub version: V,
    /// Step configuration, from the `params` field
    pub params: Option<P>,
}

/// Input of the "out" step of the resource
#[allow(missing_debug_implementations)]
#[derive(Deserialize)]
pub struct OutInput<S, P> {
    /// Resource configuration, from the `source` field
    pub source: Option<S>,
    /// Step configuration, from the `params` field
    pub params: Option<P>,
}
