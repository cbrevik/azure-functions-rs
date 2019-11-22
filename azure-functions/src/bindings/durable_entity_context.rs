use crate::durable::{EntityId, EntityState};
use crate::rpc::{typed_data::Data, TypedData};
use serde::Deserialize;
use serde_json::{from_str, Number, Value};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Represents the Durable Functions entity context binding.
///
/// The following binding attributes are supported:
///
/// | Name       | Description                                                      |
/// |------------|------------------------------------------------------------------|
/// | `name`     | The name of the parameter being bound.                           |
///
/// # Examples
///
/// TODO: implement
#[derive(Clone)]
pub struct DurableEntityContext {
    state: Rc<RefCell<EntityState>>,
}

impl DurableEntityContext {
    #[doc(hidden)]
    pub fn new(data: TypedData, _metadata: HashMap<String, TypedData>) -> Self {
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct Operation {
            name: Option<String>,
            signal: Option<bool>,
            input: Option<String>,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct BindingData {
            #[serde(rename = "self")]
            id: EntityId,
            exists: bool,
            state: Option<String>,
            batch: Vec<Operation>,
        }

        match &data.data {
            Some(Data::String(s)) => {
                let data: BindingData = from_str(s).expect("failed to parse entity context data");

                println!("{:#?}", data);

                Self {
                    // TODO: FIX ME!
                    state: Rc::new(RefCell::new(EntityState::new(data.exists, data.state))),
                }
            }
            _ => panic!("expected JSON data for entity context data"),
        }
    }

    #[doc(hidden)]
    pub fn state(&self) -> Rc<RefCell<EntityState>> {
        self.state.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::typed_data::Data;

    // #[test]
    // fn it_constructs() {
    //     let data = TypedData {
    //         data: Some(Data::String("bar".to_string())),
    //     };

    //     let mut metadata = HashMap::new();
    //     metadata.insert(
    //         INSTANCE_ID_KEY.to_string(),
    //         TypedData {
    //             data: Some(Data::String("foo".to_string())),
    //         },
    //     );

    //     let context = DurableEntityContext::new(data, metadata);
    // }
}
