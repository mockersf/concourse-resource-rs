# concourse-resource-rs [![License: MIT](https://img.shields.io/badge/License-Apache_2.0-yellow.svg)](https://opensource.org/licenses/Apache-2.0) [![Build Status](https://travis-ci.org/mockersf/concourse-resource-rs.svg?branch=master)](https://travis-ci.org/mockersf/concourse-resource-rs) [![Realease Doc](https://docs.rs/concourse-resource/badge.svg)](https://docs.rs/concourse-resource) [![Crate](https://img.shields.io/crates/v/concourse-resource.svg)](https://crates.io/crates/concourse-resource)

The API docs for the master branch are published [here](https://mockersf.github.io/concourse-resource-rs/).

Helper to create a [Concourse](https://concourse-ci.org) resource in Rust following https://concourse-ci.org/implementing-resource-types.html.

See [examples](https://github.com/mockersf/concourse-resource-rs/tree/master/examples) on how to use it.

The included multi-stage Dockerfile shows how to build a minimal docker image to deploy the concourse resource. To run it:
```
docker build --build-arg EXAMPLE=hello_world .
```
