# LinodeType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID representing the Linode Type. | [optional][readonly]
**label** | Option<**String**> | The Linode Type's label is for display purposes only.  | [optional][readonly]
**disk** | Option<**i32**> | The Disk size, in MB, of the Linode Type.  | [optional][readonly]
**class** | Option<**String**> | The class of the Linode Type. We currently offer five classes of Linodes:    * nanode - Nanode instances are good for low-duty workloads,     where performance isn't critical. **Note:** As of June 16th, 2020, Nanodes became     1 GB Linodes in the Cloud Manager, however, the API, the CLI, and billing will     continue to refer to these instances as Nanodes.   * standard - Standard Shared instances are good for medium-duty workloads and     are a good mix of performance, resources, and price. **Note:** As of June 16th, 2020,     Standard Linodes in the Cloud Manager became Shared Linodes, however, the API, the CLI, and     billing will continue to refer to these instances as Standard Linodes.   * dedicated - Dedicated CPU instances are good for full-duty workloads     where consistent performance is important.   * gpu - Linodes with dedicated NVIDIA Quadro &reg; RTX 6000 GPUs accelerate highly     specialized applications such as machine learning, AI, and video transcoding.   * highmem - High Memory instances favor RAM over other resources, and can be     good for memory hungry use cases like caching and in-memory databases.     All High Memory plans contain dedicated CPU cores.  | [optional][readonly]
**price** | Option<[**crate::models::LinodeTypePrice**](LinodeType_price.md)> |  | [optional]
**addons** | Option<[**crate::models::LinodeTypeAddons**](LinodeType_addons.md)> |  | [optional]
**network_out** | Option<**i32**> | The Mbits outbound bandwidth allocation.  | [optional][readonly]
**memory** | Option<**i32**> | Amount of RAM included in this Linode Type.  | [optional][readonly]
**successor** | Option<**String**> | The Linode Type that a [mutate](/docs/api/linode-instances/#linode-upgrade) will upgrade to for a Linode of this type.  If \"null\", a Linode of this type may not mutate.  | [optional][readonly]
**transfer** | Option<**i32**> | The monthly outbound transfer amount, in MB.  | [optional][readonly]
**vcpus** | Option<**i32**> | The number of VCPU cores this Linode Type offers.  | [optional][readonly]
**gpus** | Option<**i32**> | The number of GPUs this Linode Type offers.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


