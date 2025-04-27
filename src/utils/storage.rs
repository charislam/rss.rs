use leptos::{leptos_dom::logging::console_error, prelude::*};
use serde::de::DeserializeOwned;

pub(crate) fn get_local_storage_key<T: DeserializeOwned>(key: &str) -> Option<T> {
    match window().local_storage() {
        Ok(Some(storage)) => match storage.get_item(key) {
            Ok(Some(value)) => match serde_json::from_str::<T>(&value) {
                Ok(deserialized) => Some(deserialized),
                Err(error) => {
                    console_error(&format!("Error deserializing key {key} from local storage: {error:?}"));
                    None
                },
            },
            Ok(None) => None,
            Err(error) => {
                console_error(&format!("Error getting key {key} from local storage: {error:?}"));
                None
            },
        },
        Ok(None) => None,
        Err(error) => {
            console_error(&format!("Error accessing local storage: {error:?}"));
            None
        }
    }
}