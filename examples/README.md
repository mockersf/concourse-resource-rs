# Examples

* [simple hello world](#simple%20hello%20world)
* [hello world](#hello%20world)

## simple hello world

This example will create a resource that will create a file `hello_world.txt" that contains "hello, world!". This resource has no configuration.

### How to build:
```
docker build --build-arg EXAMPLE=simple_hello_world ../
```

## hello world

This example will create a resource that will create a file `hello_world.txt" that contains "Hello, world!". Unlike the simple example, both parts of this message can be customized.

The name can be set on either the ressource or the step, with the step overriding the ressource configuration.

The action can be set on the step to be either "hello" or "goodbye".

### How to build:
```
docker build --build-arg EXAMPLE=hello_world ../
```

### Example pipeline:

```yaml
resource_types:
- name: hello-world
  type: docker-image
  source:
    repository: mockersf/concourse-resource-rs-examples
    tag: hello_world

resources:
  - name: hello-world
    type: hello-world

jobs:
  - name: say hello to the world!
    plan:
      - get: hello-world
        params:
          name: François
      - task: hello-world
        config:
          platform: linux
          image_resource:
            type: docker-image
            source: {repository: busybox}
          inputs:
            - name: hello-world
          run:
            path: sh
            args:
            - -exc
            - cat hello-world/hello_world.txt
```

![pipeline build](https://raw.githubusercontent.com/mockersf/concourse-resource-rs/master/examples/imgs/hello-world.png "pipeline build")