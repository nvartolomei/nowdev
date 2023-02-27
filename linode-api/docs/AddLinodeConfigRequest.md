# AddLinodeConfigRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of this Config. | [optional][readonly]
**kernel** | Option<**String**> | A Kernel ID to boot a Linode with. Defaults to \"linode/latest-64bit\". | [optional]
**comments** | Option<**String**> | Optional field for arbitrary User comments on this Config. | [optional]
**memory_limit** | Option<**i32**> | Defaults to the total RAM of the Linode.  | [optional]
**run_level** | Option<**String**> | Defines the state of your Linode after booting. Defaults to `default`.  | [optional]
**virt_mode** | Option<**String**> | Controls the virtualization mode. Defaults to `paravirt`. * `paravirt` is suitable for most cases. Linodes running in paravirt mode   share some qualities with the host, ultimately making it run faster since   there is less transition between it and the host. * `fullvirt` affords more customization, but is slower because 100% of the VM   is virtualized.  | [optional]
**interfaces** | Option<[**Vec<crate::models::CreateLinodeInstanceRequestAllOf1InterfacesInner>**](createLinodeInstance_request_allOf_1_interfaces_inner.md)> | An array of Network Interfaces to add to this Linode's Configuration Profile.  Up to three interface objects can be entered in this array. The position in the array determines the interface to which the settings apply:  - First/0:  eth0 - Second/1: eth1 - Third/2:  eth2  When updating a Linode's interfaces, *each interface must be redefined*. An empty interfaces array results in a default public interface configuration only.  If no public interface is configured, public IP addresses are still assigned to the Linode but will not be usable without manual configuration.  **Note:** Changes to Linode interface configurations can be enabled by rebooting the Linode.  **Note:** Only Next Generation Network (NGN) data centers support VLANs. Use the Regions ([/regions](/docs/api/regions/)) endpoint to view the capabilities of data center regions. If a VLAN is attached to your Linode and you attempt to migrate or clone it to a non-NGN data center, the migration or cloning will not initiate. If a Linode cannot be migrated because of an incompatibility, you will be prompted to select a different data center or contact support.  **Note:** See the [VLANs Overview](/docs/products/networking/vlans/#technical-specifications) guide to view additional specifications and limitations.  | [optional]
**helpers** | Option<[**crate::models::GetLinodeConfigs200ResponseDataInnerHelpers**](getLinodeConfigs_200_response_data_inner_helpers.md)> |  | [optional]
**label** | **String** | The Config's label is for display purposes only.  | 
**devices** | [**crate::models::GetLinodeConfigs200ResponseDataInnerDevices**](getLinodeConfigs_200_response_data_inner_devices.md) |  | 
**root_device** | Option<**String**> | The root device to boot. * If no value or an invalid value is provided, root device will default to `/dev/sda`. * If the device specified at the root device location is not mounted, the Linode will not boot until a device is mounted.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


