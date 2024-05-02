/*
CARGO_LOG=trace cargo run --package azure_mgmt_compute --example create_vm_20240501

Preparation:
- Create a resource group
- Create a virtual network
- Create a subnet
- Create NAT Gateway to fix outbound traffic issue
- Create NIC first (HARDCODE)

RUST_LOG=trace cargo run
*/

use azure_identity::AzureCliCredential;
// #[warn(unused_imports)]
// use futures::stream::StreamExt;
use std::sync::Arc;

use azure_mgmt_compute::{
    models::{
        hardware_profile::VmSize, linux_patch_settings, network_interface_reference_properties,
        network_profile::NetworkApiVersion, os_disk,
        virtual_machine_network_interface_configuration_properties,
        virtual_machine_network_interface_ip_configuration_properties, Caching, CreateOption,
        DataDisk, DeleteOption, HardwareProfile, ImageReference, LinuxConfiguration,
        LinuxPatchSettings, ManagedDiskParameters, NetworkInterfaceReference,
        NetworkInterfaceReferenceProperties, NetworkProfile, OsDisk, OsProfile, Resource,
        StorageAccountType, StorageProfile, SubResource, VirtualMachine,
        VirtualMachineNetworkInterfaceConfiguration,
        VirtualMachineNetworkInterfaceConfigurationProperties,
        VirtualMachineNetworkInterfaceDnsSettingsConfiguration,
        VirtualMachineNetworkInterfaceIpConfiguration,
        VirtualMachineNetworkInterfaceIpConfigurationProperties, VirtualMachineProperties,
    },
    // Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id: Result<String, azure_core::error::Error>  = AzureCliCredential::get_subscription();
    //shadow
    match subscription_id {
        Ok(ref id) => println!("subscription_id: {}", id),
        Err(ref e) => println!("Error: {}", e),
    }

    let client = azure_mgmt_compute::Client::builder(credential).build()?;

    // let resource_group = std::env::args().nth(1).expect("please specify resource group name");
    // let vm_name = std::env::args().nth(2).expect("please specify vm name");
    // #[warn(unused_variables)]
    // let location = std::env::args().nth(3).expect("please specify location");

    let resource_group = "rg-rust-lab-westus3".to_string();
    let vm_name = "vm-rust-lab-westus3".to_string();
    let location = "westus3".to_string();
    let subnet_id = "/subscriptions/0a4374d1-bc72-46f6-a4ae-a9d8401369db/resourceGroups/rg-rust-lab-westus3/providers/Microsoft.Network/virtualNetworks/vnet-rust-westus3/subnets/subnet-vm".to_string();
    // let nic_id = "/subscriptions/0a4374d1-bc72-46f6-a4ae-a9d8401369db/resourceGroups/rg-rust-lab-westus3/providers/Microsoft.Network/networkInterfaces/nic-rust-westus3".to_string();

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.HardwareProfile.html
    let vm_hardware_profile = HardwareProfile {
        // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/hardware_profile/enum.VmSize.html
        vm_size: Some(VmSize::StandardB1ms),
        vm_size_properties: None,
    };

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.ImageReference.html
    let image_reference = ImageReference {
        sub_resource: SubResource::new(),
        publisher: Some("canonical".to_string()),
        offer: Some("0001-com-ubuntu-minimal-jammy".to_string()),
        sku: Some("minimal-22_04-lts-gen2".to_string()),
        version: Some("22.04.202404100".to_string()),
        exact_version: None,
        shared_gallery_image_id: None,
        community_gallery_image_id: None,
    };

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.OsDisk.html
    let os_disk = OsDisk {
        os_type: Some(os_disk::OsType::Linux),
        encryption_settings: None,
        name: Some("osdisk".to_string()), //TODO
        vhd: None,
        image: None,
        caching: Some(Caching::ReadWrite),
        write_accelerator_enabled: Some(false),
        diff_disk_settings: None,
        create_option: CreateOption::FromImage,
        disk_size_gb: None,
        // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.ManagedDiskParameters.html
        managed_disk: Some(ManagedDiskParameters {
            sub_resource: azure_mgmt_compute::models::SubResource { id: None },
            storage_account_type: Some(StorageAccountType::PremiumLrs),
            disk_encryption_set: None,
            security_profile: None,
        }),
        delete_option: Some(DeleteOption::Delete),
    };

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.DataDisk.html
    let data_disk_1 = DataDisk {
        lun: 0,
        name: Some("datadisk1".to_string()), //TODO
        vhd: None,
        image: None,
        caching: Some(Caching::ReadWrite),
        write_accelerator_enabled: Some(false),
        create_option: CreateOption::Empty,
        disk_size_gb: Some(10),
        // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.ManagedDiskParameters.html
        managed_disk: Some(ManagedDiskParameters {
            sub_resource: azure_mgmt_compute::models::SubResource { id: None },
            storage_account_type: Some(StorageAccountType::PremiumLrs),
            disk_encryption_set: None,
            security_profile: None,
        }),
        to_be_detached: None,
        disk_iops_read_write: None,
        disk_m_bps_read_write: None,
        detach_option: None,
        delete_option: Some(DeleteOption::Delete),
    };

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.StorageProfile.html
    let storage_profile = StorageProfile {
        image_reference: Some(image_reference),
        os_disk: Some(os_disk),
        data_disks: vec![data_disk_1],
        disk_controller_type: None,
    };

    // https://docs.rs/azure_mgmt_compute/0.20.0/azure_mgmt_compute/package_2023_09_01/models/struct.NetworkInterfaceReference.html
    // let _network_interface_primary = NetworkInterfaceReference {
    //     sub_resource: azure_mgmt_compute::models::SubResource {
    //         id: Some(nic_id), //HARDCODE
    //     },
    //     // properties: Some(network_interface_reference_properties_primary),
    //     // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.NetworkInterfaceReferenceProperties.html
    //     properties: Some(NetworkInterfaceReferenceProperties {
    //         primary: Some(true),
    //         delete_option: Some(network_interface_reference_properties::DeleteOption::Delete),
    //     }),
    // };

    // https://docs.rs/azure_mgmt_compute/0.20.0/azure_mgmt_compute/package_2023_09_01/models/struct.VirtualMachineNetworkInterfaceDnsSettingsConfiguration.html
    let vm_nic_dns_setting = VirtualMachineNetworkInterfaceDnsSettingsConfiguration {
        dns_servers: vec!["168.63.129.16".to_string()],
    };

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.VirtualMachineNetworkInterfaceIpConfigurationProperties.html
    let vm_nic_face_ip_configuration_propeties = VirtualMachineNetworkInterfaceIpConfigurationProperties {
        subnet: Some(azure_mgmt_compute::models::SubResource {
            id: Some(subnet_id) //HARDCODE
        }),
        primary: Some(true),
        public_ip_address_configuration: None,
        private_ip_address_version: Some(virtual_machine_network_interface_ip_configuration_properties::PrivateIpAddressVersion::IPv4),
        application_security_groups: Vec::new(),
        application_gateway_backend_address_pools: Vec::new(),
        load_balancer_backend_address_pools: Vec::new(),
    };

    let vm_nic_face_ip_configuration = VirtualMachineNetworkInterfaceIpConfiguration {
        name: "ipconfig1".to_string(),
        properties: Some(vm_nic_face_ip_configuration_propeties),
    };

    // https://docs.rs/azure_mgmt_compute/0.20.0/azure_mgmt_compute/package_2023_09_01/models/struct.VirtualMachineNetworkInterfaceConfigurationProperties.html
    let vm_nic_configuraiton_primary = VirtualMachineNetworkInterfaceConfigurationProperties {
        primary: Some(true),
        delete_option: Some(
            virtual_machine_network_interface_configuration_properties::DeleteOption::Delete,
        ),
        enable_accelerated_networking: Some(false), //Force to false because b1ms didn't support
        disable_tcp_state_tracking: None,           //Force to None
        enable_fpga: None,                          //Force to None
        enable_ip_forwarding: Some(true),
        network_security_group: None,
        dns_settings: Some(vm_nic_dns_setting),
        ip_configurations: vec![vm_nic_face_ip_configuration],
        dscp_configuration: None,
        auxiliary_mode: Some(
            virtual_machine_network_interface_configuration_properties::AuxiliaryMode::None,
        ),
        auxiliary_sku: Some(
            virtual_machine_network_interface_configuration_properties::AuxiliarySku::None,
        ),
    };

    let network_interface_configurations_primary = VirtualMachineNetworkInterfaceConfiguration {
        name: "nic1".to_string(),
        properties: Some(vm_nic_configuraiton_primary),
    };

    // https://docs.rs/azure_mgmt_compute/0.20.0/azure_mgmt_compute/package_2023_09_01/models/struct.NetworkProfile.html
    let network_profile = NetworkProfile {
        // network_interfaces: vec![_network_interface_primary],
        network_interfaces: Vec::new(),
        network_api_version: Some(NetworkApiVersion::N2020_11_01), // https://docs.rs/azure_mgmt_compute/0.20.0/azure_mgmt_compute/package_2023_09_01/models/network_profile/enum.NetworkApiVersion.html
        network_interface_configurations: vec![network_interface_configurations_primary],
    };

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.LinuxPatchSettings.html
    // let linux_patch_settings = LinuxPatchSettings {
    //     patch_mode: Some(linux_patch_settings::PatchMode::ImageDefault),
    //     assessment_mode: Some(linux_patch_settings::AssessmentMode::ImageDefault),
    //     automatic_by_platform_settings: None
    // };

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.LinuxConfiguration.html
    let linux_configuration = LinuxConfiguration {
        disable_password_authentication: Some(false),
        ssh: None,
        provision_vm_agent: None,
        // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.LinuxPatchSettings.html
        // patch_settings: Some(linux_patch_settings),
        patch_settings: Some(LinuxPatchSettings {
            patch_mode: Some(linux_patch_settings::PatchMode::ImageDefault),
            assessment_mode: Some(linux_patch_settings::AssessmentMode::ImageDefault),
            automatic_by_platform_settings: None,
        }),
        enable_vm_agent_platform_updates: None,
    };

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.OsProfile.html
    let os_profile = OsProfile {
        computer_name: Some(vm_name.clone()),
        admin_username: Some("repairman".to_string()),
        admin_password: Some("Y5Qq}oXdEHp*Pv:JUjYQ".to_string()),
        custom_data: None,
        windows_configuration: None,
        linux_configuration: Some(linux_configuration),
        secrets: Vec::new(),
        allow_extension_operations: None,
        require_guest_provision_signal: None,
    };

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.VirtualMachineProperties.html
    let properties_pinhuang = VirtualMachineProperties {
        hardware_profile: Some(vm_hardware_profile),
        storage_profile: Some(storage_profile),
        additional_capabilities: None,
        os_profile: Some(os_profile),
        network_profile: Some(network_profile),
        security_profile: None,
        diagnostics_profile: None,
        availability_set: None,
        virtual_machine_scale_set: None,
        proximity_placement_group: None,
        priority: None,
        eviction_policy: None,
        billing_profile: None,
        host: None,
        host_group: None,
        provisioning_state: None,
        instance_view: None,
        license_type: None,
        vm_id: None,
        extensions_time_budget: None,
        platform_fault_domain: None,
        scheduled_events_profile: None,
        user_data: None,
        capacity_reservation: None,
        application_profile: None,
        time_created: None,
    };

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.Resource.html
    // let resource_vm = Resource::new(location);
    let resource_vm = Resource {
        id: None,
        name: None,
        type_: None,
        location: location,
        tags: None,
    };

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_09_01/models/struct.VirtualMachine.html
    // let parameters = VirtualMachine::new(resource_vm);
    let parameters_pinhuang = VirtualMachine {
        resource: resource_vm,
        plan: None,
        properties: Some(properties_pinhuang),
        resources: Vec::new(),
        identity: None,
        zones: Vec::new(),
        extended_location: None,
        managed_by: None,
        etag: None,
    };

    // println!("{parameters_pinhuang:#?}");

    // https://docs.rs/azure_mgmt_compute/latest/azure_mgmt_compute/package_2023_10_02/virtual_machines/struct.Client.html#method.create_or_update


    let _vm = client
    .virtual_machines_client()
    .create_or_update(
        resource_group,
        vm_name,
        parameters_pinhuang,
        subscription_id.unwrap(),
    )
    .send();
    //.await?;

    // Get the raw response and print the header AZURE_ASYNCOPERATION

    let binding = _vm.await.expect("SOME THING WRONG");
    let raw_response = binding.as_raw_response();

    // https://docs.rs/azure_core/latest/azure_core/struct.Response.html
    if let azure_asyncoperation = raw_response.headers().get_str(&azure_core::headers::AZURE_ASYNCOPERATION) {
        println!("{:?}", azure_asyncoperation);
    } else {
        println!("No header");
    }

    Ok(())
}
