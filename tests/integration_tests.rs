use concourse_resource::{internal::OutOutputKV, IntoMetadataKV};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, IntoMetadataKV, Clone)]
pub struct Metadata {
    pub commit: String,
    pub author: String,
    pub url: String,
    #[serde_as(as = "DisplayFromStr")]
    pub mr_iid: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub project_id: u64,
}

#[derive(Serialize, Deserialize)]
struct Version {
    ver: String,
}

#[test]
fn test_serialisation() {
    let v = Version {
        ver: String::from("1.1"),
    };
    let md = Metadata {
        commit: String::from("commit_sha"),
        author: String::from("Han Solo"),
        url: String::from("https://gitlab.com/group/repo"),
        mr_iid: 1,
        project_id: 20000,
    };
    let output = serde_json::to_string(&OutOutputKV {
        version: v,
        metadata: Some(md.into_metadata_kv()),
    })
    .expect("error serializing output");
    println!("{}", output);
    assert_eq!(output, "{\"version\":{\"ver\":\"1.1\"},\"metadata\":[{\"name\":\"commit\",\"value\":\"commit_sha\"},{\"name\":\"author\",\"value\":\"Han Solo\"},{\"name\":\"url\",\"value\":\"https://gitlab.com/group/repo\"},{\"name\":\"mr_iid\",\"value\":\"1\"},{\"name\":\"project_id\",\"value\":\"20000\"}]}");
}

#[test]
fn test_serde_tostring() {
    let value = String::from("test");
    assert_eq!(serde_json::to_value(&value).unwrap(), "test");
    assert_eq!(
        serde_json::to_string(&value)
            .unwrap()
            .strip_prefix('"')
            .unwrap()
            .strip_suffix('"')
            .unwrap(),
        "test"
    );
}
