use std::{fmt, fs::File, io::Write, path::Path};

use serde::{Deserialize, Serialize};

use concourse_resource::*;

struct HelloWorld {}

#[derive(Serialize, Deserialize)]
struct Version {
    ver: String,
}

#[derive(Deserialize, Default)]
struct Source {
    name: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct InParams {
    name: Option<String>,
    action: Action,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum Action {
    Hello,
    Goodbye,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Hello => write!(f, "Hello"),
            Action::Goodbye => write!(f, "Goodbye"),
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::Hello
    }
}

#[derive(Serialize, Debug, IntoMetadataKV)]
struct InMetadata {
    said: String,
}

impl Resource for HelloWorld {
    type Version = Version;

    type Source = Source;

    type InParams = InParams;
    type InMetadata = InMetadata;

    type OutParams = concourse_resource::Empty;
    type OutMetadata = concourse_resource::Empty;

    fn resource_check(
        _source: Option<Self::Source>,
        _version: Option<Self::Version>,
    ) -> Vec<Self::Version> {
        vec![Self::Version {
            ver: String::from("static"),
        }]
    }

    fn resource_in(
        source: Option<Self::Source>,
        _version: Self::Version,
        params: Option<Self::InParams>,
        output_path: &str,
    ) -> Result<InOutput<Self::Version, Self::InMetadata>, Box<std::error::Error>> {
        let action = params
            .as_ref()
            .map(|p| p.action)
            .unwrap_or_else(|| Action::default());
        let name = params
            .and_then(|p| p.name)
            .or(source.and_then(|s| s.name))
            .unwrap_or_else(|| String::from("world"));

        let hello_world = format!("{}, {}!", action, name);

        let mut path = Path::new(output_path).to_path_buf();
        path.push("hello_world.txt");
        let mut file = File::create(path)?;
        file.write_all(hello_world.as_bytes())?;

        Ok(InOutput {
            version: Self::Version {
                ver: String::from("static"),
            },
            metadata: Some(InMetadata { said: hello_world }),
        })
    }

    fn resource_out(
        _source: Option<Self::Source>,
        _params: Option<Self::OutParams>,
        _input_path: &str,
    ) -> OutOutput<Self::Version, Self::OutMetadata> {
        OutOutput {
            version: Self::Version {
                ver: String::from("static"),
            },
            metadata: None,
        }
    }
}

create_resource!(HelloWorld);
