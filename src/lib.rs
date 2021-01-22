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

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Map, Value};

pub use concourse_resource_derive::*;

pub mod internal;

/// Output of the "in" step of the resource
#[allow(missing_debug_implementations)]
#[derive(Serialize)]
pub struct InOutput<V, M> {
    /// The fetched version.
    pub version: V,
    /// A list of key-value pairs. This data is intended for public consumption and will make
    /// it upstream, intended to be shown on the build's page.
    pub metadata: Option<M>,
}

/// Output of the "out" step of the resource
#[allow(missing_debug_implementations)]
#[derive(Serialize)]
pub struct OutOutput<V, M> {
    /// The resulting version.
    pub version: V,
    /// A list of key-value pairs. This data is intended for public consumption and will make
    /// it upstream, intended to be shown on the build's page.
    pub metadata: Option<M>,
}

/// Trait for Metadata to be usable as Concourse Metadata. This trait can be derived if the
/// base struct implement `serde::Deserialize`
pub trait IntoMetadataKV {
    /// Turn `self` into a `Vec` of `internal::KV`
    fn into_metadata_kv(self) -> Vec<internal::KV>;
}

/// Empty value that can be used as `InParams`, `InMetadata`, `OutParams` or `OutMetadata` for
/// a `Resource`
#[allow(missing_debug_implementations)]
#[derive(Serialize, Deserialize, Copy, Clone, Default)]
pub struct Empty;
impl IntoMetadataKV for Empty {
    fn into_metadata_kv(self) -> Vec<internal::KV> {
        vec![]
    }
}

/// When used in a "get" or "put" step, metadata about the running build is made available
/// via environment variables.
///
/// If the build is a one-off, `name`, `job_name`, `pipeline_name`, and `pipeline_instance_vars`
/// will be `None`. `pipeline_instance_vars` will also be `None` if the build's pipeline is not a
/// pipeline instance (i.e. it is a regular pipeline).
///
/// [Concourse documentation](https://concourse-ci.org/implementing-resource-types.html#resource-metadata)
#[derive(Debug)]
pub struct BuildMetadata {
    /// The internal identifier for the build. Right now this is numeric but it may become
    /// a guid in the future. Treat it as an absolute reference to the build.
    pub id: String,
    /// The build number within the build's job.
    pub name: Option<String>,
    /// The name of the build's job.
    pub job_name: Option<String>,
    /// The pipeline that the build's job lives in.
    pub pipeline_name: Option<String>,
    /// The pipeline's instance vars, used to differentiate pipeline instances.
    pub pipeline_instance_vars: Option<Map<String, Value>>,
    /// The team that the build belongs to.
    pub team_name: String,
    /// The public URL for your ATC; useful for debugging.
    pub atc_external_url: String,
}

/// The methods and associated types needed to implement a resource
pub trait Resource {
    /// A version of the resource
    type Version: Serialize + DeserializeOwned;

    /// Resource configuration, from the `source` field
    type Source: DeserializeOwned;

    /// Parameters for the "in" step, from the `params` field
    type InParams: DeserializeOwned;
    /// A list of key-value pairs for the "in" step. This data is intended for public
    /// consumption and will make it upstream, intended to be shown on the build's page.
    type InMetadata: Serialize + IntoMetadataKV;

    /// Parameters for the "out" step, from the `params` field
    type OutParams: DeserializeOwned;
    /// A list of key-value pairs for the "out" step. This data is intended for public
    /// consumption and will make it upstream, intended to be shown on the build's page.
    type OutMetadata: Serialize + IntoMetadataKV;

    /// A resource type's check method is invoked to detect new versions of the resource. It is
    /// given the configured source and current version, and must return the array of new
    /// versions, in chronological order, including the requested version if it's still valid.
    ///
    /// [Concourse documentation](https://concourse-ci.org/implementing-resource-types.html#resource-check)
    fn resource_check(
        source: Option<Self::Source>,
        version: Option<Self::Version>,
    ) -> Vec<Self::Version>;

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
        source: Option<Self::Source>,
        version: Self::Version,
        params: Option<Self::InParams>,
        output_path: &str,
    ) -> Result<InOutput<Self::Version, Self::InMetadata>, Box<dyn std::error::Error>>;

    /// The out method is called with the resource's source configuration, the configured params
    /// and a path to the directory containing the build's full set of sources.
    ///
    /// The script must return the resulting version of the resource. Additionally, it may return
    /// metadata as a list of key-value pairs. This data is intended for public consumption and
    /// will make it upstream, intended to be shown on the build's page.
    ///
    /// [Concourse documentation](https://concourse-ci.org/implementing-resource-types.html#out)
    fn resource_out(
        source: Option<Self::Source>,
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
            name: std::env::var("BUILD_NAME").ok(),
            job_name: std::env::var("BUILD_JOB_NAME").ok(),
            pipeline_name: std::env::var("BUILD_PIPELINE_NAME").ok(),
            pipeline_instance_vars: std::env::var("BUILD_PIPELINE_INSTANCE_VARS")
                .ok()
                .and_then(|instance_vars| serde_json::from_str(&instance_vars[..]).ok()),
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
        use std::io::Read;

        use concourse_resource::internal::*;

        fn main() {
            let mut input_buffer = String::new();
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();

            handle.read_to_string(&mut input_buffer).unwrap();

            let mut args = std::env::args();

            match args.next().expect("should have a bin name").as_ref() {
                "/opt/resource/check" => {
                    let input: CheckInput<
                        <$resource as Resource>::Source,
                        <$resource as Resource>::Version,
                    > = serde_json::from_str(&input_buffer).expect("error deserializing input");
                    let result =
                        <$resource as Resource>::resource_check(input.source, input.version);
                    println!(
                        "{}",
                        serde_json::to_string(&result).expect("error serializing output")
                    );
                }
                "/opt/resource/in" => {
                    let input: InInput<
                        <$resource as Resource>::Source,
                        <$resource as Resource>::Version,
                        <$resource as Resource>::InParams,
                    > = serde_json::from_str(&input_buffer).expect("error deserializing input");
                    let result = <$resource as Resource>::resource_in(
                        input.source,
                        input.version,
                        input.params,
                        &args.next().expect("expected path as first parameter"),
                    );
                    match result {
                        Err(error) => {
                            eprintln!("Error! {}", error);
                            std::process::exit(1);
                        }
                        Ok(InOutput { version, metadata }) => println!(
                            "{}",
                            serde_json::to_string(&InOutputKV {
                                version,
                                metadata: metadata.map(|md| md.into_metadata_kv())
                            })
                            .expect("error serializing output")
                        ),
                    };
                }
                "/opt/resource/out" => {
                    let input: OutInput<
                        <$resource as Resource>::Source,
                        <$resource as Resource>::OutParams,
                    > = serde_json::from_str(&input_buffer).expect("error deserializing input");
                    let result = <$resource as Resource>::resource_out(
                        input.source,
                        input.params,
                        &args.next().expect("expected path as first parameter"),
                    );
                    println!(
                        "{}",
                        serde_json::to_string(&OutOutputKV {
                            version: result.version,
                            metadata: result.metadata.map(|md| md.into_metadata_kv())
                        })
                        .expect("error serializing output")
                    );
                }
                v => eprintln!("unexpected being called as '{}'", v),
            }
        }
    };
}
