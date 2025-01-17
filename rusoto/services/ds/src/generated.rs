// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl DirectoryServiceClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "ds", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptSharedDirectoryRequest {
    /// <p>Identifier of the shared directory in the directory consumer account. This identifier is different for each directory owner account. </p>
    #[serde(rename = "sharedDirectoryId")]
    pub shared_directory_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptSharedDirectoryResult {
    /// <p>The shared directory in the directory consumer account.</p>
    #[serde(rename = "sharedDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory: Option<SharedDirectory>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddIpRoutesRequest {
    /// <p>Identifier (ID) of the directory to which to add the address block.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>IP address blocks, using CIDR format, of the traffic to route. This is often the IP address block of the DNS server used for your on-premises domain.</p>
    #[serde(rename = "ipRoutes")]
    pub ip_routes: Vec<IpRoute>,
    /// <p>If set to true, updates the inbound and outbound rules of the security group that has the description: "AWS created security group for <i>directory ID</i> directory controllers." Following are the new rules: </p> <p>Inbound:</p> <ul> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 88, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 123, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 138, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 389, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 464, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 445, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 88, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 135, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 445, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 464, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 636, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 1024-65535, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 3268-33269, Source: 0.0.0.0/0</p> </li> <li> <p>Type: DNS (UDP), Protocol: UDP, Range: 53, Source: 0.0.0.0/0</p> </li> <li> <p>Type: DNS (TCP), Protocol: TCP, Range: 53, Source: 0.0.0.0/0</p> </li> <li> <p>Type: LDAP, Protocol: TCP, Range: 389, Source: 0.0.0.0/0</p> </li> <li> <p>Type: All ICMP, Protocol: All, Range: N/A, Source: 0.0.0.0/0</p> </li> </ul> <p/> <p>Outbound:</p> <ul> <li> <p>Type: All traffic, Protocol: All, Range: All, Destination: 0.0.0.0/0</p> </li> </ul> <p>These security rules impact an internal network interface that is not exposed publicly.</p>
    #[serde(rename = "updateSecurityGroupForDirectoryControllers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_security_group_for_directory_controllers: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddIpRoutesResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddRegionRequest {
    /// <p>The identifier of the directory to which you want to add Region replication.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The name of the Region where you want to add domain controllers for replication. For example, <code>us-east-1</code>.</p>
    #[serde(rename = "regionName")]
    pub region_name: String,
    #[serde(rename = "vPCSettings")]
    pub vpc_settings: DirectoryVpcSettings,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddRegionResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsToResourceRequest {
    /// <p>Identifier (ID) for the directory to which to add the tag.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>The tags to be assigned to the directory.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddTagsToResourceResult {}

/// <p>Represents a named directory attribute.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Attribute {
    /// <p>The name of the attribute.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the attribute.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelSchemaExtensionRequest {
    /// <p>The identifier of the directory whose schema extension will be canceled.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The identifier of the schema extension that will be canceled.</p>
    #[serde(rename = "schemaExtensionId")]
    pub schema_extension_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelSchemaExtensionResult {}

/// <p>Information about the certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Certificate {
    /// <p>The identifier of the certificate.</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    /// <p>A <code>ClientCertAuthSettings</code> object that contains client certificate authentication settings.</p>
    #[serde(rename = "clientCertAuthSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cert_auth_settings: Option<ClientCertAuthSettings>,
    /// <p>The common name for the certificate.</p>
    #[serde(rename = "commonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// <p>The date and time when the certificate will expire.</p>
    #[serde(rename = "expiryDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date_time: Option<f64>,
    /// <p>The date and time that the certificate was registered.</p>
    #[serde(rename = "registeredDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_date_time: Option<f64>,
    /// <p>The state of the certificate.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Describes a state change for the certificate.</p>
    #[serde(rename = "stateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    /// <p>The function that the registered certificate performs. Valid values include <code>ClientLDAPS</code> or <code>ClientCertAuth</code>. The default value is <code>ClientLDAPS</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains general information about a certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CertificateInfo {
    /// <p>The identifier of the certificate.</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    /// <p>The common name for the certificate.</p>
    #[serde(rename = "commonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// <p>The date and time when the certificate will expire.</p>
    #[serde(rename = "expiryDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date_time: Option<f64>,
    /// <p>The state of the certificate.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The function that the registered certificate performs. Valid values include <code>ClientLDAPS</code> or <code>ClientCertAuth</code>. The default value is <code>ClientLDAPS</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains information about the client certificate authentication settings for the <code>RegisterCertificate</code> and <code>DescribeCertificate</code> operations. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ClientCertAuthSettings {
    /// <p>Specifies the URL of the default OCSP server used to check for revocation status. A secondary value to any OCSP address found in the AIA extension of the user certificate.</p>
    #[serde(rename = "oCSPUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_url: Option<String>,
}

/// <p>Contains information about a computer account in a directory.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Computer {
    /// <p>An array of <a>Attribute</a> objects containing the LDAP attributes that belong to the computer account.</p>
    #[serde(rename = "computerAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_attributes: Option<Vec<Attribute>>,
    /// <p>The identifier of the computer.</p>
    #[serde(rename = "computerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_id: Option<String>,
    /// <p>The computer name.</p>
    #[serde(rename = "computerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
}

/// <p>Points to a remote domain with which you are setting up a trust relationship. Conditional forwarders are required in order to set up a trust relationship with another domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConditionalForwarder {
    /// <p>The IP addresses of the remote DNS server associated with RemoteDomainName. This is the IP address of the DNS server that your conditional forwarder points to.</p>
    #[serde(rename = "dnsIpAddrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addrs: Option<Vec<String>>,
    /// <p>The fully qualified domain name (FQDN) of the remote domains pointed to by the conditional forwarder.</p>
    #[serde(rename = "remoteDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_name: Option<String>,
    /// <p>The replication scope of the conditional forwarder. The only allowed value is <code>Domain</code>, which will replicate the conditional forwarder to all of the domain controllers for your AWS directory.</p>
    #[serde(rename = "replicationScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_scope: Option<String>,
}

/// <p>Contains the inputs for the <a>ConnectDirectory</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConnectDirectoryRequest {
    /// <p>A <a>DirectoryConnectSettings</a> object that contains additional information for the operation.</p>
    #[serde(rename = "connectSettings")]
    pub connect_settings: DirectoryConnectSettings,
    /// <p>A description for the directory.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The fully qualified name of the on-premises directory, such as <code>corp.example.com</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The password for the on-premises user account.</p>
    #[serde(rename = "password")]
    pub password: String,
    /// <p>The NetBIOS name of the on-premises directory, such as <code>CORP</code>.</p>
    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// <p>The size of the directory.</p>
    #[serde(rename = "size")]
    pub size: String,
    /// <p>The tags to be assigned to AD Connector.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Contains the results of the <a>ConnectDirectory</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConnectDirectoryResult {
    /// <p>The identifier of the new directory.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

/// <p>Contains the inputs for the <a>CreateAlias</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAliasRequest {
    /// <p>The requested alias.</p> <p>The alias must be unique amongst all aliases in AWS. This operation throws an <code>EntityAlreadyExistsException</code> error if the alias already exists.</p>
    #[serde(rename = "alias")]
    pub alias: String,
    /// <p>The identifier of the directory for which to create the alias.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
}

/// <p>Contains the results of the <a>CreateAlias</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAliasResult {
    /// <p>The alias for the directory.</p>
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

/// <p>Contains the inputs for the <a>CreateComputer</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateComputerRequest {
    /// <p>An array of <a>Attribute</a> objects that contain any LDAP attributes to apply to the computer account.</p>
    #[serde(rename = "computerAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_attributes: Option<Vec<Attribute>>,
    /// <p>The name of the computer account.</p>
    #[serde(rename = "computerName")]
    pub computer_name: String,
    /// <p>The identifier of the directory in which to create the computer account.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The fully-qualified distinguished name of the organizational unit to place the computer account in.</p>
    #[serde(rename = "organizationalUnitDistinguishedName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<String>,
    /// <p>A one-time password that is used to join the computer to the directory. You should generate a random, strong password to use for this parameter.</p>
    #[serde(rename = "password")]
    pub password: String,
}

/// <p>Contains the results for the <a>CreateComputer</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateComputerResult {
    /// <p>A <a>Computer</a> object that represents the computer account.</p>
    #[serde(rename = "computer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer: Option<Computer>,
}

/// <p>Initiates the creation of a conditional forwarder for your AWS Directory Service for Microsoft Active Directory. Conditional forwarders are required in order to set up a trust relationship with another domain.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConditionalForwarderRequest {
    /// <p>The directory ID of the AWS directory for which you are creating the conditional forwarder.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p>
    #[serde(rename = "dnsIpAddrs")]
    pub dns_ip_addrs: Vec<String>,
    /// <p>The fully qualified domain name (FQDN) of the remote domain with which you will set up a trust relationship.</p>
    #[serde(rename = "remoteDomainName")]
    pub remote_domain_name: String,
}

/// <p>The result of a CreateConditinalForwarder request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConditionalForwarderResult {}

/// <p>Contains the inputs for the <a>CreateDirectory</a> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDirectoryRequest {
    /// <p>A description for the directory.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The fully qualified name for the directory, such as <code>corp.example.com</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The password for the directory administrator. The directory creation process creates a directory administrator account with the user name <code>Administrator</code> and this password.</p> <p>If you need to change the password for the administrator account, you can use the <a>ResetUserPassword</a> API call.</p> <p>The regex pattern for this string is made up of the following conditions:</p> <ul> <li> <p>Length (?=^.{8,64}$) – Must be between 8 and 64 characters</p> </li> </ul> <p>AND any 3 of the following password complexity rules required by Active Directory:</p> <ul> <li> <p>Numbers and upper case and lowercase (?=.*\d)(?=.*[A-Z])(?=.*[a-z])</p> </li> <li> <p>Numbers and special characters and lower case (?=.*\d)(?=.*[^A-Za-z0-9\s])(?=.*[a-z])</p> </li> <li> <p>Special characters and upper case and lower case (?=.*[^A-Za-z0-9\s])(?=.*[A-Z])(?=.*[a-z])</p> </li> <li> <p>Numbers and upper case and special characters (?=.*\d)(?=.*[A-Z])(?=.*[^A-Za-z0-9\s])</p> </li> </ul> <p>For additional information about how Active Directory passwords are enforced, see <a href="https://docs.microsoft.com/en-us/windows/security/threat-protection/security-policy-settings/password-must-meet-complexity-requirements">Password must meet complexity requirements</a> on the Microsoft website.</p>
    #[serde(rename = "password")]
    pub password: String,
    /// <p>The NetBIOS name of the directory, such as <code>CORP</code>.</p>
    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// <p>The size of the directory.</p>
    #[serde(rename = "size")]
    pub size: String,
    /// <p>The tags to be assigned to the Simple AD directory.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A <a>DirectoryVpcSettings</a> object that contains additional information for the operation.</p>
    #[serde(rename = "vpcSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettings>,
}

/// <p>Contains the results of the <a>CreateDirectory</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDirectoryResult {
    /// <p>The identifier of the directory that was created.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLogSubscriptionRequest {
    /// <p>Identifier of the directory to which you want to subscribe and receive real-time logs to your specified CloudWatch log group.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The name of the CloudWatch log group where the real-time domain controller logs are forwarded.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLogSubscriptionResult {}

/// <p>Creates an AWS Managed Microsoft AD directory.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMicrosoftADRequest {
    /// <p>A description for the directory. This label will appear on the AWS console <code>Directory Details</code> page after the directory is created.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>AWS Managed Microsoft AD is available in two editions: <code>Standard</code> and <code>Enterprise</code>. <code>Enterprise</code> is the default.</p>
    #[serde(rename = "edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    /// <p>The fully qualified domain name for the AWS Managed Microsoft AD directory, such as <code>corp.example.com</code>. This name will resolve inside your VPC only. It does not need to be publicly resolvable.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The password for the default administrative user named <code>Admin</code>.</p> <p>If you need to change the password for the administrator account, you can use the <a>ResetUserPassword</a> API call.</p>
    #[serde(rename = "password")]
    pub password: String,
    /// <p>The NetBIOS name for your domain, such as <code>CORP</code>. If you don't specify a NetBIOS name, it will default to the first part of your directory DNS. For example, <code>CORP</code> for the directory DNS <code>corp.example.com</code>. </p>
    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// <p>The tags to be assigned to the AWS Managed Microsoft AD directory.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Contains VPC information for the <a>CreateDirectory</a> or <a>CreateMicrosoftAD</a> operation.</p>
    #[serde(rename = "vpcSettings")]
    pub vpc_settings: DirectoryVpcSettings,
}

/// <p>Result of a CreateMicrosoftAD request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMicrosoftADResult {
    /// <p>The identifier of the directory that was created.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

/// <p>Contains the inputs for the <a>CreateSnapshot</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSnapshotRequest {
    /// <p>The identifier of the directory of which to take a snapshot.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The descriptive name to apply to the snapshot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains the results of the <a>CreateSnapshot</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSnapshotResult {
    /// <p>The identifier of the snapshot that was created.</p>
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

/// <p>AWS Directory Service for Microsoft Active Directory allows you to configure trust relationships. For example, you can establish a trust between your AWS Managed Microsoft AD directory, and your existing on-premises Microsoft Active Directory. This would allow you to provide users and groups access to resources in either domain, with a single set of credentials.</p> <p>This action initiates the creation of the AWS side of a trust relationship between an AWS Managed Microsoft AD directory and an external domain.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTrustRequest {
    /// <p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p>
    #[serde(rename = "conditionalForwarderIpAddrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_forwarder_ip_addrs: Option<Vec<String>>,
    /// <p>The Directory ID of the AWS Managed Microsoft AD directory for which to establish the trust relationship.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The Fully Qualified Domain Name (FQDN) of the external domain for which to create the trust relationship.</p>
    #[serde(rename = "remoteDomainName")]
    pub remote_domain_name: String,
    /// <p>Optional parameter to enable selective authentication for the trust.</p>
    #[serde(rename = "selectiveAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_auth: Option<String>,
    /// <p>The direction of the trust relationship.</p>
    #[serde(rename = "trustDirection")]
    pub trust_direction: String,
    /// <p>The trust password. The must be the same password that was used when creating the trust relationship on the external domain.</p>
    #[serde(rename = "trustPassword")]
    pub trust_password: String,
    /// <p>The trust relationship type. <code>Forest</code> is the default.</p>
    #[serde(rename = "trustType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_type: Option<String>,
}

/// <p>The result of a CreateTrust request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTrustResult {
    /// <p>A unique identifier for the trust relationship that was created.</p>
    #[serde(rename = "trustId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}

/// <p>Deletes a conditional forwarder.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConditionalForwarderRequest {
    /// <p>The directory ID for which you are deleting the conditional forwarder.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The fully qualified domain name (FQDN) of the remote domain with which you are deleting the conditional forwarder.</p>
    #[serde(rename = "remoteDomainName")]
    pub remote_domain_name: String,
}

/// <p>The result of a DeleteConditionalForwarder request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConditionalForwarderResult {}

/// <p>Contains the inputs for the <a>DeleteDirectory</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDirectoryRequest {
    /// <p>The identifier of the directory to delete.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
}

/// <p>Contains the results of the <a>DeleteDirectory</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDirectoryResult {
    /// <p>The directory identifier.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLogSubscriptionRequest {
    /// <p>Identifier of the directory whose log subscription you want to delete.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLogSubscriptionResult {}

/// <p>Contains the inputs for the <a>DeleteSnapshot</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSnapshotRequest {
    /// <p>The identifier of the directory snapshot to be deleted.</p>
    #[serde(rename = "snapshotId")]
    pub snapshot_id: String,
}

/// <p>Contains the results of the <a>DeleteSnapshot</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSnapshotResult {
    /// <p>The identifier of the directory snapshot that was deleted.</p>
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

/// <p>Deletes the local side of an existing trust relationship between the AWS Managed Microsoft AD directory and the external domain.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTrustRequest {
    /// <p>Delete a conditional forwarder as part of a DeleteTrustRequest.</p>
    #[serde(rename = "deleteAssociatedConditionalForwarder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_associated_conditional_forwarder: Option<bool>,
    /// <p>The Trust ID of the trust relationship to be deleted.</p>
    #[serde(rename = "trustId")]
    pub trust_id: String,
}

/// <p>The result of a DeleteTrust request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTrustResult {
    /// <p>The Trust ID of the trust relationship that was deleted.</p>
    #[serde(rename = "trustId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterCertificateRequest {
    /// <p>The identifier of the certificate.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterCertificateResult {}

/// <p>Removes the specified directory as a publisher to the specified SNS topic.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterEventTopicRequest {
    /// <p>The Directory ID to remove as a publisher. This directory will no longer send messages to the specified SNS topic.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The name of the SNS topic from which to remove the directory as a publisher.</p>
    #[serde(rename = "topicName")]
    pub topic_name: String,
}

/// <p>The result of a DeregisterEventTopic request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterEventTopicResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCertificateRequest {
    /// <p>The identifier of the certificate.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCertificateResult {
    /// <p>Information about the certificate, including registered date time, certificate state, the reason for the state, expiration date time, and certificate common name.</p>
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
}

/// <p>Describes a conditional forwarder.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConditionalForwardersRequest {
    /// <p>The directory ID for which to get the list of associated conditional forwarders.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The fully qualified domain names (FQDN) of the remote domains for which to get the list of associated conditional forwarders. If this member is null, all conditional forwarders are returned.</p>
    #[serde(rename = "remoteDomainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_names: Option<Vec<String>>,
}

/// <p>The result of a DescribeConditionalForwarder request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConditionalForwardersResult {
    /// <p>The list of conditional forwarders that have been created.</p>
    #[serde(rename = "conditionalForwarders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_forwarders: Option<Vec<ConditionalForwarder>>,
}

/// <p>Contains the inputs for the <a>DescribeDirectories</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDirectoriesRequest {
    /// <p>A list of identifiers of the directories for which to obtain the information. If this member is null, all directories that belong to the current account are returned.</p> <p>An empty list results in an <code>InvalidParameterException</code> being thrown.</p>
    #[serde(rename = "directoryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return. If this value is zero, the maximum number of items is specified by the limitations of the operation.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>DescribeDirectoriesResult.NextToken</code> value from a previous call to <a>DescribeDirectories</a>. Pass null if this is the first call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains the results of the <a>DescribeDirectories</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDirectoriesResult {
    /// <p>The list of <a>DirectoryDescription</a> objects that were retrieved.</p> <p>It is possible that this list contains less than the number of items specified in the <code>Limit</code> member of the request. This occurs if there are less than the requested number of items left to retrieve, or if the limitations of the operation have been exceeded.</p>
    #[serde(rename = "directoryDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_descriptions: Option<Vec<DirectoryDescription>>,
    /// <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <a>DescribeDirectories</a> to retrieve the next set of items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDomainControllersRequest {
    /// <p>Identifier of the directory for which to retrieve the domain controller information.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>A list of identifiers for the domain controllers whose information will be provided.</p>
    #[serde(rename = "domainControllerIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controller_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <i>DescribeDomainControllers.NextToken</i> value from a previous call to <a>DescribeDomainControllers</a>. Pass null if this is the first call. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDomainControllersResult {
    /// <p>List of the <a>DomainController</a> objects that were retrieved.</p>
    #[serde(rename = "domainControllers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controllers: Option<Vec<DomainController>>,
    /// <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <a>DescribeDomainControllers</a> retrieve the next set of items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Describes event topics.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventTopicsRequest {
    /// <p>The Directory ID for which to get the list of associated SNS topics. If this member is null, associations for all Directory IDs are returned.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>A list of SNS topic names for which to obtain the information. If this member is null, all associations for the specified Directory ID are returned.</p> <p>An empty list results in an <code>InvalidParameterException</code> being thrown.</p>
    #[serde(rename = "topicNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_names: Option<Vec<String>>,
}

