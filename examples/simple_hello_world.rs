use std::{fs::File, io::Write, path::Path};

use serde::{Deserialize, Serialize};

use concourse_resource::*;

struct HelloWorld {}

#[derive(Serialize, Deserialize)]
struct Version {
    ver: String,
}

impl Resource for HelloWorld {
    type Version = Version;

    type Source = concourse_resource::Empty;

    type InParams = concourse_resource::Empty;
    type InMetadata = concourse_resource::Empty;

    type OutParams = concourse_resource::Empty;
    type OutMetadata = concourse_resource::Empty;

    fn resource_check(_: Option<Self::Source>, _: Option<Self::Version>) -> Vec<Self::Version> {
        vec![Self::Version {
            ver: String::from("static"),
        }]
    }

    fn resource_in(
        _source: Option<Self::Source>,
        _version: Self::Version,
        _params: Option<Self::InParams>,
        output_path: &str,
    ) -> Result<InOutput<Self::Version, Self::InMetadata>, Box<dyn std::error::Error>> {
        let mut path = Path::new(output_path).to_path_buf();
        path.push("hello_world.txt");
        let mut file = File::create(path)?;
        file.write_all(b"hello, world!")?;

        Ok(InOutput {
            version: Self::Version {
                ver: String::from("static"),
            },
            metadata: None,
        })
    }

    fn resource_out(
        _: Option<Self::Source>,
        _: Option<Self::OutParams>,
        _: &str,
    ) -> OutOutput<Self::Version, Self::OutMetadata> {
        unimplemented!()
    }
}

create_resource!(HelloWorld);
