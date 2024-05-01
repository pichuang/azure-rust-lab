/*
Lists the virtual , similar to:
az vm list --query [].id

cargo run --package azure_mgmt_compute --example check_provioningstat.rs
*/
use azure_identity::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription().await?;
    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_10_02/struct.Client.html
    let client = azure_mgmt_compute::Client::builder(credential).build()?;
    let resource_group_name: String = "RG-HUB-ER-TAIWANNORTH".to_string();

    let mut count = 0;
    let mut vms = client.virtual_machines_client().list_all(subscription_id.clone()).into_stream();
    while let Some(vms) = vms.next().await {
        let vms = vms?;
        count += vms.value.len();
        for vm in vms.value {
            // println!("VM Name: {:?} / Provisionging Stats: {:?} / CreatedTime: {:?}", &vm.resource.name, &vm.properties.clone().unwrap().provisioning_state, &vm.properties.unwrap().time_created);
            match &vm.properties.clone().unwrap().provisioning_state {
                //https://learn.microsoft.com/en-us/azure/virtual-machines/states-billing#provisioning-states
                #[allow(non_camel_case_types)]
                Succeeded => {
                    println!("VM Name: {:?} / Provisionging Stats: {:?} / CreatedTime: {:?}", &vm.resource.name, &vm.properties.clone().unwrap().provisioning_state, &vm.properties.unwrap().time_created);
                }
                #[allow(non_camel_case_types)]
                Creating => {
                    // Virtual machine is being created.
                    // Action: Wait 3 minutes, and check again manually
                    // TODO
                    println!("VM Name: {:?} / Provisionging Stats: {:?} / CreatedTime: {:?}", &vm.resource.name, &vm.properties.clone().unwrap().provisioning_state, &vm.properties.unwrap().time_created);
                }
                #[allow(non_camel_case_types)]
                Updating => {
                    // Virtual machine is updating to the latest model. Some non-model changes to a virtual machine such as start and restart fall under the updating state.
                    // Action: Wait 3 minutes, and check again manually
                    // TODO
                    println!("VM Name: {:?} / Provisionging Stats: {:?} / CreatedTime: {:?}", &vm.resource.name, &vm.properties.clone().unwrap().provisioning_state, &vm.properties.unwrap().time_created);
                }
                #[allow(non_camel_case_types)]
                Failed => {
                    //	Last operation on the virtual machine resource was unsuccessful.
                    // Action: Delete or Redeploy immediately
                    //
                    // 1. Redeploy: Shuts down the virtual machine, moves it to a new node, and powers it back on.
                    //
                    // client.virtual_machines_client().redeploy(
                    //     subscription_id.clone(),
                    //     resource_group_name.clone(),
                    //     &vm.resource.name.expect("CAN NOT FIND VM").to_string());
                    //
                    // 2. Delete: The operation to delete a virtual machine.
                    //
                    client.virtual_machines_client().delete(
                        subscription_id.clone(),
                        resource_group_name.clone(),
                        &vm.resource.name.expect("CAN NOT FIND VM").to_string());
                }
                #[allow(non_camel_case_types)]
                Deleting => {
                    // Virtual machine is being deleted.
                    // Action: None
                    println!("VM Name: {:?} / Provisionging Stats: {:?} / CreatedTime: {:?}", &vm.resource.name, &vm.properties.clone().unwrap().provisioning_state, &vm.properties.unwrap().time_created);
                }
                #[allow(non_camel_case_types)]
                Migrating => {
                    // Seen when migrating from Azure Service Manager to Azure Resource Manager.
                    // Action: None
                    println!("VM Name: {:?} / Provisionging Stats: {:?} / CreatedTime: {:?}", &vm.resource.name, &vm.properties.clone().unwrap().provisioning_state, &vm.properties.unwrap().time_created);
                }
                _ => {
                    println!("Ain't special");
                }

            }
       }
    }
    println!("# of virtual machines {count}");

    Ok(())
}
