extern crate argonaut;
extern crate value_trait;
extern crate simd_json;

use value_trait::*;
use value_trait::Value;
use value_trait::Mutable;
use value_trait::Builder;
use simd_json::value::owned::Value as OwnedValue;

#[derive(Debug)]
enum Side {
    Light,
    Dark
}

struct Jedi {
    name: String,
    side: Side
}

fn jedi_array() -> Vec<Jedi> {
    vec![
        Jedi { name: "Saes Rrogon".to_string(), side: Side::Dark },
        Jedi { name: "Qui-Gon Jinn".to_string(), side: Side::Light },
        Jedi { name: "Obi-Wan Kenobi".to_string(), side: Side::Light }
    ]
}

#[test]
fn simple_array_of_objects() {
    let jedi_array = jedi_array();

    let json = argonaut::array(|json| {
        json.objects(jedi_array.iter(), |jedi, json| {
            match jedi.side {
                Side::Light => {
                    json.set("name".to_string(), OwnedValue::from(jedi.name.to_string()));
                    json.set("side".to_string(), OwnedValue::from(format!("{:?}", jedi.side)));
                },
                Side::Dark => json.skip()
            }
        })
    }).unwrap();

    let array = json.as_array().unwrap();

    assert_eq!(array.len(), 2);
}

#[test]
fn simple_array_of_arrays() {
    let jedi_array = jedi_array();

    let json = argonaut::array(|json| {
        json.objects(jedi_array.iter(), |jedi, json| {
            match jedi.side {
                Side::Light => {
                    json.set("name".to_string(), OwnedValue::from(jedi.name.to_string()));
                    json.set("side".to_string(), OwnedValue::from(format!("{:?}", jedi.side)));
                },
                Side::Dark => json.skip()
            }
        })
    }).unwrap();

    let array = json.as_array().unwrap();
    assert_eq!(array.len(), 2);
}