/// <p>The result of a DescribeEventTopic request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventTopicsResult {
    /// <p>A list of SNS topic names that receive status messages from the specified Directory ID.</p>
    #[serde(rename = "eventTopics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_topics: Option<Vec<EventTopic>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLDAPSSettingsRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>Specifies the number of items that should be displayed on one page.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The type of next token used for pagination.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of LDAP security to enable. Currently only the value <code>Client</code> is supported.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLDAPSSettingsResult {
    /// <p>Information about LDAP security for the specified directory, including status of enablement, state last updated date time, and the reason for the state.</p>
    #[serde(rename = "lDAPSSettingsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldaps_settings_info: Option<Vec<LDAPSSettingInfo>>,
    /// <p>The next token used to retrieve the LDAPS settings if the number of setting types exceeds page limit and there is another page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRegionsRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The <code>DescribeRegionsResult.NextToken</code> value from a previous call to <a>DescribeRegions</a>. Pass null if this is the first call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the Region. For example, <code>us-east-1</code>.</p>
    #[serde(rename = "regionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRegionsResult {
    /// <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <a>DescribeRegions</a> to retrieve the next set of items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of Region information related to the directory for each replicated Region.</p>
    #[serde(rename = "regionsDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions_description: Option<Vec<RegionDescription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSharedDirectoriesRequest {
    /// <p>The number of shared directories to return in the response object.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>DescribeSharedDirectoriesResult.NextToken</code> value from a previous call to <a>DescribeSharedDirectories</a>. Pass null if this is the first call. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns the identifier of the directory in the directory owner account. </p>
    #[serde(rename = "ownerDirectoryId")]
    pub owner_directory_id: String,
    /// <p>A list of identifiers of all shared directories in your account. </p>
    #[serde(rename = "sharedDirectoryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSharedDirectoriesResult {
    /// <p>If not null, token that indicates that more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <a>DescribeSharedDirectories</a> to retrieve the next set of items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of all shared directories in your account.</p>
    #[serde(rename = "sharedDirectories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directories: Option<Vec<SharedDirectory>>,
}

/// <p>Contains the inputs for the <a>DescribeSnapshots</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSnapshotsRequest {
    /// <p>The identifier of the directory for which to retrieve snapshot information.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The maximum number of objects to return.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <i>DescribeSnapshotsResult.NextToken</i> value from a previous call to <a>DescribeSnapshots</a>. Pass null if this is the first call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of identifiers of the snapshots to obtain the information for. If this member is null or empty, all snapshots are returned using the <i>Limit</i> and <i>NextToken</i> members.</p>
    #[serde(rename = "snapshotIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_ids: Option<Vec<String>>,
}

/// <p>Contains the results of the <a>DescribeSnapshots</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSnapshotsResult {
    /// <p>If not null, more results are available. Pass this value in the <i>NextToken</i> member of a subsequent call to <a>DescribeSnapshots</a>.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of <a>Snapshot</a> objects that were retrieved.</p> <p>It is possible that this list contains less than the number of items specified in the <i>Limit</i> member of the request. This occurs if there are less than the requested number of items left to retrieve, or if the limitations of the operation have been exceeded.</p>
    #[serde(rename = "snapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<Vec<Snapshot>>,
}

/// <p>Describes the trust relationships for a particular AWS Managed Microsoft AD directory. If no input parameters are are provided, such as directory ID or trust ID, this request describes all the trust relationships.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTrustsRequest {
    /// <p>The Directory ID of the AWS directory that is a part of the requested trust relationship.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The maximum number of objects to return.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <i>DescribeTrustsResult.NextToken</i> value from a previous call to <a>DescribeTrusts</a>. Pass null if this is the first call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of identifiers of the trust relationships for which to obtain the information. If this member is null, all trust relationships that belong to the current account are returned.</p> <p>An empty list results in an <code>InvalidParameterException</code> being thrown.</p>
    #[serde(rename = "trustIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_ids: Option<Vec<String>>,
}

/// <p>The result of a DescribeTrust request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTrustsResult {
    /// <p>If not null, more results are available. Pass this value for the <i>NextToken</i> parameter in a subsequent call to <a>DescribeTrusts</a> to retrieve the next set of items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of Trust objects that were retrieved.</p> <p>It is possible that this list contains less than the number of items specified in the <i>Limit</i> member of the request. This occurs if there are less than the requested number of items left to retrieve, or if the limitations of the operation have been exceeded.</p>
    #[serde(rename = "trusts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusts: Option<Vec<Trust>>,
}

/// <p>Contains information for the <a>ConnectDirectory</a> operation when an AD Connector directory is being created.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DirectoryConnectSettings {
    /// <p>A list of one or more IP addresses of DNS servers or domain controllers in the on-premises directory.</p>
    #[serde(rename = "customerDnsIps")]
    pub customer_dns_ips: Vec<String>,
    /// <p><p>The user name of an account in the on-premises directory that is used to connect to the directory. This account must have the following permissions:</p> <ul> <li> <p>Read users and groups</p> </li> <li> <p>Create computer objects</p> </li> <li> <p>Join computers to the domain</p> </li> </ul></p>
    #[serde(rename = "customerUserName")]
    pub customer_user_name: String,
    /// <p>A list of subnet identifiers in the VPC in which the AD Connector is created.</p>
    #[serde(rename = "subnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The identifier of the VPC in which the AD Connector is created.</p>
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
}

/// <p>Contains information about an AD Connector directory.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DirectoryConnectSettingsDescription {
    /// <p>A list of the Availability Zones that the directory is in.</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The IP addresses of the AD Connector servers.</p>
    #[serde(rename = "connectIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_ips: Option<Vec<String>>,
    /// <p>The user name of the service account in the on-premises directory.</p>
    #[serde(rename = "customerUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user_name: Option<String>,
    /// <p>The security group identifier for the AD Connector directory.</p>
    #[serde(rename = "securityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// <p>A list of subnet identifiers in the VPC that the AD Connector is in.</p>
    #[serde(rename = "subnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The identifier of the VPC that the AD Connector is in.</p>
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Contains information about an AWS Directory Service directory.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DirectoryDescription {
    /// <p>The access URL for the directory, such as <code>http://&lt;alias&gt;.awsapps.com</code>. If no alias has been created for the directory, <code>&lt;alias&gt;</code> is the directory identifier, such as <code>d-XXXXXXXXXX</code>.</p>
    #[serde(rename = "accessUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_url: Option<String>,
    /// <p>The alias for the directory. If no alias has been created for the directory, the alias is the directory identifier, such as <code>d-XXXXXXXXXX</code>.</p>
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// <p>A <a>DirectoryConnectSettingsDescription</a> object that contains additional information about an AD Connector directory. This member is only present if the directory is an AD Connector directory.</p>
    #[serde(rename = "connectSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_settings: Option<DirectoryConnectSettingsDescription>,
    /// <p>The description for the directory.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The desired number of domain controllers in the directory if the directory is Microsoft AD.</p>
    #[serde(rename = "desiredNumberOfDomainControllers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_number_of_domain_controllers: Option<i64>,
    /// <p>The directory identifier.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The IP addresses of the DNS servers for the directory. For a Simple AD or Microsoft AD directory, these are the IP addresses of the Simple AD or Microsoft AD directory servers. For an AD Connector directory, these are the IP addresses of the DNS servers or domain controllers in the on-premises directory to which the AD Connector is connected.</p>
    #[serde(rename = "dnsIpAddrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addrs: Option<Vec<String>>,
    /// <p>The edition associated with this directory.</p>
    #[serde(rename = "edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    /// <p>Specifies when the directory was created.</p>
    #[serde(rename = "launchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<f64>,
    /// <p>The fully qualified name of the directory.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Describes the AWS Managed Microsoft AD directory in the directory owner account.</p>
    #[serde(rename = "ownerDirectoryDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_directory_description: Option<OwnerDirectoryDescription>,
    /// <p>A <a>RadiusSettings</a> object that contains information about the RADIUS server configured for this directory.</p>
    #[serde(rename = "radiusSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_settings: Option<RadiusSettings>,
    /// <p>The status of the RADIUS MFA server connection.</p>
    #[serde(rename = "radiusStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_status: Option<String>,
    /// <p>Lists the Regions where the directory has replicated.</p>
    #[serde(rename = "regionsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions_info: Option<RegionsInfo>,
    /// <p>The method used when sharing a directory to determine whether the directory should be shared within your AWS organization (<code>ORGANIZATIONS</code>) or with any AWS account by sending a shared directory request (<code>HANDSHAKE</code>).</p>
    #[serde(rename = "shareMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_method: Option<String>,
    /// <p>A directory share request that is sent by the directory owner to the directory consumer. The request includes a typed message to help the directory consumer administrator determine whether to approve or reject the share invitation.</p>
    #[serde(rename = "shareNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_notes: Option<String>,
    /// <p>Current directory status of the shared AWS Managed Microsoft AD directory.</p>
    #[serde(rename = "shareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    /// <p>The short name of the directory.</p>
    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// <p>The directory size.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// <p>Indicates if single sign-on is enabled for the directory. For more information, see <a>EnableSso</a> and <a>DisableSso</a>.</p>
    #[serde(rename = "ssoEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_enabled: Option<bool>,
    /// <p>The current stage of the directory.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// <p>The date and time that the stage was last updated.</p>
    #[serde(rename = "stageLastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_last_updated_date_time: Option<f64>,
    /// <p>Additional information about the directory stage.</p>
    #[serde(rename = "stageReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_reason: Option<String>,
    /// <p>The directory size.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>A <a>DirectoryVpcSettingsDescription</a> object that contains additional information about a directory. This member is only present if the directory is a Simple AD or Managed AD directory.</p>
    #[serde(rename = "vpcSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettingsDescription>,
}

/// <p>Contains directory limit information for a Region.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DirectoryLimits {
    /// <p>The current number of cloud directories in the Region.</p>
    #[serde(rename = "cloudOnlyDirectoriesCurrentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_directories_current_count: Option<i64>,
    /// <p>The maximum number of cloud directories allowed in the Region.</p>
    #[serde(rename = "cloudOnlyDirectoriesLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_directories_limit: Option<i64>,
    /// <p>Indicates if the cloud directory limit has been reached.</p>
    #[serde(rename = "cloudOnlyDirectoriesLimitReached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_directories_limit_reached: Option<bool>,
    /// <p>The current number of AWS Managed Microsoft AD directories in the region.</p>
    #[serde(rename = "cloudOnlyMicrosoftADCurrentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_microsoft_ad_current_count: Option<i64>,
    /// <p>The maximum number of AWS Managed Microsoft AD directories allowed in the region.</p>
    #[serde(rename = "cloudOnlyMicrosoftADLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_microsoft_ad_limit: Option<i64>,
    /// <p>Indicates if the AWS Managed Microsoft AD directory limit has been reached.</p>
    #[serde(rename = "cloudOnlyMicrosoftADLimitReached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_microsoft_ad_limit_reached: Option<bool>,
    /// <p>The current number of connected directories in the Region.</p>
    #[serde(rename = "connectedDirectoriesCurrentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_directories_current_count: Option<i64>,
    /// <p>The maximum number of connected directories allowed in the Region.</p>
    #[serde(rename = "connectedDirectoriesLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_directories_limit: Option<i64>,
    /// <p>Indicates if the connected directory limit has been reached.</p>
    #[serde(rename = "connectedDirectoriesLimitReached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_directories_limit_reached: Option<bool>,
}

/// <p>Contains VPC information for the <a>CreateDirectory</a> or <a>CreateMicrosoftAD</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DirectoryVpcSettings {
    /// <p>The identifiers of the subnets for the directory servers. The two subnets must be in different Availability Zones. AWS Directory Service creates a directory server and a DNS server in each of these subnets.</p>
    #[serde(rename = "subnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The identifier of the VPC in which to create the directory.</p>
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
}

/// <p>Contains information about the directory.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DirectoryVpcSettingsDescription {
    /// <p>The list of Availability Zones that the directory is in.</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The domain controller security group identifier for the directory.</p>
    #[serde(rename = "securityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// <p>The identifiers of the subnets for the directory servers.</p>
    #[serde(rename = "subnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The identifier of the VPC that the directory is in.</p>
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableClientAuthenticationRequest {
    /// <p>The identifier of the directory </p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The type of client authentication to disable. Currently, only the parameter, <code>SmartCard</code> is supported.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableClientAuthenticationResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableLDAPSRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The type of LDAP security to enable. Currently only the value <code>Client</code> is supported.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableLDAPSResult {}

/// <p>Contains the inputs for the <a>DisableRadius</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableRadiusRequest {
    /// <p>The identifier of the directory for which to disable MFA.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
}

/// <p>Contains the results of the <a>DisableRadius</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableRadiusResult {}

/// <p>Contains the inputs for the <a>DisableSso</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableSsoRequest {
    /// <p>The identifier of the directory for which to disable single-sign on.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The password of an alternate account to use to disable single-sign on. This is only used for AD Connector directories. For more information, see the <i>UserName</i> parameter.</p>
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The username of an alternate account to use to disable single-sign on. This is only used for AD Connector directories. This account must have privileges to remove a service principal name.</p> <p>If the AD Connector service account does not have privileges to remove a service principal name, you can specify an alternate account with the <i>UserName</i> and <i>Password</i> parameters. These credentials are only used to disable single sign-on and are not stored by the service. The AD Connector service account is not changed.</p>
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>Contains the results of the <a>DisableSso</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableSsoResult {}

/// <p>Contains information about the domain controllers for a specified directory.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainController {
    /// <p>The Availability Zone where the domain controller is located.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>Identifier of the directory where the domain controller resides.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The IP address of the domain controller.</p>
    #[serde(rename = "dnsIpAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addr: Option<String>,
    /// <p>Identifies a specific domain controller in the directory.</p>
    #[serde(rename = "domainControllerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controller_id: Option<String>,
    /// <p>Specifies when the domain controller was created.</p>
    #[serde(rename = "launchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<f64>,
    /// <p>The status of the domain controller.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The date and time that the status was last updated.</p>
    #[serde(rename = "statusLastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_last_updated_date_time: Option<f64>,
    /// <p>A description of the domain controller state.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>Identifier of the subnet in the VPC that contains the domain controller.</p>
    #[serde(rename = "subnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The identifier of the VPC that contains the domain controller.</p>
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableClientAuthenticationRequest {
    /// <p>The identifier of the specified directory. </p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The type of client authentication to enable. Currently only the value <code>SmartCard</code> is supported. Smart card authentication in AD Connector requires that you enable Kerberos Constrained Delegation for the Service User to the LDAP service in the on-premises AD. </p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableClientAuthenticationResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableLDAPSRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The type of LDAP security to enable. Currently only the value <code>Client</code> is supported.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableLDAPSResult {}

/// <p>Contains the inputs for the <a>EnableRadius</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableRadiusRequest {
    /// <p>The identifier of the directory for which to enable MFA.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>A <a>RadiusSettings</a> object that contains information about the RADIUS server.</p>
    #[serde(rename = "radiusSettings")]
    pub radius_settings: RadiusSettings,
}

/// <p>Contains the results of the <a>EnableRadius</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableRadiusResult {}

/// <p>Contains the inputs for the <a>EnableSso</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableSsoRequest {
    /// <p>The identifier of the directory for which to enable single-sign on.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The password of an alternate account to use to enable single-sign on. This is only used for AD Connector directories. For more information, see the <i>UserName</i> parameter.</p>
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The username of an alternate account to use to enable single-sign on. This is only used for AD Connector directories. This account must have privileges to add a service principal name.</p> <p>If the AD Connector service account does not have privileges to add a service principal name, you can specify an alternate account with the <i>UserName</i> and <i>Password</i> parameters. These credentials are only used to enable single sign-on and are not stored by the service. The AD Connector service account is not changed.</p>
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>Contains the results of the <a>EnableSso</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableSsoResult {}

/// <p>Information about SNS topic and AWS Directory Service directory associations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventTopic {
    /// <p>The date and time of when you associated your directory with the SNS topic.</p>
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<f64>,
    /// <p>The Directory ID of an AWS Directory Service directory that will publish status messages to an SNS topic.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The topic registration status.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The SNS topic ARN (Amazon Resource Name).</p>
    #[serde(rename = "topicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    /// <p>The name of an AWS SNS topic the receives status messages from the directory.</p>
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

/// <p>Contains the inputs for the <a>GetDirectoryLimits</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDirectoryLimitsRequest {}

/// <p>Contains the results of the <a>GetDirectoryLimits</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDirectoryLimitsResult {
    /// <p>A <a>DirectoryLimits</a> object that contains the directory limits for the current rRegion.</p>
    #[serde(rename = "directoryLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_limits: Option<DirectoryLimits>,
}

/// <p>Contains the inputs for the <a>GetSnapshotLimits</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSnapshotLimitsRequest {
    /// <p>Contains the identifier of the directory to obtain the limits for.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
}

/// <p>Contains the results of the <a>GetSnapshotLimits</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSnapshotLimitsResult {
    /// <p>A <a>SnapshotLimits</a> object that contains the manual snapshot limits for the specified directory.</p>
    #[serde(rename = "snapshotLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_limits: Option<SnapshotLimits>,
}

/// <p>IP address block. This is often the address block of the DNS server used for your on-premises domain. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IpRoute {
    /// <p>IP address block using CIDR format, for example 10.0.0.0/24. This is often the address block of the DNS server used for your on-premises domain. For a single IP address use a CIDR address block with /32. For example 10.0.0.0/32.</p>
    #[serde(rename = "cidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<String>,
    /// <p>Description of the address block.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p>Information about one or more IP address blocks.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IpRouteInfo {
    /// <p>The date and time the address block was added to the directory.</p>
    #[serde(rename = "addedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_date_time: Option<f64>,
    /// <p>IP address block in the <a>IpRoute</a>.</p>
    #[serde(rename = "cidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<String>,
    /// <p>Description of the <a>IpRouteInfo</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Identifier (ID) of the directory associated with the IP addresses.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The status of the IP address block.</p>
    #[serde(rename = "ipRouteStatusMsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_route_status_msg: Option<String>,
    /// <p>The reason for the IpRouteStatusMsg.</p>
    #[serde(rename = "ipRouteStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_route_status_reason: Option<String>,
}

