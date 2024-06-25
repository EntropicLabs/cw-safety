use cosmwasm_schema::serde::{de::DeserializeOwned, Serialize};
use cosmwasm_std::{to_json_string, Event, StdResult};

pub use cosmwasm_schema::schemars;
pub use cosmwasm_schema::serde;
pub use cw_events_macros::event;

pub trait TypedEvent: Serialize + DeserializeOwned {
    fn type_name(&self) -> String;

    fn as_event(&self) -> StdResult<Event> {
        let as_json = to_json_string(&self)?;
        Ok(Event::new(self.type_name()).add_attribute("_json", as_json))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::mock_dependencies;

    #[event("TestEvent")]
    struct TestEvent {
        field1: String,
        field2: i32,
    }

    #[test]
    fn test_type_name() {
        let event = TestEvent {
            field1: "test".to_string(),
            field2: 42,
        };
        assert_eq!(event.type_name(), "TestEvent");
    }

    #[test]
    fn test_as_event() {
        let mut deps = mock_dependencies();
        let event = TestEvent {
            field1: "test".to_string(),
            field2: 42,
        };
        let result = event.as_event();
        assert!(result.is_ok());
        let cosmos_event = result.unwrap();
        assert_eq!(cosmos_event.ty, "TestEvent");
        assert_eq!(cosmos_event.attributes.len(), 1);
        assert_eq!(cosmos_event.attributes[0].key, "_json");
        assert_eq!(
            cosmos_event.attributes[0].value,
            r#"{"field1":"test","field2":42}"#
        );
    }

    #[test]
    fn test_derive_serialize_deserialize() {
        let event = TestEvent {
            field1: "test".to_string(),
            field2: 42,
        };
        let serialized = serde_json_wasm::to_string(&event).unwrap();
        let deserialized: TestEvent = serde_json_wasm::from_str(&serialized).unwrap();
        assert_eq!(event.field1, deserialized.field1);
        assert_eq!(event.field2, deserialized.field2);
    }

    #[test]
    fn test_derive_clone_debug() {
        let event = TestEvent {
            field1: "test".to_string(),
            field2: 42,
        };
        let cloned_event = event.clone();
        assert_eq!(format!("{:?}", event), format!("{:?}", cloned_event));
    }

    #[test]
    fn test_json_schema() {
        let schema = schemars::schema_for!(TestEvent);
        assert!(schema.schema.object.is_some());
        let properties = schema.schema.object.unwrap().properties;
        assert!(properties.contains_key("field1"));
        assert!(properties.contains_key("field2"));
    }
}
