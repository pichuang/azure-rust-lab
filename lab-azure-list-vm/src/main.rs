/*
Lists the virtual , similar to:
az vm list --query [].id

RUST_LOG=azure_mgmt_compute=trace CARGO_LOG=trace  cargo run --package azure_mgmt_compute --example vm_list
*/

use azure_identity::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription().await?;
    let client = azure_mgmt_compute::Client::builder(credential).build()?;

    let mut count = 0;

    let mut vms = client.virtual_machines_client().list_all(subscription_id).into_stream();

    while let Some(vms) = vms.next().await {
        let vms = vms?;
        count += vms.value.len();
        for _vm in vms.value {
            println!("{:?}", &_vm.resource.name
            .expect(format!("Could not parse '{:?}'", stringify!($var)).as_str())
            .to_string());
        }
    }
    println!("# of virtual machines {count}");

    Ok(())
}