/// <p>Contains general information about the LDAPS settings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LDAPSSettingInfo {
    /// <p>The state of the LDAPS settings.</p>
    #[serde(rename = "lDAPSStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldaps_status: Option<String>,
    /// <p>Describes a state change for LDAPS.</p>
    #[serde(rename = "lDAPSStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldaps_status_reason: Option<String>,
    /// <p>The date and time when the LDAPS settings were last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCertificatesRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The number of items that should show up on one page</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A token for requesting another page of certificates if the <code>NextToken</code> response element indicates that more certificates are available. Use the value of the returned <code>NextToken</code> element in your request until the token comes back as <code>null</code>. Pass <code>null</code> if this is the first call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCertificatesResult {
    /// <p>A list of certificates with basic details including certificate ID, certificate common name, certificate state.</p>
    #[serde(rename = "certificatesInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates_info: Option<Vec<CertificateInfo>>,
    /// <p>Indicates whether another page of certificates is available when the number of available certificates exceeds the page limit.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIpRoutesRequest {
    /// <p>Identifier (ID) of the directory for which you want to retrieve the IP addresses.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>Maximum number of items to return. If this value is zero, the maximum number of items is specified by the limitations of the operation.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <i>ListIpRoutes.NextToken</i> value from a previous call to <a>ListIpRoutes</a>. Pass null if this is the first call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListIpRoutesResult {
    /// <p>A list of <a>IpRoute</a>s.</p>
    #[serde(rename = "ipRoutesInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_routes_info: Option<Vec<IpRouteInfo>>,
    /// <p>If not null, more results are available. Pass this value for the <i>NextToken</i> parameter in a subsequent call to <a>ListIpRoutes</a> to retrieve the next set of items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLogSubscriptionsRequest {
    /// <p>If a <i>DirectoryID</i> is provided, lists only the log subscription associated with that directory. If no <i>DirectoryId</i> is provided, lists all log subscriptions associated with your AWS account. If there are no log subscriptions for the AWS account or the directory, an empty list will be returned.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The maximum number of items returned.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token for the next set of items to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLogSubscriptionsResult {
    /// <p>A list of active <a>LogSubscription</a> objects for calling the AWS account.</p>
    #[serde(rename = "logSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_subscriptions: Option<Vec<LogSubscription>>,
    /// <p>The token for the next set of items to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSchemaExtensionsRequest {
    /// <p>The identifier of the directory from which to retrieve the schema extension information.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>ListSchemaExtensions.NextToken</code> value from a previous call to <code>ListSchemaExtensions</code>. Pass null if this is the first call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSchemaExtensionsResult {
    /// <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <code>ListSchemaExtensions</code> to retrieve the next set of items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the schema extensions applied to the directory.</p>
    #[serde(rename = "schemaExtensionsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extensions_info: Option<Vec<SchemaExtensionInfo>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Identifier (ID) of the directory for which you want to retrieve tags.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResult {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of tags returned by the ListTagsForResource operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents a log subscription, which tracks real-time data from a chosen log group to a specified destination.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LogSubscription {
    /// <p>Identifier (ID) of the directory that you want to associate with the log subscription.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>The date and time that the log subscription was created.</p>
    #[serde(rename = "subscriptionCreatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_created_date_time: Option<f64>,
}

/// <p>Describes the directory owner account details that have been shared to the directory consumer account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OwnerDirectoryDescription {
    /// <p>Identifier of the directory owner account.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Identifier of the AWS Managed Microsoft AD directory in the directory owner account.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>IP address of the directory’s domain controllers.</p>
    #[serde(rename = "dnsIpAddrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addrs: Option<Vec<String>>,
    /// <p>A <a>RadiusSettings</a> object that contains information about the RADIUS server.</p>
    #[serde(rename = "radiusSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_settings: Option<RadiusSettings>,
    /// <p>Information about the status of the RADIUS server.</p>
    #[serde(rename = "radiusStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_status: Option<String>,
    /// <p>Information about the VPC settings for the directory.</p>
    #[serde(rename = "vpcSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettingsDescription>,
}

/// <p>Contains information about a Remote Authentication Dial In User Service (RADIUS) server.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RadiusSettings {
    /// <p>The protocol specified for your RADIUS endpoints.</p>
    #[serde(rename = "authenticationProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_protocol: Option<String>,
    /// <p>Not currently used.</p>
    #[serde(rename = "displayLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_label: Option<String>,
    /// <p>The port that your RADIUS server is using for communications. Your on-premises network must allow inbound traffic over this port from the AWS Directory Service servers.</p>
    #[serde(rename = "radiusPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_port: Option<i64>,
    /// <p>The maximum number of times that communication with the RADIUS server is attempted.</p>
    #[serde(rename = "radiusRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_retries: Option<i64>,
    /// <p>An array of strings that contains the fully qualified domain name (FQDN) or IP addresses of the RADIUS server endpoints, or the FQDN or IP addresses of your RADIUS server load balancer.</p>
    #[serde(rename = "radiusServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_servers: Option<Vec<String>>,
    /// <p>The amount of time, in seconds, to wait for the RADIUS server to respond.</p>
    #[serde(rename = "radiusTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_timeout: Option<i64>,
    /// <p>Required for enabling RADIUS on the directory.</p>
    #[serde(rename = "sharedSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_secret: Option<String>,
    /// <p>Not currently used.</p>
    #[serde(rename = "useSameUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_same_username: Option<bool>,
}

/// <p>The replicated Region information for a directory.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegionDescription {
    /// <p>The desired number of domain controllers in the specified Region for the specified directory.</p>
    #[serde(rename = "desiredNumberOfDomainControllers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_number_of_domain_controllers: Option<i64>,
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The date and time that the Region description was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>Specifies when the Region replication began.</p>
    #[serde(rename = "launchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<f64>,
    /// <p>The name of the Region. For example, <code>us-east-1</code>.</p>
    #[serde(rename = "regionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    /// <p>Specifies whether the Region is the primary Region or an additional Region.</p>
    #[serde(rename = "regionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_type: Option<String>,
    /// <p>The status of the replication process for the specified Region.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The date and time that the Region status was last updated.</p>
    #[serde(rename = "statusLastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_last_updated_date_time: Option<f64>,
    #[serde(rename = "vpcSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettings>,
}

/// <p>Provides information about the Regions that are configured for multi-Region replication.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegionsInfo {
    /// <p>Lists the Regions where the directory has been replicated, excluding the primary Region.</p>
    #[serde(rename = "additionalRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_regions: Option<Vec<String>>,
    /// <p>The Region where the AWS Managed Microsoft AD directory was originally created.</p>
    #[serde(rename = "primaryRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_region: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterCertificateRequest {
    /// <p>The certificate PEM string that needs to be registered.</p>
    #[serde(rename = "certificateData")]
    pub certificate_data: String,
    /// <p>A <code>ClientCertAuthSettings</code> object that contains client certificate authentication settings.</p>
    #[serde(rename = "clientCertAuthSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cert_auth_settings: Option<ClientCertAuthSettings>,
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The function that the registered certificate performs. Valid values include <code>ClientLDAPS</code> or <code>ClientCertAuth</code>. The default value is <code>ClientLDAPS</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterCertificateResult {
    /// <p>The identifier of the certificate.</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}

/// <p>Registers a new event topic.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterEventTopicRequest {
    /// <p>The Directory ID that will publish status messages to the SNS topic.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The SNS topic name to which the directory will publish status messages. This SNS topic must be in the same region as the specified Directory ID.</p>
    #[serde(rename = "topicName")]
    pub topic_name: String,
}

/// <p>The result of a RegisterEventTopic request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterEventTopicResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RejectSharedDirectoryRequest {
    /// <p>Identifier of the shared directory in the directory consumer account. This identifier is different for each directory owner account.</p>
    #[serde(rename = "sharedDirectoryId")]
    pub shared_directory_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RejectSharedDirectoryResult {
    /// <p>Identifier of the shared directory in the directory consumer account.</p>
    #[serde(rename = "sharedDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveIpRoutesRequest {
    /// <p>IP address blocks that you want to remove.</p>
    #[serde(rename = "cidrIps")]
    pub cidr_ips: Vec<String>,
    /// <p>Identifier (ID) of the directory from which you want to remove the IP addresses.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveIpRoutesResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveRegionRequest {
    /// <p>The identifier of the directory for which you want to remove Region replication.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveRegionResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsFromResourceRequest {
    /// <p>Identifier (ID) of the directory from which to remove the tag.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>The tag key (name) of the tag to be removed.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveTagsFromResourceResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResetUserPasswordRequest {
    /// <p>Identifier of the AWS Managed Microsoft AD or Simple AD directory in which the user resides.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The new password that will be reset.</p>
    #[serde(rename = "newPassword")]
    pub new_password: String,
    /// <p>The user name of the user whose password will be reset.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResetUserPasswordResult {}

/// <p>An object representing the inputs for the <a>RestoreFromSnapshot</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestoreFromSnapshotRequest {
    /// <p>The identifier of the snapshot to restore from.</p>
    #[serde(rename = "snapshotId")]
    pub snapshot_id: String,
}

/// <p>Contains the results of the <a>RestoreFromSnapshot</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RestoreFromSnapshotResult {}

