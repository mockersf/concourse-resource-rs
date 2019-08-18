# concourse-resource-rs [![License: MIT](https://img.shields.io/badge/License-Apache_2.0-yellow.svg)](https://opensource.org/licenses/Apache-2.0) [![Build Status](https://travis-ci.org/mockersf/concourse-resource-rs.svg?branch=master)](https://travis-ci.org/mockersf/concourse-resource-rs) [![Realease Doc](https://docs.rs/concourse-resource/badge.svg)](https://docs.rs/concourse-resource) [![Crate](https://img.shields.io/crates/v/concourse-resource.svg)](https://crates.io/crates/concourse-resource)

The API docs for the master branch are published [here](https://mockersf.github.io/concourse-resource-rs/).

Helper to create a [Concourse](https://concourse-ci.org) resource in Rust following https://concourse-ci.org/implementing-resource-types.html.

## Examples

See [examples](https://github.com/mockersf/concourse-resource-rs/tree/master/examples) for more examples.

The included multi-stage Dockerfile shows how to build a minimal docker image to deploy the concourse resource. To run it:
```
docker build --build-arg EXAMPLE=hello_world .
```

### Simple hello world

```rust
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
```