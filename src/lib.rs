#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

//! Helper to implement a [Concourse](https://concourse-ci.org/) resource in Rust
//!
//! [Concourse documentation](https://concourse-ci.org/implementing-resource-types.html)

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Output of the "in" step of the resource
#[derive(Serialize, Debug)]
pub struct InOutput<V, M> {
    /// The fetched version.
    pub version: V,
    /// A list of key-value pairs. This data is intended for public consumption and will make
    /// it upstream, intended to be shown on the build's page.
    pub metadata: Option<M>,
}

/// Output of the "out" step of the resource
#[derive(Serialize, Debug)]
pub struct OutOutput<V, M> {
    /// The resulting version.
    pub version: V,
    /// A list of key-value pairs. This data is intended for public consumption and will make
    /// it upstream, intended to be shown on the build's page.
    pub metadata: Option<M>,
}

///
#[derive(Serialize, Debug)]
pub struct KV {
    ///
    pub name: String,
    ///
    pub value: String,
}

/// Marker struct for an empty value
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Empty;
impl From<Empty> for Vec<KV> {
    fn from(_: Empty) -> Vec<KV> {
        vec![]
    }
}

/// Output of the "in" step of the resource
#[derive(Serialize, Debug)]
pub struct InOutputKV<V> {
    /// The fetched version.
    pub version: V,
    /// A list of key-value pairs. This data is intended for public consumption and will make
    /// it upstream, intended to be shown on the build's page.
    pub metadata: Option<Vec<KV>>,
}

/// Output of the "out" step of the resource
#[derive(Serialize, Debug)]
pub struct OutOutputKV<V> {
    /// The resulting version.
    pub version: V,
    /// A list of key-value pairs. This data is intended for public consumption and will make
    /// it upstream, intended to be shown on the build's page.
    pub metadata: Option<Vec<KV>>,
}

/// Input of the "check" step of the resource
#[derive(Deserialize, Debug)]
pub struct CheckInput<S, V> {
    /// Resource configuration, from the `source` field
    pub source: S,
    /// Latest version retrieved, or `None` on first check
    pub version: Option<V>,
}

/// Input of the "in" step of the resource
#[derive(Deserialize, Debug)]
pub struct InInput<S, V, P> {
    /// Resource configuration, from the `source` field
    pub source: S,
    /// Version to retrieve
    pub version: V,
    /// Step configuration, from the `params` field
    pub params: Option<P>,
}

/// Input of the "out" step of the resource
#[derive(Deserialize, Debug)]
pub struct OutInput<S, P> {
    /// Resource configuration, from the `source` field
    pub source: S,
    /// Step configuration, from the `params` field
    pub params: Option<P>,
}

/// When used in a "get" or "put" step, metadata about the running build is made available
/// via environment variables.
///
/// If the build is a one-off, `build_name`, `build_job_name`, and `build_pipeline_name`
/// will be `None`.
///
/// [Concourse documentation](https://concourse-ci.org/implementing-resource-types.html#resource-metadata)
#[derive(Debug)]
pub struct BuildMetadata {
    /// The internal identifier for the build. Right now this is numeric but it may become
    /// a guid in the future. Treat it as an absolute reference to the build.
    pub id: String,
    /// The build number within the build's job.
    pub name: Option<u32>,
    /// The name of the build's job.
    pub job_name: Option<String>,
    /// The pipeline that the build's job lives in.
    pub pipeline_name: Option<String>,
    /// The team that the build belongs to.
    pub team_name: String,
    /// The public URL for your ATC; useful for debugging.
    pub atc_external_url: String,
}

/// The methods and associated types needed to implement a resource
pub trait Resource {
    /// Resource configuration, from the `source` field
    type Source: DeserializeOwned + Debug;
    /// A version of the resource
    type Version: Serialize + DeserializeOwned + Debug;

    /// Parameters for the "in" step, from the `params` field
    type InParams: DeserializeOwned + Debug;
    /// A list of key-value pairs for the "in" step. This data is intended for public
    /// consumption and will make it upstream, intended to be shown on the build's page.
    type InMetadata: Serialize + Debug + Into<Vec<KV>>;
    /// Parameters for the "out" step, from the `params` field
    type OutParams: DeserializeOwned + Debug;
    /// A list of key-value pairs for the "out" step. This data is intended for public
    /// consumption and will make it upstream, intended to be shown on the build's page.
    type OutMetadata: Serialize + Debug + Into<Vec<KV>>;

    /// A resource type's check method is invoked to detect new versions of the resource. It is
    /// given the configured source and current version, and must return the array of new
    /// versions, in chronological order, including the requested version if it's still valid.
    ///
    /// [Concourse documentation](https://concourse-ci.org/implementing-resource-types.html#resource-check)
    fn resource_check(source: Self::Source, version: Option<Self::Version>) -> Vec<Self::Version>;