/// <p>Information about a schema extension.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SchemaExtensionInfo {
    /// <p>A description of the schema extension.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the directory to which the schema extension is applied.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The date and time that the schema extension was completed.</p>
    #[serde(rename = "endDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    /// <p>The identifier of the schema extension.</p>
    #[serde(rename = "schemaExtensionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_id: Option<String>,
    /// <p>The current status of the schema extension.</p>
    #[serde(rename = "schemaExtensionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_status: Option<String>,
    /// <p>The reason for the <code>SchemaExtensionStatus</code>.</p>
    #[serde(rename = "schemaExtensionStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_status_reason: Option<String>,
    /// <p>The date and time that the schema extension started being applied to the directory.</p>
    #[serde(rename = "startDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ShareDirectoryRequest {
    /// <p>Identifier of the AWS Managed Microsoft AD directory that you want to share with other AWS accounts.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The method used when sharing a directory to determine whether the directory should be shared within your AWS organization (<code>ORGANIZATIONS</code>) or with any AWS account by sending a directory sharing request (<code>HANDSHAKE</code>).</p>
    #[serde(rename = "shareMethod")]
    pub share_method: String,
    /// <p>A directory share request that is sent by the directory owner to the directory consumer. The request includes a typed message to help the directory consumer administrator determine whether to approve or reject the share invitation.</p>
    #[serde(rename = "shareNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_notes: Option<String>,
    /// <p>Identifier for the directory consumer account with whom the directory is to be shared.</p>
    #[serde(rename = "shareTarget")]
    pub share_target: ShareTarget,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ShareDirectoryResult {
    /// <p>Identifier of the directory that is stored in the directory consumer account that is shared from the specified directory (<code>DirectoryId</code>).</p>
    #[serde(rename = "sharedDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

/// <p>Identifier that contains details about the directory consumer account.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ShareTarget {
    /// <p>Identifier of the directory consumer account.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>Type of identifier to be used in the <code>Id</code> field.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Details about the shared directory in the directory owner account for which the share request in the directory consumer account has been accepted.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SharedDirectory {
    /// <p>The date and time that the shared directory was created.</p>
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<f64>,
    /// <p>The date and time that the shared directory was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>Identifier of the directory owner account, which contains the directory that has been shared to the consumer account.</p>
    #[serde(rename = "ownerAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    /// <p>Identifier of the directory in the directory owner account. </p>
    #[serde(rename = "ownerDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_directory_id: Option<String>,
    /// <p>The method used when sharing a directory to determine whether the directory should be shared within your AWS organization (<code>ORGANIZATIONS</code>) or with any AWS account by sending a shared directory request (<code>HANDSHAKE</code>).</p>
    #[serde(rename = "shareMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_method: Option<String>,
    /// <p>A directory share request that is sent by the directory owner to the directory consumer. The request includes a typed message to help the directory consumer administrator determine whether to approve or reject the share invitation.</p>
    #[serde(rename = "shareNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_notes: Option<String>,
    /// <p>Current directory status of the shared AWS Managed Microsoft AD directory.</p>
    #[serde(rename = "shareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    /// <p>Identifier of the directory consumer account that has access to the shared directory (<code>OwnerDirectoryId</code>) in the directory owner account.</p>
    #[serde(rename = "sharedAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_account_id: Option<String>,
    /// <p>Identifier of the shared directory in the directory consumer account. This identifier is different for each directory owner account.</p>
    #[serde(rename = "sharedDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

/// <p>Describes a directory snapshot.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Snapshot {
    /// <p>The directory identifier.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The descriptive name of the snapshot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The snapshot identifier.</p>
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The date and time that the snapshot was taken.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The snapshot status.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The snapshot type.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains manual snapshot limit information for a directory.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SnapshotLimits {
    /// <p>The current number of manual snapshots of the directory.</p>
    #[serde(rename = "manualSnapshotsCurrentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshots_current_count: Option<i64>,
    /// <p>The maximum number of manual snapshots allowed.</p>
    #[serde(rename = "manualSnapshotsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshots_limit: Option<i64>,
    /// <p>Indicates if the manual snapshot limit has been reached.</p>
    #[serde(rename = "manualSnapshotsLimitReached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshots_limit_reached: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartSchemaExtensionRequest {
    /// <p>If true, creates a snapshot of the directory before applying the schema extension.</p>
    #[serde(rename = "createSnapshotBeforeSchemaExtension")]
    pub create_snapshot_before_schema_extension: bool,
    /// <p>A description of the schema extension.</p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p>The identifier of the directory for which the schema extension will be applied to.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The LDIF file represented as a string. To construct the LdifContent string, precede each line as it would be formatted in an ldif file with \n. See the example request below for more details. The file size can be no larger than 1MB.</p>
    #[serde(rename = "ldifContent")]
    pub ldif_content: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartSchemaExtensionResult {
    /// <p>The identifier of the schema extension that will be applied.</p>
    #[serde(rename = "schemaExtensionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_id: Option<String>,
}

/// <p>Metadata assigned to a directory consisting of a key-value pair.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>Required name of the tag. The string value can be Unicode characters and cannot be prefixed with "aws:". The string can contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The optional value of the tag. The string value can be Unicode characters. The string can contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>Describes a trust relationship between an AWS Managed Microsoft AD directory and an external domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Trust {
    /// <p>The date and time that the trust relationship was created.</p>
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<f64>,
    /// <p>The Directory ID of the AWS directory involved in the trust relationship.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The date and time that the trust relationship was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The Fully Qualified Domain Name (FQDN) of the external domain involved in the trust relationship.</p>
    #[serde(rename = "remoteDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_name: Option<String>,
    /// <p>Current state of selective authentication for the trust.</p>
    #[serde(rename = "selectiveAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_auth: Option<String>,
    /// <p>The date and time that the TrustState was last updated.</p>
    #[serde(rename = "stateLastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_last_updated_date_time: Option<f64>,
    /// <p>The trust relationship direction.</p>
    #[serde(rename = "trustDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_direction: Option<String>,
    /// <p>The unique ID of the trust relationship.</p>
    #[serde(rename = "trustId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
    /// <p>The trust relationship state.</p>
    #[serde(rename = "trustState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_state: Option<String>,
    /// <p>The reason for the TrustState.</p>
    #[serde(rename = "trustStateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_state_reason: Option<String>,
    /// <p>The trust relationship type. <code>Forest</code> is the default.</p>
    #[serde(rename = "trustType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UnshareDirectoryRequest {
    /// <p>The identifier of the AWS Managed Microsoft AD directory that you want to stop sharing.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>Identifier for the directory consumer account with whom the directory has to be unshared.</p>
    #[serde(rename = "unshareTarget")]
    pub unshare_target: UnshareTarget,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnshareDirectoryResult {
    /// <p>Identifier of the directory stored in the directory consumer account that is to be unshared from the specified directory (<code>DirectoryId</code>).</p>
    #[serde(rename = "sharedDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

/// <p>Identifier that contains details about the directory consumer account with whom the directory is being unshared.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UnshareTarget {
    /// <p>Identifier of the directory consumer account.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>Type of identifier to be used in the <i>Id</i> field.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Updates a conditional forwarder.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConditionalForwarderRequest {
    /// <p>The directory ID of the AWS directory for which to update the conditional forwarder.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>The updated IP addresses of the remote DNS server associated with the conditional forwarder.</p>
    #[serde(rename = "dnsIpAddrs")]
    pub dns_ip_addrs: Vec<String>,
    /// <p>The fully qualified domain name (FQDN) of the remote domain with which you will set up a trust relationship.</p>
    #[serde(rename = "remoteDomainName")]
    pub remote_domain_name: String,
}

/// <p>The result of an UpdateConditionalForwarder request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConditionalForwarderResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateNumberOfDomainControllersRequest {
    /// <p>The number of domain controllers desired in the directory.</p>
    #[serde(rename = "desiredNumber")]
    pub desired_number: i64,
    /// <p>Identifier of the directory to which the domain controllers will be added or removed.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNumberOfDomainControllersResult {}

/// <p>Contains the inputs for the <a>UpdateRadius</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRadiusRequest {
    /// <p>The identifier of the directory for which to update the RADIUS server information.</p>
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    /// <p>A <a>RadiusSettings</a> object that contains information about the RADIUS server.</p>
    #[serde(rename = "radiusSettings")]
    pub radius_settings: RadiusSettings,
}

/// <p>Contains the results of the <a>UpdateRadius</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRadiusResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTrustRequest {
    /// <p>Updates selective authentication for the trust.</p>
    #[serde(rename = "selectiveAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_auth: Option<String>,
    /// <p>Identifier of the trust relationship.</p>
    #[serde(rename = "trustId")]
    pub trust_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTrustResult {
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Identifier of the trust relationship.</p>
    #[serde(rename = "trustId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}

/// <p>Initiates the verification of an existing trust relationship between an AWS Managed Microsoft AD directory and an external domain.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VerifyTrustRequest {
    /// <p>The unique Trust ID of the trust relationship to verify.</p>
    #[serde(rename = "trustId")]
    pub trust_id: String,
}

/// <p>Result of a VerifyTrust request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VerifyTrustResult {
    /// <p>The unique Trust ID of the trust relationship that was verified.</p>
    #[serde(rename = "trustId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}

/// Errors returned by AcceptSharedDirectory
#[derive(Debug, PartialEq)]
pub enum AcceptSharedDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory has already been shared with this AWS account.</p>
    DirectoryAlreadyShared(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl AcceptSharedDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptSharedDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(AcceptSharedDirectoryError::Client(err.msg))
                }
                "DirectoryAlreadySharedException" => {
                    return RusotoError::Service(
                        AcceptSharedDirectoryError::DirectoryAlreadyShared(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(AcceptSharedDirectoryError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AcceptSharedDirectoryError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(AcceptSharedDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptSharedDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptSharedDirectoryError::Client(ref cause) => write!(f, "{}", cause),
            AcceptSharedDirectoryError::DirectoryAlreadyShared(ref cause) => write!(f, "{}", cause),
            AcceptSharedDirectoryError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            AcceptSharedDirectoryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AcceptSharedDirectoryError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AcceptSharedDirectoryError {}
/// Errors returned by AddIpRoutes
#[derive(Debug, PartialEq)]
pub enum AddIpRoutesError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The maximum allowed number of IP addresses was exceeded. The default limit is 100 IP address blocks.</p>
    IpRouteLimitExceeded(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl AddIpRoutesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddIpRoutesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(AddIpRoutesError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(AddIpRoutesError::DirectoryUnavailable(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(AddIpRoutesError::EntityAlreadyExists(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(AddIpRoutesError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AddIpRoutesError::InvalidParameter(err.msg))
                }
                "IpRouteLimitExceededException" => {
                    return RusotoError::Service(AddIpRoutesError::IpRouteLimitExceeded(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AddIpRoutesError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddIpRoutesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddIpRoutesError::Client(ref cause) => write!(f, "{}", cause),
            AddIpRoutesError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            AddIpRoutesError::EntityAlreadyExists(ref cause) => write!(f, "{}", cause),
            AddIpRoutesError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            AddIpRoutesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AddIpRoutesError::IpRouteLimitExceeded(ref cause) => write!(f, "{}", cause),
            AddIpRoutesError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddIpRoutesError {}
/// Errors returned by AddRegion
#[derive(Debug, PartialEq)]
pub enum AddRegionError {
    /// <p>Client authentication is not available in this region at this time.</p>
    AccessDenied(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The Region you specified is the same Region where the AWS Managed Microsoft AD directory was created. Specify a different Region and try again.</p>
    DirectoryAlreadyInRegion(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>You have reached the limit for maximum number of simultaneous Region replications per directory.</p>
    RegionLimitExceeded(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl AddRegionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddRegionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AddRegionError::AccessDenied(err.msg))
                }
                "ClientException" => return RusotoError::Service(AddRegionError::Client(err.msg)),
                "DirectoryAlreadyInRegionException" => {
                    return RusotoError::Service(AddRegionError::DirectoryAlreadyInRegion(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(AddRegionError::DirectoryDoesNotExist(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(AddRegionError::DirectoryUnavailable(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(AddRegionError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AddRegionError::InvalidParameter(err.msg))
                }
                "RegionLimitExceededException" => {
                    return RusotoError::Service(AddRegionError::RegionLimitExceeded(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AddRegionError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(AddRegionError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddRegionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddRegionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AddRegionError::Client(ref cause) => write!(f, "{}", cause),
            AddRegionError::DirectoryAlreadyInRegion(ref cause) => write!(f, "{}", cause),
            AddRegionError::DirectoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            AddRegionError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            AddRegionError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            AddRegionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AddRegionError::RegionLimitExceeded(ref cause) => write!(f, "{}", cause),
            AddRegionError::Service(ref cause) => write!(f, "{}", cause),
            AddRegionError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddRegionError {}
/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The maximum allowed number of tags was exceeded.</p>
    TagLimitExceeded(String),
}

impl AddTagsToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(AddTagsToResourceError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(AddTagsToResourceError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AddTagsToResourceError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AddTagsToResourceError::Service(err.msg))
                }
                "TagLimitExceededException" => {
                    return RusotoError::Service(AddTagsToResourceError::TagLimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddTagsToResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsToResourceError::Client(ref cause) => write!(f, "{}", cause),
            AddTagsToResourceError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            AddTagsToResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AddTagsToResourceError::Service(ref cause) => write!(f, "{}", cause),
            AddTagsToResourceError::TagLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsToResourceError {}
/// Errors returned by CancelSchemaExtension
#[derive(Debug, PartialEq)]
pub enum CancelSchemaExtensionError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl CancelSchemaExtensionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelSchemaExtensionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CancelSchemaExtensionError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CancelSchemaExtensionError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CancelSchemaExtensionError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelSchemaExtensionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelSchemaExtensionError::Client(ref cause) => write!(f, "{}", cause),
            CancelSchemaExtensionError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            CancelSchemaExtensionError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelSchemaExtensionError {}
/// Errors returned by ConnectDirectory
#[derive(Debug, PartialEq)]
pub enum ConnectDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The maximum number of directories in the region has been reached. You can use the <a>GetDirectoryLimits</a> operation to determine your directory limits in the region.</p>
    DirectoryLimitExceeded(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl ConnectDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConnectDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ConnectDirectoryError::Client(err.msg))
                }
                "DirectoryLimitExceededException" => {
                    return RusotoError::Service(ConnectDirectoryError::DirectoryLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ConnectDirectoryError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ConnectDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ConnectDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectDirectoryError::Client(ref cause) => write!(f, "{}", cause),
            ConnectDirectoryError::DirectoryLimitExceeded(ref cause) => write!(f, "{}", cause),
            ConnectDirectoryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ConnectDirectoryError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ConnectDirectoryError {}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl CreateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateAliasError::Client(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(CreateAliasError::EntityAlreadyExists(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CreateAliasError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateAliasError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateAliasError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAliasError::Client(ref cause) => write!(f, "{}", cause),
            CreateAliasError::EntityAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateAliasError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateAliasError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateAliasError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAliasError {}
/// Errors returned by CreateComputer
#[derive(Debug, PartialEq)]
pub enum CreateComputerError {
    /// <p>An authentication error occurred.</p>
    AuthenticationFailed(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl CreateComputerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateComputerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthenticationFailedException" => {
                    return RusotoError::Service(CreateComputerError::AuthenticationFailed(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(CreateComputerError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(CreateComputerError::DirectoryUnavailable(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(CreateComputerError::EntityAlreadyExists(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CreateComputerError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateComputerError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateComputerError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateComputerError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateComputerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateComputerError::AuthenticationFailed(ref cause) => write!(f, "{}", cause),
            CreateComputerError::Client(ref cause) => write!(f, "{}", cause),
            CreateComputerError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            CreateComputerError::EntityAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateComputerError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateComputerError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateComputerError::Service(ref cause) => write!(f, "{}", cause),
            CreateComputerError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateComputerError {}
/// Errors returned by CreateConditionalForwarder
#[derive(Debug, PartialEq)]
pub enum CreateConditionalForwarderError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl CreateConditionalForwarderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateConditionalForwarderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateConditionalForwarderError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        CreateConditionalForwarderError::DirectoryUnavailable(err.msg),
                    )
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateConditionalForwarderError::EntityAlreadyExists(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        CreateConditionalForwarderError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateConditionalForwarderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateConditionalForwarderError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        CreateConditionalForwarderError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateConditionalForwarderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConditionalForwarderError::Client(ref cause) => write!(f, "{}", cause),
            CreateConditionalForwarderError::DirectoryUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConditionalForwarderError::EntityAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConditionalForwarderError::EntityDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConditionalForwarderError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateConditionalForwarderError::Service(ref cause) => write!(f, "{}", cause),
            CreateConditionalForwarderError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateConditionalForwarderError {}
/// Errors returned by CreateDirectory
#[derive(Debug, PartialEq)]
pub enum CreateDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The maximum number of directories in the region has been reached. You can use the <a>GetDirectoryLimits</a> operation to determine your directory limits in the region.</p>
    DirectoryLimitExceeded(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl CreateDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateDirectoryError::Client(err.msg))
                }
                "DirectoryLimitExceededException" => {
                    return RusotoError::Service(CreateDirectoryError::DirectoryLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateDirectoryError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDirectoryError::Client(ref cause) => write!(f, "{}", cause),
            CreateDirectoryError::DirectoryLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDirectoryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateDirectoryError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDirectoryError {}
/// Errors returned by CreateLogSubscription
#[derive(Debug, PartialEq)]
pub enum CreateLogSubscriptionError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The account does not have sufficient permission to perform the operation.</p>
    InsufficientPermissions(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl CreateLogSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLogSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateLogSubscriptionError::Client(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(CreateLogSubscriptionError::EntityAlreadyExists(
                        err.msg,
                    ))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CreateLogSubscriptionError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(
                        CreateLogSubscriptionError::InsufficientPermissions(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateLogSubscriptionError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateLogSubscriptionError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLogSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLogSubscriptionError::Client(ref cause) => write!(f, "{}", cause),
            CreateLogSubscriptionError::EntityAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateLogSubscriptionError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateLogSubscriptionError::InsufficientPermissions(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLogSubscriptionError::Service(ref cause) => write!(f, "{}", cause),
            CreateLogSubscriptionError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLogSubscriptionError {}
/// Errors returned by CreateMicrosoftAD
#[derive(Debug, PartialEq)]
pub enum CreateMicrosoftADError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The maximum number of directories in the region has been reached. You can use the <a>GetDirectoryLimits</a> operation to determine your directory limits in the region.</p>
    DirectoryLimitExceeded(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl CreateMicrosoftADError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMicrosoftADError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateMicrosoftADError::Client(err.msg))
                }
                "DirectoryLimitExceededException" => {
                    return RusotoError::Service(CreateMicrosoftADError::DirectoryLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateMicrosoftADError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateMicrosoftADError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateMicrosoftADError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMicrosoftADError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMicrosoftADError::Client(ref cause) => write!(f, "{}", cause),
            CreateMicrosoftADError::DirectoryLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateMicrosoftADError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateMicrosoftADError::Service(ref cause) => write!(f, "{}", cause),
            CreateMicrosoftADError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMicrosoftADError {}
/// Errors returned by CreateSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateSnapshotError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The maximum number of manual snapshots for the directory has been reached. You can use the <a>GetSnapshotLimits</a> operation to determine the snapshot limits for a directory.</p>
    SnapshotLimitExceeded(String),
}

impl CreateSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateSnapshotError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CreateSnapshotError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateSnapshotError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateSnapshotError::Service(err.msg))
                }
                "SnapshotLimitExceededException" => {
                    return RusotoError::Service(CreateSnapshotError::SnapshotLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSnapshotError::Client(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::SnapshotLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSnapshotError {}
/// Errors returned by CreateTrust
#[derive(Debug, PartialEq)]
pub enum CreateTrustError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl CreateTrustError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTrustError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateTrustError::Client(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(CreateTrustError::EntityAlreadyExists(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CreateTrustError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateTrustError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateTrustError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateTrustError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTrustError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTrustError::Client(ref cause) => write!(f, "{}", cause),
            CreateTrustError::EntityAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateTrustError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateTrustError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateTrustError::Service(ref cause) => write!(f, "{}", cause),
            CreateTrustError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTrustError {}
/// Errors returned by DeleteConditionalForwarder
#[derive(Debug, PartialEq)]
pub enum DeleteConditionalForwarderError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DeleteConditionalForwarderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConditionalForwarderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteConditionalForwarderError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        DeleteConditionalForwarderError::DirectoryUnavailable(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        DeleteConditionalForwarderError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteConditionalForwarderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteConditionalForwarderError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DeleteConditionalForwarderError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConditionalForwarderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConditionalForwarderError::Client(ref cause) => write!(f, "{}", cause),
            DeleteConditionalForwarderError::DirectoryUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteConditionalForwarderError::EntityDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteConditionalForwarderError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteConditionalForwarderError::Service(ref cause) => write!(f, "{}", cause),
            DeleteConditionalForwarderError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteConditionalForwarderError {}
/// Errors returned by DeleteDirectory
#[derive(Debug, PartialEq)]
pub enum DeleteDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DeleteDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteDirectoryError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DeleteDirectoryError::EntityDoesNotExist(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDirectoryError::Client(ref cause) => write!(f, "{}", cause),
            DeleteDirectoryError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeleteDirectoryError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDirectoryError {}
/// Errors returned by DeleteLogSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteLogSubscriptionError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DeleteLogSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLogSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteLogSubscriptionError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DeleteLogSubscriptionError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteLogSubscriptionError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DeleteLogSubscriptionError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLogSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLogSubscriptionError::Client(ref cause) => write!(f, "{}", cause),
            DeleteLogSubscriptionError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeleteLogSubscriptionError::Service(ref cause) => write!(f, "{}", cause),
            DeleteLogSubscriptionError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLogSubscriptionError {}
/// Errors returned by DeleteSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteSnapshotError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DeleteSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteSnapshotError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DeleteSnapshotError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteSnapshotError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteSnapshotError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSnapshotError::Client(ref cause) => write!(f, "{}", cause),
            DeleteSnapshotError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeleteSnapshotError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteSnapshotError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSnapshotError {}
/// Errors returned by DeleteTrust
#[derive(Debug, PartialEq)]
pub enum DeleteTrustError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DeleteTrustError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTrustError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteTrustError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DeleteTrustError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteTrustError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteTrustError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DeleteTrustError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTrustError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTrustError::Client(ref cause) => write!(f, "{}", cause),
            DeleteTrustError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeleteTrustError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteTrustError::Service(ref cause) => write!(f, "{}", cause),
            DeleteTrustError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTrustError {}
/// Errors returned by DeregisterCertificate
#[derive(Debug, PartialEq)]
pub enum DeregisterCertificateError {
    /// <p>The certificate is not present in the system for describe or deregister activities.</p>
    CertificateDoesNotExist(String),
    /// <p>The certificate is being used for the LDAP security connection and cannot be removed without disabling LDAP security.</p>
    CertificateInUse(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DeregisterCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CertificateDoesNotExistException" => {
                    return RusotoError::Service(
                        DeregisterCertificateError::CertificateDoesNotExist(err.msg),
                    )
                }
                "CertificateInUseException" => {
                    return RusotoError::Service(DeregisterCertificateError::CertificateInUse(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(DeregisterCertificateError::Client(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(DeregisterCertificateError::DirectoryDoesNotExist(
                        err.msg,
                    ))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(DeregisterCertificateError::DirectoryUnavailable(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeregisterCertificateError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeregisterCertificateError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DeregisterCertificateError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterCertificateError::CertificateDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterCertificateError::CertificateInUse(ref cause) => write!(f, "{}", cause),
            DeregisterCertificateError::Client(ref cause) => write!(f, "{}", cause),
            DeregisterCertificateError::DirectoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeregisterCertificateError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            DeregisterCertificateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeregisterCertificateError::Service(ref cause) => write!(f, "{}", cause),
            DeregisterCertificateError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterCertificateError {}
/// Errors returned by DeregisterEventTopic
#[derive(Debug, PartialEq)]
pub enum DeregisterEventTopicError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DeregisterEventTopicError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterEventTopicError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeregisterEventTopicError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DeregisterEventTopicError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeregisterEventTopicError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeregisterEventTopicError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterEventTopicError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterEventTopicError::Client(ref cause) => write!(f, "{}", cause),
            DeregisterEventTopicError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeregisterEventTopicError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeregisterEventTopicError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterEventTopicError {}
/// Errors returned by DescribeCertificate
#[derive(Debug, PartialEq)]
pub enum DescribeCertificateError {
    /// <p>The certificate is not present in the system for describe or deregister activities.</p>
    CertificateDoesNotExist(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CertificateDoesNotExistException" => {
                    return RusotoError::Service(DescribeCertificateError::CertificateDoesNotExist(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(DescribeCertificateError::Client(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(DescribeCertificateError::DirectoryDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeCertificateError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeCertificateError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DescribeCertificateError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCertificateError::CertificateDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeCertificateError::Client(ref cause) => write!(f, "{}", cause),
            DescribeCertificateError::DirectoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeCertificateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeCertificateError::Service(ref cause) => write!(f, "{}", cause),
            DescribeCertificateError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCertificateError {}
/// Errors returned by DescribeConditionalForwarders
#[derive(Debug, PartialEq)]
pub enum DescribeConditionalForwardersError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeConditionalForwardersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConditionalForwardersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeConditionalForwardersError::Client(
                        err.msg,
                    ))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        DescribeConditionalForwardersError::DirectoryUnavailable(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeConditionalForwardersError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeConditionalForwardersError::InvalidParameter(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeConditionalForwardersError::Service(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DescribeConditionalForwardersError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConditionalForwardersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConditionalForwardersError::Client(ref cause) => write!(f, "{}", cause),
            DescribeConditionalForwardersError::DirectoryUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConditionalForwardersError::EntityDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConditionalForwardersError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConditionalForwardersError::Service(ref cause) => write!(f, "{}", cause),
            DescribeConditionalForwardersError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeConditionalForwardersError {}
/// Errors returned by DescribeDirectories
#[derive(Debug, PartialEq)]
pub enum DescribeDirectoriesError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DescribeDirectoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDirectoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeDirectoriesError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DescribeDirectoriesError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeDirectoriesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeDirectoriesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeDirectoriesError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDirectoriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDirectoriesError::Client(ref cause) => write!(f, "{}", cause),
            DescribeDirectoriesError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeDirectoriesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeDirectoriesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeDirectoriesError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDirectoriesError {}
/// Errors returned by DescribeDomainControllers
#[derive(Debug, PartialEq)]
pub enum DescribeDomainControllersError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeDomainControllersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDomainControllersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeDomainControllersError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeDomainControllersError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeDomainControllersError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeDomainControllersError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeDomainControllersError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DescribeDomainControllersError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDomainControllersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDomainControllersError::Client(ref cause) => write!(f, "{}", cause),
            DescribeDomainControllersError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeDomainControllersError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeDomainControllersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeDomainControllersError::Service(ref cause) => write!(f, "{}", cause),
            DescribeDomainControllersError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDomainControllersError {}
/// Errors returned by DescribeEventTopics
#[derive(Debug, PartialEq)]
pub enum DescribeEventTopicsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DescribeEventTopicsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventTopicsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeEventTopicsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DescribeEventTopicsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeEventTopicsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeEventTopicsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEventTopicsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventTopicsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeEventTopicsError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeEventTopicsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeEventTopicsError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEventTopicsError {}
/// Errors returned by DescribeLDAPSSettings
#[derive(Debug, PartialEq)]
pub enum DescribeLDAPSSettingsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeLDAPSSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLDAPSSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeLDAPSSettingsError::Client(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(DescribeLDAPSSettingsError::DirectoryDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeLDAPSSettingsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeLDAPSSettingsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeLDAPSSettingsError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DescribeLDAPSSettingsError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLDAPSSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLDAPSSettingsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeLDAPSSettingsError::DirectoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeLDAPSSettingsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeLDAPSSettingsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeLDAPSSettingsError::Service(ref cause) => write!(f, "{}", cause),
            DescribeLDAPSSettingsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLDAPSSettingsError {}
/// Errors returned by DescribeRegions
#[derive(Debug, PartialEq)]
pub enum DescribeRegionsError {
    /// <p>Client authentication is not available in this region at this time.</p>
    AccessDenied(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeRegionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRegionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeRegionsError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(DescribeRegionsError::Client(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(DescribeRegionsError::DirectoryDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeRegionsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeRegionsError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeRegionsError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DescribeRegionsError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRegionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRegionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeRegionsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeRegionsError::DirectoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeRegionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeRegionsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeRegionsError::Service(ref cause) => write!(f, "{}", cause),
            DescribeRegionsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRegionsError {}
/// Errors returned by DescribeSharedDirectories
#[derive(Debug, PartialEq)]
pub enum DescribeSharedDirectoriesError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeSharedDirectoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSharedDirectoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeSharedDirectoriesError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeSharedDirectoriesError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeSharedDirectoriesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeSharedDirectoriesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeSharedDirectoriesError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DescribeSharedDirectoriesError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSharedDirectoriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSharedDirectoriesError::Client(ref cause) => write!(f, "{}", cause),
            DescribeSharedDirectoriesError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeSharedDirectoriesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeSharedDirectoriesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeSharedDirectoriesError::Service(ref cause) => write!(f, "{}", cause),
            DescribeSharedDirectoriesError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeSharedDirectoriesError {}
/// Errors returned by DescribeSnapshots
#[derive(Debug, PartialEq)]
pub enum DescribeSnapshotsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DescribeSnapshotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSnapshotsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeSnapshotsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DescribeSnapshotsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeSnapshotsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeSnapshotsError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeSnapshotsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSnapshotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSnapshotsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeSnapshotsError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeSnapshotsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeSnapshotsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeSnapshotsError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSnapshotsError {}
/// Errors returned by DescribeTrusts
#[derive(Debug, PartialEq)]
pub enum DescribeTrustsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeTrustsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTrustsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeTrustsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DescribeTrustsError::EntityDoesNotExist(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeTrustsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeTrustsError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeTrustsError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DescribeTrustsError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTrustsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTrustsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeTrustsError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeTrustsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeTrustsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeTrustsError::Service(ref cause) => write!(f, "{}", cause),
            DescribeTrustsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTrustsError {}
/// Errors returned by DisableClientAuthentication
#[derive(Debug, PartialEq)]
pub enum DisableClientAuthenticationError {
    /// <p>Client authentication is not available in this region at this time.</p>
    AccessDenied(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>Client authentication is already enabled.</p>
    InvalidClientAuthStatus(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DisableClientAuthenticationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisableClientAuthenticationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisableClientAuthenticationError::AccessDenied(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(DisableClientAuthenticationError::Client(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(
                        DisableClientAuthenticationError::DirectoryDoesNotExist(err.msg),
                    )
                }
                "InvalidClientAuthStatusException" => {
                    return RusotoError::Service(
                        DisableClientAuthenticationError::InvalidClientAuthStatus(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DisableClientAuthenticationError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DisableClientAuthenticationError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableClientAuthenticationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableClientAuthenticationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisableClientAuthenticationError::Client(ref cause) => write!(f, "{}", cause),
            DisableClientAuthenticationError::DirectoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableClientAuthenticationError::InvalidClientAuthStatus(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableClientAuthenticationError::Service(ref cause) => write!(f, "{}", cause),
            DisableClientAuthenticationError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisableClientAuthenticationError {}
/// Errors returned by DisableLDAPS
#[derive(Debug, PartialEq)]
pub enum DisableLDAPSError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The LDAP activities could not be performed because they are limited by the LDAPS status.</p>
    InvalidLDAPSStatus(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DisableLDAPSError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableLDAPSError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DisableLDAPSError::Client(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(DisableLDAPSError::DirectoryDoesNotExist(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(DisableLDAPSError::DirectoryUnavailable(err.msg))
                }
                "InvalidLDAPSStatusException" => {
                    return RusotoError::Service(DisableLDAPSError::InvalidLDAPSStatus(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DisableLDAPSError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DisableLDAPSError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DisableLDAPSError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableLDAPSError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableLDAPSError::Client(ref cause) => write!(f, "{}", cause),
            DisableLDAPSError::DirectoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            DisableLDAPSError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            DisableLDAPSError::InvalidLDAPSStatus(ref cause) => write!(f, "{}", cause),
            DisableLDAPSError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisableLDAPSError::Service(ref cause) => write!(f, "{}", cause),
            DisableLDAPSError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableLDAPSError {}
/// Errors returned by DisableRadius
#[derive(Debug, PartialEq)]
pub enum DisableRadiusError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DisableRadiusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableRadiusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DisableRadiusError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DisableRadiusError::EntityDoesNotExist(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DisableRadiusError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableRadiusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableRadiusError::Client(ref cause) => write!(f, "{}", cause),
            DisableRadiusError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DisableRadiusError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableRadiusError {}
/// Errors returned by DisableSso
#[derive(Debug, PartialEq)]
pub enum DisableSsoError {
    /// <p>An authentication error occurred.</p>
    AuthenticationFailed(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The account does not have sufficient permission to perform the operation.</p>
    InsufficientPermissions(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DisableSsoError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableSsoError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthenticationFailedException" => {
                    return RusotoError::Service(DisableSsoError::AuthenticationFailed(err.msg))
                }
                "ClientException" => return RusotoError::Service(DisableSsoError::Client(err.msg)),
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DisableSsoError::EntityDoesNotExist(err.msg))
                }
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(DisableSsoError::InsufficientPermissions(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DisableSsoError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableSsoError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableSsoError::AuthenticationFailed(ref cause) => write!(f, "{}", cause),
            DisableSsoError::Client(ref cause) => write!(f, "{}", cause),
            DisableSsoError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DisableSsoError::InsufficientPermissions(ref cause) => write!(f, "{}", cause),
            DisableSsoError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableSsoError {}
/// Errors returned by EnableClientAuthentication
#[derive(Debug, PartialEq)]
pub enum EnableClientAuthenticationError {
    /// <p>Client authentication is not available in this region at this time.</p>
    AccessDenied(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>Client authentication is already enabled.</p>
    InvalidClientAuthStatus(String),
    /// <p>Client authentication setup could not be completed because at least one valid certificate must be registered in the system.</p>
    NoAvailableCertificate(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl EnableClientAuthenticationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<EnableClientAuthenticationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(EnableClientAuthenticationError::AccessDenied(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(EnableClientAuthenticationError::Client(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(
                        EnableClientAuthenticationError::DirectoryDoesNotExist(err.msg),
                    )
                }
                "InvalidClientAuthStatusException" => {
                    return RusotoError::Service(
                        EnableClientAuthenticationError::InvalidClientAuthStatus(err.msg),
                    )
                }
                "NoAvailableCertificateException" => {
                    return RusotoError::Service(
                        EnableClientAuthenticationError::NoAvailableCertificate(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(EnableClientAuthenticationError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        EnableClientAuthenticationError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableClientAuthenticationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableClientAuthenticationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            EnableClientAuthenticationError::Client(ref cause) => write!(f, "{}", cause),
            EnableClientAuthenticationError::DirectoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            EnableClientAuthenticationError::InvalidClientAuthStatus(ref cause) => {
                write!(f, "{}", cause)
            }
            EnableClientAuthenticationError::NoAvailableCertificate(ref cause) => {
                write!(f, "{}", cause)
            }
            EnableClientAuthenticationError::Service(ref cause) => write!(f, "{}", cause),
            EnableClientAuthenticationError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for EnableClientAuthenticationError {}
/// Errors returned by EnableLDAPS
#[derive(Debug, PartialEq)]
pub enum EnableLDAPSError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The LDAP activities could not be performed because they are limited by the LDAPS status.</p>
    InvalidLDAPSStatus(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Client authentication setup could not be completed because at least one valid certificate must be registered in the system.</p>
    NoAvailableCertificate(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl EnableLDAPSError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableLDAPSError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(EnableLDAPSError::Client(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(EnableLDAPSError::DirectoryDoesNotExist(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(EnableLDAPSError::DirectoryUnavailable(err.msg))
                }
                "InvalidLDAPSStatusException" => {
                    return RusotoError::Service(EnableLDAPSError::InvalidLDAPSStatus(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(EnableLDAPSError::InvalidParameter(err.msg))
                }
                "NoAvailableCertificateException" => {
                    return RusotoError::Service(EnableLDAPSError::NoAvailableCertificate(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnableLDAPSError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(EnableLDAPSError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableLDAPSError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableLDAPSError::Client(ref cause) => write!(f, "{}", cause),
            EnableLDAPSError::DirectoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            EnableLDAPSError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            EnableLDAPSError::InvalidLDAPSStatus(ref cause) => write!(f, "{}", cause),
            EnableLDAPSError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            EnableLDAPSError::NoAvailableCertificate(ref cause) => write!(f, "{}", cause),
            EnableLDAPSError::Service(ref cause) => write!(f, "{}", cause),
            EnableLDAPSError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableLDAPSError {}
/// Errors returned by EnableRadius
#[derive(Debug, PartialEq)]
pub enum EnableRadiusError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl EnableRadiusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableRadiusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(EnableRadiusError::Client(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(EnableRadiusError::EntityAlreadyExists(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(EnableRadiusError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(EnableRadiusError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnableRadiusError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableRadiusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableRadiusError::Client(ref cause) => write!(f, "{}", cause),
            EnableRadiusError::EntityAlreadyExists(ref cause) => write!(f, "{}", cause),
            EnableRadiusError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            EnableRadiusError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            EnableRadiusError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableRadiusError {}
/// Errors returned by EnableSso
#[derive(Debug, PartialEq)]
pub enum EnableSsoError {
    /// <p>An authentication error occurred.</p>
    AuthenticationFailed(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The account does not have sufficient permission to perform the operation.</p>
    InsufficientPermissions(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl EnableSsoError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableSsoError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthenticationFailedException" => {
                    return RusotoError::Service(EnableSsoError::AuthenticationFailed(err.msg))
                }
                "ClientException" => return RusotoError::Service(EnableSsoError::Client(err.msg)),
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(EnableSsoError::EntityDoesNotExist(err.msg))
                }
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(EnableSsoError::InsufficientPermissions(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnableSsoError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableSsoError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableSsoError::AuthenticationFailed(ref cause) => write!(f, "{}", cause),
            EnableSsoError::Client(ref cause) => write!(f, "{}", cause),
            EnableSsoError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            EnableSsoError::InsufficientPermissions(ref cause) => write!(f, "{}", cause),
            EnableSsoError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableSsoError {}
/// Errors returned by GetDirectoryLimits
#[derive(Debug, PartialEq)]
pub enum GetDirectoryLimitsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl GetDirectoryLimitsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDirectoryLimitsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(GetDirectoryLimitsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(GetDirectoryLimitsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetDirectoryLimitsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDirectoryLimitsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDirectoryLimitsError::Client(ref cause) => write!(f, "{}", cause),
            GetDirectoryLimitsError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetDirectoryLimitsError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDirectoryLimitsError {}
/// Errors returned by GetSnapshotLimits
#[derive(Debug, PartialEq)]
pub enum GetSnapshotLimitsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl GetSnapshotLimitsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSnapshotLimitsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(GetSnapshotLimitsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(GetSnapshotLimitsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetSnapshotLimitsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSnapshotLimitsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSnapshotLimitsError::Client(ref cause) => write!(f, "{}", cause),
            GetSnapshotLimitsError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetSnapshotLimitsError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSnapshotLimitsError {}
/// Errors returned by ListCertificates
#[derive(Debug, PartialEq)]
pub enum ListCertificatesError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl ListCertificatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCertificatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListCertificatesError::Client(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(ListCertificatesError::DirectoryDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListCertificatesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListCertificatesError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListCertificatesError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ListCertificatesError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListCertificatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCertificatesError::Client(ref cause) => write!(f, "{}", cause),
            ListCertificatesError::DirectoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListCertificatesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListCertificatesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListCertificatesError::Service(ref cause) => write!(f, "{}", cause),
            ListCertificatesError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListCertificatesError {}
/// Errors returned by ListIpRoutes
#[derive(Debug, PartialEq)]
pub enum ListIpRoutesError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl ListIpRoutesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIpRoutesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListIpRoutesError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ListIpRoutesError::EntityDoesNotExist(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListIpRoutesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListIpRoutesError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListIpRoutesError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListIpRoutesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIpRoutesError::Client(ref cause) => write!(f, "{}", cause),
            ListIpRoutesError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListIpRoutesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListIpRoutesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListIpRoutesError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIpRoutesError {}
/// Errors returned by ListLogSubscriptions
#[derive(Debug, PartialEq)]
pub enum ListLogSubscriptionsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl ListLogSubscriptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLogSubscriptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListLogSubscriptionsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ListLogSubscriptionsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListLogSubscriptionsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListLogSubscriptionsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLogSubscriptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLogSubscriptionsError::Client(ref cause) => write!(f, "{}", cause),
            ListLogSubscriptionsError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListLogSubscriptionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListLogSubscriptionsError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLogSubscriptionsError {}
/// Errors returned by ListSchemaExtensions
#[derive(Debug, PartialEq)]
pub enum ListSchemaExtensionsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl ListSchemaExtensionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSchemaExtensionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListSchemaExtensionsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ListSchemaExtensionsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListSchemaExtensionsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListSchemaExtensionsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSchemaExtensionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSchemaExtensionsError::Client(ref cause) => write!(f, "{}", cause),
            ListSchemaExtensionsError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListSchemaExtensionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListSchemaExtensionsError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSchemaExtensionsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListTagsForResourceError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ListTagsForResourceError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListTagsForResourceError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::Client(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by RegisterCertificate
#[derive(Debug, PartialEq)]
pub enum RegisterCertificateError {
    /// <p>The certificate has already been registered into the system.</p>
    CertificateAlreadyExists(String),
    /// <p>The certificate could not be added because the certificate limit has been reached.</p>
    CertificateLimitExceeded(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The certificate PEM that was provided has incorrect encoding.</p>
    InvalidCertificate(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl RegisterCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CertificateAlreadyExistsException" => {
                    return RusotoError::Service(
                        RegisterCertificateError::CertificateAlreadyExists(err.msg),
                    )
                }
                "CertificateLimitExceededException" => {
                    return RusotoError::Service(
                        RegisterCertificateError::CertificateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(RegisterCertificateError::Client(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(RegisterCertificateError::DirectoryDoesNotExist(
                        err.msg,
                    ))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(RegisterCertificateError::DirectoryUnavailable(
                        err.msg,
                    ))
                }
                "InvalidCertificateException" => {
                    return RusotoError::Service(RegisterCertificateError::InvalidCertificate(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RegisterCertificateError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(RegisterCertificateError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(RegisterCertificateError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterCertificateError::CertificateAlreadyExists(ref cause) => write!(f, "{}", cause),
            RegisterCertificateError::CertificateLimitExceeded(ref cause) => write!(f, "{}", cause),
            RegisterCertificateError::Client(ref cause) => write!(f, "{}", cause),
            RegisterCertificateError::DirectoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            RegisterCertificateError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            RegisterCertificateError::InvalidCertificate(ref cause) => write!(f, "{}", cause),
            RegisterCertificateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RegisterCertificateError::Service(ref cause) => write!(f, "{}", cause),
            RegisterCertificateError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterCertificateError {}
/// Errors returned by RegisterEventTopic
#[derive(Debug, PartialEq)]
pub enum RegisterEventTopicError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl RegisterEventTopicError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterEventTopicError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RegisterEventTopicError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(RegisterEventTopicError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RegisterEventTopicError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(RegisterEventTopicError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterEventTopicError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterEventTopicError::Client(ref cause) => write!(f, "{}", cause),
            RegisterEventTopicError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            RegisterEventTopicError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RegisterEventTopicError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterEventTopicError {}
/// Errors returned by RejectSharedDirectory
#[derive(Debug, PartialEq)]
pub enum RejectSharedDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory has already been shared with this AWS account.</p>
    DirectoryAlreadyShared(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl RejectSharedDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RejectSharedDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RejectSharedDirectoryError::Client(err.msg))
                }
                "DirectoryAlreadySharedException" => {
                    return RusotoError::Service(
                        RejectSharedDirectoryError::DirectoryAlreadyShared(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(RejectSharedDirectoryError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RejectSharedDirectoryError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(RejectSharedDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RejectSharedDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RejectSharedDirectoryError::Client(ref cause) => write!(f, "{}", cause),
            RejectSharedDirectoryError::DirectoryAlreadyShared(ref cause) => write!(f, "{}", cause),
            RejectSharedDirectoryError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            RejectSharedDirectoryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RejectSharedDirectoryError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RejectSharedDirectoryError {}
/// Errors returned by RemoveIpRoutes
#[derive(Debug, PartialEq)]
pub enum RemoveIpRoutesError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl RemoveIpRoutesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveIpRoutesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RemoveIpRoutesError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(RemoveIpRoutesError::DirectoryUnavailable(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(RemoveIpRoutesError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RemoveIpRoutesError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(RemoveIpRoutesError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveIpRoutesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveIpRoutesError::Client(ref cause) => write!(f, "{}", cause),
            RemoveIpRoutesError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            RemoveIpRoutesError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            RemoveIpRoutesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RemoveIpRoutesError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveIpRoutesError {}
/// Errors returned by RemoveRegion
#[derive(Debug, PartialEq)]
pub enum RemoveRegionError {
    /// <p>Client authentication is not available in this region at this time.</p>
    AccessDenied(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExist(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl RemoveRegionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveRegionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RemoveRegionError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(RemoveRegionError::Client(err.msg))
                }
                "DirectoryDoesNotExistException" => {
                    return RusotoError::Service(RemoveRegionError::DirectoryDoesNotExist(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(RemoveRegionError::DirectoryUnavailable(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(RemoveRegionError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(RemoveRegionError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveRegionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveRegionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RemoveRegionError::Client(ref cause) => write!(f, "{}", cause),
            RemoveRegionError::DirectoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            RemoveRegionError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            RemoveRegionError::Service(ref cause) => write!(f, "{}", cause),
            RemoveRegionError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveRegionError {}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl RemoveTagsFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveTagsFromResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsFromResourceError::Client(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromResourceError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromResourceError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsFromResourceError {}
/// Errors returned by ResetUserPassword
#[derive(Debug, PartialEq)]
pub enum ResetUserPasswordError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The new password provided by the user does not meet the password complexity requirements defined in your directory.</p>
    InvalidPassword(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// <p>The user provided a username that does not exist in your directory.</p>
    UserDoesNotExist(String),
}

impl ResetUserPasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetUserPasswordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ResetUserPasswordError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(ResetUserPasswordError::DirectoryUnavailable(
                        err.msg,
                    ))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ResetUserPasswordError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(ResetUserPasswordError::InvalidPassword(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ResetUserPasswordError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ResetUserPasswordError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "UserDoesNotExistException" => {
                    return RusotoError::Service(ResetUserPasswordError::UserDoesNotExist(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ResetUserPasswordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResetUserPasswordError::Client(ref cause) => write!(f, "{}", cause),
            ResetUserPasswordError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            ResetUserPasswordError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            ResetUserPasswordError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            ResetUserPasswordError::Service(ref cause) => write!(f, "{}", cause),
            ResetUserPasswordError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
            ResetUserPasswordError::UserDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResetUserPasswordError {}
/// Errors returned by RestoreFromSnapshot
#[derive(Debug, PartialEq)]
pub enum RestoreFromSnapshotError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl RestoreFromSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RestoreFromSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RestoreFromSnapshotError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(RestoreFromSnapshotError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RestoreFromSnapshotError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(RestoreFromSnapshotError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RestoreFromSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestoreFromSnapshotError::Client(ref cause) => write!(f, "{}", cause),
            RestoreFromSnapshotError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            RestoreFromSnapshotError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RestoreFromSnapshotError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RestoreFromSnapshotError {}
/// Errors returned by ShareDirectory
#[derive(Debug, PartialEq)]
pub enum ShareDirectoryError {
    /// <p>Client authentication is not available in this region at this time.</p>
    AccessDenied(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory has already been shared with this AWS account.</p>
    DirectoryAlreadyShared(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified shared target is not valid.</p>
    InvalidTarget(String),
    /// <p>Exception encountered while trying to access your AWS organization.</p>
    Organizations(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The maximum number of AWS accounts that you can share with this directory has been reached.</p>
    ShareLimitExceeded(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl ShareDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ShareDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ShareDirectoryError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(ShareDirectoryError::Client(err.msg))
                }
                "DirectoryAlreadySharedException" => {
                    return RusotoError::Service(ShareDirectoryError::DirectoryAlreadyShared(
                        err.msg,
                    ))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ShareDirectoryError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ShareDirectoryError::InvalidParameter(err.msg))
                }
                "InvalidTargetException" => {
                    return RusotoError::Service(ShareDirectoryError::InvalidTarget(err.msg))
                }
                "OrganizationsException" => {
                    return RusotoError::Service(ShareDirectoryError::Organizations(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ShareDirectoryError::Service(err.msg))
                }
                "ShareLimitExceededException" => {
                    return RusotoError::Service(ShareDirectoryError::ShareLimitExceeded(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ShareDirectoryError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ShareDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShareDirectoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ShareDirectoryError::Client(ref cause) => write!(f, "{}", cause),
            ShareDirectoryError::DirectoryAlreadyShared(ref cause) => write!(f, "{}", cause),
            ShareDirectoryError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            ShareDirectoryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ShareDirectoryError::InvalidTarget(ref cause) => write!(f, "{}", cause),
            ShareDirectoryError::Organizations(ref cause) => write!(f, "{}", cause),
            ShareDirectoryError::Service(ref cause) => write!(f, "{}", cause),
            ShareDirectoryError::ShareLimitExceeded(ref cause) => write!(f, "{}", cause),
            ShareDirectoryError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ShareDirectoryError {}
/// Errors returned by StartSchemaExtension
#[derive(Debug, PartialEq)]
pub enum StartSchemaExtensionError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The maximum number of manual snapshots for the directory has been reached. You can use the <a>GetSnapshotLimits</a> operation to determine the snapshot limits for a directory.</p>
    SnapshotLimitExceeded(String),
}

impl StartSchemaExtensionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartSchemaExtensionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(StartSchemaExtensionError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(StartSchemaExtensionError::DirectoryUnavailable(
                        err.msg,
                    ))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(StartSchemaExtensionError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartSchemaExtensionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(StartSchemaExtensionError::Service(err.msg))
                }
                "SnapshotLimitExceededException" => {
                    return RusotoError::Service(StartSchemaExtensionError::SnapshotLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartSchemaExtensionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartSchemaExtensionError::Client(ref cause) => write!(f, "{}", cause),
            StartSchemaExtensionError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            StartSchemaExtensionError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            StartSchemaExtensionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartSchemaExtensionError::Service(ref cause) => write!(f, "{}", cause),
            StartSchemaExtensionError::SnapshotLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartSchemaExtensionError {}
/// Errors returned by UnshareDirectory
#[derive(Debug, PartialEq)]
pub enum UnshareDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory has not been shared with this AWS account.</p>
    DirectoryNotShared(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The specified shared target is not valid.</p>
    InvalidTarget(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl UnshareDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnshareDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UnshareDirectoryError::Client(err.msg))
                }
                "DirectoryNotSharedException" => {
                    return RusotoError::Service(UnshareDirectoryError::DirectoryNotShared(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(UnshareDirectoryError::EntityDoesNotExist(err.msg))
                }
                "InvalidTargetException" => {
                    return RusotoError::Service(UnshareDirectoryError::InvalidTarget(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UnshareDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UnshareDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnshareDirectoryError::Client(ref cause) => write!(f, "{}", cause),
            UnshareDirectoryError::DirectoryNotShared(ref cause) => write!(f, "{}", cause),
            UnshareDirectoryError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            UnshareDirectoryError::InvalidTarget(ref cause) => write!(f, "{}", cause),
            UnshareDirectoryError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UnshareDirectoryError {}
/// Errors returned by UpdateConditionalForwarder
#[derive(Debug, PartialEq)]
pub enum UpdateConditionalForwarderError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl UpdateConditionalForwarderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateConditionalForwarderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateConditionalForwarderError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        UpdateConditionalForwarderError::DirectoryUnavailable(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdateConditionalForwarderError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateConditionalForwarderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateConditionalForwarderError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        UpdateConditionalForwarderError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateConditionalForwarderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConditionalForwarderError::Client(ref cause) => write!(f, "{}", cause),
            UpdateConditionalForwarderError::DirectoryUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateConditionalForwarderError::EntityDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateConditionalForwarderError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateConditionalForwarderError::Service(ref cause) => write!(f, "{}", cause),
            UpdateConditionalForwarderError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateConditionalForwarderError {}
/// Errors returned by UpdateNumberOfDomainControllers
#[derive(Debug, PartialEq)]
pub enum UpdateNumberOfDomainControllersError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The maximum allowed number of domain controllers per directory was exceeded. The default limit per directory is 20 domain controllers.</p>
    DomainControllerLimitExceeded(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl UpdateNumberOfDomainControllersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateNumberOfDomainControllersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateNumberOfDomainControllersError::Client(
                        err.msg,
                    ))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        UpdateNumberOfDomainControllersError::DirectoryUnavailable(err.msg),
                    )
                }
                "DomainControllerLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateNumberOfDomainControllersError::DomainControllerLimitExceeded(
                            err.msg,
                        ),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdateNumberOfDomainControllersError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateNumberOfDomainControllersError::InvalidParameter(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateNumberOfDomainControllersError::Service(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        UpdateNumberOfDomainControllersError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateNumberOfDomainControllersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNumberOfDomainControllersError::Client(ref cause) => write!(f, "{}", cause),
            UpdateNumberOfDomainControllersError::DirectoryUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateNumberOfDomainControllersError::DomainControllerLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateNumberOfDomainControllersError::EntityDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateNumberOfDomainControllersError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateNumberOfDomainControllersError::Service(ref cause) => write!(f, "{}", cause),
            UpdateNumberOfDomainControllersError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateNumberOfDomainControllersError {}
/// Errors returned by UpdateRadius
#[derive(Debug, PartialEq)]
pub enum UpdateRadiusError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl UpdateRadiusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRadiusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateRadiusError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(UpdateRadiusError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateRadiusError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateRadiusError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRadiusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRadiusError::Client(ref cause) => write!(f, "{}", cause),
            UpdateRadiusError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            UpdateRadiusError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateRadiusError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRadiusError {}
/// Errors returned by UpdateTrust
#[derive(Debug, PartialEq)]
pub enum UpdateTrustError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl UpdateTrustError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTrustError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateTrustError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(UpdateTrustError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateTrustError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateTrustError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateTrustError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTrustError::Client(ref cause) => write!(f, "{}", cause),
            UpdateTrustError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            UpdateTrustError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateTrustError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTrustError {}
/// Errors returned by VerifyTrust
#[derive(Debug, PartialEq)]
pub enum VerifyTrustError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl VerifyTrustError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<VerifyTrustError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(VerifyTrustError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(VerifyTrustError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(VerifyTrustError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(VerifyTrustError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(VerifyTrustError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for VerifyTrustError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VerifyTrustError::Client(ref cause) => write!(f, "{}", cause),
            VerifyTrustError::EntityDoesNotExist(ref cause) => write!(f, "{}", cause),
            VerifyTrustError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            VerifyTrustError::Service(ref cause) => write!(f, "{}", cause),
            VerifyTrustError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for VerifyTrustError {}
/// Trait representing the capabilities of the Directory Service API. Directory Service clients implement this trait.
#[async_trait]
pub trait DirectoryService {
    /// <p>Accepts a directory sharing request that was sent from the directory owner account.</p>
    async fn accept_shared_directory(
        &self,
        input: AcceptSharedDirectoryRequest,
    ) -> Result<AcceptSharedDirectoryResult, RusotoError<AcceptSharedDirectoryError>>;

    /// <p>If the DNS server for your on-premises domain uses a publicly addressable IP address, you must add a CIDR address block to correctly route traffic to and from your Microsoft AD on Amazon Web Services. <i>AddIpRoutes</i> adds this address block. You can also use <i>AddIpRoutes</i> to facilitate routing traffic that uses public IP ranges from your Microsoft AD on AWS to a peer VPC. </p> <p>Before you call <i>AddIpRoutes</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>AddIpRoutes</i> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    async fn add_ip_routes(
        &self,
        input: AddIpRoutesRequest,
    ) -> Result<AddIpRoutesResult, RusotoError<AddIpRoutesError>>;

    /// <p>Adds two domain controllers in the specified Region for the specified directory.</p>
    async fn add_region(
        &self,
        input: AddRegionRequest,
    ) -> Result<AddRegionResult, RusotoError<AddRegionError>>;

    /// <p>Adds or overwrites one or more tags for the specified directory. Each directory can have a maximum of 50 tags. Each tag consists of a key and optional value. Tag keys must be unique to each resource.</p>
    async fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceRequest,
    ) -> Result<AddTagsToResourceResult, RusotoError<AddTagsToResourceError>>;

    /// <p>Cancels an in-progress schema extension to a Microsoft AD directory. Once a schema extension has started replicating to all domain controllers, the task can no longer be canceled. A schema extension can be canceled during any of the following states; <code>Initializing</code>, <code>CreatingSnapshot</code>, and <code>UpdatingSchema</code>.</p>
    async fn cancel_schema_extension(
        &self,
        input: CancelSchemaExtensionRequest,
    ) -> Result<CancelSchemaExtensionResult, RusotoError<CancelSchemaExtensionError>>;

    /// <p>Creates an AD Connector to connect to an on-premises directory.</p> <p>Before you call <code>ConnectDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>ConnectDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    async fn connect_directory(
        &self,
        input: ConnectDirectoryRequest,
    ) -> Result<ConnectDirectoryResult, RusotoError<ConnectDirectoryError>>;

    /// <p><p>Creates an alias for a directory and assigns the alias to the directory. The alias is used to construct the access URL for the directory, such as <code>http://&lt;alias&gt;.awsapps.com</code>.</p> <important> <p>After an alias has been created, it cannot be deleted or reused, so this operation should only be used when absolutely necessary.</p> </important></p>
    async fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> Result<CreateAliasResult, RusotoError<CreateAliasError>>;

    /// <p>Creates an Active Directory computer object in the specified directory.</p>
    async fn create_computer(
        &self,
        input: CreateComputerRequest,
    ) -> Result<CreateComputerResult, RusotoError<CreateComputerError>>;

    /// <p>Creates a conditional forwarder associated with your AWS directory. Conditional forwarders are required in order to set up a trust relationship with another domain. The conditional forwarder points to the trusted domain.</p>
    async fn create_conditional_forwarder(
        &self,
        input: CreateConditionalForwarderRequest,
    ) -> Result<CreateConditionalForwarderResult, RusotoError<CreateConditionalForwarderError>>;

    /// <p>Creates a Simple AD directory. For more information, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/directory_simple_ad.html">Simple Active Directory</a> in the <i>AWS Directory Service Admin Guide</i>.</p> <p>Before you call <code>CreateDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>CreateDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    async fn create_directory(
        &self,
        input: CreateDirectoryRequest,
    ) -> Result<CreateDirectoryResult, RusotoError<CreateDirectoryError>>;

    /// <p>Creates a subscription to forward real-time Directory Service domain controller security logs to the specified Amazon CloudWatch log group in your AWS account.</p>
    async fn create_log_subscription(
        &self,
        input: CreateLogSubscriptionRequest,
    ) -> Result<CreateLogSubscriptionResult, RusotoError<CreateLogSubscriptionError>>;

    /// <p>Creates a Microsoft AD directory in the AWS Cloud. For more information, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/directory_microsoft_ad.html">AWS Managed Microsoft AD</a> in the <i>AWS Directory Service Admin Guide</i>.</p> <p>Before you call <i>CreateMicrosoftAD</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>CreateMicrosoftAD</i> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    async fn create_microsoft_ad(
        &self,
        input: CreateMicrosoftADRequest,
    ) -> Result<CreateMicrosoftADResult, RusotoError<CreateMicrosoftADError>>;

    /// <p><p>Creates a snapshot of a Simple AD or Microsoft AD directory in the AWS cloud.</p> <note> <p>You cannot take snapshots of AD Connector directories.</p> </note></p>
    async fn create_snapshot(
        &self,
        input: CreateSnapshotRequest,
    ) -> Result<CreateSnapshotResult, RusotoError<CreateSnapshotError>>;

    /// <p>AWS Directory Service for Microsoft Active Directory allows you to configure trust relationships. For example, you can establish a trust between your AWS Managed Microsoft AD directory, and your existing on-premises Microsoft Active Directory. This would allow you to provide users and groups access to resources in either domain, with a single set of credentials.</p> <p>This action initiates the creation of the AWS side of a trust relationship between an AWS Managed Microsoft AD directory and an external domain. You can create either a forest trust or an external trust.</p>
    async fn create_trust(
        &self,
        input: CreateTrustRequest,
    ) -> Result<CreateTrustResult, RusotoError<CreateTrustError>>;

    /// <p>Deletes a conditional forwarder that has been set up for your AWS directory.</p>
    async fn delete_conditional_forwarder(
        &self,
        input: DeleteConditionalForwarderRequest,
    ) -> Result<DeleteConditionalForwarderResult, RusotoError<DeleteConditionalForwarderError>>;

    /// <p>Deletes an AWS Directory Service directory.</p> <p>Before you call <code>DeleteDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>DeleteDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    async fn delete_directory(
        &self,
        input: DeleteDirectoryRequest,
    ) -> Result<DeleteDirectoryResult, RusotoError<DeleteDirectoryError>>;

    /// <p>Deletes the specified log subscription.</p>
    async fn delete_log_subscription(
        &self,
        input: DeleteLogSubscriptionRequest,
    ) -> Result<DeleteLogSubscriptionResult, RusotoError<DeleteLogSubscriptionError>>;

    /// <p>Deletes a directory snapshot.</p>
    async fn delete_snapshot(
        &self,
        input: DeleteSnapshotRequest,
    ) -> Result<DeleteSnapshotResult, RusotoError<DeleteSnapshotError>>;

    /// <p>Deletes an existing trust relationship between your AWS Managed Microsoft AD directory and an external domain.</p>
    async fn delete_trust(
        &self,
        input: DeleteTrustRequest,
    ) -> Result<DeleteTrustResult, RusotoError<DeleteTrustError>>;

    /// <p>Deletes from the system the certificate that was registered for secure LDAP or client certificate authentication.</p>
    async fn deregister_certificate(
        &self,
        input: DeregisterCertificateRequest,
    ) -> Result<DeregisterCertificateResult, RusotoError<DeregisterCertificateError>>;

    /// <p>Removes the specified directory as a publisher to the specified SNS topic.</p>
    async fn deregister_event_topic(
        &self,
        input: DeregisterEventTopicRequest,
    ) -> Result<DeregisterEventTopicResult, RusotoError<DeregisterEventTopicError>>;

    /// <p>Displays information about the certificate registered for secure LDAP or client certificate authentication.</p>
    async fn describe_certificate(
        &self,
        input: DescribeCertificateRequest,
    ) -> Result<DescribeCertificateResult, RusotoError<DescribeCertificateError>>;

    /// <p>Obtains information about the conditional forwarders for this account.</p> <p>If no input parameters are provided for RemoteDomainNames, this request describes all conditional forwarders for the specified directory ID.</p>
    async fn describe_conditional_forwarders(
        &self,
        input: DescribeConditionalForwardersRequest,
    ) -> Result<DescribeConditionalForwardersResult, RusotoError<DescribeConditionalForwardersError>>;

    /// <p>Obtains information about the directories that belong to this account.</p> <p>You can retrieve information about specific directories by passing the directory identifiers in the <code>DirectoryIds</code> parameter. Otherwise, all directories that belong to the current account are returned.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> request and response parameters. If more results are available, the <code>DescribeDirectoriesResult.NextToken</code> member contains a token that you pass in the next call to <a>DescribeDirectories</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <code>Limit</code> parameter.</p>
    async fn describe_directories(
        &self,
        input: DescribeDirectoriesRequest,
    ) -> Result<DescribeDirectoriesResult, RusotoError<DescribeDirectoriesError>>;

    /// <p>Provides information about any domain controllers in your directory.</p>
    async fn describe_domain_controllers(
        &self,
        input: DescribeDomainControllersRequest,
    ) -> Result<DescribeDomainControllersResult, RusotoError<DescribeDomainControllersError>>;

    /// <p>Obtains information about which SNS topics receive status messages from the specified directory.</p> <p>If no input parameters are provided, such as DirectoryId or TopicName, this request describes all of the associations in the account.</p>
    async fn describe_event_topics(
        &self,
        input: DescribeEventTopicsRequest,
    ) -> Result<DescribeEventTopicsResult, RusotoError<DescribeEventTopicsError>>;

    /// <p>Describes the status of LDAP security for the specified directory.</p>
    async fn describe_ldaps_settings(
        &self,
        input: DescribeLDAPSSettingsRequest,
    ) -> Result<DescribeLDAPSSettingsResult, RusotoError<DescribeLDAPSSettingsError>>;

    /// <p>Provides information about the Regions that are configured for multi-Region replication.</p>
    async fn describe_regions(
        &self,
        input: DescribeRegionsRequest,
    ) -> Result<DescribeRegionsResult, RusotoError<DescribeRegionsError>>;

    /// <p>Returns the shared directories in your account. </p>
    async fn describe_shared_directories(
        &self,
        input: DescribeSharedDirectoriesRequest,
    ) -> Result<DescribeSharedDirectoriesResult, RusotoError<DescribeSharedDirectoriesError>>;

    /// <p>Obtains information about the directory snapshots that belong to this account.</p> <p>This operation supports pagination with the use of the <i>NextToken</i> request and response parameters. If more results are available, the <i>DescribeSnapshots.NextToken</i> member contains a token that you pass in the next call to <a>DescribeSnapshots</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <i>Limit</i> parameter.</p>
    async fn describe_snapshots(
        &self,
        input: DescribeSnapshotsRequest,
    ) -> Result<DescribeSnapshotsResult, RusotoError<DescribeSnapshotsError>>;

    /// <p>Obtains information about the trust relationships for this account.</p> <p>If no input parameters are provided, such as DirectoryId or TrustIds, this request describes all the trust relationships belonging to the account.</p>
    async fn describe_trusts(
        &self,
        input: DescribeTrustsRequest,
    ) -> Result<DescribeTrustsResult, RusotoError<DescribeTrustsError>>;

    /// <p>Disables alternative client authentication methods for the specified directory. </p>
    async fn disable_client_authentication(
        &self,
        input: DisableClientAuthenticationRequest,
    ) -> Result<DisableClientAuthenticationResult, RusotoError<DisableClientAuthenticationError>>;

    /// <p>Deactivates LDAP secure calls for the specified directory.</p>
    async fn disable_ldaps(
        &self,
        input: DisableLDAPSRequest,
    ) -> Result<DisableLDAPSResult, RusotoError<DisableLDAPSError>>;

    /// <p>Disables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector or Microsoft AD directory.</p>
    async fn disable_radius(
        &self,
        input: DisableRadiusRequest,
    ) -> Result<DisableRadiusResult, RusotoError<DisableRadiusError>>;

    /// <p>Disables single-sign on for a directory.</p>
    async fn disable_sso(
        &self,
        input: DisableSsoRequest,
    ) -> Result<DisableSsoResult, RusotoError<DisableSsoError>>;

    /// <p>Enables alternative client authentication methods for the specified directory.</p>
    async fn enable_client_authentication(
        &self,
        input: EnableClientAuthenticationRequest,
    ) -> Result<EnableClientAuthenticationResult, RusotoError<EnableClientAuthenticationError>>;

    /// <p>Activates the switch for the specific directory to always use LDAP secure calls.</p>
    async fn enable_ldaps(
        &self,
        input: EnableLDAPSRequest,
    ) -> Result<EnableLDAPSResult, RusotoError<EnableLDAPSError>>;

    /// <p>Enables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector or Microsoft AD directory.</p>
    async fn enable_radius(
        &self,
        input: EnableRadiusRequest,
    ) -> Result<EnableRadiusResult, RusotoError<EnableRadiusError>>;

    /// <p>Enables single sign-on for a directory. Single sign-on allows users in your directory to access certain AWS services from a computer joined to the directory without having to enter their credentials separately.</p>
    async fn enable_sso(
        &self,
        input: EnableSsoRequest,
    ) -> Result<EnableSsoResult, RusotoError<EnableSsoError>>;

    /// <p>Obtains directory limit information for the current Region.</p>
    async fn get_directory_limits(
        &self,
    ) -> Result<GetDirectoryLimitsResult, RusotoError<GetDirectoryLimitsError>>;

    /// <p>Obtains the manual snapshot limits for a directory.</p>
    async fn get_snapshot_limits(
        &self,
        input: GetSnapshotLimitsRequest,
    ) -> Result<GetSnapshotLimitsResult, RusotoError<GetSnapshotLimitsError>>;

    /// <p>For the specified directory, lists all the certificates registered for a secure LDAP or client certificate authentication.</p>
    async fn list_certificates(
        &self,
        input: ListCertificatesRequest,
    ) -> Result<ListCertificatesResult, RusotoError<ListCertificatesError>>;

    /// <p>Lists the address blocks that you have added to a directory.</p>
    async fn list_ip_routes(
        &self,
        input: ListIpRoutesRequest,
    ) -> Result<ListIpRoutesResult, RusotoError<ListIpRoutesError>>;

    /// <p>Lists the active log subscriptions for the AWS account.</p>
    async fn list_log_subscriptions(
        &self,
        input: ListLogSubscriptionsRequest,
    ) -> Result<ListLogSubscriptionsResult, RusotoError<ListLogSubscriptionsError>>;

    /// <p>Lists all schema extensions applied to a Microsoft AD Directory.</p>
    async fn list_schema_extensions(
        &self,
        input: ListSchemaExtensionsRequest,
    ) -> Result<ListSchemaExtensionsResult, RusotoError<ListSchemaExtensionsError>>;

    /// <p>Lists all tags on a directory.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResult, RusotoError<ListTagsForResourceError>>;

    /// <p>Registers a certificate for a secure LDAP or client certificate authentication.</p>
    async fn register_certificate(
        &self,
        input: RegisterCertificateRequest,
    ) -> Result<RegisterCertificateResult, RusotoError<RegisterCertificateError>>;

    /// <p>Associates a directory with an SNS topic. This establishes the directory as a publisher to the specified SNS topic. You can then receive email or text (SMS) messages when the status of your directory changes. You get notified if your directory goes from an Active status to an Impaired or Inoperable status. You also receive a notification when the directory returns to an Active status.</p>
    async fn register_event_topic(
        &self,
        input: RegisterEventTopicRequest,
    ) -> Result<RegisterEventTopicResult, RusotoError<RegisterEventTopicError>>;

    /// <p>Rejects a directory sharing request that was sent from the directory owner account.</p>
    async fn reject_shared_directory(
        &self,
        input: RejectSharedDirectoryRequest,
    ) -> Result<RejectSharedDirectoryResult, RusotoError<RejectSharedDirectoryError>>;

    /// <p>Removes IP address blocks from a directory.</p>
    async fn remove_ip_routes(
        &self,
        input: RemoveIpRoutesRequest,
    ) -> Result<RemoveIpRoutesResult, RusotoError<RemoveIpRoutesError>>;

    /// <p>Stops all replication and removes the domain controllers from the specified Region. You cannot remove the primary Region with this operation. Instead, use the <code>DeleteDirectory</code> API.</p>
    async fn remove_region(
        &self,
        input: RemoveRegionRequest,
    ) -> Result<RemoveRegionResult, RusotoError<RemoveRegionError>>;

    /// <p>Removes tags from a directory.</p>
    async fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceRequest,
    ) -> Result<RemoveTagsFromResourceResult, RusotoError<RemoveTagsFromResourceError>>;

    /// <p><p>Resets the password for any user in your AWS Managed Microsoft AD or Simple AD directory.</p> <p>You can reset the password for any user in your directory with the following exceptions:</p> <ul> <li> <p>For Simple AD, you cannot reset the password for any user that is a member of either the <b>Domain Admins</b> or <b>Enterprise Admins</b> group except for the administrator user.</p> </li> <li> <p>For AWS Managed Microsoft AD, you can only reset the password for a user that is in an OU based off of the NetBIOS name that you typed when you created your directory. For example, you cannot reset the password for a user in the <b>AWS Reserved</b> OU. For more information about the OU structure for an AWS Managed Microsoft AD directory, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/ms_ad_getting_started_what_gets_created.html">What Gets Created</a> in the <i>AWS Directory Service Administration Guide</i>.</p> </li> </ul></p>
    async fn reset_user_password(
        &self,
        input: ResetUserPasswordRequest,
    ) -> Result<ResetUserPasswordResult, RusotoError<ResetUserPasswordError>>;

    /// <p>Restores a directory using an existing directory snapshot.</p> <p>When you restore a directory from a snapshot, any changes made to the directory after the snapshot date are overwritten.</p> <p>This action returns as soon as the restore operation is initiated. You can monitor the progress of the restore operation by calling the <a>DescribeDirectories</a> operation with the directory identifier. When the <b>DirectoryDescription.Stage</b> value changes to <code>Active</code>, the restore operation is complete.</p>
    async fn restore_from_snapshot(
        &self,
        input: RestoreFromSnapshotRequest,
    ) -> Result<RestoreFromSnapshotResult, RusotoError<RestoreFromSnapshotError>>;

    /// <p>Shares a specified directory (<code>DirectoryId</code>) in your AWS account (directory owner) with another AWS account (directory consumer). With this operation you can use your directory from any AWS account and from any Amazon VPC within an AWS Region.</p> <p>When you share your AWS Managed Microsoft AD directory, AWS Directory Service creates a shared directory in the directory consumer account. This shared directory contains the metadata to provide access to the directory within the directory owner account. The shared directory is visible in all VPCs in the directory consumer account.</p> <p>The <code>ShareMethod</code> parameter determines whether the specified directory can be shared between AWS accounts inside the same AWS organization (<code>ORGANIZATIONS</code>). It also determines whether you can share the directory with any other AWS account either inside or outside of the organization (<code>HANDSHAKE</code>).</p> <p>The <code>ShareNotes</code> parameter is only used when <code>HANDSHAKE</code> is called, which sends a directory sharing request to the directory consumer. </p>
    async fn share_directory(
        &self,
        input: ShareDirectoryRequest,
    ) -> Result<ShareDirectoryResult, RusotoError<ShareDirectoryError>>;

    /// <p>Applies a schema extension to a Microsoft AD directory.</p>
    async fn start_schema_extension(
        &self,
        input: StartSchemaExtensionRequest,
    ) -> Result<StartSchemaExtensionResult, RusotoError<StartSchemaExtensionError>>;

    /// <p>Stops the directory sharing between the directory owner and consumer accounts. </p>
    async fn unshare_directory(
        &self,
        input: UnshareDirectoryRequest,
    ) -> Result<UnshareDirectoryResult, RusotoError<UnshareDirectoryError>>;

    /// <p>Updates a conditional forwarder that has been set up for your AWS directory.</p>
    async fn update_conditional_forwarder(
        &self,
        input: UpdateConditionalForwarderRequest,
    ) -> Result<UpdateConditionalForwarderResult, RusotoError<UpdateConditionalForwarderError>>;

    /// <p>Adds or removes domain controllers to or from the directory. Based on the difference between current value and new value (provided through this API call), domain controllers will be added or removed. It may take up to 45 minutes for any new domain controllers to become fully active once the requested number of domain controllers is updated. During this time, you cannot make another update request.</p>
    async fn update_number_of_domain_controllers(
        &self,
        input: UpdateNumberOfDomainControllersRequest,
    ) -> Result<
        UpdateNumberOfDomainControllersResult,
        RusotoError<UpdateNumberOfDomainControllersError>,
    >;

    /// <p>Updates the Remote Authentication Dial In User Service (RADIUS) server information for an AD Connector or Microsoft AD directory.</p>
    async fn update_radius(
        &self,
        input: UpdateRadiusRequest,
    ) -> Result<UpdateRadiusResult, RusotoError<UpdateRadiusError>>;

    /// <p>Updates the trust that has been set up between your AWS Managed Microsoft AD directory and an on-premises Active Directory.</p>
    async fn update_trust(
        &self,
        input: UpdateTrustRequest,
    ) -> Result<UpdateTrustResult, RusotoError<UpdateTrustError>>;

    /// <p>AWS Directory Service for Microsoft Active Directory allows you to configure and verify trust relationships.</p> <p>This action verifies a trust relationship between your AWS Managed Microsoft AD directory and an external domain.</p>
    async fn verify_trust(
        &self,
        input: VerifyTrustRequest,
    ) -> Result<VerifyTrustResult, RusotoError<VerifyTrustError>>;
}
/// A client for the Directory Service API.
#[derive(Clone)]
pub struct DirectoryServiceClient {
    client: Client,
    region: region::Region,
}

impl DirectoryServiceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DirectoryServiceClient {
        DirectoryServiceClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DirectoryServiceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DirectoryServiceClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DirectoryServiceClient {
        DirectoryServiceClient { client, region }
    }
}

#[async_trait]
impl DirectoryService for DirectoryServiceClient {
    /// <p>Accepts a directory sharing request that was sent from the directory owner account.</p>
    async fn accept_shared_directory(
        &self,
        input: AcceptSharedDirectoryRequest,
    ) -> Result<AcceptSharedDirectoryResult, RusotoError<AcceptSharedDirectoryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.AcceptSharedDirectory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AcceptSharedDirectoryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AcceptSharedDirectoryResult, _>()
    }

    /// <p>If the DNS server for your on-premises domain uses a publicly addressable IP address, you must add a CIDR address block to correctly route traffic to and from your Microsoft AD on Amazon Web Services. <i>AddIpRoutes</i> adds this address block. You can also use <i>AddIpRoutes</i> to facilitate routing traffic that uses public IP ranges from your Microsoft AD on AWS to a peer VPC. </p> <p>Before you call <i>AddIpRoutes</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>AddIpRoutes</i> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    async fn add_ip_routes(
        &self,
        input: AddIpRoutesRequest,
    ) -> Result<AddIpRoutesResult, RusotoError<AddIpRoutesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.AddIpRoutes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddIpRoutesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AddIpRoutesResult, _>()
    }

    /// <p>Adds two domain controllers in the specified Region for the specified directory.</p>
    async fn add_region(
        &self,
        input: AddRegionRequest,
    ) -> Result<AddRegionResult, RusotoError<AddRegionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.AddRegion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddRegionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AddRegionResult, _>()
    }

    /// <p>Adds or overwrites one or more tags for the specified directory. Each directory can have a maximum of 50 tags. Each tag consists of a key and optional value. Tag keys must be unique to each resource.</p>
    async fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceRequest,
    ) -> Result<AddTagsToResourceResult, RusotoError<AddTagsToResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.AddTagsToResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddTagsToResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AddTagsToResourceResult, _>()
    }

    /// <p>Cancels an in-progress schema extension to a Microsoft AD directory. Once a schema extension has started replicating to all domain controllers, the task can no longer be canceled. A schema extension can be canceled during any of the following states; <code>Initializing</code>, <code>CreatingSnapshot</code>, and <code>UpdatingSchema</code>.</p>
    async fn cancel_schema_extension(
        &self,
        input: CancelSchemaExtensionRequest,
    ) -> Result<CancelSchemaExtensionResult, RusotoError<CancelSchemaExtensionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.CancelSchemaExtension",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CancelSchemaExtensionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CancelSchemaExtensionResult, _>()
    }

    /// <p>Creates an AD Connector to connect to an on-premises directory.</p> <p>Before you call <code>ConnectDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>ConnectDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    async fn connect_directory(
        &self,
        input: ConnectDirectoryRequest,
    ) -> Result<ConnectDirectoryResult, RusotoError<ConnectDirectoryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.ConnectDirectory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ConnectDirectoryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ConnectDirectoryResult, _>()
    }

    /// <p><p>Creates an alias for a directory and assigns the alias to the directory. The alias is used to construct the access URL for the directory, such as <code>http://&lt;alias&gt;.awsapps.com</code>.</p> <important> <p>After an alias has been created, it cannot be deleted or reused, so this operation should only be used when absolutely necessary.</p> </important></p>
    async fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> Result<CreateAliasResult, RusotoError<CreateAliasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateAliasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateAliasResult, _>()
    }

    /// <p>Creates an Active Directory computer object in the specified directory.</p>
    async fn create_computer(
        &self,
        input: CreateComputerRequest,
    ) -> Result<CreateComputerResult, RusotoError<CreateComputerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateComputer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateComputerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateComputerResult, _>()
    }

    /// <p>Creates a conditional forwarder associated with your AWS directory. Conditional forwarders are required in order to set up a trust relationship with another domain. The conditional forwarder points to the trusted domain.</p>
    async fn create_conditional_forwarder(
        &self,
        input: CreateConditionalForwarderRequest,
    ) -> Result<CreateConditionalForwarderResult, RusotoError<CreateConditionalForwarderError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.CreateConditionalForwarder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateConditionalForwarderError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateConditionalForwarderResult, _>()
    }

    /// <p>Creates a Simple AD directory. For more information, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/directory_simple_ad.html">Simple Active Directory</a> in the <i>AWS Directory Service Admin Guide</i>.</p> <p>Before you call <code>CreateDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>CreateDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    async fn create_directory(
        &self,
        input: CreateDirectoryRequest,
    ) -> Result<CreateDirectoryResult, RusotoError<CreateDirectoryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateDirectory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateDirectoryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateDirectoryResult, _>()
    }

    /// <p>Creates a subscription to forward real-time Directory Service domain controller security logs to the specified Amazon CloudWatch log group in your AWS account.</p>
    async fn create_log_subscription(
        &self,
        input: CreateLogSubscriptionRequest,
    ) -> Result<CreateLogSubscriptionResult, RusotoError<CreateLogSubscriptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.CreateLogSubscription",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLogSubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateLogSubscriptionResult, _>()
    }

    /// <p>Creates a Microsoft AD directory in the AWS Cloud. For more information, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/directory_microsoft_ad.html">AWS Managed Microsoft AD</a> in the <i>AWS Directory Service Admin Guide</i>.</p> <p>Before you call <i>CreateMicrosoftAD</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>CreateMicrosoftAD</i> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    async fn create_microsoft_ad(
        &self,
        input: CreateMicrosoftADRequest,
    ) -> Result<CreateMicrosoftADResult, RusotoError<CreateMicrosoftADError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.CreateMicrosoftAD",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateMicrosoftADError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateMicrosoftADResult, _>()
    }

    /// <p><p>Creates a snapshot of a Simple AD or Microsoft AD directory in the AWS cloud.</p> <note> <p>You cannot take snapshots of AD Connector directories.</p> </note></p>
    async fn create_snapshot(
        &self,
        input: CreateSnapshotRequest,
    ) -> Result<CreateSnapshotResult, RusotoError<CreateSnapshotError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateSnapshotError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateSnapshotResult, _>()
    }

    /// <p>AWS Directory Service for Microsoft Active Directory allows you to configure trust relationships. For example, you can establish a trust between your AWS Managed Microsoft AD directory, and your existing on-premises Microsoft Active Directory. This would allow you to provide users and groups access to resources in either domain, with a single set of credentials.</p> <p>This action initiates the creation of the AWS side of a trust relationship between an AWS Managed Microsoft AD directory and an external domain. You can create either a forest trust or an external trust.</p>
    async fn create_trust(
        &self,
        input: CreateTrustRequest,
    ) -> Result<CreateTrustResult, RusotoError<CreateTrustError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateTrust");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateTrustError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateTrustResult, _>()
    }

    /// <p>Deletes a conditional forwarder that has been set up for your AWS directory.</p>
    async fn delete_conditional_forwarder(
        &self,
        input: DeleteConditionalForwarderRequest,
    ) -> Result<DeleteConditionalForwarderResult, RusotoError<DeleteConditionalForwarderError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DeleteConditionalForwarder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteConditionalForwarderError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteConditionalForwarderResult, _>()
    }

    /// <p>Deletes an AWS Directory Service directory.</p> <p>Before you call <code>DeleteDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>DeleteDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    async fn delete_directory(
        &self,
        input: DeleteDirectoryRequest,
    ) -> Result<DeleteDirectoryResult, RusotoError<DeleteDirectoryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.DeleteDirectory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteDirectoryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteDirectoryResult, _>()
    }

    /// <p>Deletes the specified log subscription.</p>
    async fn delete_log_subscription(
        &self,
        input: DeleteLogSubscriptionRequest,
    ) -> Result<DeleteLogSubscriptionResult, RusotoError<DeleteLogSubscriptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DeleteLogSubscription",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteLogSubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteLogSubscriptionResult, _>()
    }

    /// <p>Deletes a directory snapshot.</p>
    async fn delete_snapshot(
        &self,
        input: DeleteSnapshotRequest,
    ) -> Result<DeleteSnapshotResult, RusotoError<DeleteSnapshotError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.DeleteSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteSnapshotError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteSnapshotResult, _>()
    }

    /// <p>Deletes an existing trust relationship between your AWS Managed Microsoft AD directory and an external domain.</p>
    async fn delete_trust(
        &self,
        input: DeleteTrustRequest,
    ) -> Result<DeleteTrustResult, RusotoError<DeleteTrustError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.DeleteTrust");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteTrustError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteTrustResult, _>()
    }

    /// <p>Deletes from the system the certificate that was registered for secure LDAP or client certificate authentication.</p>
    async fn deregister_certificate(
        &self,
        input: DeregisterCertificateRequest,
    ) -> Result<DeregisterCertificateResult, RusotoError<DeregisterCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DeregisterCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeregisterCertificateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeregisterCertificateResult, _>()
    }

    /// <p>Removes the specified directory as a publisher to the specified SNS topic.</p>
    async fn deregister_event_topic(
        &self,
        input: DeregisterEventTopicRequest,
    ) -> Result<DeregisterEventTopicResult, RusotoError<DeregisterEventTopicError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DeregisterEventTopic",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeregisterEventTopicError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeregisterEventTopicResult, _>()
    }

    /// <p>Displays information about the certificate registered for secure LDAP or client certificate authentication.</p>
    async fn describe_certificate(
        &self,
        input: DescribeCertificateRequest,
    ) -> Result<DescribeCertificateResult, RusotoError<DescribeCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeCertificateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeCertificateResult, _>()
    }

    /// <p>Obtains information about the conditional forwarders for this account.</p> <p>If no input parameters are provided for RemoteDomainNames, this request describes all conditional forwarders for the specified directory ID.</p>
    async fn describe_conditional_forwarders(
        &self,
        input: DescribeConditionalForwardersRequest,
    ) -> Result<DescribeConditionalForwardersResult, RusotoError<DescribeConditionalForwardersError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeConditionalForwarders",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeConditionalForwardersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeConditionalForwardersResult, _>()
    }

    /// <p>Obtains information about the directories that belong to this account.</p> <p>You can retrieve information about specific directories by passing the directory identifiers in the <code>DirectoryIds</code> parameter. Otherwise, all directories that belong to the current account are returned.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> request and response parameters. If more results are available, the <code>DescribeDirectoriesResult.NextToken</code> member contains a token that you pass in the next call to <a>DescribeDirectories</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <code>Limit</code> parameter.</p>
    async fn describe_directories(
        &self,
        input: DescribeDirectoriesRequest,
    ) -> Result<DescribeDirectoriesResult, RusotoError<DescribeDirectoriesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeDirectories",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeDirectoriesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeDirectoriesResult, _>()
    }

    /// <p>Provides information about any domain controllers in your directory.</p>
    async fn describe_domain_controllers(
        &self,
        input: DescribeDomainControllersRequest,
    ) -> Result<DescribeDomainControllersResult, RusotoError<DescribeDomainControllersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeDomainControllers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeDomainControllersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeDomainControllersResult, _>()
    }

    /// <p>Obtains information about which SNS topics receive status messages from the specified directory.</p> <p>If no input parameters are provided, such as DirectoryId or TopicName, this request describes all of the associations in the account.</p>
    async fn describe_event_topics(
        &self,
        input: DescribeEventTopicsRequest,
    ) -> Result<DescribeEventTopicsResult, RusotoError<DescribeEventTopicsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeEventTopics",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEventTopicsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeEventTopicsResult, _>()
    }

    /// <p>Describes the status of LDAP security for the specified directory.</p>
    async fn describe_ldaps_settings(
        &self,
        input: DescribeLDAPSSettingsRequest,
    ) -> Result<DescribeLDAPSSettingsResult, RusotoError<DescribeLDAPSSettingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeLDAPSSettings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeLDAPSSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeLDAPSSettingsResult, _>()
    }

    /// <p>Provides information about the Regions that are configured for multi-Region replication.</p>
    async fn describe_regions(
        &self,
        input: DescribeRegionsRequest,
    ) -> Result<DescribeRegionsResult, RusotoError<DescribeRegionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.DescribeRegions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeRegionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeRegionsResult, _>()
    }

    /// <p>Returns the shared directories in your account. </p>
    async fn describe_shared_directories(
        &self,
        input: DescribeSharedDirectoriesRequest,
    ) -> Result<DescribeSharedDirectoriesResult, RusotoError<DescribeSharedDirectoriesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeSharedDirectories",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeSharedDirectoriesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeSharedDirectoriesResult, _>()
    }

    /// <p>Obtains information about the directory snapshots that belong to this account.</p> <p>This operation supports pagination with the use of the <i>NextToken</i> request and response parameters. If more results are available, the <i>DescribeSnapshots.NextToken</i> member contains a token that you pass in the next call to <a>DescribeSnapshots</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <i>Limit</i> parameter.</p>
    async fn describe_snapshots(
        &self,
        input: DescribeSnapshotsRequest,
    ) -> Result<DescribeSnapshotsResult, RusotoError<DescribeSnapshotsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeSnapshots",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeSnapshotsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeSnapshotsResult, _>()
    }

    /// <p>Obtains information about the trust relationships for this account.</p> <p>If no input parameters are provided, such as DirectoryId or TrustIds, this request describes all the trust relationships belonging to the account.</p>
    async fn describe_trusts(
        &self,
        input: DescribeTrustsRequest,
    ) -> Result<DescribeTrustsResult, RusotoError<DescribeTrustsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.DescribeTrusts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTrustsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeTrustsResult, _>()
    }

    /// <p>Disables alternative client authentication methods for the specified directory. </p>
    async fn disable_client_authentication(
        &self,
        input: DisableClientAuthenticationRequest,
    ) -> Result<DisableClientAuthenticationResult, RusotoError<DisableClientAuthenticationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DisableClientAuthentication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisableClientAuthenticationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisableClientAuthenticationResult, _>()
    }

    /// <p>Deactivates LDAP secure calls for the specified directory.</p>
    async fn disable_ldaps(
        &self,
        input: DisableLDAPSRequest,
    ) -> Result<DisableLDAPSResult, RusotoError<DisableLDAPSError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.DisableLDAPS");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisableLDAPSError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DisableLDAPSResult, _>()
    }

    /// <p>Disables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector or Microsoft AD directory.</p>
    async fn disable_radius(
        &self,
        input: DisableRadiusRequest,
    ) -> Result<DisableRadiusResult, RusotoError<DisableRadiusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.DisableRadius");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisableRadiusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DisableRadiusResult, _>()
    }

    /// <p>Disables single-sign on for a directory.</p>
    async fn disable_sso(
        &self,
        input: DisableSsoRequest,
    ) -> Result<DisableSsoResult, RusotoError<DisableSsoError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.DisableSso");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisableSsoError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DisableSsoResult, _>()
    }

    /// <p>Enables alternative client authentication methods for the specified directory.</p>
    async fn enable_client_authentication(
        &self,
        input: EnableClientAuthenticationRequest,
    ) -> Result<EnableClientAuthenticationResult, RusotoError<EnableClientAuthenticationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.EnableClientAuthentication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, EnableClientAuthenticationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<EnableClientAuthenticationResult, _>()
    }

    /// <p>Activates the switch for the specific directory to always use LDAP secure calls.</p>
    async fn enable_ldaps(
        &self,
        input: EnableLDAPSRequest,
    ) -> Result<EnableLDAPSResult, RusotoError<EnableLDAPSError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.EnableLDAPS");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, EnableLDAPSError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<EnableLDAPSResult, _>()
    }

    /// <p>Enables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector or Microsoft AD directory.</p>
    async fn enable_radius(
        &self,
        input: EnableRadiusRequest,
    ) -> Result<EnableRadiusResult, RusotoError<EnableRadiusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.EnableRadius");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, EnableRadiusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<EnableRadiusResult, _>()
    }

    /// <p>Enables single sign-on for a directory. Single sign-on allows users in your directory to access certain AWS services from a computer joined to the directory without having to enter their credentials separately.</p>
    async fn enable_sso(
        &self,
        input: EnableSsoRequest,
    ) -> Result<EnableSsoResult, RusotoError<EnableSsoError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.EnableSso");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, EnableSsoError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<EnableSsoResult, _>()
    }

    /// <p>Obtains directory limit information for the current Region.</p>
    async fn get_directory_limits(
        &self,
    ) -> Result<GetDirectoryLimitsResult, RusotoError<GetDirectoryLimitsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.GetDirectoryLimits",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, GetDirectoryLimitsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetDirectoryLimitsResult, _>()
    }

    /// <p>Obtains the manual snapshot limits for a directory.</p>
    async fn get_snapshot_limits(
        &self,
        input: GetSnapshotLimitsRequest,
    ) -> Result<GetSnapshotLimitsResult, RusotoError<GetSnapshotLimitsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.GetSnapshotLimits",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetSnapshotLimitsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetSnapshotLimitsResult, _>()
    }

    /// <p>For the specified directory, lists all the certificates registered for a secure LDAP or client certificate authentication.</p>
    async fn list_certificates(
        &self,
        input: ListCertificatesRequest,
    ) -> Result<ListCertificatesResult, RusotoError<ListCertificatesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.ListCertificates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListCertificatesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListCertificatesResult, _>()
    }

    /// <p>Lists the address blocks that you have added to a directory.</p>
    async fn list_ip_routes(
        &self,
        input: ListIpRoutesRequest,
    ) -> Result<ListIpRoutesResult, RusotoError<ListIpRoutesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.ListIpRoutes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListIpRoutesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListIpRoutesResult, _>()
    }

    /// <p>Lists the active log subscriptions for the AWS account.</p>
    async fn list_log_subscriptions(
        &self,
        input: ListLogSubscriptionsRequest,
    ) -> Result<ListLogSubscriptionsResult, RusotoError<ListLogSubscriptionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.ListLogSubscriptions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLogSubscriptionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListLogSubscriptionsResult, _>()
    }

    /// <p>Lists all schema extensions applied to a Microsoft AD Directory.</p>
    async fn list_schema_extensions(
        &self,
        input: ListSchemaExtensionsRequest,
    ) -> Result<ListSchemaExtensionsResult, RusotoError<ListSchemaExtensionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.ListSchemaExtensions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListSchemaExtensionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListSchemaExtensionsResult, _>()
    }

    /// <p>Lists all tags on a directory.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResult, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResult, _>()
    }

    /// <p>Registers a certificate for a secure LDAP or client certificate authentication.</p>
    async fn register_certificate(
        &self,
        input: RegisterCertificateRequest,
    ) -> Result<RegisterCertificateResult, RusotoError<RegisterCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.RegisterCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RegisterCertificateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RegisterCertificateResult, _>()
    }

    /// <p>Associates a directory with an SNS topic. This establishes the directory as a publisher to the specified SNS topic. You can then receive email or text (SMS) messages when the status of your directory changes. You get notified if your directory goes from an Active status to an Impaired or Inoperable status. You also receive a notification when the directory returns to an Active status.</p>
    async fn register_event_topic(
        &self,
        input: RegisterEventTopicRequest,
    ) -> Result<RegisterEventTopicResult, RusotoError<RegisterEventTopicError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.RegisterEventTopic",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RegisterEventTopicError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RegisterEventTopicResult, _>()
    }

    /// <p>Rejects a directory sharing request that was sent from the directory owner account.</p>
    async fn reject_shared_directory(
        &self,
        input: RejectSharedDirectoryRequest,
    ) -> Result<RejectSharedDirectoryResult, RusotoError<RejectSharedDirectoryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.RejectSharedDirectory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RejectSharedDirectoryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RejectSharedDirectoryResult, _>()
    }

    /// <p>Removes IP address blocks from a directory.</p>
    async fn remove_ip_routes(
        &self,
        input: RemoveIpRoutesRequest,
    ) -> Result<RemoveIpRoutesResult, RusotoError<RemoveIpRoutesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.RemoveIpRoutes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RemoveIpRoutesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RemoveIpRoutesResult, _>()
    }

    /// <p>Stops all replication and removes the domain controllers from the specified Region. You cannot remove the primary Region with this operation. Instead, use the <code>DeleteDirectory</code> API.</p>
    async fn remove_region(
        &self,
        input: RemoveRegionRequest,
    ) -> Result<RemoveRegionResult, RusotoError<RemoveRegionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.RemoveRegion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RemoveRegionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RemoveRegionResult, _>()
    }

    /// <p>Removes tags from a directory.</p>
    async fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceRequest,
    ) -> Result<RemoveTagsFromResourceResult, RusotoError<RemoveTagsFromResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.RemoveTagsFromResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RemoveTagsFromResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<RemoveTagsFromResourceResult, _>()
    }

    /// <p><p>Resets the password for any user in your AWS Managed Microsoft AD or Simple AD directory.</p> <p>You can reset the password for any user in your directory with the following exceptions:</p> <ul> <li> <p>For Simple AD, you cannot reset the password for any user that is a member of either the <b>Domain Admins</b> or <b>Enterprise Admins</b> group except for the administrator user.</p> </li> <li> <p>For AWS Managed Microsoft AD, you can only reset the password for a user that is in an OU based off of the NetBIOS name that you typed when you created your directory. For example, you cannot reset the password for a user in the <b>AWS Reserved</b> OU. For more information about the OU structure for an AWS Managed Microsoft AD directory, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/ms_ad_getting_started_what_gets_created.html">What Gets Created</a> in the <i>AWS Directory Service Administration Guide</i>.</p> </li> </ul></p>
    async fn reset_user_password(
        &self,
        input: ResetUserPasswordRequest,
    ) -> Result<ResetUserPasswordResult, RusotoError<ResetUserPasswordError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.ResetUserPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ResetUserPasswordError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ResetUserPasswordResult, _>()
    }

    /// <p>Restores a directory using an existing directory snapshot.</p> <p>When you restore a directory from a snapshot, any changes made to the directory after the snapshot date are overwritten.</p> <p>This action returns as soon as the restore operation is initiated. You can monitor the progress of the restore operation by calling the <a>DescribeDirectories</a> operation with the directory identifier. When the <b>DirectoryDescription.Stage</b> value changes to <code>Active</code>, the restore operation is complete.</p>
    async fn restore_from_snapshot(
        &self,
        input: RestoreFromSnapshotRequest,
    ) -> Result<RestoreFromSnapshotResult, RusotoError<RestoreFromSnapshotError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.RestoreFromSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RestoreFromSnapshotError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RestoreFromSnapshotResult, _>()
    }

    /// <p>Shares a specified directory (<code>DirectoryId</code>) in your AWS account (directory owner) with another AWS account (directory consumer). With this operation you can use your directory from any AWS account and from any Amazon VPC within an AWS Region.</p> <p>When you share your AWS Managed Microsoft AD directory, AWS Directory Service creates a shared directory in the directory consumer account. This shared directory contains the metadata to provide access to the directory within the directory owner account. The shared directory is visible in all VPCs in the directory consumer account.</p> <p>The <code>ShareMethod</code> parameter determines whether the specified directory can be shared between AWS accounts inside the same AWS organization (<code>ORGANIZATIONS</code>). It also determines whether you can share the directory with any other AWS account either inside or outside of the organization (<code>HANDSHAKE</code>).</p> <p>The <code>ShareNotes</code> parameter is only used when <code>HANDSHAKE</code> is called, which sends a directory sharing request to the directory consumer. </p>
    async fn share_directory(
        &self,
        input: ShareDirectoryRequest,
    ) -> Result<ShareDirectoryResult, RusotoError<ShareDirectoryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.ShareDirectory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ShareDirectoryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ShareDirectoryResult, _>()
    }

    /// <p>Applies a schema extension to a Microsoft AD directory.</p>
    async fn start_schema_extension(
        &self,
        input: StartSchemaExtensionRequest,
    ) -> Result<StartSchemaExtensionResult, RusotoError<StartSchemaExtensionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.StartSchemaExtension",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartSchemaExtensionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartSchemaExtensionResult, _>()
    }

    /// <p>Stops the directory sharing between the directory owner and consumer accounts. </p>
    async fn unshare_directory(
        &self,
        input: UnshareDirectoryRequest,
    ) -> Result<UnshareDirectoryResult, RusotoError<UnshareDirectoryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.UnshareDirectory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UnshareDirectoryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UnshareDirectoryResult, _>()
    }

    /// <p>Updates a conditional forwarder that has been set up for your AWS directory.</p>
    async fn update_conditional_forwarder(
        &self,
        input: UpdateConditionalForwarderRequest,
    ) -> Result<UpdateConditionalForwarderResult, RusotoError<UpdateConditionalForwarderError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.UpdateConditionalForwarder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateConditionalForwarderError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateConditionalForwarderResult, _>()
    }

    /// <p>Adds or removes domain controllers to or from the directory. Based on the difference between current value and new value (provided through this API call), domain controllers will be added or removed. It may take up to 45 minutes for any new domain controllers to become fully active once the requested number of domain controllers is updated. During this time, you cannot make another update request.</p>
    async fn update_number_of_domain_controllers(
        &self,
        input: UpdateNumberOfDomainControllersRequest,
    ) -> Result<
        UpdateNumberOfDomainControllersResult,
        RusotoError<UpdateNumberOfDomainControllersError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.UpdateNumberOfDomainControllers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateNumberOfDomainControllersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateNumberOfDomainControllersResult, _>()
    }

    /// <p>Updates the Remote Authentication Dial In User Service (RADIUS) server information for an AD Connector or Microsoft AD directory.</p>
    async fn update_radius(
        &self,
        input: UpdateRadiusRequest,
    ) -> Result<UpdateRadiusResult, RusotoError<UpdateRadiusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.UpdateRadius");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateRadiusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateRadiusResult, _>()
    }

    /// <p>Updates the trust that has been set up between your AWS Managed Microsoft AD directory and an on-premises Active Directory.</p>
    async fn update_trust(
        &self,
        input: UpdateTrustRequest,
    ) -> Result<UpdateTrustResult, RusotoError<UpdateTrustError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.UpdateTrust");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateTrustError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateTrustResult, _>()
    }

    /// <p>AWS Directory Service for Microsoft Active Directory allows you to configure and verify trust relationships.</p> <p>This action verifies a trust relationship between your AWS Managed Microsoft AD directory and an external domain.</p>
    async fn verify_trust(
        &self,
        input: VerifyTrustRequest,
    ) -> Result<VerifyTrustResult, RusotoError<VerifyTrustError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DirectoryService_20150416.VerifyTrust");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, VerifyTrustError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<VerifyTrustResult, _>()
    }
}
