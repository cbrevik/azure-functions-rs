use azure_functions::{bindings::DurableOrchestrationContext, func};
use log::{error, info};
use serde_json::Value;

#[func]
pub async fn counter_orchestration(context: DurableOrchestrationContext) {
    let v = context
        .call_entity("counter", "foo", "get", Value::Null)
        .await;

    // var entityId = new EntityId(nameof(Counter), "myCounter");

    //     // Two-way call to the entity which returns a value - awaits the response
    //     int currentValue = await context.CallEntityAsync<int>(entityId, "Get");
    //     if (currentValue < 10)
    //     {
    //         // One-way signal to the entity which updates the value - does not await a response
    //         context.SignalEntity(entityId, "Add", 1);
    //     }
    // }
}
