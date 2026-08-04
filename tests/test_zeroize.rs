use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SensitiveData {
    username: String,
    secret: String,
}

#[test]
fn test_zeroize_deserialization_roundtrip() {
    let json_data = r#"{"username":"admin","secret":"super_secret_password\nwith_escapes\u0021"}"#;
    
    let parsed: SensitiveData = serde_json_zeroize::from_str(json_data).expect("Deserialization failed");
    assert_eq!(parsed.username, "admin");
    assert_eq!(parsed.secret, "super_secret_password\nwith_escapes!");

    let serialized = serde_json_zeroize::to_string(&parsed).expect("Serialization failed");
    assert!(serialized.contains("admin"));
    assert!(serialized.contains("super_secret_password"));
}

#[test]
fn test_zeroize_value_deserialization() {
    let json_data = r#"{"key": "value_with_escape_\t_char"}"#;
    let v: serde_json_zeroize::Value = serde_json_zeroize::from_str(json_data).expect("Value deserialization failed");
    assert_eq!(v["key"], "value_with_escape_\t_char");
}

#[test]
fn test_zeroizing_helpers() {
    let data = SensitiveData {
        username: "admin".to_string(),
        secret: "p@ssword".to_string(),
    };

    let z_str = serde_json_zeroize::to_zeroizing_string(&data).unwrap();
    assert!(z_str.contains("admin"));

    let z_vec = serde_json_zeroize::to_zeroizing_vec(&data).unwrap();
    assert!(z_vec.starts_with(b"{"));
}

#[test]
fn test_value_and_map_zeroize() {
    use zeroize::Zeroize;

    let mut val: serde_json_zeroize::Value = serde_json_zeroize::from_str(r#"{"secret": "sensitive_data"}"#).unwrap();
    assert_eq!(val["secret"], "sensitive_data");

    val.zeroize();
    assert_eq!(val, serde_json_zeroize::Value::Null);
}