    /// The in method is passed the configured source, a precise version of the resource to fetch
    /// and a destination directory. The method must fetch the resource and place it in the given
    /// directory.
    ///
    /// If the desired resource version is unavailable (for example, if it was deleted), the
    /// method must return an error.
    ///
    /// The method must return the fetched version, and may return metadata as a list of
    /// key-value pairs. This data is intended for public consumption and will make it upstream,
    /// intended to be shown on the build's page.
    ///
    /// [Concourse documentation](https://concourse-ci.org/implementing-resource-types.html#in)
    fn resource_in(
        source: Self::Source,
        version: Self::Version,
        params: Option<Self::InParams>,
        output_path: &str,
    ) -> Result<InOutput<Self::Version, Self::InMetadata>, Box<std::error::Error>>;

    /// The out method is called with the resource's source configuration, the configured params
    /// and a path to the directory containing the build's full set of sources.
    ///
    /// The script must return the resulting version of the resource. Additionally, it may return
    /// metadata as a list of key-value pairs. This data is intended for public consumption and
    /// will make it upstream, intended to be shown on the build's page.
    ///
    /// [Concourse documentation](https://concourse-ci.org/implementing-resource-types.html#out)
    fn resource_out(
        source: Self::Source,
        params: Option<Self::OutParams>,
        input_path: &str,
    ) -> OutOutput<Self::Version, Self::OutMetadata>;

    /// When used in a "get" or "put" step, will return [metadata](struct.BuildMetadata.html) about the running build is
    /// made available via environment variables.
    ///
    /// [Concourse documentation](https://concourse-ci.org/implementing-resource-types.html#resource-metadata)
    fn build_metadata() -> BuildMetadata {
        BuildMetadata {
            id: std::env::var("BUILD_ID").expect("environment variable BUILD_ID should be present"),
            name: std::env::var("BUILD_NAME")
                .map_err(|_| ())
                .and_then(|v| v.parse::<u32>().map_err(|_| ()))
                .ok(),
            job_name: std::env::var("BUILD_JOB_NAME").ok(),
            pipeline_name: std::env::var("BUILD_PIPELINE_NAME").ok(),
            team_name: std::env::var("BUILD_TEAM_NAME")
                .expect("environment variable BUILD_TEAM_NAME should be present"),
            atc_external_url: std::env::var("ATC_EXTERNAL_URL")
                .expect("environment variable ATC_EXTERNAL_URL should be present"),
        }
    }
}

/// Macro that will build the `main` function from a struct implementing the `Resource` trait
#[macro_export]
macro_rules! create_resource {
    ($resource:ty) => {
        fn main() {
            use std::io::Read;
            let mut input_buffer = String::new();
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();

            handle.read_to_string(&mut input_buffer).unwrap();

            match std::env::args()
                .next()
                .expect("should have a bin name")
                .as_ref()
            {
                "/opt/resource/check" => {
                    let input: CheckInput<
                        <$resource as Resource>::Source,
                        <$resource as Resource>::Version,
                    > = dbg!(serde_json::from_str(&input_buffer))
                        .expect("error deserializing input");
                    let result =
                        <$resource as Resource>::resource_check(input.source, input.version);
                    println!(
                        "{}",
                        dbg!(serde_json::to_string(&result)).expect("error serializing response")
                    );
                }
                "/opt/resource/in" => {
                    let input: InInput<
                        <$resource as Resource>::Source,
                        <$resource as Resource>::Version,
                        <$resource as Resource>::InParams,
                    > = dbg!(serde_json::from_str(&input_buffer))
                        .expect("error deserializing input");
                    let result = <$resource as Resource>::resource_in(
                        input.source,
                        input.version,
                        input.params,
                        &std::env::args()
                            .next()
                            .expect("expected path as first parameter"),
                    );
                    match result {
                        Err(error) => {
                            eprintln!("Error! {}", error);
                            std::process::exit(1);
                        }
                        Ok(InOutput { version, metadata }) => println!(
                            "{}",
                            dbg!(serde_json::to_string(&InOutputKV {
                                version,
                                metadata: metadata.map(|md| md.into())
                            }))
                            .expect("error serializing response")
                        ),
                    };
                }
                "/opt/resource/out" => {
                    let input: OutInput<
                        <$resource as Resource>::Source,
                        <$resource as Resource>::OutParams,
                    > = dbg!(serde_json::from_str(&input_buffer))
                        .expect("error deserializing input");
                    let result = <$resource as Resource>::resource_out(
                        input.source,
                        input.params,
                        &std::env::args()
                            .next()
                            .expect("expected path as first parameter"),
                    );
                    println!(
                        "{}",
                        dbg!(serde_json::to_string(&OutOutputKV {
                            version: result.version,
                            metadata: result.metadata.map(|md| md.into())
                        }))
                        .expect("error serializing response")
                    );
                }
                v => eprintln!("unexpected being called as '{}'", v),
            }
        }
    };
}
