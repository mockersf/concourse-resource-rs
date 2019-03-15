/// https://concourse-ci.org/implementing-resource-types.html
use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InOutput<V, M> {
    pub version: V,
    pub metadata: Option<M>,
}

#[derive(Serialize, Deserialize)]
pub struct OutOutput<V, M> {
    pub version: V,
    pub metadata: Option<M>,
}

#[derive(Deserialize, Debug)]
pub struct CheckInput<S, V> {
    pub source: S,
    pub version: Option<V>,
}

#[derive(Deserialize, Debug)]
pub struct InInput<S, V, P> {
    pub source: S,
    pub version: V,
    pub params: Option<P>,
}

#[derive(Deserialize, Debug)]
pub struct OutInput<S, P> {
    pub source: S,
    pub params: Option<P>,
}

pub struct BuildParameters {
    pub build_id: String,
    pub build_name: Option<String>,
    pub build_job_name: Option<String>,
    pub build_pipeline_name: Option<String>,
    pub build_team_name: String,
    pub atc_external_url: String,
}

pub trait Resource {
    type Source: DeserializeOwned + Debug;
    type Version: Serialize + DeserializeOwned + Debug;

    type InParams: DeserializeOwned + Debug;
    type InMetadata: Serialize + Debug;
    type OutParams: DeserializeOwned + Debug;
    type OutMetadata: Serialize + Debug;

    /// https://concourse-ci.org/implementing-resource-types.html#resource-check
    fn resource_check(source: Self::Source, version: Option<Self::Version>) -> Vec<Self::Version>;

    /// https://concourse-ci.org/implementing-resource-types.html#in
    fn resource_in(
        source: Self::Source,
        version: Self::Version,
        params: Option<Self::InParams>,
        path: &str,
    ) -> InOutput<Self::Version, Self::InMetadata>;

    /// https://concourse-ci.org/implementing-resource-types.html#out
    fn resource_out(
        source: Self::Source,
        params: Option<Self::OutParams>,
    ) -> OutOutput<Self::Version, Self::OutMetadata>;

    /// https://concourse-ci.org/implementing-resource-types.html#resource-metadata
    fn build_metadata() -> BuildParameters {
        BuildParameters {
            build_id: std::env::var("BUILD_ID")
                .expect("environment variable BUILD_ID should be present"),
            build_name: std::env::var("BUILD_NAME").ok(),
            build_job_name: std::env::var("BUILD_JOB_NAME").ok(),
            build_pipeline_name: std::env::var("BUILD_PIPELINE_NAME").ok(),
            build_team_name: std::env::var("BUILD_TEAM_NAME")
                .expect("environment variable BUILD_TEAM_NAME should be present"),
            atc_external_url: std::env::var("ATC_EXTERNAL_URL")
                .expect("environment variable ATC_EXTERNAL_URL should be present"),
        }
    }
}

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
                        serde_json::to_string(&result).expect("error serializing response")
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
                    println!(
                        "{}",
                        serde_json::to_string(&result).expect("error serializing response")
                    );
                }
                "/opt/resource/out" => {
                    let input: OutInput<
                        <$resource as Resource>::Source,
                        <$resource as Resource>::OutParams,
                    > = dbg!(serde_json::from_str(&input_buffer))
                        .expect("error deserializing input");
                    let result = <$resource as Resource>::resource_out(input.source, input.params);
                    println!(
                        "{}",
                        serde_json::to_string(&result).expect("error serializing response")
                    );
                }
                v => eprintln!("unexpected being called as '{}'", v),
            }
        }
    };
}
