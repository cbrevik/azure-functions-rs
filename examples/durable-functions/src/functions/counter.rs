use azure_functions::{bindings::DurableEntityContext, func};

#[func]
pub fn counter(context: DurableEntityContext) {
    // switch (ctx.OperationName.ToLowerInvariant())
    // {
    //     case "add":
    //         ctx.SetState(ctx.GetState<int>() + ctx.GetInput<int>());
    //         break;
    //     case "reset":
    //         ctx.SetState(0);
    //         break;
    //     case "get":
    //         ctx.Return(ctx.GetState<int>()));
    //         break;
    // }
}
