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
