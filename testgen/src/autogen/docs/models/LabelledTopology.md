# LabelledTopology

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exclusion** | [**::std::collections::HashMap<String, String>**](.md) | Excludes resources with the same $label name, eg:  \"Zone\" would not allow for resources with the same \"Zone\" value  to be used for a certain operation, eg:  A node with \"Zone: A\" would not be paired up with a node with \"Zone: A\",  but it could be paired up with a node with \"Zone: B\"  exclusive label NAME in the form \"NAME\", and not \"NAME: VALUE\" | 
**inclusion** | [**::std::collections::HashMap<String, String>**](.md) | Includes resources with the same $label or $label:$value eg:  if label is \"Zone: A\":  A resource with \"Zone: A\" would be paired up with a resource with \"Zone: A\",  but not with a resource with \"Zone: B\"  if label is \"Zone\":  A resource with \"Zone: A\" would be paired up with a resource with \"Zone: B\",  but not with a resource with \"OtherLabel: B\"  inclusive label key value in the form \"NAME: VALUE\" | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

