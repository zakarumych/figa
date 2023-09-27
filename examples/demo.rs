use std::collections::BTreeMap;

#[derive(figa::Figa, serde_derive::Serialize, serde_derive::Deserialize)]
struct ConfigValue {
    foo: String,
    #[figa(append)]
    bar: String,
}

#[derive(figa::Figa, serde_derive::Serialize, serde_derive::Deserialize)]
struct DemoConfig {
    a: u32,
    b: String,
    c: Vec<u32>,
    #[figa(replace)]
    d: Vec<String>,
    #[figa(update)]
    e: BTreeMap<String, ConfigValue>,
    #[figa(append)]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    f: Option<String>,
}

#[derive(figa::Figa, serde_derive::Deserialize)]
struct DemoConfig2(
    u32,
    String,
    Vec<u32>,
    #[figa(replace)] Vec<String>,
    #[figa(update)] BTreeMap<String, ConfigValue>,
);

fn main() {
    let cfg1 =
        r#"{"a":1,"b":"qwe","c":[1,2,3],"d":["asd","zxc"],"e":{"ccc":{"foo":"jkl","bar":"xcv"}}}"#;
    let cfg2_update = r#"{}"#;
    let cfg2 =
        r#"{"a":1,"b":"qwe","c":[1,2,3],"d":["asd","zxc"],"e":{"ccc":{"foo":"jkl","bar":"xcv"}}}"#;
    let cfg3_update = r#"{"a":2}"#;
    let cfg3 =
        r#"{"a":2,"b":"qwe","c":[1,2,3],"d":["asd","zxc"],"e":{"ccc":{"foo":"jkl","bar":"xcv"}}}"#;
    let cfg4_update = r#"{"b":"asd"}"#;
    let cfg4 =
        r#"{"a":2,"b":"asd","c":[1,2,3],"d":["asd","zxc"],"e":{"ccc":{"foo":"jkl","bar":"xcv"}}}"#;
    let cfg5_update = r#"{"c":[4,5,6]}"#;
    let cfg5 = r#"{"a":2,"b":"asd","c":[1,2,3,4,5,6],"d":["asd","zxc"],"e":{"ccc":{"foo":"jkl","bar":"xcv"}}}"#;
    let cfg6_update = r#"{"d":["qwe","bbb","uio"]}"#;
    let cfg6 = r#"{"a":2,"b":"asd","c":[1,2,3,4,5,6],"d":["qwe","bbb","uio"],"e":{"ccc":{"foo":"jkl","bar":"xcv"}}}"#;
    let cfg7_update = r#"{"e":{"aaa":{"foo":"pop","bar":"nop"}}}"#;
    let cfg7 = r#"{"a":2,"b":"asd","c":[1,2,3,4,5,6],"d":["qwe","bbb","uio"],"e":{"aaa":{"foo":"pop","bar":"nop"},"ccc":{"foo":"jkl","bar":"xcv"}}}"#;
    let cfg8_update = r#"{"e":{"ccc":{"foo":"ghj"}}}"#;
    let cfg8 = r#"{"a":2,"b":"asd","c":[1,2,3,4,5,6],"d":["qwe","bbb","uio"],"e":{"aaa":{"foo":"pop","bar":"nop"},"ccc":{"foo":"ghj","bar":"xcv"}}}"#;
    let cfg9_update = r#"{"e":{"ccc":{"bar":"zxc"}}}"#;
    let cfg9 = r#"{"a":2,"b":"asd","c":[1,2,3,4,5,6],"d":["qwe","bbb","uio"],"e":{"aaa":{"foo":"pop","bar":"nop"},"ccc":{"foo":"ghj","bar":"xcvzxc"}}}"#;
    let cfg10_update = r#"{"f":"qwe"}"#;
    let cfg10 = r#"{"a":2,"b":"asd","c":[1,2,3,4,5,6],"d":["qwe","bbb","uio"],"e":{"aaa":{"foo":"pop","bar":"nop"},"ccc":{"foo":"ghj","bar":"xcvzxc"}},"f":"qwe"}"#;
    std::env::set_var("FIGA_DEMO_D", "y , z, \"q\\x20\"");
    let cfg11 = r#"{"a":2,"b":"asd","c":[1,2,3,4,5,6],"d":["y","z","q "],"e":{"aaa":{"foo":"pop","bar":"nop"},"ccc":{"foo":"ghj","bar":"xcvzxc"}},"f":"qwe"}"#;

    let mut cfg: DemoConfig = serde_json::from_str(cfg1).unwrap();

    figa::Figa::update(
        &mut cfg,
        &mut serde_json::Deserializer::from_str(cfg2_update),
    )
    .unwrap();
    assert_eq!(serde_json::to_string(&cfg).unwrap(), cfg2);

    figa::Figa::update(
        &mut cfg,
        &mut serde_json::Deserializer::from_str(cfg3_update),
    )
    .unwrap();
    assert_eq!(serde_json::to_string(&cfg).unwrap(), cfg3);

    figa::Figa::update(
        &mut cfg,
        &mut serde_json::Deserializer::from_str(cfg4_update),
    )
    .unwrap();
    assert_eq!(serde_json::to_string(&cfg).unwrap(), cfg4);

    figa::Figa::update(
        &mut cfg,
        &mut serde_json::Deserializer::from_str(cfg5_update),
    )
    .unwrap();
    assert_eq!(serde_json::to_string(&cfg).unwrap(), cfg5);

    figa::Figa::update(
        &mut cfg,
        &mut serde_json::Deserializer::from_str(cfg6_update),
    )
    .unwrap();
    assert_eq!(serde_json::to_string(&cfg).unwrap(), cfg6);

    figa::Figa::update(
        &mut cfg,
        &mut serde_json::Deserializer::from_str(cfg7_update),
    )
    .unwrap();
    assert_eq!(serde_json::to_string(&cfg).unwrap(), cfg7);

    figa::Figa::update(
        &mut cfg,
        &mut serde_json::Deserializer::from_str(cfg8_update),
    )
    .unwrap();
    assert_eq!(serde_json::to_string(&cfg).unwrap(), cfg8);

    figa::Figa::update(
        &mut cfg,
        &mut serde_json::Deserializer::from_str(cfg9_update),
    )
    .unwrap();
    assert_eq!(serde_json::to_string(&cfg).unwrap(), cfg9);

    figa::Figa::update(
        &mut cfg,
        &mut serde_json::Deserializer::from_str(cfg10_update),
    )
    .unwrap();
    assert_eq!(serde_json::to_string(&cfg).unwrap(), cfg10);

    figa::Figa::update(
        &mut cfg,
        denvars::Deserializer::from_prefixed_env_vars("FIGA_DEMO_"),
    )
    .unwrap();
    assert_eq!(serde_json::to_string(&cfg).unwrap(), cfg11);
}
