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

impl KmsClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "kms", &self.region, request_uri);

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
/// <p>Contains information about an alias.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AliasListEntry {
    /// <p>String that contains the key ARN.</p>
    #[serde(rename = "aliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
    /// <p>String that contains the alias. This value begins with <code>alias/</code>.</p>
    #[serde(rename = "aliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// <p>Date and time that the alias was most recently created in the account and Region. Formatted as Unix time.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>Date and time that the alias was most recently associated with a CMK in the account and Region. Formatted as Unix time.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>String that contains the key identifier of the CMK associated with the alias.</p>
    #[serde(rename = "targetKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_key_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelKeyDeletionRequest {
    /// <p>Identifies the customer master key (CMK) whose deletion is being canceled.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelKeyDeletionResponse {
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the CMK whose deletion is canceled.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConnectCustomKeyStoreRequest {
    /// <p>Enter the key store ID of the custom key store that you want to connect. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "customKeyStoreId")]
    pub custom_key_store_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConnectCustomKeyStoreResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAliasRequest {
    /// <p>Specifies the alias name. This value must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. </p> <p>The <code>AliasName</code> value must be string of 1-256 characters. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMKs</a>.</p>
    #[serde(rename = "aliasName")]
    pub alias_name: String,
    /// <p>Associates the alias with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a>. The CMK must be in the same AWS Region. </p> <p>A valid CMK ID is required. If you supply a null or empty string value, this operation returns an error.</p> <p>For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/viewing-keys.html#find-cmk-id-arn">Finding the Key ID and ARN</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "targetKeyId")]
    pub target_key_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCustomKeyStoreRequest {
    /// <p>Identifies the AWS CloudHSM cluster for the custom key store. Enter the cluster ID of any active AWS CloudHSM cluster that is not already associated with a custom key store. To find the cluster ID, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    #[serde(rename = "cloudHsmClusterId")]
    pub cloud_hsm_cluster_id: String,
    /// <p>Specifies a friendly name for the custom key store. The name must be unique in your AWS account.</p>
    #[serde(rename = "customKeyStoreName")]
    pub custom_key_store_name: String,
    /// <p>Enter the password of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user (CU) account</a> in the specified AWS CloudHSM cluster. AWS KMS logs into the cluster as this user to manage key material on your behalf.</p> <p>The password must be a string of 7 to 32 characters. Its value is case sensitive.</p> <p>This parameter tells AWS KMS the <code>kmsuser</code> account password; it does not change the password in the AWS CloudHSM cluster.</p>
    #[serde(rename = "keyStorePassword")]
    pub key_store_password: String,
    /// <p>Enter the content of the trust anchor certificate for the cluster. This is the content of the <code>customerCA.crt</code> file that you created when you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html">initialized the cluster</a>.</p>
    #[serde(rename = "trustAnchorCertificate")]
    pub trust_anchor_certificate: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCustomKeyStoreResponse {
    /// <p>A unique identifier for the new custom key store.</p>
    #[serde(rename = "customKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGrantRequest {
    /// <p>Specifies a grant constraint. </p> <p>AWS KMS supports the <code>EncryptionContextEquals</code> and <code>EncryptionContextSubset</code> grant constraints. Each constraint value can include up to 8 encryption context pairs. The encryption context value in each constraint cannot exceed 384 characters.</p> <p>These grant constraints allow a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operation</a> only when the encryption context in the request matches (<code>EncryptionContextEquals</code>) or includes (<code>EncryptionContextSubset</code>) the encryption context specified in this structure. For more information about encryption context, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>. For information about grant constraints, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-grant-overview.html#grant-constraints">Using grant constraints</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The encryption context grant constraints are supported only on operations that include an encryption context. You cannot use an encryption context grant constraint for cryptographic operations with asymmetric CMKs or for management operations, such as <a>DescribeKey</a> or <a>RetireGrant</a>.</p>
    #[serde(rename = "constraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<GrantConstraints>,
    /// <p>A list of grant tokens. </p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>The identity that gets the permissions specified in the grant.</p> <p>To specify the principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, IAM roles, federated users, and assumed role users. For examples of the ARN syntax to use for specifying a principal, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "granteePrincipal")]
    pub grantee_principal: String,
    /// <p>Identifies the customer master key (CMK) for the grant. The grant gives principals permission to use this CMK.</p> <p>Specify the key ID or key ARN of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>A friendly name for the grant. Use this value to prevent the unintended creation of duplicate grants when retrying this request.</p> <p>When this value is absent, all <code>CreateGrant</code> requests result in a new grant with a unique <code>GrantId</code> even if all the supplied parameters are identical. This can result in unintended duplicates when you retry the <code>CreateGrant</code> request.</p> <p>When this value is present, you can retry a <code>CreateGrant</code> request with identical parameters; if the grant already exists, the original <code>GrantId</code> is returned without creating a new grant. Note that the returned grant token is unique with every <code>CreateGrant</code> request, even when a duplicate <code>GrantId</code> is returned. All grant tokens for the same grant ID can be used interchangeably.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of operations that the grant permits. </p> <p>The operation must be supported on the CMK. For example, you cannot create a grant for a symmetric CMK that allows the <a>Sign</a> operation, or a grant for an asymmetric CMK that allows the <a>GenerateDataKey</a> operation. If you try, AWS KMS returns a <code>ValidationError</code> exception. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-grant-operations">Grant operations</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "operations")]
    pub operations: Vec<String>,
    /// <p>The principal that is given permission to retire the grant by using <a>RetireGrant</a> operation.</p> <p>To specify the principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, federated users, and assumed role users. For examples of the ARN syntax to use for specifying a principal, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "retiringPrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retiring_principal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGrantResponse {
    /// <p>The unique identifier for the grant.</p> <p>You can use the <code>GrantId</code> in a <a>ListGrants</a>, <a>RetireGrant</a>, or <a>RevokeGrant</a> operation.</p>
    #[serde(rename = "grantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// <p>The grant token.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateKeyRequest {
    /// <p>A flag to indicate whether to bypass the key policy lockout safety check.</p> <important> <p>Setting this value to true increases the risk that the CMK becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </important> <p>Use this parameter only when you include a policy in the request and you intend to prevent the principal that is making the request from making a subsequent <a>PutKeyPolicy</a> request on the CMK.</p> <p>The default value is false.</p>
    #[serde(rename = "bypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// <p>Creates the CMK in the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> and the key material in its associated AWS CloudHSM cluster. To create a CMK in a custom key store, you must also specify the <code>Origin</code> parameter with a value of <code>AWS_CLOUDHSM</code>. The AWS CloudHSM cluster that is associated with the custom key store must have at least two active HSMs, each in a different Availability Zone in the Region.</p> <p>This parameter is valid only for symmetric CMKs and regional CMKs. You cannot create an asymmetric CMK or a multi-Region CMK in a custom key store.</p> <p>To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>The response includes the custom key store ID and the ID of the AWS CloudHSM cluster.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    #[serde(rename = "customKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p><p>Specifies the type of CMK to create. The default value, <code>SYMMETRIC<em>DEFAULT</code>, creates a CMK with a 256-bit symmetric key for encryption and decryption. For help choosing a key spec for your CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-choose.html">How to Choose Your CMK Configuration</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The <code>CustomerMasterKeySpec</code> determines whether the CMK contains a symmetric key or an asymmetric key pair. It also determines the encryption algorithms or signing algorithms that the CMK supports. You can&#39;t change the <code>CustomerMasterKeySpec</code> after the CMK is created. To further restrict the algorithms that can be used with the CMK, use a condition key in its key policy or IAM policy. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/policy-conditions.html#conditions-kms-encryption-algorithm">kms:EncryptionAlgorithm</a> or <a href="https://docs.aws.amazon.com/kms/latest/developerguide/policy-conditions.html#conditions-kms-signing-algorithm">kms:Signing Algorithm</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <important> <p> &lt;a href=&quot;http://aws.amazon.com/kms/features/#AWS</em>Service<em>Integration&quot;&gt;AWS services that are integrated with AWS KMS</a> use symmetric CMKs to protect your data. These services do not support asymmetric CMKs. For help determining whether a CMK is symmetric or asymmetric, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/find-symm-asymm.html">Identifying Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </important> <p>AWS KMS supports the following key specs for CMKs:</p> <ul> <li> <p>Symmetric key (default)</p> <ul> <li> <p> <code>SYMMETRIC</em>DEFAULT</code> (AES-256-GCM)</p> </li> </ul> </li> <li> <p>Asymmetric RSA key pairs</p> <ul> <li> <p> <code>RSA<em>2048</code> </p> </li> <li> <p> <code>RSA</em>3072</code> </p> </li> <li> <p> <code>RSA<em>4096</code> </p> </li> </ul> </li> <li> <p>Asymmetric NIST-recommended elliptic curve key pairs</p> <ul> <li> <p> <code>ECC</em>NIST<em>P256</code> (secp256r1)</p> </li> <li> <p> <code>ECC</em>NIST<em>P384</code> (secp384r1)</p> </li> <li> <p> <code>ECC</em>NIST<em>P521</code> (secp521r1)</p> </li> </ul> </li> <li> <p>Other asymmetric elliptic curve key pairs</p> <ul> <li> <p> <code>ECC</em>SECG_P256K1</code> (secp256k1), commonly used for cryptocurrencies.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "customerMasterKeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_master_key_spec: Option<String>,
    /// <p>A description of the CMK.</p> <p>Use a description that helps you decide whether the CMK is appropriate for a task. The default value is an empty string (no description).</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>Determines the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a> for which you can use the CMK. The default value is <code>ENCRYPT<em>DECRYPT</code>. This parameter is required only for asymmetric CMKs. You can&#39;t change the <code>KeyUsage</code> value after the CMK is created.</p> <p>Select only one valid value.</p> <ul> <li> <p>For symmetric CMKs, omit the parameter or specify <code>ENCRYPT</em>DECRYPT</code>.</p> </li> <li> <p>For asymmetric CMKs with RSA key material, specify <code>ENCRYPT<em>DECRYPT</code> or <code>SIGN</em>VERIFY</code>.</p> </li> <li> <p>For asymmetric CMKs with ECC key material, specify <code>SIGN_VERIFY</code>.</p> </li> </ul></p>
    #[serde(rename = "keyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// <p>Creates a multi-Region primary key that you can replicate into other AWS Regions. You cannot change this value after you create the CMK. </p> <p>For a multi-Region key, set this parameter to <code>True</code>. For a single-Region CMK, omit this parameter or set it to <code>False</code>. The default value is <code>False</code>.</p> <p>This operation supports <i>multi-Region keys</i>, an AWS KMS feature that lets you create multiple interoperable CMKs in different AWS Regions. Because these CMKs have the same key ID, key material, and other metadata, you can use them to encrypt data in one AWS Region and decrypt it in a different AWS Region without making a cross-Region call or exposing the plaintext data. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>This value creates a <i>primary key</i>, not a replica. To create a <i>replica key</i>, use the <a>ReplicateKey</a> operation. </p> <p>You can create a symmetric or asymmetric multi-Region CMK, and you can create a multi-Region CMK with imported key material. However, you cannot create a multi-Region CMK in a custom key store.</p>
    #[serde(rename = "multiRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region: Option<bool>,
    /// <p>The source of the key material for the CMK. You cannot change the origin after you create the CMK. The default is <code>AWS_KMS</code>, which means that AWS KMS creates the key material.</p> <p>To create a CMK with no key material (for imported key material), set the value to <code>EXTERNAL</code>. For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>. This value is valid only for symmetric CMKs.</p> <p>To create a CMK in an AWS KMS <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> and create its key material in the associated AWS CloudHSM cluster, set this value to <code>AWS_CLOUDHSM</code>. You must also use the <code>CustomKeyStoreId</code> parameter to identify the custom key store. This value is valid only for symmetric CMKs.</p>
    #[serde(rename = "origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// <p>The key policy to attach to the CMK.</p> <p>If you provide a key policy, it must meet the following criteria:</p> <ul> <li> <p>If you don't set <code>BypassPolicyLockoutSafetyCheck</code> to true, the key policy must allow the principal that is making the <code>CreateKey</code> request to make a subsequent <a>PutKeyPolicy</a> request on the CMK. This reduces the risk that the CMK becomes unmanageable. For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section of the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </li> <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to AWS KMS. When you create a new AWS principal (for example, an IAM user or role), you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to AWS KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> </li> </ul> <p>If you do not provide a key policy, AWS KMS attaches a default key policy to the CMK. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default">Default Key Policy</a> in the <i>AWS Key Management Service Developer Guide</i>. </p> <p>The key policy size quota is 32 kilobytes (32768 bytes).</p> <p>For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>IAM User Guide</i> </i>.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>Assigns one or more tags to the CMK. Use this parameter to tag the CMK when it is created. To tag an existing CMK, use the <a>TagResource</a> operation.</p> <note> <p>Tagging or untagging a CMK can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>To use this parameter, you must have <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> permission in an IAM policy.</p> <p>Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string. You cannot have more than one tag on a CMK with the same tag key. If you specify an existing tag key with a different tag value, AWS KMS replaces the current tag value with the specified one.</p> <p>When you assign tags to an AWS resource, AWS generates a cost allocation report with usage and costs aggregated by tags. Tags can also be used to control access to a CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging Keys</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateKeyResponse {
    /// <p>Metadata associated with the CMK.</p>
    #[serde(rename = "keyMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<KeyMetadata>,
}

/// <p>Contains information about each custom key store in the custom key store list.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CustomKeyStoresListEntry {
    /// <p>A unique identifier for the AWS CloudHSM cluster that is associated with the custom key store.</p>
    #[serde(rename = "cloudHsmClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    /// <p><p>Describes the connection error. This field appears in the response only when the <code>ConnectionState</code> is <code>FAILED</code>. For help resolving these errors, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-failed">How to Fix a Connection Failure</a> in <i>AWS Key Management Service Developer Guide</i>.</p> <p>Valid values are:</p> <ul> <li> <p> <code>CLUSTER<em>NOT</em>FOUND</code> - AWS KMS cannot find the AWS CloudHSM cluster with the specified cluster ID.</p> </li> <li> <p> <code>INSUFFICIENT<em>CLOUDHSM</em>HSMS</code> - The associated AWS CloudHSM cluster does not contain any active HSMs. To connect a custom key store to its AWS CloudHSM cluster, the cluster must contain at least one active HSM.</p> </li> <li> <p> <code>INTERNAL<em>ERROR</code> - AWS KMS could not complete the request due to an internal error. Retry the request. For <code>ConnectCustomKeyStore</code> requests, disconnect the custom key store before trying to connect again.</p> </li> <li> <p> <code>INVALID</em>CREDENTIALS</code> - AWS KMS does not have the correct password for the <code>kmsuser</code> crypto user in the AWS CloudHSM cluster. Before you can connect your custom key store to its AWS CloudHSM cluster, you must change the <code>kmsuser</code> account password and update the key store password value for the custom key store.</p> </li> <li> <p> <code>NETWORK<em>ERRORS</code> - Network errors are preventing AWS KMS from connecting to the custom key store.</p> </li> <li> <p> <code>SUBNET</em>NOT<em>FOUND</code> - A subnet in the AWS CloudHSM cluster configuration was deleted. If AWS KMS cannot find all of the subnets in the cluster configuration, attempts to connect the custom key store to the AWS CloudHSM cluster fail. To fix this error, create a cluster from a recent backup and associate it with your custom key store. (This process creates a new cluster configuration with a VPC and private subnets.) For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-failed">How to Fix a Connection Failure</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p> <code>USER</em>LOCKED<em>OUT</code> - The <code>kmsuser</code> CU account is locked out of the associated AWS CloudHSM cluster due to too many failed password attempts. Before you can connect your custom key store to its AWS CloudHSM cluster, you must change the <code>kmsuser</code> account password and update the key store password value for the custom key store.</p> </li> <li> <p> <code>USER</em>LOGGED<em>IN</code> - The <code>kmsuser</code> CU account is logged into the the associated AWS CloudHSM cluster. This prevents AWS KMS from rotating the <code>kmsuser</code> account password and logging into the cluster. Before you can connect your custom key store to its AWS CloudHSM cluster, you must log the <code>kmsuser</code> CU out of the cluster. If you changed the <code>kmsuser</code> password to log into the cluster, you must also and update the key store password value for the custom key store. For help, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#login-kmsuser-2">How to Log Out and Reconnect</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p> <code>USER</em>NOT_FOUND</code> - AWS KMS cannot find a <code>kmsuser</code> CU account in the associated AWS CloudHSM cluster. Before you can connect your custom key store to its AWS CloudHSM cluster, you must create a <code>kmsuser</code> CU account in the cluster, and then update the key store password value for the custom key store.</p> </li> </ul></p>
    #[serde(rename = "connectionErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_error_code: Option<String>,
    /// <p>Indicates whether the custom key store is connected to its AWS CloudHSM cluster.</p> <p>You can create and use CMKs in your custom key stores only when its connection state is <code>CONNECTED</code>.</p> <p>The value is <code>DISCONNECTED</code> if the key store has never been connected or you use the <a>DisconnectCustomKeyStore</a> operation to disconnect it. If the value is <code>CONNECTED</code> but you are having trouble using the custom key store, make sure that its associated AWS CloudHSM cluster is active and contains at least one active HSM.</p> <p>A value of <code>FAILED</code> indicates that an attempt to connect was unsuccessful. The <code>ConnectionErrorCode</code> field in the response indicates the cause of the failure. For help resolving a connection failure, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>The date and time when the custom key store was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique identifier for the custom key store.</p>
    #[serde(rename = "customKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>The user-specified friendly name for the custom key store.</p>
    #[serde(rename = "customKeyStoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_name: Option<String>,
    /// <p>The trust anchor certificate of the associated AWS CloudHSM cluster. When you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html#sign-csr">initialize the cluster</a>, you create this certificate and save it in the <code>customerCA.crt</code> file.</p>
    #[serde(rename = "trustAnchorCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchor_certificate: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DecryptRequest {
    /// <p>Ciphertext to be decrypted. The blob includes metadata.</p>
    #[serde(rename = "ciphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub ciphertext_blob: bytes::Bytes,
    /// <p>Specifies the encryption algorithm that will be used to decrypt the ciphertext. Specify the same algorithm that was used to encrypt the data. If you specify a different algorithm, the <code>Decrypt</code> operation fails.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric CMK. The default value, <code>SYMMETRIC_DEFAULT</code>, represents the only supported algorithm that is valid for symmetric CMKs.</p>
    #[serde(rename = "encryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// <p>Specifies the encryption context to use when decrypting the data. An encryption context is valid only for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a> with a symmetric CMK. The standard asymmetric encryption algorithms that AWS KMS uses do not support an encryption context.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "encryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens. </p> <p>Use a grant token when your permission to call this operation comes from a newly created grant that has not yet achieved eventual consistency. Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Specifies the customer master key (CMK) that AWS KMS uses to decrypt the ciphertext. Enter a key ID of the CMK that was used to encrypt the ciphertext.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric CMK. If you used a symmetric CMK, AWS KMS can get the CMK from metadata that it adds to the symmetric ciphertext blob. However, it is always recommended as a best practice. This practice ensures that you use the CMK that you intend.</p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DecryptResponse {
    /// <p>The encryption algorithm that was used to decrypt the ciphertext.</p>
    #[serde(rename = "encryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the CMK that was used to decrypt the ciphertext.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>Decrypted plaintext data. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<bytes::Bytes>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAliasRequest {
    /// <p>The alias to be deleted. The alias name must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>.</p>
    #[serde(rename = "aliasName")]
    pub alias_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCustomKeyStoreRequest {
    /// <p>Enter the ID of the custom key store you want to delete. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "customKeyStoreId")]
    pub custom_key_store_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCustomKeyStoreResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteImportedKeyMaterialRequest {
    /// <p>Identifies the CMK from which you are deleting imported key material. The <code>Origin</code> of the CMK must be <code>EXTERNAL</code>.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCustomKeyStoresRequest {
    /// <p>Gets only information about the specified custom key store. Enter the key store ID.</p> <p>By default, this operation gets information about all custom key stores in the account and Region. To limit the output to a particular custom key store, you can use either the <code>CustomKeyStoreId</code> or <code>CustomKeyStoreName</code> parameter, but not both.</p>
    #[serde(rename = "customKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>Gets only information about the specified custom key store. Enter the friendly name of the custom key store.</p> <p>By default, this operation gets information about all custom key stores in the account and Region. To limit the output to a particular custom key store, you can use either the <code>CustomKeyStoreId</code> or <code>CustomKeyStoreName</code> parameter, but not both.</p>
    #[serde(rename = "customKeyStoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_name: Option<String>,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCustomKeyStoresResponse {
    /// <p>Contains metadata about each custom key store.</p>
    #[serde(rename = "customKeyStores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_stores: Option<Vec<CustomKeyStoresListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeKeyRequest {
    /// <p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Describes the specified customer master key (CMK). </p> <p>If you specify a predefined AWS alias (an AWS alias with no key ID), KMS associates the alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">AWS managed CMK</a> and returns its <code>KeyId</code> and <code>Arn</code> in the response.</p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeKeyResponse {
    /// <p>Metadata associated with the key.</p>
    #[serde(rename = "keyMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<KeyMetadata>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableKeyRequest {
    /// <p>Identifies the customer master key (CMK) to disable.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableKeyRotationRequest {
    /// <p>Identifies a symmetric customer master key (CMK). You cannot enable or disable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html#asymmetric-cmks">asymmetric CMKs</a>, CMKs with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisconnectCustomKeyStoreRequest {
    /// <p>Enter the ID of the custom key store you want to disconnect. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "customKeyStoreId")]
    pub custom_key_store_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisconnectCustomKeyStoreResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableKeyRequest {
    /// <p>Identifies the customer master key (CMK) to enable.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableKeyRotationRequest {
    /// <p>Identifies a symmetric customer master key (CMK). You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#asymmetric-cmks">asymmetric CMKs</a>, CMKs with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html#mrk-replica-key">multi-Region keys</a>, set the property on the primary key.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EncryptRequest {
    /// <p>Specifies the encryption algorithm that AWS KMS will use to encrypt the plaintext message. The algorithm must be compatible with the CMK that you specify.</p> <p>This parameter is required only for asymmetric CMKs. The default value, <code>SYMMETRIC_DEFAULT</code>, is the algorithm used for symmetric CMKs. If you are using an asymmetric CMK, we recommend RSAES_OAEP_SHA_256.</p>
    #[serde(rename = "encryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// <p>Specifies the encryption context that will be used to encrypt the data. An encryption context is valid only for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a> with a symmetric CMK. The standard asymmetric encryption algorithms that AWS KMS uses do not support an encryption context. </p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "encryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Identifies the customer master key (CMK) to use in the encryption operation.</p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>Data to be encrypted.</p>
    #[serde(rename = "plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub plaintext: bytes::Bytes,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EncryptResponse {
    /// <p>The encrypted plaintext. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "ciphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The encryption algorithm that was used to encrypt the plaintext.</p>
    #[serde(rename = "encryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the CMK that was used to encrypt the plaintext.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GenerateDataKeyPairRequest {
    /// <p>Specifies the encryption context that will be used when encrypting the private key in the data key pair.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "encryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Specifies the symmetric CMK that encrypts the private key in the data key pair. You cannot specify an asymmetric CMK or a CMK in a custom key store. To get the type and origin of your CMK, use the <a>DescribeKey</a> operation.</p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>Determines the type of data key pair that is generated. </p> <p>The AWS KMS rule that restricts the use of asymmetric RSA CMKs to encrypt and decrypt or to sign and verify (but not both), and the rule that permits you to use ECC CMKs only to sign and verify, are not effective outside of AWS KMS.</p>
    #[serde(rename = "keyPairSpec")]
    pub key_pair_spec: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateDataKeyPairResponse {
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the CMK that encrypted the private key.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The type of data key pair that was generated.</p>
    #[serde(rename = "keyPairSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_spec: Option<String>,
    /// <p>The encrypted copy of the private key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "privateKeyCiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The plaintext copy of the private key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "privateKeyPlaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_plaintext: Option<bytes::Bytes>,
    /// <p>The public key (in plaintext).</p>
    #[serde(rename = "publicKey")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<bytes::Bytes>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GenerateDataKeyPairWithoutPlaintextRequest {
    /// <p>Specifies the encryption context that will be used when encrypting the private key in the data key pair.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "encryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Specifies the CMK that encrypts the private key in the data key pair. You must specify a symmetric CMK. You cannot use an asymmetric CMK or a CMK in a custom key store. To get the type and origin of your CMK, use the <a>DescribeKey</a> operation. </p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>Determines the type of data key pair that is generated.</p> <p>The AWS KMS rule that restricts the use of asymmetric RSA CMKs to encrypt and decrypt or to sign and verify (but not both), and the rule that permits you to use ECC CMKs only to sign and verify, are not effective outside of AWS KMS.</p>
    #[serde(rename = "keyPairSpec")]
    pub key_pair_spec: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateDataKeyPairWithoutPlaintextResponse {
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the CMK that encrypted the private key.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The type of data key pair that was generated.</p>
    #[serde(rename = "keyPairSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_spec: Option<String>,
    /// <p>The encrypted copy of the private key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "privateKeyCiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The public key (in plaintext).</p>
    #[serde(rename = "publicKey")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<bytes::Bytes>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GenerateDataKeyRequest {
    /// <p>Specifies the encryption context that will be used when encrypting the data key.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "encryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Identifies the symmetric CMK that encrypts the data key.</p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>Specifies the length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p> <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    #[serde(rename = "keySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// <p>Specifies the length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For 128-bit (16-byte) and 256-bit (32-byte) data keys, use the <code>KeySpec</code> parameter.</p> <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    #[serde(rename = "numberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateDataKeyResponse {
    /// <p>The encrypted copy of the data key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "ciphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the CMK that encrypted the data key.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The plaintext data key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded. Use this data key to encrypt your data outside of KMS. Then, remove it from memory as soon as possible.</p>
    #[serde(rename = "plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<bytes::Bytes>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GenerateDataKeyWithoutPlaintextRequest {
    /// <p>Specifies the encryption context that will be used when encrypting the data key.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "encryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>The identifier of the symmetric customer master key (CMK) that encrypts the data key.</p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>The length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>
    #[serde(rename = "keySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// <p>The length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use the <code>KeySpec</code> field instead of this one.</p>
    #[serde(rename = "numberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateDataKeyWithoutPlaintextResponse {
    /// <p>The encrypted data key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "ciphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the CMK that encrypted the data key.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GenerateRandomRequest {
    /// <p>Generates the random byte string in the AWS CloudHSM cluster that is associated with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "customKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>The length of the byte string.</p>
    #[serde(rename = "numberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateRandomResponse {
    /// <p>The random byte string. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<bytes::Bytes>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetKeyPolicyRequest {
    /// <p>Gets the key policy for the specified customer master key (CMK).</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>Specifies the name of the key policy. The only valid name is <code>default</code>. To get the names of key policies, use <a>ListKeyPolicies</a>.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetKeyPolicyResponse {
    /// <p>A key policy document in JSON format.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetKeyRotationStatusRequest {
    /// <p>Gets the rotation status for the specified customer master key (CMK).</p> <p>Specify the key ID or key ARN of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetKeyRotationStatusResponse {
    /// <p>A Boolean value that specifies whether key rotation is enabled.</p>
    #[serde(rename = "keyRotationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetParametersForImportRequest {
    /// <p>The identifier of the symmetric CMK into which you will import key material. The <code>Origin</code> of the CMK must be <code>EXTERNAL</code>.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>The algorithm you will use to encrypt the key material before importing it with <a>ImportKeyMaterial</a>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-encrypt-key-material.html">Encrypt the Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "wrappingAlgorithm")]
    pub wrapping_algorithm: String,
    /// <p>The type of wrapping key (public key) to return in the response. Only 2048-bit RSA public keys are supported.</p>
    #[serde(rename = "wrappingKeySpec")]
    pub wrapping_key_spec: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetParametersForImportResponse {
    /// <p>The import token to send in a subsequent <a>ImportKeyMaterial</a> request.</p>
    #[serde(rename = "importToken")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_token: Option<bytes::Bytes>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the CMK to use in a subsequent <a>ImportKeyMaterial</a> request. This is the same CMK specified in the <code>GetParametersForImport</code> request.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The time at which the import token and public key are no longer valid. After this time, you cannot use them to make an <a>ImportKeyMaterial</a> request and you must send another <code>GetParametersForImport</code> request to get new ones.</p>
    #[serde(rename = "parametersValidTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters_valid_to: Option<f64>,
    /// <p>The public key to use to encrypt the key material before importing it with <a>ImportKeyMaterial</a>.</p>
    #[serde(rename = "publicKey")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<bytes::Bytes>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPublicKeyRequest {
    /// <p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Identifies the asymmetric CMK that includes the public key.</p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPublicKeyResponse {
    /// <p>The type of the of the public key that was downloaded.</p>
    #[serde(rename = "customerMasterKeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_master_key_spec: Option<String>,
    /// <p>The encryption algorithms that AWS KMS supports for this key. </p> <p>This information is critical. If a public key encrypts data outside of AWS KMS by using an unsupported encryption algorithm, the ciphertext cannot be decrypted. </p> <p>This field appears in the response only when the <code>KeyUsage</code> of the public key is <code>ENCRYPT_DECRYPT</code>.</p>
    #[serde(rename = "encryptionAlgorithms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithms: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the asymmetric CMK from which the public key was downloaded.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The permitted use of the public key. Valid values are <code>ENCRYPT_DECRYPT</code> or <code>SIGN_VERIFY</code>. </p> <p>This information is critical. If a public key with <code>SIGN_VERIFY</code> key usage encrypts data outside of AWS KMS, the ciphertext cannot be decrypted. </p>
    #[serde(rename = "keyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// <p><p>The exported public key. </p> <p>The value is a DER-encoded X.509 public key, also known as <code>SubjectPublicKeyInfo</code> (SPKI), as defined in <a href="https://tools.ietf.org/html/rfc5280">RFC 5280</a>. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p> <p/></p>
    #[serde(rename = "publicKey")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<bytes::Bytes>,
    /// <p>The signing algorithms that AWS KMS supports for this key.</p> <p>This field appears in the response only when the <code>KeyUsage</code> of the public key is <code>SIGN_VERIFY</code>.</p>
    #[serde(rename = "signingAlgorithms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithms: Option<Vec<String>>,
}

/// <p><p>Use this structure to allow <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a> in the grant only when the operation request includes the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">encryption context</a>. </p> <p>AWS KMS applies the grant constraints only to cryptographic operations that support an encryption context, that is, all cryptographic operations with a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#symmetric-cmks">symmetric CMK</a>. Grant constraints are not applied to operations that do not support an encryption context, such as cryptographic operations with asymmetric CMKs and management operations, such as <a>DescribeKey</a> or <a>RetireGrant</a>.</p> <important> <p>In a cryptographic operation, the encryption context in the decryption operation must be an exact, case-sensitive match for the keys and values in the encryption context of the encryption operation. Only the order of the pairs can vary.</p> <p>However, in a grant constraint, the key in each key-value pair is not case sensitive, but the value is case sensitive.</p> <p>To avoid confusion, do not use multiple encryption context pairs that differ only by case. To require a fully case-sensitive encryption context, use the <code>kms:EncryptionContext:</code> and <code>kms:EncryptionContextKeys</code> conditions in an IAM or key policy. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/policy-conditions.html#conditions-kms-encryption-context">kms:EncryptionContext:</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </important></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GrantConstraints {
    /// <p>A list of key-value pairs that must match the encryption context in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operation</a> request. The grant allows the operation only when the encryption context in the request is the same as the encryption context specified in this constraint.</p>
    #[serde(rename = "encryptionContextEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_equals: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of key-value pairs that must be included in the encryption context of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operation</a> request. The grant allows the cryptographic operation only when the encryption context in the request includes the key-value pairs specified in this constraint, although it can include additional key-value pairs.</p>
    #[serde(rename = "encryptionContextSubset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_subset: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Contains information about a grant.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GrantListEntry {
    /// <p>A list of key-value pairs that must be present in the encryption context of certain subsequent operations that the grant allows.</p>
    #[serde(rename = "constraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<GrantConstraints>,
    /// <p>The date and time when the grant was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The unique identifier for the grant.</p>
    #[serde(rename = "grantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// <p>The identity that gets the permissions in the grant.</p> <p>The <code>GranteePrincipal</code> field in the <code>ListGrants</code> response usually contains the user or role designated as the grantee principal in the grant. However, when the grantee principal in the grant is an AWS service, the <code>GranteePrincipal</code> field contains the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#principal-services">service principal</a>, which might represent several different grantee principals.</p>
    #[serde(rename = "granteePrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee_principal: Option<String>,
    /// <p>The AWS account under which the grant was issued.</p>
    #[serde(rename = "issuingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_account: Option<String>,
    /// <p>The unique identifier for the customer master key (CMK) to which the grant applies.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The friendly name that identifies the grant. If a name was provided in the <a>CreateGrant</a> request, that name is returned. Otherwise this value is null.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The list of operations permitted by the grant.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    /// <p>The principal that can retire the grant.</p>
    #[serde(rename = "retiringPrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retiring_principal: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportKeyMaterialRequest {
    /// <p>The encrypted key material to import. The key material must be encrypted with the public wrapping key that <a>GetParametersForImport</a> returned, using the wrapping algorithm that you specified in the same <code>GetParametersForImport</code> request.</p>
    #[serde(rename = "encryptedKeyMaterial")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub encrypted_key_material: bytes::Bytes,
    /// <p>Specifies whether the key material expires. The default is <code>KEY_MATERIAL_EXPIRES</code>, in which case you must include the <code>ValidTo</code> parameter. When this parameter is set to <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>, you must omit the <code>ValidTo</code> parameter.</p>
    #[serde(rename = "expirationModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_model: Option<String>,
    /// <p>The import token that you received in the response to a previous <a>GetParametersForImport</a> request. It must be from the same response that contained the public key that you used to encrypt the key material.</p>
    #[serde(rename = "importToken")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub import_token: bytes::Bytes,
    /// <p>The identifier of the symmetric CMK that receives the imported key material. The CMK's <code>Origin</code> must be <code>EXTERNAL</code>. This must be the same CMK specified in the <code>KeyID</code> parameter of the corresponding <a>GetParametersForImport</a> request.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>The time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. You must omit this parameter when the <code>ExpirationModel</code> parameter is set to <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>. Otherwise it is required.</p>
    #[serde(rename = "validTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportKeyMaterialResponse {}

/// <p>Contains information about each entry in the key list.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KeyListEntry {
    /// <p>ARN of the key.</p>
    #[serde(rename = "keyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
    /// <p>Unique identifier of the key.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

/// <p>Contains metadata about a customer master key (CMK).</p> <p>This data type is used as a response element for the <a>CreateKey</a> and <a>DescribeKey</a> operations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KeyMetadata {
    /// <p>The twelve-digit account ID of the AWS account that owns the CMK.</p>
    #[serde(rename = "aWSAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the CMK. For examples, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms">AWS Key Management Service (AWS KMS)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The cluster ID of the AWS CloudHSM cluster that contains the key material for the CMK. When you create a CMK in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, AWS KMS creates the key material for the CMK in the associated AWS CloudHSM cluster. This value is present only when the CMK is created in a custom key store.</p>
    #[serde(rename = "cloudHsmClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    /// <p>The date and time when the CMK was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique identifier for the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> that contains the CMK. This value is present only when the CMK is created in a custom key store.</p>
    #[serde(rename = "customKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>Describes the type of key material in the CMK.</p>
    #[serde(rename = "customerMasterKeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_master_key_spec: Option<String>,
    /// <p>The date and time after which AWS KMS deletes this CMK. This value is present only when the CMK is scheduled for deletion, that is, when its <code>KeyState</code> is <code>PendingDeletion</code>.</p> <p>When the primary key in a multi-Region key is scheduled for deletion but still has replica keys, its key state is <code>PendingReplicaDeletion</code> and the length of its waiting period is displayed in the <code>PendingDeletionWindowInDays</code> field.</p>
    #[serde(rename = "deletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    /// <p>The description of the CMK.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Specifies whether the CMK is enabled. When <code>KeyState</code> is <code>Enabled</code> this value is true, otherwise it is false.</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The encryption algorithms that the CMK supports. You cannot use the CMK with other encryption algorithms within AWS KMS.</p> <p>This value is present only when the <code>KeyUsage</code> of the CMK is <code>ENCRYPT_DECRYPT</code>.</p>
    #[serde(rename = "encryptionAlgorithms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithms: Option<Vec<String>>,
    /// <p>Specifies whether the CMK's key material expires. This value is present only when <code>Origin</code> is <code>EXTERNAL</code>, otherwise this value is omitted.</p>
    #[serde(rename = "expirationModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_model: Option<String>,
    /// <p>The globally unique identifier for the CMK.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>The manager of the CMK. CMKs in your AWS account are either customer managed or AWS managed. For more information about the difference, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "keyManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_manager: Option<String>,
    /// <p>The current status of the CMK.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "keyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    /// <p>The <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a> for which you can use the CMK.</p>
    #[serde(rename = "keyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// <p>Indicates whether the CMK is a multi-Region (<code>True</code>) or regional (<code>False</code>) key. This value is <code>True</code> for multi-Region primary and replica CMKs and <code>False</code> for regional CMKs.</p> <p>For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "multiRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region: Option<bool>,
    /// <p><p>Lists the primary and replica CMKs in same multi-Region CMK. This field is present only when the value of the <code>MultiRegion</code> field is <code>True</code>.</p> <p>For more information about any listed CMK, use the <a>DescribeKey</a> operation.</p> <ul> <li> <p> <code>MultiRegionKeyType</code> indicates whether the CMK is a <code>PRIMARY</code> or <code>REPLICA</code> key.</p> </li> <li> <p> <code>PrimaryKey</code> displays the key ARN and Region of the primary key. This field displays the current CMK if it is the primary key.</p> </li> <li> <p> <code>ReplicaKeys</code> displays the key ARNs and Regions of all replica keys. This field includes the current CMK if it is a replica key.</p> </li> </ul></p>
    #[serde(rename = "multiRegionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_configuration: Option<MultiRegionConfiguration>,
    /// <p>The source of the CMK's key material. When this value is <code>AWS_KMS</code>, AWS KMS created the key material. When this value is <code>EXTERNAL</code>, the key material was imported from your existing key management infrastructure or the CMK lacks key material. When this value is <code>AWS_CLOUDHSM</code>, the key material was created in the AWS CloudHSM cluster associated with a custom key store.</p>
    #[serde(rename = "origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// <p>The waiting period before the primary key in a multi-Region key is deleted. This waiting period begins when the last of its replica keys is deleted. This value is present only when the <code>KeyState</code> of the CMK is <code>PendingReplicaDeletion</code>. That indicates that the CMK is the primary key in a multi-Region key, it is scheduled for deletion, and it still has existing replica keys.</p> <p>When a regional CMK or a replica key in a multi-Region key is scheduled for deletion, its deletion date is displayed in the <code>DeletionDate</code> field. However, when the primary key in a multi-Region key is scheduled for deletion, its waiting period doesn't begin until all of its replica keys are deleted. This value displays that waiting period. When the last replica key in the multi-Region key is deleted, the <code>KeyState</code> of the scheduled primary key changes from <code>PendingReplicaDeletion</code> to <code>PendingDeletion</code> and the deletion date appears in the <code>DeletionDate</code> field.</p>
    #[serde(rename = "pendingDeletionWindowInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_deletion_window_in_days: Option<i64>,
    /// <p>The signing algorithms that the CMK supports. You cannot use the CMK with other signing algorithms within AWS KMS.</p> <p>This field appears only when the <code>KeyUsage</code> of the CMK is <code>SIGN_VERIFY</code>.</p>
    #[serde(rename = "signingAlgorithms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithms: Option<Vec<String>>,
    /// <p>The time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. This value is present only for CMKs whose <code>Origin</code> is <code>EXTERNAL</code> and whose <code>ExpirationModel</code> is <code>KEY_MATERIAL_EXPIRES</code>, otherwise this value is omitted.</p>
    #[serde(rename = "validTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAliasesRequest {
    /// <p>Lists only aliases that are associated with the specified CMK. Enter a CMK in your AWS account. </p> <p>This parameter is optional. If you omit it, <code>ListAliases</code> returns all aliases in the account and Region.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAliasesResponse {
    /// <p>A list of aliases.</p>
    #[serde(rename = "aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<AliasListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGrantsRequest {
    /// <p>Returns only the grant with the specified grant ID. The grant ID uniquely identifies the grant. </p>
    #[serde(rename = "grantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// <p>Returns only grants where the specified principal is the grantee principal for the grant.</p>
    #[serde(rename = "granteePrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee_principal: Option<String>,
    /// <p>Returns only grants for the specified customer master key (CMK). This parameter is required.</p> <p>Specify the key ID or key ARN of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGrantsResponse {
    /// <p>A list of grants.</p>
    #[serde(rename = "grants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<GrantListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListKeyPoliciesRequest {
    /// <p>Gets the names of key policies for the specified customer master key (CMK).</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p> <p>Only one policy can be attached to a key.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListKeyPoliciesResponse {
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A list of key policy names. The only valid value is <code>default</code>.</p>
    #[serde(rename = "policyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListKeysRequest {
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListKeysResponse {
    /// <p>A list of customer master keys (CMKs).</p>
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<KeyListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourceTagsRequest {
    /// <p>Gets tags on the specified customer master key (CMK).</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 50, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p> <p>Do not attempt to construct this value. Use only the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourceTagsResponse {
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p> <p>Do not assume or infer any information from this value.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p>A list of tags. Each tag consists of a tag key and a tag value.</p> <note> <p>Tagging or untagging a CMK can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRetirableGrantsRequest {
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The retiring principal for which to list grants. Enter a principal in your AWS account.</p> <p>To specify the retiring principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, federated users, and assumed role users. For examples of the ARN syntax for specifying a principal, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "retiringPrincipal")]
    pub retiring_principal: String,
}

/// <p>Describes the configuration of this multi-Region CMK. This field appears only when the CMK is a primary or replica of a multi-Region CMK.</p> <p>For more information about any listed CMK, use the <a>DescribeKey</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MultiRegionConfiguration {
    /// <p>Indicates whether the CMK is a <code>PRIMARY</code> or <code>REPLICA</code> key.</p>
    #[serde(rename = "multiRegionKeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_key_type: Option<String>,
    /// <p>Displays the key ARN and Region of the primary key. This field includes the current CMK if it is the primary key.</p>
    #[serde(rename = "primaryKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<MultiRegionKey>,
    /// <p>displays the key ARNs and Regions of all replica keys. This field includes the current CMK if it is a replica key.</p>
    #[serde(rename = "replicaKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_keys: Option<Vec<MultiRegionKey>>,
}

/// <p>Describes the primary or replica key in a multi-Region key.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MultiRegionKey {
    /// <p>Displays the key ARN of a primary or replica key of a multi-Region key.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Displays the AWS Region of a primary or replica key in a multi-Region key.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutKeyPolicyRequest {
    /// <p>A flag to indicate whether to bypass the key policy lockout safety check.</p> <important> <p>Setting this value to true increases the risk that the CMK becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section in the <i>AWS Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <code>PutKeyPolicy</code> request on the CMK.</p> <p>The default value is false.</p>
    #[serde(rename = "bypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// <p>Sets the key policy on the specified customer master key (CMK).</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>The key policy to attach to the CMK.</p> <p>The key policy must meet the following criteria:</p> <ul> <li> <p>If you don't set <code>BypassPolicyLockoutSafetyCheck</code> to true, the key policy must allow the principal that is making the <code>PutKeyPolicy</code> request to make a subsequent <code>PutKeyPolicy</code> request on the CMK. This reduces the risk that the CMK becomes unmanageable. For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section of the <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to AWS KMS. When you create a new AWS principal (for example, an IAM user or role), you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to AWS KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> </li> </ul> <p>The key policy cannot exceed 32 kilobytes (32768 bytes). For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/resource-limits.html">Resource Quotas</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "policy")]
    pub policy: String,
    /// <p>The name of the key policy. The only valid value is <code>default</code>.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReEncryptRequest {
    /// <p>Ciphertext of the data to reencrypt.</p>
    #[serde(rename = "ciphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub ciphertext_blob: bytes::Bytes,
    /// <p>Specifies the encryption algorithm that AWS KMS will use to reecrypt the data after it has decrypted it. The default value, <code>SYMMETRIC_DEFAULT</code>, represents the encryption algorithm used for symmetric CMKs.</p> <p>This parameter is required only when the destination CMK is an asymmetric CMK.</p>
    #[serde(rename = "destinationEncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_algorithm: Option<String>,
    /// <p>Specifies that encryption context to use when the reencrypting the data.</p> <p>A destination encryption context is valid only when the destination CMK is a symmetric CMK. The standard ciphertext format for asymmetric CMKs does not include fields for metadata.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "destinationEncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique identifier for the CMK that is used to reencrypt the data. Specify a symmetric or asymmetric CMK with a <code>KeyUsage</code> value of <code>ENCRYPT_DECRYPT</code>. To find the <code>KeyUsage</code> value of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "destinationKeyId")]
    pub destination_key_id: String,
    /// <p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Specifies the encryption algorithm that AWS KMS will use to decrypt the ciphertext before it is reencrypted. The default value, <code>SYMMETRIC_DEFAULT</code>, represents the algorithm used for symmetric CMKs.</p> <p>Specify the same algorithm that was used to encrypt the ciphertext. If you specify a different algorithm, the decrypt attempt fails.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric CMK.</p>
    #[serde(rename = "sourceEncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_algorithm: Option<String>,
    /// <p>Specifies the encryption context to use to decrypt the ciphertext. Enter the same encryption context that was used to encrypt the ciphertext.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "sourceEncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies the customer master key (CMK) that AWS KMS will use to decrypt the ciphertext before it is re-encrypted. Enter a key ID of the CMK that was used to encrypt the ciphertext.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric CMK. If you used a symmetric CMK, AWS KMS can get the CMK from metadata that it adds to the symmetric ciphertext blob. However, it is always recommended as a best practice. This practice ensures that you use the CMK that you intend.</p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "sourceKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReEncryptResponse {
    /// <p>The reencrypted data. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "ciphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The encryption algorithm that was used to reencrypt the data.</p>
    #[serde(rename = "destinationEncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_algorithm: Option<String>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the CMK that was used to reencrypt the data.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The encryption algorithm that was used to decrypt the ciphertext before it was reencrypted.</p>
    #[serde(rename = "sourceEncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_algorithm: Option<String>,
    /// <p>Unique identifier of the CMK used to originally encrypt the data.</p>
    #[serde(rename = "sourceKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReplicateKeyRequest {
    /// <p>A flag to indicate whether to bypass the key policy lockout safety check.</p> <important> <p>Setting this value to true increases the risk that the CMK becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section in the <i>AWS Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <code>PutKeyPolicy</code> request on the CMK.</p> <p>The default value is false.</p>
    #[serde(rename = "bypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// <p>A description of the CMK. Use a description that helps you decide whether the CMK is appropriate for a task. The default value is an empty string (no description).</p> <p>The description is not a shared property of multi-Region keys. You can specify the same description or a different description for each key in a set of related multi-Region keys. AWS KMS does not synchronize this property.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Identifies the multi-Region primary key that is being replicated. To determine whether a CMK is a multi-Region primary key, use the <a>DescribeKey</a> operation to check the value of the <code>MultiRegionKeyType</code> property.</p> <p>Specify the key ID or key ARN of a multi-Region primary key.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p><p>The key policy to attach to the CMK. This parameter is optional. If you do not provide a key policy, AWS KMS attaches the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default">default key policy</a> to the CMK.</p> <p>The key policy is not a shared property of multi-Region keys. You can specify the same key policy or a different key policy for each key in a set of related multi-Region keys. AWS KMS does not synchronize this property.</p> <p>If you provide a key policy, it must meet the following criteria:</p> <ul> <li> <p>If you don&#39;t set <code>BypassPolicyLockoutSafetyCheck</code> to true, the key policy must give the caller <code>kms:PutKeyPolicy</code> permission on the replica CMK. This reduces the risk that the CMK becomes unmanageable. For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section of the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </li> <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to AWS KMS. When you create a new AWS principal (for example, an IAM user or role), you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to AWS KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> </li> <li> <p>The key policy size quota is 32 kilobytes (32768 bytes).</p> </li> </ul></p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The Region ID of the AWS Region for this replica key. </p> <p>Enter the Region ID, such as <code>us-east-1</code> or <code>ap-southeast-2</code>. For a list of AWS Regions in which AWS KMS is supported, see <a href="https://docs.aws.amazon.com/general/latest/gr/kms.html#kms_region">AWS KMS service endpoints</a> in the <i>Amazon Web Services General Reference</i>.</p> <p>The replica must be in a different AWS Region than its primary key and other replicas of that primary key, but in the same AWS partition. AWS KMS must be available in the replica Region. If the Region is not enabled by default, the AWS account must be enabled in the Region. </p> <p>For information about AWS partitions, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) in the <i>Amazon Web Services General Reference</i>.</a> For information about enabling and disabling Regions, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-enable">Enabling a Region</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-disable">Disabling a Region</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "replicaRegion")]
    pub replica_region: String,
    /// <p>Assigns one or more tags to the replica key. Use this parameter to tag the CMK when it is created. To tag an existing CMK, use the <a>TagResource</a> operation.</p> <note> <p>Tagging or untagging a CMK can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>To use this parameter, you must have <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> permission in an IAM policy.</p> <p>Tags are not a shared property of multi-Region keys. You can specify the same tags or different tags for each key in a set of related multi-Region keys. AWS KMS does not synchronize this property.</p> <p>Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string. You cannot have more than one tag on a CMK with the same tag key. If you specify an existing tag key with a different tag value, AWS KMS replaces the current tag value with the specified one.</p> <p>When you assign tags to an AWS resource, AWS generates a cost allocation report with usage and costs aggregated by tags. Tags can also be used to control access to a CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging Keys</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicateKeyResponse {
    /// <p>Displays details about the new replica CMK, including its Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">key state</a>. It also includes the ARN and AWS Region of its primary key and other replica keys.</p>
    #[serde(rename = "replicaKeyMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_key_metadata: Option<KeyMetadata>,
    /// <p>The key policy of the new replica key. The value is a key policy document in JSON format.</p>
    #[serde(rename = "replicaPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_policy: Option<String>,
    /// <p>The tags on the new replica key. The value is a list of tag key and tag value pairs.</p>
    #[serde(rename = "replicaTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RetireGrantRequest {
    /// <p><p>Identifies the grant to retire. To get the grant ID, use <a>CreateGrant</a>, <a>ListGrants</a>, or <a>ListRetirableGrants</a>.</p> <ul> <li> <p>Grant ID Example - 0123456789012345678901234567890123456789012345678901234567890123</p> </li> </ul></p>
    #[serde(rename = "grantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// <p>Identifies the grant to be retired. You can use a grant token to identify a new grant even before it has achieved eventual consistency.</p> <p>Only the <a>CreateGrant</a> operation returns a grant token. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-eventual-consistency">Eventual consistency</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_token: Option<String>,
    /// <p>The key ARN CMK associated with the grant. To find the key ARN, use the <a>ListKeys</a> operation.</p> <p>For example: <code>arn:aws:kms:us-east-2:444455556666:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RevokeGrantRequest {
    /// <p>Identifies the grant to revoke. To get the grant ID, use <a>CreateGrant</a>, <a>ListGrants</a>, or <a>ListRetirableGrants</a>.</p>
    #[serde(rename = "grantId")]
    pub grant_id: String,
    /// <p>A unique identifier for the customer master key (CMK) associated with the grant. To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> <p>Specify the key ID or key ARN of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ScheduleKeyDeletionRequest {
    /// <p>The unique identifier of the customer master key (CMK) to delete.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the customer master key (CMK).</p> <p>If the CMK is a multi-Region primary key with replicas, the waiting period begins when the last of its replica keys is deleted. Otherwise, the waiting period begins immediately.</p> <p>This value is optional. If you include a value, it must be between 7 and 30, inclusive. If you do not include a value, it defaults to 30.</p>
    #[serde(rename = "pendingWindowInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_window_in_days: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScheduleKeyDeletionResponse {
    /// <p>The date and time after which AWS KMS deletes the customer master key (CMK).</p> <p>If the CMK is a multi-Region primary key with replica keys, this field does not appear. The deletion date for the primary key isn't known until its last replica key is deleted.</p>
    #[serde(rename = "deletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the CMK whose deletion is scheduled.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The current status of the CMK.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "keyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    /// <p>The waiting period before the CMK is deleted. </p> <p>If the CMK is a multi-Region primary key with replicas, the waiting period begins when the last of its replica keys is deleted. Otherwise, the waiting period begins immediately.</p>
    #[serde(rename = "pendingWindowInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_window_in_days: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SignRequest {
    /// <p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Identifies an asymmetric CMK. AWS KMS uses the private key in the asymmetric CMK to sign the message. The <code>KeyUsage</code> type of the CMK must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>Specifies the message or message digest to sign. Messages can be 0-4096 bytes. To sign a larger message, provide the message digest.</p> <p>If you provide a message, AWS KMS generates a hash digest of the message and then signs it.</p>
    #[serde(rename = "message")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub message: bytes::Bytes,
    /// <p>Tells AWS KMS whether the value of the <code>Message</code> parameter is a message or message digest. The default value, RAW, indicates a message. To indicate a message digest, enter <code>DIGEST</code>.</p>
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// <p>Specifies the signing algorithm to use when signing the message. </p> <p>Choose an algorithm that is compatible with the type and size of the specified asymmetric CMK.</p>
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SignResponse {
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the asymmetric CMK that was used to sign the message.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The cryptographic signature that was generated for the message. </p> <ul> <li> <p>When used with the supported RSA signing algorithms, the encoding of this value is defined by <a href="https://tools.ietf.org/html/rfc8017">PKCS #1 in RFC 8017</a>.</p> </li> <li> <p>When used with the <code>ECDSA_SHA_256</code>, <code>ECDSA_SHA_384</code>, or <code>ECDSA_SHA_512</code> signing algorithms, this value is a DER-encoded object as defined by ANS X9.62–2005 and <a href="https://tools.ietf.org/html/rfc3279#section-2.2.3">RFC 3279 Section 2.2.3</a>. This is the most commonly used signature format and is appropriate for most uses. </p> </li> </ul> <p>When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "signature")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<bytes::Bytes>,
    /// <p>The signing algorithm that was used to sign the message.</p>
    #[serde(rename = "signingAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<String>,
}

/// <p>A key-value pair. A tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "tagKey")]
    pub tag_key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "tagValue")]
    pub tag_value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>Identifies a customer managed CMK in the account and Region.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>One or more tags. </p> <p>Each tag consists of a tag key and a tag value. The tag value can be an empty (null) string. </p> <p>You cannot have more than one tag on a CMK with the same tag key. If you specify an existing tag key with a different tag value, AWS KMS replaces the current tag value with the specified one.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>Identifies the CMK from which you are removing tags.</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>One or more tag keys. Specify only the tag keys, not the tag values.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAliasRequest {
    /// <p>Identifies the alias that is changing its CMK. This value must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>. You cannot use UpdateAlias to change the alias name.</p>
    #[serde(rename = "aliasName")]
    pub alias_name: String,
    /// <p>Identifies the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a> to associate with the alias. You don't have permission to associate an alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMK</a>.</p> <p>The CMK must be in the same AWS account and Region as the alias. Also, the new target CMK must be the same type as the current target CMK (both symmetric or both asymmetric) and they must have the same key usage. </p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> <p>To verify that the alias is mapped to the correct CMK, use <a>ListAliases</a>.</p>
    #[serde(rename = "targetKeyId")]
    pub target_key_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCustomKeyStoreRequest {
    /// <p>Associates the custom key store with a related AWS CloudHSM cluster. </p> <p>Enter the cluster ID of the cluster that you used to create the custom key store or a cluster that shares a backup history and has the same cluster certificate as the original cluster. You cannot use this parameter to associate a custom key store with an unrelated cluster. In addition, the replacement cluster must <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">fulfill the requirements</a> for a cluster associated with a custom key store. To view the cluster certificate of a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    #[serde(rename = "cloudHsmClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    /// <p>Identifies the custom key store that you want to update. Enter the ID of the custom key store. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "customKeyStoreId")]
    pub custom_key_store_id: String,
    /// <p>Enter the current password of the <code>kmsuser</code> crypto user (CU) in the AWS CloudHSM cluster that is associated with the custom key store.</p> <p>This parameter tells AWS KMS the current password of the <code>kmsuser</code> crypto user (CU). It does not set or change the password of any users in the AWS CloudHSM cluster.</p>
    #[serde(rename = "keyStorePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_store_password: Option<String>,
    /// <p>Changes the friendly name of the custom key store to the value that you specify. The custom key store name must be unique in the AWS account.</p>
    #[serde(rename = "newCustomKeyStoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_custom_key_store_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCustomKeyStoreResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateKeyDescriptionRequest {
    /// <p>New description for the CMK.</p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p>Updates the description of the specified customer master key (CMK).</p> <p>Specify the key ID or key ARN of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePrimaryRegionRequest {
    /// <p>Identifies the current primary key. When the operation completes, this CMK will be a replica key.</p> <p>Specify the key ID or key ARN of a multi-Region primary key.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>The AWS Region of the new primary key. Enter the Region ID, such as <code>us-east-1</code> or <code>ap-southeast-2</code>. There must be an existing replica key in this Region. </p> <p>When the operation completes, the multi-Region key in this Region will be the primary key.</p>
    #[serde(rename = "primaryRegion")]
    pub primary_region: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VerifyRequest {
    /// <p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant token</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "grantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Identifies the asymmetric CMK that will be used to verify the signature. This must be the same CMK that was used to generate the signature. If you specify a different CMK, the signature verification fails.</p> <p>To specify a CMK, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>Specifies the message that was signed. You can submit a raw message of up to 4096 bytes, or a hash digest of the message. If you submit a digest, use the <code>MessageType</code> parameter with a value of <code>DIGEST</code>.</p> <p>If the message specified here is different from the message that was signed, the signature verification fails. A message and its hash digest are considered to be the same message.</p>
    #[serde(rename = "message")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub message: bytes::Bytes,
    /// <p><p>Tells AWS KMS whether the value of the <code>Message</code> parameter is a message or message digest. The default value, RAW, indicates a message. To indicate a message digest, enter <code>DIGEST</code>.</p> <important> <p>Use the <code>DIGEST</code> value only when the value of the <code>Message</code> parameter is a message digest. If you use the <code>DIGEST</code> value with a raw message, the security of the verification operation can be compromised.</p> </important></p>
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// <p>The signature that the <code>Sign</code> operation generated.</p>
    #[serde(rename = "signature")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub signature: bytes::Bytes,
    /// <p>The signing algorithm that was used to sign the message. If you submit a different algorithm, the signature verification fails.</p>
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VerifyResponse {
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the asymmetric CMK that was used to verify the signature.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>A Boolean value that indicates whether the signature was verified. A value of <code>True</code> indicates that the <code>Signature</code> was produced by signing the <code>Message</code> with the specified <code>KeyID</code> and <code>SigningAlgorithm.</code> If the signature is not verified, the <code>Verify</code> operation fails with a <code>KMSInvalidSignatureException</code> exception. </p>
    #[serde(rename = "signatureValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_valid: Option<bool>,
    /// <p>The signing algorithm that was used to verify the signature.</p>
    #[serde(rename = "signingAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<String>,
}

/// Errors returned by CancelKeyDeletion
#[derive(Debug, PartialEq)]
pub enum CancelKeyDeletionError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl CancelKeyDeletionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelKeyDeletionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CancelKeyDeletionError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CancelKeyDeletionError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CancelKeyDeletionError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(CancelKeyDeletionError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CancelKeyDeletionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelKeyDeletionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelKeyDeletionError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            CancelKeyDeletionError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CancelKeyDeletionError::KMSInternal(ref cause) => write!(f, "{}", cause),
            CancelKeyDeletionError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            CancelKeyDeletionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelKeyDeletionError {}
/// Errors returned by ConnectCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum ConnectCustomKeyStoreError {
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p>The request was rejected because the AWS CloudHSM cluster that is associated with the custom key store is not active. Initialize and activate the cluster and try the command again. For detailed instructions, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/getting-started.html">Getting Started</a> in the <i>AWS CloudHSM User Guide</i>.</p>
    CloudHsmClusterNotActive(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl ConnectCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConnectCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CloudHsmClusterNotActiveException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CloudHsmClusterNotActive(err.msg),
                    )
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CustomKeyStoreNotFound(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ConnectCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ConnectCustomKeyStoreError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            ConnectCustomKeyStoreError::CloudHsmClusterNotActive(ref cause) => {
                write!(f, "{}", cause)
            }
            ConnectCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            ConnectCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => write!(f, "{}", cause),
            ConnectCustomKeyStoreError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ConnectCustomKeyStoreError {}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    /// <p>The request was rejected because it attempted to create a resource that already exists.</p>
    AlreadyExists(String),
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified alias name is not valid.</p>
    InvalidAliasName(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a quota was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Quotas</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl CreateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateAliasError::AlreadyExists(err.msg))
                }
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CreateAliasError::DependencyTimeout(err.msg))
                }
                "InvalidAliasNameException" => {
                    return RusotoError::Service(CreateAliasError::InvalidAliasName(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateAliasError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(CreateAliasError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAliasError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateAliasError::NotFound(err.msg))
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
            CreateAliasError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateAliasError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            CreateAliasError::InvalidAliasName(ref cause) => write!(f, "{}", cause),
            CreateAliasError::KMSInternal(ref cause) => write!(f, "{}", cause),
            CreateAliasError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            CreateAliasError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateAliasError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAliasError {}
/// Errors returned by CreateCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum CreateCustomKeyStoreError {
    /// <p>The request was rejected because the specified AWS CloudHSM cluster is already associated with a custom key store or it shares a backup history with a cluster that is associated with a custom key store. Each custom key store must be associated with a different AWS CloudHSM cluster.</p> <p>Clusters that share a backup history have the same cluster certificate. To view the cluster certificate of a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    CloudHsmClusterInUse(String),
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p>The request was rejected because the AWS CloudHSM cluster that is associated with the custom key store is not active. Initialize and activate the cluster and try the command again. For detailed instructions, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/getting-started.html">Getting Started</a> in the <i>AWS CloudHSM User Guide</i>.</p>
    CloudHsmClusterNotActive(String),
    /// <p>The request was rejected because AWS KMS cannot find the AWS CloudHSM cluster with the specified cluster ID. Retry the request with a different cluster ID.</p>
    CloudHsmClusterNotFound(String),
    /// <p>The request was rejected because the specified custom key store name is already assigned to another custom key store in the account. Try again with a custom key store name that is unique in the account.</p>
    CustomKeyStoreNameInUse(String),
    /// <p>The request was rejected because the trust anchor certificate in the request is not the trust anchor certificate for the specified AWS CloudHSM cluster.</p> <p>When you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html#sign-csr">initialize the cluster</a>, you create the trust anchor certificate and save it in the <code>customerCA.crt</code> file.</p>
    IncorrectTrustAnchor(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl CreateCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInUseException" => {
                    return RusotoError::Service(CreateCustomKeyStoreError::CloudHsmClusterInUse(
                        err.msg,
                    ))
                }
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CloudHsmClusterNotActiveException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CloudHsmClusterNotActive(err.msg),
                    )
                }
                "CloudHsmClusterNotFoundException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CloudHsmClusterNotFound(err.msg),
                    )
                }
                "CustomKeyStoreNameInUseException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CustomKeyStoreNameInUse(err.msg),
                    )
                }
                "IncorrectTrustAnchorException" => {
                    return RusotoError::Service(CreateCustomKeyStoreError::IncorrectTrustAnchor(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCustomKeyStoreError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCustomKeyStoreError::CloudHsmClusterInUse(ref cause) => write!(f, "{}", cause),
            CreateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCustomKeyStoreError::CloudHsmClusterNotActive(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCustomKeyStoreError::CloudHsmClusterNotFound(ref cause) => write!(f, "{}", cause),
            CreateCustomKeyStoreError::CustomKeyStoreNameInUse(ref cause) => write!(f, "{}", cause),
            CreateCustomKeyStoreError::IncorrectTrustAnchor(ref cause) => write!(f, "{}", cause),
            CreateCustomKeyStoreError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCustomKeyStoreError {}
/// Errors returned by CreateGrant
#[derive(Debug, PartialEq)]
pub enum CreateGrantError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a quota was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Quotas</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl CreateGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CreateGrantError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(CreateGrantError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateGrantError::InvalidArn(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(CreateGrantError::InvalidGrantToken(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateGrantError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(CreateGrantError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGrantError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateGrantError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGrantError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGrantError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            CreateGrantError::Disabled(ref cause) => write!(f, "{}", cause),
            CreateGrantError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CreateGrantError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            CreateGrantError::KMSInternal(ref cause) => write!(f, "{}", cause),
            CreateGrantError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            CreateGrantError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGrantError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGrantError {}
/// Errors returned by CreateKey
#[derive(Debug, PartialEq)]
pub enum CreateKeyError {
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because a quota was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Quotas</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified policy is not syntactically or semantically correct.</p>
    MalformedPolicyDocument(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl CreateKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        CreateKeyError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(CreateKeyError::CustomKeyStoreInvalidState(
                        err.msg,
                    ))
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(CreateKeyError::CustomKeyStoreNotFound(err.msg))
                }
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CreateKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateKeyError::KMSInternal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateKeyError::LimitExceeded(err.msg))
                }
                "MalformedPolicyDocumentException" => {
                    return RusotoError::Service(CreateKeyError::MalformedPolicyDocument(err.msg))
                }
                "TagException" => return RusotoError::Service(CreateKeyError::Tag(err.msg)),
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateKeyError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateKeyError::CloudHsmClusterInvalidConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateKeyError::CustomKeyStoreInvalidState(ref cause) => write!(f, "{}", cause),
            CreateKeyError::CustomKeyStoreNotFound(ref cause) => write!(f, "{}", cause),
            CreateKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            CreateKeyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CreateKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            CreateKeyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateKeyError::MalformedPolicyDocument(ref cause) => write!(f, "{}", cause),
            CreateKeyError::Tag(ref cause) => write!(f, "{}", cause),
            CreateKeyError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateKeyError {}
/// Errors returned by Decrypt
#[derive(Debug, PartialEq)]
pub enum DecryptError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified CMK cannot decrypt the data. The <code>KeyId</code> in a <a>Decrypt</a> request and the <code>SourceKeyId</code> in a <a>ReEncrypt</a> request must identify the same CMK that was used to encrypt the ciphertext.</p>
    IncorrectKey(String),
    /// <p>From the <a>Decrypt</a> or <a>ReEncrypt</a> operation, the request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the encryption context, is corrupted, missing, or otherwise invalid.</p> <p>From the <a>ImportKeyMaterial</a> operation, the request was rejected because AWS KMS could not decrypt the encrypted (wrapped) key material. </p>
    InvalidCiphertext(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DecryptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DecryptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DecryptError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(DecryptError::Disabled(err.msg))
                }
                "IncorrectKeyException" => {
                    return RusotoError::Service(DecryptError::IncorrectKey(err.msg))
                }
                "InvalidCiphertextException" => {
                    return RusotoError::Service(DecryptError::InvalidCiphertext(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(DecryptError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(DecryptError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DecryptError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DecryptError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(DecryptError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DecryptError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DecryptError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecryptError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DecryptError::Disabled(ref cause) => write!(f, "{}", cause),
            DecryptError::IncorrectKey(ref cause) => write!(f, "{}", cause),
            DecryptError::InvalidCiphertext(ref cause) => write!(f, "{}", cause),
            DecryptError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            DecryptError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            DecryptError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DecryptError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            DecryptError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            DecryptError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DecryptError {}
/// Errors returned by DeleteAlias
#[derive(Debug, PartialEq)]
pub enum DeleteAliasError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DeleteAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DeleteAliasError::DependencyTimeout(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DeleteAliasError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DeleteAliasError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAliasError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAliasError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAliasError {}
/// Errors returned by DeleteCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum DeleteCustomKeyStoreError {
    /// <p>The request was rejected because the custom key store contains AWS KMS customer master keys (CMKs). After verifying that you do not need to use the CMKs, use the <a>ScheduleKeyDeletion</a> operation to delete the CMKs. After they are deleted, you can delete the custom key store.</p>
    CustomKeyStoreHasCMKs(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl DeleteCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreHasCMKsException" => {
                    return RusotoError::Service(DeleteCustomKeyStoreError::CustomKeyStoreHasCMKs(
                        err.msg,
                    ))
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        DeleteCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(DeleteCustomKeyStoreError::CustomKeyStoreNotFound(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DeleteCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteCustomKeyStoreError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCustomKeyStoreError::CustomKeyStoreHasCMKs(ref cause) => write!(f, "{}", cause),
            DeleteCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => write!(f, "{}", cause),
            DeleteCustomKeyStoreError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCustomKeyStoreError {}
/// Errors returned by DeleteImportedKeyMaterial
#[derive(Debug, PartialEq)]
pub enum DeleteImportedKeyMaterialError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl DeleteImportedKeyMaterialError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteImportedKeyMaterialError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::InvalidArn(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::KMSInternal(
                        err.msg,
                    ))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DeleteImportedKeyMaterialError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteImportedKeyMaterialError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteImportedKeyMaterialError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DeleteImportedKeyMaterialError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DeleteImportedKeyMaterialError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DeleteImportedKeyMaterialError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            DeleteImportedKeyMaterialError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteImportedKeyMaterialError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteImportedKeyMaterialError {}
/// Errors returned by DescribeCustomKeyStores
#[derive(Debug, PartialEq)]
pub enum DescribeCustomKeyStoresError {
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl DescribeCustomKeyStoresError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCustomKeyStoresError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(
                        DescribeCustomKeyStoresError::CustomKeyStoreNotFound(err.msg),
                    )
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(DescribeCustomKeyStoresError::InvalidMarker(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DescribeCustomKeyStoresError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCustomKeyStoresError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCustomKeyStoresError::CustomKeyStoreNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCustomKeyStoresError::InvalidMarker(ref cause) => write!(f, "{}", cause),
            DescribeCustomKeyStoresError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCustomKeyStoresError {}
/// Errors returned by DescribeKey
#[derive(Debug, PartialEq)]
pub enum DescribeKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DescribeKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DescribeKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DescribeKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DescribeKeyError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DescribeKeyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DescribeKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DescribeKeyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeKeyError {}
/// Errors returned by DisableKey
#[derive(Debug, PartialEq)]
pub enum DisableKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DisableKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DisableKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DisableKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DisableKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DisableKeyError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisableKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DisableKeyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DisableKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DisableKeyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            DisableKeyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableKeyError {}
/// Errors returned by DisableKeyRotation
#[derive(Debug, PartialEq)]
pub enum DisableKeyRotationError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl DisableKeyRotationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableKeyRotationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DisableKeyRotationError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "DisabledException" => {
                    return RusotoError::Service(DisableKeyRotationError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DisableKeyRotationError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DisableKeyRotationError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DisableKeyRotationError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisableKeyRotationError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DisableKeyRotationError::UnsupportedOperation(
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
impl fmt::Display for DisableKeyRotationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableKeyRotationError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::Disabled(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::NotFound(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableKeyRotationError {}
/// Errors returned by DisconnectCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum DisconnectCustomKeyStoreError {
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl DisconnectCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisconnectCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        DisconnectCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(
                        DisconnectCustomKeyStoreError::CustomKeyStoreNotFound(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DisconnectCustomKeyStoreError::KMSInternal(
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
impl fmt::Display for DisconnectCustomKeyStoreError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisconnectCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            DisconnectCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisconnectCustomKeyStoreError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisconnectCustomKeyStoreError {}
/// Errors returned by EnableKey
#[derive(Debug, PartialEq)]
pub enum EnableKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a quota was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Quotas</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl EnableKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(EnableKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(EnableKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(EnableKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(EnableKeyError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(EnableKeyError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(EnableKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            EnableKeyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            EnableKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            EnableKeyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            EnableKeyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            EnableKeyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableKeyError {}
/// Errors returned by EnableKeyRotation
#[derive(Debug, PartialEq)]
pub enum EnableKeyRotationError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl EnableKeyRotationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableKeyRotationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(EnableKeyRotationError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(EnableKeyRotationError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(EnableKeyRotationError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(EnableKeyRotationError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(EnableKeyRotationError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(EnableKeyRotationError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(EnableKeyRotationError::UnsupportedOperation(
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
impl fmt::Display for EnableKeyRotationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableKeyRotationError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::Disabled(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::InvalidArn(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::KMSInternal(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::NotFound(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableKeyRotationError {}
/// Errors returned by Encrypt
#[derive(Debug, PartialEq)]
pub enum EncryptError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl EncryptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EncryptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(EncryptError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(EncryptError::Disabled(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(EncryptError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(EncryptError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(EncryptError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(EncryptError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(EncryptError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(EncryptError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EncryptError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncryptError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            EncryptError::Disabled(ref cause) => write!(f, "{}", cause),
            EncryptError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            EncryptError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            EncryptError::KMSInternal(ref cause) => write!(f, "{}", cause),
            EncryptError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            EncryptError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            EncryptError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EncryptError {}
/// Errors returned by GenerateDataKey
#[derive(Debug, PartialEq)]
pub enum GenerateDataKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl GenerateDataKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateDataKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GenerateDataKeyError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(GenerateDataKeyError::Disabled(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(GenerateDataKeyError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(GenerateDataKeyError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GenerateDataKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GenerateDataKeyError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(GenerateDataKeyError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GenerateDataKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GenerateDataKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateDataKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::Disabled(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateDataKeyError {}
/// Errors returned by GenerateDataKeyPair
#[derive(Debug, PartialEq)]
pub enum GenerateDataKeyPairError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl GenerateDataKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateDataKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "DisabledException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::Disabled(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::InvalidGrantToken(
                        err.msg,
                    ))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::UnsupportedOperation(
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
impl fmt::Display for GenerateDataKeyPairError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateDataKeyPairError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::Disabled(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::NotFound(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateDataKeyPairError {}
/// Errors returned by GenerateDataKeyPairWithoutPlaintext
#[derive(Debug, PartialEq)]
pub enum GenerateDataKeyPairWithoutPlaintextError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl GenerateDataKeyPairWithoutPlaintextError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GenerateDataKeyPairWithoutPlaintextError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::DependencyTimeout(err.msg),
                    )
                }
                "DisabledException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::Disabled(err.msg),
                    )
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::InvalidGrantToken(err.msg),
                    )
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::InvalidKeyUsage(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::KMSInternal(err.msg),
                    )
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::KMSInvalidState(err.msg),
                    )
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::KeyUnavailable(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::NotFound(err.msg),
                    )
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GenerateDataKeyPairWithoutPlaintextError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateDataKeyPairWithoutPlaintextError::DependencyTimeout(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::Disabled(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairWithoutPlaintextError::InvalidGrantToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::InvalidKeyUsage(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::KMSInternal(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::KMSInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::KeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::NotFound(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairWithoutPlaintextError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GenerateDataKeyPairWithoutPlaintextError {}
/// Errors returned by GenerateDataKeyWithoutPlaintext
#[derive(Debug, PartialEq)]
pub enum GenerateDataKeyWithoutPlaintextError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl GenerateDataKeyWithoutPlaintextError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GenerateDataKeyWithoutPlaintextError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::DependencyTimeout(err.msg),
                    )
                }
                "DisabledException" => {
                    return RusotoError::Service(GenerateDataKeyWithoutPlaintextError::Disabled(
                        err.msg,
                    ))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::InvalidGrantToken(err.msg),
                    )
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::InvalidKeyUsage(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GenerateDataKeyWithoutPlaintextError::KMSInternal(
                        err.msg,
                    ))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::KMSInvalidState(err.msg),
                    )
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::KeyUnavailable(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GenerateDataKeyWithoutPlaintextError::NotFound(
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
impl fmt::Display for GenerateDataKeyWithoutPlaintextError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateDataKeyWithoutPlaintextError::DependencyTimeout(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyWithoutPlaintextError::Disabled(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyWithoutPlaintextError::InvalidGrantToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyWithoutPlaintextError::InvalidKeyUsage(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyWithoutPlaintextError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyWithoutPlaintextError::KMSInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyWithoutPlaintextError::KeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyWithoutPlaintextError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateDataKeyWithoutPlaintextError {}
/// Errors returned by GenerateRandom
#[derive(Debug, PartialEq)]
pub enum GenerateRandomError {
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl GenerateRandomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateRandomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(GenerateRandomError::CustomKeyStoreInvalidState(
                        err.msg,
                    ))
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(GenerateRandomError::CustomKeyStoreNotFound(
                        err.msg,
                    ))
                }
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GenerateRandomError::DependencyTimeout(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GenerateRandomError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GenerateRandomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateRandomError::CustomKeyStoreInvalidState(ref cause) => write!(f, "{}", cause),
            GenerateRandomError::CustomKeyStoreNotFound(ref cause) => write!(f, "{}", cause),
            GenerateRandomError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GenerateRandomError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateRandomError {}
/// Errors returned by GetKeyPolicy
#[derive(Debug, PartialEq)]
pub enum GetKeyPolicyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl GetKeyPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GetKeyPolicyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetKeyPolicyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GetKeyPolicyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetKeyPolicyError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetKeyPolicyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetKeyPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetKeyPolicyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GetKeyPolicyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetKeyPolicyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GetKeyPolicyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GetKeyPolicyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetKeyPolicyError {}
/// Errors returned by GetKeyRotationStatus
#[derive(Debug, PartialEq)]
pub enum GetKeyRotationStatusError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl GetKeyRotationStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyRotationStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::UnsupportedOperation(
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
impl fmt::Display for GetKeyRotationStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetKeyRotationStatusError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GetKeyRotationStatusError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetKeyRotationStatusError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GetKeyRotationStatusError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GetKeyRotationStatusError::NotFound(ref cause) => write!(f, "{}", cause),
            GetKeyRotationStatusError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetKeyRotationStatusError {}
/// Errors returned by GetParametersForImport
#[derive(Debug, PartialEq)]
pub enum GetParametersForImportError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl GetParametersForImportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetParametersForImportError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GetParametersForImportError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetParametersForImportError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GetParametersForImportError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetParametersForImportError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetParametersForImportError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetParametersForImportError::UnsupportedOperation(
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
impl fmt::Display for GetParametersForImportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetParametersForImportError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GetParametersForImportError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetParametersForImportError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GetParametersForImportError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GetParametersForImportError::NotFound(ref cause) => write!(f, "{}", cause),
            GetParametersForImportError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetParametersForImportError {}
/// Errors returned by GetPublicKey
#[derive(Debug, PartialEq)]
pub enum GetPublicKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl GetPublicKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPublicKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GetPublicKeyError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(GetPublicKeyError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetPublicKeyError::InvalidArn(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(GetPublicKeyError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(GetPublicKeyError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GetPublicKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetPublicKeyError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(GetPublicKeyError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetPublicKeyError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetPublicKeyError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPublicKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPublicKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::Disabled(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::NotFound(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPublicKeyError {}
/// Errors returned by ImportKeyMaterial
#[derive(Debug, PartialEq)]
pub enum ImportKeyMaterialError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified import token is expired. Use <a>GetParametersForImport</a> to get a new import token and public key, use the new public key to encrypt the key material, and then try the request again.</p>
    ExpiredImportToken(String),
    /// <p>The request was rejected because the key material in the request is, expired, invalid, or is not the same key material that was previously imported into this customer master key (CMK).</p>
    IncorrectKeyMaterial(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>From the <a>Decrypt</a> or <a>ReEncrypt</a> operation, the request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the encryption context, is corrupted, missing, or otherwise invalid.</p> <p>From the <a>ImportKeyMaterial</a> operation, the request was rejected because AWS KMS could not decrypt the encrypted (wrapped) key material. </p>
    InvalidCiphertext(String),
    /// <p>The request was rejected because the provided import token is invalid or is associated with a different customer master key (CMK).</p>
    InvalidImportToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl ImportKeyMaterialError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportKeyMaterialError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ImportKeyMaterialError::DependencyTimeout(err.msg))
                }
                "ExpiredImportTokenException" => {
                    return RusotoError::Service(ImportKeyMaterialError::ExpiredImportToken(
                        err.msg,
                    ))
                }
                "IncorrectKeyMaterialException" => {
                    return RusotoError::Service(ImportKeyMaterialError::IncorrectKeyMaterial(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ImportKeyMaterialError::InvalidArn(err.msg))
                }
                "InvalidCiphertextException" => {
                    return RusotoError::Service(ImportKeyMaterialError::InvalidCiphertext(err.msg))
                }
                "InvalidImportTokenException" => {
                    return RusotoError::Service(ImportKeyMaterialError::InvalidImportToken(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ImportKeyMaterialError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ImportKeyMaterialError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ImportKeyMaterialError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ImportKeyMaterialError::UnsupportedOperation(
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
impl fmt::Display for ImportKeyMaterialError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportKeyMaterialError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::ExpiredImportToken(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::IncorrectKeyMaterial(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::InvalidCiphertext(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::InvalidImportToken(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::NotFound(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportKeyMaterialError {}
/// Errors returned by ListAliases
#[derive(Debug, PartialEq)]
pub enum ListAliasesError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAliasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListAliasesError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListAliasesError::InvalidArn(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListAliasesError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListAliasesError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListAliasesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAliasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAliasesError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ListAliasesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListAliasesError::InvalidMarker(ref cause) => write!(f, "{}", cause),
            ListAliasesError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ListAliasesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAliasesError {}
/// Errors returned by ListGrants
#[derive(Debug, PartialEq)]
pub enum ListGrantsError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified <code>GrantId</code> is not valid.</p>
    InvalidGrantId(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListGrantsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGrantsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListGrantsError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListGrantsError::InvalidArn(err.msg))
                }
                "InvalidGrantIdException" => {
                    return RusotoError::Service(ListGrantsError::InvalidGrantId(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListGrantsError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListGrantsError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ListGrantsError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListGrantsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGrantsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGrantsError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ListGrantsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListGrantsError::InvalidGrantId(ref cause) => write!(f, "{}", cause),
            ListGrantsError::InvalidMarker(ref cause) => write!(f, "{}", cause),
            ListGrantsError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ListGrantsError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            ListGrantsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGrantsError {}
/// Errors returned by ListKeyPolicies
#[derive(Debug, PartialEq)]
pub enum ListKeyPoliciesError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListKeyPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListKeyPoliciesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListKeyPoliciesError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListKeyPoliciesError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListKeyPoliciesError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ListKeyPoliciesError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListKeyPoliciesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListKeyPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListKeyPoliciesError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ListKeyPoliciesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListKeyPoliciesError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ListKeyPoliciesError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            ListKeyPoliciesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListKeyPoliciesError {}
/// Errors returned by ListKeys
#[derive(Debug, PartialEq)]
pub enum ListKeysError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl ListKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListKeysError::DependencyTimeout(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListKeysError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListKeysError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListKeysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListKeysError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ListKeysError::InvalidMarker(ref cause) => write!(f, "{}", cause),
            ListKeysError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListKeysError {}
/// Errors returned by ListResourceTags
#[derive(Debug, PartialEq)]
pub enum ListResourceTagsError {
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListResourceTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourceTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(ListResourceTagsError::InvalidArn(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListResourceTagsError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListResourceTagsError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListResourceTagsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResourceTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourceTagsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListResourceTagsError::InvalidMarker(ref cause) => write!(f, "{}", cause),
            ListResourceTagsError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ListResourceTagsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourceTagsError {}
/// Errors returned by ListRetirableGrants
#[derive(Debug, PartialEq)]
pub enum ListRetirableGrantsError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListRetirableGrantsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRetirableGrantsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListRetirableGrantsError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListRetirableGrantsError::InvalidArn(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListRetirableGrantsError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListRetirableGrantsError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListRetirableGrantsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRetirableGrantsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRetirableGrantsError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ListRetirableGrantsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListRetirableGrantsError::InvalidMarker(ref cause) => write!(f, "{}", cause),
            ListRetirableGrantsError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ListRetirableGrantsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRetirableGrantsError {}
/// Errors returned by PutKeyPolicy
#[derive(Debug, PartialEq)]
pub enum PutKeyPolicyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a quota was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Quotas</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified policy is not syntactically or semantically correct.</p>
    MalformedPolicyDocument(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl PutKeyPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutKeyPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(PutKeyPolicyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(PutKeyPolicyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(PutKeyPolicyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(PutKeyPolicyError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutKeyPolicyError::LimitExceeded(err.msg))
                }
                "MalformedPolicyDocumentException" => {
                    return RusotoError::Service(PutKeyPolicyError::MalformedPolicyDocument(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutKeyPolicyError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(PutKeyPolicyError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutKeyPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutKeyPolicyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::MalformedPolicyDocument(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutKeyPolicyError {}
/// Errors returned by ReEncrypt
#[derive(Debug, PartialEq)]
pub enum ReEncryptError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified CMK cannot decrypt the data. The <code>KeyId</code> in a <a>Decrypt</a> request and the <code>SourceKeyId</code> in a <a>ReEncrypt</a> request must identify the same CMK that was used to encrypt the ciphertext.</p>
    IncorrectKey(String),
    /// <p>From the <a>Decrypt</a> or <a>ReEncrypt</a> operation, the request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the encryption context, is corrupted, missing, or otherwise invalid.</p> <p>From the <a>ImportKeyMaterial</a> operation, the request was rejected because AWS KMS could not decrypt the encrypted (wrapped) key material. </p>
    InvalidCiphertext(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ReEncryptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReEncryptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ReEncryptError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(ReEncryptError::Disabled(err.msg))
                }
                "IncorrectKeyException" => {
                    return RusotoError::Service(ReEncryptError::IncorrectKey(err.msg))
                }
                "InvalidCiphertextException" => {
                    return RusotoError::Service(ReEncryptError::InvalidCiphertext(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(ReEncryptError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(ReEncryptError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ReEncryptError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ReEncryptError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(ReEncryptError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ReEncryptError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ReEncryptError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReEncryptError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ReEncryptError::Disabled(ref cause) => write!(f, "{}", cause),
            ReEncryptError::IncorrectKey(ref cause) => write!(f, "{}", cause),
            ReEncryptError::InvalidCiphertext(ref cause) => write!(f, "{}", cause),
            ReEncryptError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            ReEncryptError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            ReEncryptError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ReEncryptError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            ReEncryptError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            ReEncryptError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ReEncryptError {}
/// Errors returned by ReplicateKey
#[derive(Debug, PartialEq)]
pub enum ReplicateKeyError {
    /// <p>The request was rejected because it attempted to create a resource that already exists.</p>
    AlreadyExists(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a quota was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Quotas</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified policy is not syntactically or semantically correct.</p>
    MalformedPolicyDocument(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl ReplicateKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReplicateKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(ReplicateKeyError::AlreadyExists(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(ReplicateKeyError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ReplicateKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ReplicateKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ReplicateKeyError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ReplicateKeyError::LimitExceeded(err.msg))
                }
                "MalformedPolicyDocumentException" => {
                    return RusotoError::Service(ReplicateKeyError::MalformedPolicyDocument(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ReplicateKeyError::NotFound(err.msg))
                }
                "TagException" => return RusotoError::Service(ReplicateKeyError::Tag(err.msg)),
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ReplicateKeyError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ReplicateKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReplicateKeyError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            ReplicateKeyError::Disabled(ref cause) => write!(f, "{}", cause),
            ReplicateKeyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ReplicateKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ReplicateKeyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            ReplicateKeyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ReplicateKeyError::MalformedPolicyDocument(ref cause) => write!(f, "{}", cause),
            ReplicateKeyError::NotFound(ref cause) => write!(f, "{}", cause),
            ReplicateKeyError::Tag(ref cause) => write!(f, "{}", cause),
            ReplicateKeyError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ReplicateKeyError {}
/// Errors returned by RetireGrant
#[derive(Debug, PartialEq)]
pub enum RetireGrantError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified <code>GrantId</code> is not valid.</p>
    InvalidGrantId(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl RetireGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RetireGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(RetireGrantError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(RetireGrantError::InvalidArn(err.msg))
                }
                "InvalidGrantIdException" => {
                    return RusotoError::Service(RetireGrantError::InvalidGrantId(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(RetireGrantError::InvalidGrantToken(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(RetireGrantError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(RetireGrantError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RetireGrantError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RetireGrantError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RetireGrantError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            RetireGrantError::InvalidArn(ref cause) => write!(f, "{}", cause),
            RetireGrantError::InvalidGrantId(ref cause) => write!(f, "{}", cause),
            RetireGrantError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            RetireGrantError::KMSInternal(ref cause) => write!(f, "{}", cause),
            RetireGrantError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            RetireGrantError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RetireGrantError {}
/// Errors returned by RevokeGrant
#[derive(Debug, PartialEq)]
pub enum RevokeGrantError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified <code>GrantId</code> is not valid.</p>
    InvalidGrantId(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl RevokeGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RevokeGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(RevokeGrantError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(RevokeGrantError::InvalidArn(err.msg))
                }
                "InvalidGrantIdException" => {
                    return RusotoError::Service(RevokeGrantError::InvalidGrantId(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(RevokeGrantError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(RevokeGrantError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RevokeGrantError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RevokeGrantError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RevokeGrantError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            RevokeGrantError::InvalidArn(ref cause) => write!(f, "{}", cause),
            RevokeGrantError::InvalidGrantId(ref cause) => write!(f, "{}", cause),
            RevokeGrantError::KMSInternal(ref cause) => write!(f, "{}", cause),
            RevokeGrantError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            RevokeGrantError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RevokeGrantError {}
/// Errors returned by ScheduleKeyDeletion
#[derive(Debug, PartialEq)]
pub enum ScheduleKeyDeletionError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ScheduleKeyDeletionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ScheduleKeyDeletionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ScheduleKeyDeletionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScheduleKeyDeletionError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ScheduleKeyDeletionError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ScheduleKeyDeletionError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ScheduleKeyDeletionError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            ScheduleKeyDeletionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ScheduleKeyDeletionError {}
/// Errors returned by Sign
#[derive(Debug, PartialEq)]
pub enum SignError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl SignError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SignError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(SignError::DependencyTimeout(err.msg))
                }
                "DisabledException" => return RusotoError::Service(SignError::Disabled(err.msg)),
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(SignError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(SignError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(SignError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(SignError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(SignError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(SignError::NotFound(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SignError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SignError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            SignError::Disabled(ref cause) => write!(f, "{}", cause),
            SignError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            SignError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            SignError::KMSInternal(ref cause) => write!(f, "{}", cause),
            SignError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            SignError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            SignError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SignError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a quota was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Quotas</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(TagResourceError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(TagResourceError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(TagResourceError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "TagException" => return RusotoError::Service(TagResourceError::Tag(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            TagResourceError::KMSInternal(ref cause) => write!(f, "{}", cause),
            TagResourceError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            TagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Tag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(UntagResourceError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UntagResourceError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(UntagResourceError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "TagException" => return RusotoError::Service(UntagResourceError::Tag(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UntagResourceError::KMSInternal(ref cause) => write!(f, "{}", cause),
            UntagResourceError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Tag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateAlias
#[derive(Debug, PartialEq)]
pub enum UpdateAliasError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a quota was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Quotas</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl UpdateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(UpdateAliasError::DependencyTimeout(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UpdateAliasError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(UpdateAliasError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateAliasError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAliasError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAliasError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::KMSInternal(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAliasError {}
/// Errors returned by UpdateCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum UpdateCustomKeyStoreError {
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p>The request was rejected because the AWS CloudHSM cluster that is associated with the custom key store is not active. Initialize and activate the cluster and try the command again. For detailed instructions, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/getting-started.html">Getting Started</a> in the <i>AWS CloudHSM User Guide</i>.</p>
    CloudHsmClusterNotActive(String),
    /// <p>The request was rejected because AWS KMS cannot find the AWS CloudHSM cluster with the specified cluster ID. Retry the request with a different cluster ID.</p>
    CloudHsmClusterNotFound(String),
    /// <p>The request was rejected because the specified AWS CloudHSM cluster has a different cluster certificate than the original cluster. You cannot use the operation to specify an unrelated cluster.</p> <p>Specify a cluster that shares a backup history with the original cluster. This includes clusters that were created from a backup of the current cluster, and clusters that were created from the same backup that produced the current cluster.</p> <p>Clusters that share a backup history have the same cluster certificate. To view the cluster certificate of a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    CloudHsmClusterNotRelated(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because the specified custom key store name is already assigned to another custom key store in the account. Try again with a custom key store name that is unique in the account.</p>
    CustomKeyStoreNameInUse(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl UpdateCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CloudHsmClusterNotActiveException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterNotActive(err.msg),
                    )
                }
                "CloudHsmClusterNotFoundException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterNotFound(err.msg),
                    )
                }
                "CloudHsmClusterNotRelatedException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterNotRelated(err.msg),
                    )
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNameInUseException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CustomKeyStoreNameInUse(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(UpdateCustomKeyStoreError::CustomKeyStoreNotFound(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UpdateCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateCustomKeyStoreError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCustomKeyStoreError::CloudHsmClusterNotActive(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCustomKeyStoreError::CloudHsmClusterNotFound(ref cause) => write!(f, "{}", cause),
            UpdateCustomKeyStoreError::CloudHsmClusterNotRelated(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCustomKeyStoreError::CustomKeyStoreNameInUse(ref cause) => write!(f, "{}", cause),
            UpdateCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => write!(f, "{}", cause),
            UpdateCustomKeyStoreError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateCustomKeyStoreError {}
/// Errors returned by UpdateKeyDescription
#[derive(Debug, PartialEq)]
pub enum UpdateKeyDescriptionError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl UpdateKeyDescriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateKeyDescriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateKeyDescriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateKeyDescriptionError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            UpdateKeyDescriptionError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpdateKeyDescriptionError::KMSInternal(ref cause) => write!(f, "{}", cause),
            UpdateKeyDescriptionError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            UpdateKeyDescriptionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateKeyDescriptionError {}
/// Errors returned by UpdatePrimaryRegion
#[derive(Debug, PartialEq)]
pub enum UpdatePrimaryRegionError {
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl UpdatePrimaryRegionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePrimaryRegionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DisabledException" => {
                    return RusotoError::Service(UpdatePrimaryRegionError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpdatePrimaryRegionError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UpdatePrimaryRegionError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(UpdatePrimaryRegionError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdatePrimaryRegionError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(UpdatePrimaryRegionError::UnsupportedOperation(
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
impl fmt::Display for UpdatePrimaryRegionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePrimaryRegionError::Disabled(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryRegionError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryRegionError::KMSInternal(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryRegionError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryRegionError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryRegionError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePrimaryRegionError {}
/// Errors returned by Verify
#[derive(Debug, PartialEq)]
pub enum VerifyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the signature verification failed. Signature verification fails when it cannot confirm that signature was produced by signing the specified message with the specified CMK and signing algorithm.</p>
    KMSInvalidSignature(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl VerifyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<VerifyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(VerifyError::DependencyTimeout(err.msg))
                }
                "DisabledException" => return RusotoError::Service(VerifyError::Disabled(err.msg)),
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(VerifyError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(VerifyError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(VerifyError::KMSInternal(err.msg))
                }
                "KMSInvalidSignatureException" => {
                    return RusotoError::Service(VerifyError::KMSInvalidSignature(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(VerifyError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(VerifyError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(VerifyError::NotFound(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for VerifyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VerifyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            VerifyError::Disabled(ref cause) => write!(f, "{}", cause),
            VerifyError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            VerifyError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            VerifyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            VerifyError::KMSInvalidSignature(ref cause) => write!(f, "{}", cause),
            VerifyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            VerifyError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            VerifyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for VerifyError {}
/// Trait representing the capabilities of the KMS API. KMS clients implement this trait.
#[async_trait]
pub trait Kms {
    /// <p>Cancels the deletion of a customer master key (CMK). When this operation succeeds, the key state of the CMK is <code>Disabled</code>. To enable the CMK, use <a>EnableKey</a>. </p> <p>For more information about scheduling and canceling deletion of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CancelKeyDeletion</a> (key policy)</p> <p> <b>Related operations</b>: <a>ScheduleKeyDeletion</a> </p>
    async fn cancel_key_deletion(
        &self,
        input: CancelKeyDeletionRequest,
    ) -> Result<CancelKeyDeletionResponse, RusotoError<CancelKeyDeletionError>>;

    /// <p><p>Connects or reconnects a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> to its associated AWS CloudHSM cluster.</p> <p>The custom key store must be connected before you can create customer master keys (CMKs) in the key store or use the CMKs it contains. You can disconnect and reconnect a custom key store at any time.</p> <p>To connect a custom key store, its associated AWS CloudHSM cluster must have at least one active HSM. To get the number of active HSMs in a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation. To add HSMs to the cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation. Also, the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user</a> (CU) must not be logged into the cluster. This prevents AWS KMS from using this account to log in.</p> <p>The connection process can take an extended amount of time to complete; up to 20 minutes. This operation starts the connection process, but it does not wait for it to complete. When it succeeds, this operation quickly returns an HTTP 200 response and a JSON object with no properties. However, this response does not indicate that the custom key store is connected. To get the connection state of the custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>During the connection process, AWS KMS finds the AWS CloudHSM cluster that is associated with the custom key store, creates the connection infrastructure, connects to the cluster, logs into the AWS CloudHSM client as the <code>kmsuser</code> CU, and rotates its password.</p> <p>The <code>ConnectCustomKeyStore</code> operation might fail for various reasons. To find the reason, use the <a>DescribeCustomKeyStores</a> operation and see the <code>ConnectionErrorCode</code> in the response. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>To fix the failure, use the <a>DisconnectCustomKeyStore</a> operation to disconnect the custom key store, correct the error, use the <a>UpdateCustomKeyStore</a> operation if necessary, and then use <code>ConnectCustomKeyStore</code> again.</p> <p>If you are having trouble connecting or disconnecting a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ConnectCustomKeyStore</a> (IAM policy)</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateCustomKeyStore</a> </p> </li> <li> <p> <a>DeleteCustomKeyStore</a> </p> </li> <li> <p> <a>DescribeCustomKeyStores</a> </p> </li> <li> <p> <a>DisconnectCustomKeyStore</a> </p> </li> <li> <p> <a>UpdateCustomKeyStore</a> </p> </li> </ul></p>
    async fn connect_custom_key_store(
        &self,
        input: ConnectCustomKeyStoreRequest,
    ) -> Result<ConnectCustomKeyStoreResponse, RusotoError<ConnectCustomKeyStoreError>>;

    /// <p><p>Creates a friendly name for a customer master key (CMK). </p> <note> <p>Adding, deleting, or updating an alias can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>You can use an alias to identify a CMK in the AWS KMS console, in the <a>DescribeKey</a> operation and in <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a>, such as <a>Encrypt</a> and <a>GenerateDataKey</a>. You can also change the CMK that&#39;s associated with the alias (<a>UpdateAlias</a>) or delete the alias (<a>DeleteAlias</a>) at any time. These operations don&#39;t affect the underlying CMK. </p> <p>You can associate the alias with any customer managed CMK in the same AWS Region. Each alias is associated with only one CMK at a time, but a CMK can have multiple aliases. A valid CMK is required. You can&#39;t create an alias without a CMK.</p> <p>The alias must be unique in the account and Region, but you can have aliases with the same name in different Regions. For detailed information about aliases, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html">Using aliases</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>This operation does not return a response. To get the alias that you created, use the <a>ListAliases</a> operation.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on an alias in a different AWS account.</p> <p> <b>Required permissions</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CreateAlias</a> on the alias (IAM policy).</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CreateAlias</a> on the CMK (key policy).</p> </li> </ul> <p>For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html#alias-access">Controlling access to aliases</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> </ul></p>
    async fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> Result<(), RusotoError<CreateAliasError>>;

    /// <p><p>Creates a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> that is associated with an <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/clusters.html">AWS CloudHSM cluster</a> that you own and manage.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>Before you create the custom key store, you must assemble the required elements, including an AWS CloudHSM cluster that fulfills the requirements for a custom key store. For details about the required elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>When the operation completes successfully, it returns the ID of the new custom key store. Before you can use your new custom key store, you need to use the <a>ConnectCustomKeyStore</a> operation to connect the new key store to its AWS CloudHSM cluster. Even if you are not going to use your custom key store immediately, you might want to connect it to verify that all settings are correct and then disconnect it until you are ready to use it.</p> <p>For help with failures, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CreateCustomKeyStore</a> (IAM policy).</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ConnectCustomKeyStore</a> </p> </li> <li> <p> <a>DeleteCustomKeyStore</a> </p> </li> <li> <p> <a>DescribeCustomKeyStores</a> </p> </li> <li> <p> <a>DisconnectCustomKeyStore</a> </p> </li> <li> <p> <a>UpdateCustomKeyStore</a> </p> </li> </ul></p>
    async fn create_custom_key_store(
        &self,
        input: CreateCustomKeyStoreRequest,
    ) -> Result<CreateCustomKeyStoreResponse, RusotoError<CreateCustomKeyStoreError>>;

    /// <p><p>Adds a grant to a customer master key (CMK). </p> <p>A <i>grant</i> is a policy instrument that allows AWS principals to use AWS KMS customer master keys (CMKs) in cryptographic operations. It also can allow them to view a CMK (<a>DescribeKey</a>) and create and manage grants. When authorizing access to a CMK, grants are considered along with key policies and IAM policies. Grants are often used for temporary permissions because you can create one, use its permissions, and delete it without changing your key policies or IAM policies. </p> <p>For detailed information about grants, including grant terminology, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Using grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>. For examples of working with grants in several programming languages, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/programming-grants.html">Programming grants</a>.</p> <p>The <code>CreateGrant</code> operation returns a <code>GrantToken</code> and a <code>GrantId</code>.</p> <ul> <li> <p>When you create, retire, or revoke a grant, there might be a brief delay, usually less than five minutes, until the grant is available throughout AWS KMS. This state is known as <i>eventual consistency</i>. Once the grant has achieved eventual consistency, the grantee principal can use the permissions in the grant without identifying the grant. </p> <p>However, to use the permissions in the grant immediately, use the <code>GrantToken</code> that <code>CreateGrant</code> returns. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </li> <li> <p>The <code>CreateGrant</code> operation also returns a <code>GrantId</code>. You can use the <code>GrantId</code> and a key identifier to identify the grant in the <a>RetireGrant</a> and <a>RevokeGrant</a> operations. To find the grant ID, use the <a>ListGrants</a> or <a>ListRetirableGrants</a> operations.</p> </li> </ul> <p>For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>. For more information about grants, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter. </p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CreateGrant</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ListGrants</a> </p> </li> <li> <p> <a>ListRetirableGrants</a> </p> </li> <li> <p> <a>RetireGrant</a> </p> </li> <li> <p> <a>RevokeGrant</a> </p> </li> </ul></p>
    async fn create_grant(
        &self,
        input: CreateGrantRequest,
    ) -> Result<CreateGrantResponse, RusotoError<CreateGrantError>>;

    /// <p><p>Creates a unique customer managed <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master-keys">customer master key</a> (CMK) in your AWS account and Region.</p> <p>You can use the <code>CreateKey</code> operation to create symmetric or asymmetric CMKs.</p> <ul> <li> <p> <b>Symmetric CMKs</b> contain a 256-bit symmetric key that never leaves AWS KMS unencrypted. To use the CMK, you must call AWS KMS. You can use a symmetric CMK to encrypt and decrypt small amounts of data, but they are typically used to generate <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data keys</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-key-pairs">data keys pairs</a>. For details, see <a>GenerateDataKey</a> and <a>GenerateDataKeyPair</a>.</p> </li> <li> <p> <b>Asymmetric CMKs</b> can contain an RSA key pair or an Elliptic Curve (ECC) key pair. The private key in an asymmetric CMK never leaves AWS KMS unencrypted. However, you can use the <a>GetPublicKey</a> operation to download the public key so it can be used outside of AWS KMS. CMKs with RSA key pairs can be used to encrypt or decrypt data or sign and verify messages (but not both). CMKs with ECC key pairs can be used only to sign and verify messages.</p> </li> </ul> <p>For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To create different types of CMKs, use the following guidance:</p> <dl> <dt>Asymmetric CMKs</dt> <dd> <p>To create an asymmetric CMK, use the <code>CustomerMasterKeySpec</code> parameter to specify the type of key material in the CMK. Then, use the <code>KeyUsage</code> parameter to determine whether the CMK will be used to encrypt and decrypt or sign and verify. You can&#39;t change these properties after the CMK is created.</p> <p> </p> </dd> <dt>Symmetric CMKs</dt> <dd> <p>When creating a symmetric CMK, you don&#39;t need to specify the <code>CustomerMasterKeySpec</code> or <code>KeyUsage</code> parameters. The default value for <code>CustomerMasterKeySpec</code>, <code>SYMMETRIC<em>DEFAULT</code>, and the default value for <code>KeyUsage</code>, <code>ENCRYPT</em>DECRYPT</code>, are the only valid values for symmetric CMKs. </p> <p> </p> </dd> <dt>Multi-Region primary keys</dt> <dt>Imported key material</dt> <dd> <p>To create a multi-Region <i>primary key</i> in the local AWS Region, use the <code>MultiRegion</code> parameter with a value of <code>True</code>. To create a multi-Region <i>replica key</i>, that is, a CMK with the same key ID and key material as a primary key, but in a different AWS Region, use the <a>ReplicateKey</a> operation. To change a replica key to a primary key, and its primary key to a replica key, use the <a>UpdatePrimaryRegion</a> operation.</p> <p>This operation supports <i>multi-Region keys</i>, an AWS KMS feature that lets you create multiple interoperable CMKs in different AWS Regions. Because these CMKs have the same key ID, key material, and other metadata, you can use them to encrypt data in one AWS Region and decrypt it in a different AWS Region without making a cross-Region call or exposing the plaintext data. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You can create symmetric and asymmetric multi-Region keys and multi-Region keys with imported key material. You cannot create multi-Region keys in a custom key store.</p> <p> </p> </dd> <dd> <p>To import your own key material, begin by creating a symmetric CMK with no key material. To do this, use the <code>Origin</code> parameter of <code>CreateKey</code> with a value of <code>EXTERNAL</code>. Next, use <a>GetParametersForImport</a> operation to get a public key and import token, and use the public key to encrypt your key material. Then, use <a>ImportKeyMaterial</a> with your import token to import the key material. For step-by-step instructions, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>. You cannot import the key material into an asymmetric CMK.</p> <p>To create a multi-Region primary key with imported key material, use the <code>Origin</code> parameter of <code>CreateKey</code> with a value of <code>EXTERNAL</code> and the <code>MultiRegion</code> parameter with a value of <code>True</code>. To create replicas of the multi-Region primary key, use the <a>ReplicateKey</a> operation. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> </p> </dd> <dt>Custom key store</dt> <dd> <p>To create a symmetric CMK in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, use the <code>CustomKeyStoreId</code> parameter to specify the custom key store. You must also use the <code>Origin</code> parameter with a value of <code>AWS_CLOUDHSM</code>. The AWS CloudHSM cluster that is associated with the custom key store must have at least two active HSMs in different Availability Zones in the AWS Region. </p> <p>You cannot create an asymmetric CMK or a multi-Region CMK in a custom key store. For information about custom key stores in AWS KMS see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Using Custom Key Stores</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </dd> </dl> <p> <b>Cross-account use</b>: No. You cannot use this operation to create a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CreateKey</a> (IAM policy). To use the <code>Tags</code> parameter, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> (IAM policy). For examples and information about related permissions, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/iam-policies.html#iam-policy-example-create-key">Allow a user to create CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>DescribeKey</a> </p> </li> <li> <p> <a>ListKeys</a> </p> </li> <li> <p> <a>ScheduleKeyDeletion</a> </p> </li> </ul></p>
    async fn create_key(
        &self,
        input: CreateKeyRequest,
    ) -> Result<CreateKeyResponse, RusotoError<CreateKeyError>>;

    /// <p><p>Decrypts ciphertext that was encrypted by a AWS KMS customer master key (CMK) using any of the following operations:</p> <ul> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>GenerateDataKeyPairWithoutPlaintext</a> </p> </li> </ul> <p>You can use this operation to decrypt ciphertext that was encrypted under a symmetric or asymmetric CMK. When the CMK is asymmetric, you must specify the CMK and the encryption algorithm that was used to encrypt the ciphertext. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The Decrypt operation also decrypts ciphertext that was encrypted outside of AWS KMS by the public key in an AWS KMS asymmetric CMK. However, it cannot decrypt ciphertext produced by other libraries, such as the <a href="https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/">AWS Encryption SDK</a> or <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 client-side encryption</a>. These libraries return a ciphertext format that is incompatible with AWS KMS.</p> <p>If the ciphertext was encrypted under a symmetric CMK, the <code>KeyId</code> parameter is optional. AWS KMS can get this information from metadata that it adds to the symmetric ciphertext blob. This feature adds durability to your implementation by ensuring that authorized users can decrypt ciphertext decades after it was encrypted, even if they&#39;ve lost track of the CMK ID. However, specifying the CMK is always recommended as a best practice. When you use the <code>KeyId</code> parameter to specify a CMK, AWS KMS only uses the CMK you specify. If the ciphertext was encrypted under a different CMK, the <code>Decrypt</code> operation fails. This practice ensures that you use the CMK that you intend.</p> <p>Whenever possible, use key policies to give users permission to call the <code>Decrypt</code> operation on a particular CMK, instead of using IAM policies. Otherwise, you might create an IAM user policy that gives the user <code>Decrypt</code> permission on all CMKs. This user could decrypt ciphertext that was encrypted by CMKs in other accounts if the key policy for the cross-account CMK permits it. If you must use an IAM policy for <code>Decrypt</code> permissions, limit the user to particular CMKs or particular trusted accounts. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/iam-policies.html#iam-policies-best-practices">Best practices for IAM policies</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. You can decrypt a ciphertext using a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:Decrypt</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>ReEncrypt</a> </p> </li> </ul></p>
    async fn decrypt(
        &self,
        input: DecryptRequest,
    ) -> Result<DecryptResponse, RusotoError<DecryptError>>;

    /// <p><p>Deletes the specified alias. </p> <note> <p>Adding, deleting, or updating an alias can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation. </p> <p>Each CMK can have multiple aliases. To change the alias of a CMK, use <a>DeleteAlias</a> to delete the current alias and <a>CreateAlias</a> to create a new alias. To associate an existing alias with a different customer master key (CMK), call <a>UpdateAlias</a>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on an alias in a different AWS account.</p> <p> <b>Required permissions</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DeleteAlias</a> on the alias (IAM policy).</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DeleteAlias</a> on the CMK (key policy).</p> </li> </ul> <p>For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html#alias-access">Controlling access to aliases</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> </ul></p>
    async fn delete_alias(
        &self,
        input: DeleteAliasRequest,
    ) -> Result<(), RusotoError<DeleteAliasError>>;

    /// <p><p>Deletes a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. This operation does not delete the AWS CloudHSM cluster that is associated with the custom key store, or affect any users or keys in the cluster.</p> <p>The custom key store that you delete cannot contain any AWS KMS <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">customer master keys (CMKs)</a>. Before deleting the key store, verify that you will never need to use any of the CMKs in the key store for any <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a>. Then, use <a>ScheduleKeyDeletion</a> to delete the AWS KMS customer master keys (CMKs) from the key store. When the scheduled waiting period expires, the <code>ScheduleKeyDeletion</code> operation deletes the CMKs. Then it makes a best effort to delete the key material from the associated cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>After all CMKs are deleted from AWS KMS, use <a>DisconnectCustomKeyStore</a> to disconnect the key store from AWS KMS. Then, you can delete the custom key store.</p> <p>Instead of deleting the custom key store, consider using <a>DisconnectCustomKeyStore</a> to disconnect it from AWS KMS. While the key store is disconnected, you cannot create or use the CMKs in the key store. But, you do not need to delete CMKs and you can reconnect a disconnected custom key store at any time.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DeleteCustomKeyStore</a> (IAM policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ConnectCustomKeyStore</a> </p> </li> <li> <p> <a>CreateCustomKeyStore</a> </p> </li> <li> <p> <a>DescribeCustomKeyStores</a> </p> </li> <li> <p> <a>DisconnectCustomKeyStore</a> </p> </li> <li> <p> <a>UpdateCustomKeyStore</a> </p> </li> </ul></p>
    async fn delete_custom_key_store(
        &self,
        input: DeleteCustomKeyStoreRequest,
    ) -> Result<DeleteCustomKeyStoreResponse, RusotoError<DeleteCustomKeyStoreError>>;

    /// <p><p>Deletes key material that you previously imported. This operation makes the specified customer master key (CMK) unusable. For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>. </p> <p>When the specified CMK is in the <code>PendingDeletion</code> state, this operation does not change the CMK&#39;s state. Otherwise, it changes the CMK&#39;s state to <code>PendingImport</code>.</p> <p>After you delete key material, you can use <a>ImportKeyMaterial</a> to reimport the same key material into the CMK.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DeleteImportedKeyMaterial</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>GetParametersForImport</a> </p> </li> <li> <p> <a>ImportKeyMaterial</a> </p> </li> </ul></p>
    async fn delete_imported_key_material(
        &self,
        input: DeleteImportedKeyMaterialRequest,
    ) -> Result<(), RusotoError<DeleteImportedKeyMaterialError>>;

    /// <p><p>Gets information about <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key stores</a> in the account and Region.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>By default, this operation returns information about all custom key stores in the account and Region. To get only information about a particular custom key store, use either the <code>CustomKeyStoreName</code> or <code>CustomKeyStoreId</code> parameter (but not both).</p> <p>To determine whether the custom key store is connected to its AWS CloudHSM cluster, use the <code>ConnectionState</code> element in the response. If an attempt to connect the custom key store failed, the <code>ConnectionState</code> value is <code>FAILED</code> and the <code>ConnectionErrorCode</code> element in the response indicates the cause of the failure. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>Custom key stores have a <code>DISCONNECTED</code> connection state if the key store has never been connected or you use the <a>DisconnectCustomKeyStore</a> operation to disconnect it. If your custom key store state is <code>CONNECTED</code> but you are having trouble using it, make sure that its associated AWS CloudHSM cluster is active and contains the minimum number of HSMs required for the operation, if any.</p> <p> For help repairing your custom key store, see the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting Custom Key Stores</a> topic in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DescribeCustomKeyStores</a> (IAM policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ConnectCustomKeyStore</a> </p> </li> <li> <p> <a>CreateCustomKeyStore</a> </p> </li> <li> <p> <a>DeleteCustomKeyStore</a> </p> </li> <li> <p> <a>DisconnectCustomKeyStore</a> </p> </li> <li> <p> <a>UpdateCustomKeyStore</a> </p> </li> </ul></p>
    async fn describe_custom_key_stores(
        &self,
        input: DescribeCustomKeyStoresRequest,
    ) -> Result<DescribeCustomKeyStoresResponse, RusotoError<DescribeCustomKeyStoresError>>;

    /// <p><p>Provides detailed information about a customer master key (CMK). You can run <code>DescribeKey</code> on a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a> or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMK</a>.</p> <p>This detailed information includes the key ARN, creation date (and deletion date, if applicable), the key state, and the origin and expiration date (if any) of the key material. For CMKs in custom key stores, it includes information about the custom key store, such as the key store ID and the AWS CloudHSM cluster ID. It includes fields, like <code>KeySpec</code>, that help you distinguish symmetric from asymmetric CMKs. It also provides information that is particularly important to asymmetric CMKs, such as the key usage (encryption or signing) and the encryption algorithms or signing algorithms that the CMK supports.</p> <p> <code>DescribeKey</code> does not return the following information:</p> <ul> <li> <p>Aliases associated with the CMK. To get this information, use <a>ListAliases</a>.</p> </li> <li> <p>Whether automatic key rotation is enabled on the CMK. To get this information, use <a>GetKeyRotationStatus</a>. Also, some key states prevent a CMK from being automatically rotated. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotate-keys-how-it-works">How Automatic Key Rotation Works</a> in <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p>Tags on the CMK. To get this information, use <a>ListResourceTags</a>.</p> </li> <li> <p>Key policies and grants on the CMK. To get this information, use <a>GetKeyPolicy</a> and <a>ListGrants</a>.</p> </li> </ul> <p>If you call the <code>DescribeKey</code> operation on a <i>predefined AWS alias</i>, that is, an AWS alias with no key ID, AWS KMS creates an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">AWS managed CMK</a>. Then, it associates the alias with the new CMK, and returns the <code>KeyId</code> and <code>Arn</code> of the new CMK in the response.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DescribeKey</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>GetKeyPolicy</a> </p> </li> <li> <p> <a>GetKeyRotationStatus</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>ListGrants</a> </p> </li> <li> <p> <a>ListKeys</a> </p> </li> <li> <p> <a>ListResourceTags</a> </p> </li> <li> <p> <a>ListRetirableGrants</a> </p> </li> </ul></p>
    async fn describe_key(
        &self,
        input: DescribeKeyRequest,
    ) -> Result<DescribeKeyResponse, RusotoError<DescribeKeyError>>;

    /// <p>Sets the state of a customer master key (CMK) to disabled. This change temporarily prevents use of the CMK for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a>. </p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DisableKey</a> (key policy)</p> <p> <b>Related operations</b>: <a>EnableKey</a> </p>
    async fn disable_key(
        &self,
        input: DisableKeyRequest,
    ) -> Result<(), RusotoError<DisableKeyError>>;

    /// <p><p>Disables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified symmetric customer master key (CMK).</p> <p> You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#asymmetric-cmks">asymmetric CMKs</a>, CMKs with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html#mrk-replica-key">multi-Region keys</a>, set the property on the primary key. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DisableKeyRotation</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>EnableKeyRotation</a> </p> </li> <li> <p> <a>GetKeyRotationStatus</a> </p> </li> </ul></p>
    async fn disable_key_rotation(
        &self,
        input: DisableKeyRotationRequest,
    ) -> Result<(), RusotoError<DisableKeyRotationError>>;

    /// <p><p>Disconnects the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> from its associated AWS CloudHSM cluster. While a custom key store is disconnected, you can manage the custom key store and its customer master keys (CMKs), but you cannot create or use CMKs in the custom key store. You can reconnect the custom key store at any time.</p> <note> <p>While a custom key store is disconnected, all attempts to create customer master keys (CMKs) in the custom key store or to use existing CMKs in <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a> will fail. This action can prevent users from storing and accessing sensitive data.</p> </note> <p/> <p>To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation. To reconnect a custom key store, use the <a>ConnectCustomKeyStore</a> operation.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DisconnectCustomKeyStore</a> (IAM policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ConnectCustomKeyStore</a> </p> </li> <li> <p> <a>CreateCustomKeyStore</a> </p> </li> <li> <p> <a>DeleteCustomKeyStore</a> </p> </li> <li> <p> <a>DescribeCustomKeyStores</a> </p> </li> <li> <p> <a>UpdateCustomKeyStore</a> </p> </li> </ul></p>
    async fn disconnect_custom_key_store(
        &self,
        input: DisconnectCustomKeyStoreRequest,
    ) -> Result<DisconnectCustomKeyStoreResponse, RusotoError<DisconnectCustomKeyStoreError>>;

    /// <p>Sets the key state of a customer master key (CMK) to enabled. This allows you to use the CMK for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a>. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:EnableKey</a> (key policy)</p> <p> <b>Related operations</b>: <a>DisableKey</a> </p>
    async fn enable_key(&self, input: EnableKeyRequest) -> Result<(), RusotoError<EnableKeyError>>;

    /// <p><p>Enables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified symmetric customer master key (CMK).</p> <p>You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#asymmetric-cmks">asymmetric CMKs</a>, CMKs with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html#mrk-replica-key">multi-Region keys</a>, set the property on the primary key.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:EnableKeyRotation</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>DisableKeyRotation</a> </p> </li> <li> <p> <a>GetKeyRotationStatus</a> </p> </li> </ul></p>
    async fn enable_key_rotation(
        &self,
        input: EnableKeyRotationRequest,
    ) -> Result<(), RusotoError<EnableKeyRotationError>>;

    /// <p><p>Encrypts plaintext into ciphertext by using a customer master key (CMK). The <code>Encrypt</code> operation has two primary use cases:</p> <ul> <li> <p>You can encrypt small amounts of arbitrary data, such as a personal identifier or database password, or other sensitive information. </p> </li> <li> <p>You can use the <code>Encrypt</code> operation to move encrypted data from one AWS Region to another. For example, in Region A, generate a data key and use the plaintext key to encrypt your data. Then, in Region A, use the <code>Encrypt</code> operation to encrypt the plaintext data key under a CMK in Region B. Now, you can move the encrypted data and the encrypted data key to Region B. When necessary, you can decrypt the encrypted data key and the encrypted data entirely within in Region B.</p> </li> </ul> <p>You don&#39;t need to use the <code>Encrypt</code> operation to encrypt a data key. The <a>GenerateDataKey</a> and <a>GenerateDataKeyPair</a> operations return a plaintext data key and an encrypted copy of that data key.</p> <p>When you encrypt data, you must specify a symmetric or asymmetric CMK to use in the encryption operation. The CMK must have a <code>KeyUsage</code> value of <code>ENCRYPT<em>DECRYPT.</code> To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation. </p> <p>If you use a symmetric CMK, you can use an encryption context to add additional security to your encryption operation. If you specify an <code>EncryptionContext</code> when encrypting data, you must specify the same encryption context (a case-sensitive exact match) when decrypting the data. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt</em>context&quot;&gt;Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>If you specify an asymmetric CMK, you must also specify the encryption algorithm. The algorithm must be compatible with the CMK type.</p> <important> <p>When you use an asymmetric CMK to encrypt or reencrypt data, be sure to record the CMK and encryption algorithm that you choose. You will be required to provide the same CMK and encryption algorithm when you decrypt the data. If the CMK and algorithm do not match the values used to encrypt the data, the decrypt operation fails.</p> <p>You are not required to supply the CMK ID and encryption algorithm when you decrypt with symmetric CMKs because AWS KMS stores this information in the ciphertext blob. AWS KMS cannot store metadata in ciphertext generated with asymmetric keys. The standard format for asymmetric key ciphertext does not include configurable fields.</p> </important> <p>The maximum size of the data that you can encrypt varies with the type of CMK and the encryption algorithm that you choose.</p> <ul> <li> <p>Symmetric CMKs</p> <ul> <li> <p> <code>SYMMETRIC<em>DEFAULT</code>: 4096 bytes</p> </li> </ul> </li> <li> <p> <code>RSA</em>2048</code> </p> <ul> <li> <p> <code>RSAES<em>OAEP</em>SHA<em>1</code>: 214 bytes</p> </li> <li> <p> <code>RSAES</em>OAEP<em>SHA</em>256</code>: 190 bytes</p> </li> </ul> </li> <li> <p> <code>RSA<em>3072</code> </p> <ul> <li> <p> <code>RSAES</em>OAEP<em>SHA</em>1</code>: 342 bytes</p> </li> <li> <p> <code>RSAES<em>OAEP</em>SHA<em>256</code>: 318 bytes</p> </li> </ul> </li> <li> <p> <code>RSA</em>4096</code> </p> <ul> <li> <p> <code>RSAES<em>OAEP</em>SHA<em>1</code>: 470 bytes</p> </li> <li> <p> <code>RSAES</em>OAEP<em>SHA</em>256</code>: 446 bytes</p> </li> </ul> </li> </ul> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:Encrypt</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> </ul></p>
    async fn encrypt(
        &self,
        input: EncryptRequest,
    ) -> Result<EncryptResponse, RusotoError<EncryptError>>;

    /// <p><p>Generates a unique symmetric data key for client-side encryption. This operation returns a plaintext copy of the data key and a copy that is encrypted under a customer master key (CMK) that you specify. You can use the plaintext key to encrypt your data outside of AWS KMS and store the encrypted data key with the encrypted data.</p> <p> <code>GenerateDataKey</code> returns a unique data key for each request. The bytes in the plaintext key are not related to the caller or the CMK.</p> <p>To generate a data key, specify the symmetric CMK that will be used to encrypt the data key. You cannot use an asymmetric CMK to generate data keys. To get the type of your CMK, use the <a>DescribeKey</a> operation. You must also specify the length of the data key. Use either the <code>KeySpec</code> or <code>NumberOfBytes</code> parameters (but not both). For 128-bit and 256-bit data keys, use the <code>KeySpec</code> parameter. </p> <p>To get only an encrypted copy of the data key, use <a>GenerateDataKeyWithoutPlaintext</a>. To generate an asymmetric data key pair, use the <a>GenerateDataKeyPair</a> or <a>GenerateDataKeyPairWithoutPlaintext</a> operation. To get a cryptographically secure random byte string, use <a>GenerateRandom</a>.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>How to use your data key</b> </p> <p>We recommend that you use the following pattern to encrypt data locally in your application. You can write your own code or use a client-side encryption library, such as the <a href="https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/">AWS Encryption SDK</a>, the <a href="https://docs.aws.amazon.com/dynamodb-encryption-client/latest/devguide/">Amazon DynamoDB Encryption Client</a>, or <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 client-side encryption</a> to do these tasks for you.</p> <p>To encrypt data outside of AWS KMS:</p> <ol> <li> <p>Use the <code>GenerateDataKey</code> operation to get a data key.</p> </li> <li> <p>Use the plaintext data key (in the <code>Plaintext</code> field of the response) to encrypt your data outside of AWS KMS. Then erase the plaintext data key from memory.</p> </li> <li> <p>Store the encrypted data key (in the <code>CiphertextBlob</code> field of the response) with the encrypted data.</p> </li> </ol> <p>To decrypt data outside of AWS KMS:</p> <ol> <li> <p>Use the <a>Decrypt</a> operation to decrypt the encrypted data key. The operation returns a plaintext copy of the data key.</p> </li> <li> <p>Use the plaintext data key to decrypt data outside of AWS KMS, then erase the plaintext data key from memory.</p> </li> </ol> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GenerateDataKey</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>GenerateDataKeyPairWithoutPlaintext</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> </ul></p>
    async fn generate_data_key(
        &self,
        input: GenerateDataKeyRequest,
    ) -> Result<GenerateDataKeyResponse, RusotoError<GenerateDataKeyError>>;

    /// <p><p>Generates a unique asymmetric data key pair. The <code>GenerateDataKeyPair</code> operation returns a plaintext public key, a plaintext private key, and a copy of the private key that is encrypted under the symmetric CMK you specify. You can use the data key pair to perform asymmetric cryptography outside of AWS KMS.</p> <p> <code>GenerateDataKeyPair</code> returns a unique data key pair for each request. The bytes in the keys are not related to the caller or the CMK that is used to encrypt the private key.</p> <p>You can use the public key that <code>GenerateDataKeyPair</code> returns to encrypt data or verify a signature outside of AWS KMS. Then, store the encrypted private key with the data. When you are ready to decrypt data or sign a message, you can use the <a>Decrypt</a> operation to decrypt the encrypted private key.</p> <p>To generate a data key pair, you must specify a symmetric customer master key (CMK) to encrypt the private key in a data key pair. You cannot use an asymmetric CMK or a CMK in a custom key store. To get the type and origin of your CMK, use the <a>DescribeKey</a> operation. </p> <p>If you are using the data key pair to encrypt data, or for any operation where you don&#39;t immediately need a private key, consider using the <a>GenerateDataKeyPairWithoutPlaintext</a> operation. <code>GenerateDataKeyPairWithoutPlaintext</code> returns a plaintext public key and an encrypted private key, but omits the plaintext private key that you need only to decrypt ciphertext or sign a message. Later, when you need to decrypt the data or sign a message, use the <a>Decrypt</a> operation to decrypt the encrypted private key in the data key pair.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GenerateDataKeyPair</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPairWithoutPlaintext</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> </ul></p>
    async fn generate_data_key_pair(
        &self,
        input: GenerateDataKeyPairRequest,
    ) -> Result<GenerateDataKeyPairResponse, RusotoError<GenerateDataKeyPairError>>;

    /// <p><p>Generates a unique asymmetric data key pair. The <code>GenerateDataKeyPairWithoutPlaintext</code> operation returns a plaintext public key and a copy of the private key that is encrypted under the symmetric CMK you specify. Unlike <a>GenerateDataKeyPair</a>, this operation does not return a plaintext private key. </p> <p>To generate a data key pair, you must specify a symmetric customer master key (CMK) to encrypt the private key in the data key pair. You cannot use an asymmetric CMK or a CMK in a custom key store. To get the type and origin of your CMK, use the <code>KeySpec</code> field in the <a>DescribeKey</a> response.</p> <p>You can use the public key that <code>GenerateDataKeyPairWithoutPlaintext</code> returns to encrypt data or verify a signature outside of AWS KMS. Then, store the encrypted private key with the data. When you are ready to decrypt data or sign a message, you can use the <a>Decrypt</a> operation to decrypt the encrypted private key.</p> <p> <code>GenerateDataKeyPairWithoutPlaintext</code> returns a unique data key pair for each request. The bytes in the key are not related to the caller or CMK that is used to encrypt the private key.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GenerateDataKeyPairWithoutPlaintext</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> </ul></p>
    async fn generate_data_key_pair_without_plaintext(
        &self,
        input: GenerateDataKeyPairWithoutPlaintextRequest,
    ) -> Result<
        GenerateDataKeyPairWithoutPlaintextResponse,
        RusotoError<GenerateDataKeyPairWithoutPlaintextError>,
    >;

    /// <p><p>Generates a unique symmetric data key. This operation returns a data key that is encrypted under a customer master key (CMK) that you specify. To request an asymmetric data key pair, use the <a>GenerateDataKeyPair</a> or <a>GenerateDataKeyPairWithoutPlaintext</a> operations.</p> <p> <code>GenerateDataKeyWithoutPlaintext</code> is identical to the <a>GenerateDataKey</a> operation except that returns only the encrypted copy of the data key. This operation is useful for systems that need to encrypt data at some point, but not immediately. When you need to encrypt the data, you call the <a>Decrypt</a> operation on the encrypted copy of the key. </p> <p>It&#39;s also useful in distributed systems with different levels of trust. For example, you might store encrypted data in containers. One component of your system creates new containers and stores an encrypted data key with each container. Then, a different component puts the data into the containers. That component first decrypts the data key, uses the plaintext data key to encrypt data, puts the encrypted data into the container, and then destroys the plaintext data key. In this system, the component that creates the containers never sees the plaintext data key.</p> <p> <code>GenerateDataKeyWithoutPlaintext</code> returns a unique data key for each request. The bytes in the keys are not related to the caller or CMK that is used to encrypt the private key.</p> <p>To generate a data key, you must specify the symmetric customer master key (CMK) that is used to encrypt the data key. You cannot use an asymmetric CMK to generate a data key. To get the type of your CMK, use the <a>DescribeKey</a> operation.</p> <p>If the operation succeeds, you will find the encrypted copy of the data key in the <code>CiphertextBlob</code> field.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GenerateDataKeyWithoutPlaintext</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>GenerateDataKeyPairWithoutPlaintext</a> </p> </li> </ul></p>
    async fn generate_data_key_without_plaintext(
        &self,
        input: GenerateDataKeyWithoutPlaintextRequest,
    ) -> Result<
        GenerateDataKeyWithoutPlaintextResponse,
        RusotoError<GenerateDataKeyWithoutPlaintextError>,
    >;

    /// <p>Returns a random byte string that is cryptographically secure.</p> <p>By default, the random byte string is generated in AWS KMS. To generate the byte string in the AWS CloudHSM cluster that is associated with a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, specify the custom key store ID.</p> <p>For more information about entropy and random number generation, see <a href="https://docs.aws.amazon.com/kms/latest/cryptographic-details/">AWS Key Management Service Cryptographic Details</a>.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GenerateRandom</a> (IAM policy)</p>
    async fn generate_random(
        &self,
        input: GenerateRandomRequest,
    ) -> Result<GenerateRandomResponse, RusotoError<GenerateRandomError>>;

    /// <p>Gets a key policy attached to the specified customer master key (CMK).</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GetKeyPolicy</a> (key policy)</p> <p> <b>Related operations</b>: <a>PutKeyPolicy</a> </p>
    async fn get_key_policy(
        &self,
        input: GetKeyPolicyRequest,
    ) -> Result<GetKeyPolicyResponse, RusotoError<GetKeyPolicyError>>;

    /// <p><p>Gets a Boolean value that indicates whether <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> is enabled for the specified customer master key (CMK).</p> <p>You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#asymmetric-cmks">asymmetric CMKs</a>, CMKs with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html#mrk-replica-key">multi-Region keys</a>, set the property on the primary key. The key rotation status for these CMKs is always <code>false</code>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <ul> <li> <p>Disabled: The key rotation status does not change when you disable a CMK. However, while the CMK is disabled, AWS KMS does not rotate the backing key.</p> </li> <li> <p>Pending deletion: While a CMK is pending deletion, its key rotation status is <code>false</code> and AWS KMS does not rotate the backing key. If you cancel the deletion, the original key rotation status is restored.</p> </li> </ul> <p> <b>Cross-account use</b>: Yes. To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GetKeyRotationStatus</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>DisableKeyRotation</a> </p> </li> <li> <p> <a>EnableKeyRotation</a> </p> </li> </ul></p>
    async fn get_key_rotation_status(
        &self,
        input: GetKeyRotationStatusRequest,
    ) -> Result<GetKeyRotationStatusResponse, RusotoError<GetKeyRotationStatusError>>;

    /// <p><p>Returns the items you need to import key material into a symmetric, customer managed customer master key (CMK). For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>This operation returns a public key and an import token. Use the public key to encrypt the symmetric key material. Store the import token to send with a subsequent <a>ImportKeyMaterial</a> request.</p> <p>You must specify the key ID of the symmetric CMK into which you will import key material. This CMK&#39;s <code>Origin</code> must be <code>EXTERNAL</code>. You must also specify the wrapping algorithm and type of wrapping key (public key) that you will use to encrypt the key material. You cannot perform this operation on an asymmetric CMK or on any CMK in a different AWS account.</p> <p>To import key material, you must use the public key and import token from the same response. These items are valid for 24 hours. The expiration date and time appear in the <code>GetParametersForImport</code> response. You cannot use an expired token in an <a>ImportKeyMaterial</a> request. If your key and token expire, send another <code>GetParametersForImport</code> request.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GetParametersForImport</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ImportKeyMaterial</a> </p> </li> <li> <p> <a>DeleteImportedKeyMaterial</a> </p> </li> </ul></p>
    async fn get_parameters_for_import(
        &self,
        input: GetParametersForImportRequest,
    ) -> Result<GetParametersForImportResponse, RusotoError<GetParametersForImportError>>;

    /// <p>Returns the public key of an asymmetric CMK. Unlike the private key of a asymmetric CMK, which never leaves AWS KMS unencrypted, callers with <code>kms:GetPublicKey</code> permission can download the public key of an asymmetric CMK. You can share the public key to allow others to encrypt messages and verify signatures outside of AWS KMS. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You do not need to download the public key. Instead, you can use the public key within AWS KMS by calling the <a>Encrypt</a>, <a>ReEncrypt</a>, or <a>Verify</a> operations with the identifier of an asymmetric CMK. When you use the public key within AWS KMS, you benefit from the authentication, authorization, and logging that are part of every AWS KMS operation. You also reduce of risk of encrypting data that cannot be decrypted. These features are not effective outside of AWS KMS. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/download-public-key.html#download-public-key-considerations">Special Considerations for Downloading Public Keys</a>.</p> <p>To help you use the public key safely outside of AWS KMS, <code>GetPublicKey</code> returns important information about the public key in the response, including:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-CustomerMasterKeySpec">CustomerMasterKeySpec</a>: The type of key material in the public key, such as <code>RSA_4096</code> or <code>ECC_NIST_P521</code>.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-KeyUsage">KeyUsage</a>: Whether the key is used for encryption or signing.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-EncryptionAlgorithms">EncryptionAlgorithms</a> or <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-SigningAlgorithms">SigningAlgorithms</a>: A list of the encryption algorithms or the signing algorithms for the key.</p> </li> </ul> <p>Although AWS KMS cannot enforce these restrictions on external operations, it is crucial that you use this information to prevent the public key from being used improperly. For example, you can prevent a public signing key from being used encrypt data, or prevent a public key from being used with an encryption algorithm that is not supported by AWS KMS. You can also avoid errors, such as using the wrong signing algorithm in a verification operation.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GetPublicKey</a> (key policy)</p> <p> <b>Related operations</b>: <a>CreateKey</a> </p>
    async fn get_public_key(
        &self,
        input: GetPublicKeyRequest,
    ) -> Result<GetPublicKeyResponse, RusotoError<GetPublicKeyError>>;

    /// <p><p>Imports key material into an existing symmetric AWS KMS customer master key (CMK) that was created without key material. After you successfully import key material into a CMK, you can <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#reimport-key-material">reimport the same key material</a> into that CMK, but you cannot import different key material. </p> <p>You cannot perform this operation on an asymmetric CMK or on any CMK in a different AWS account. For more information about creating CMKs with no key material and then importing key material, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Before using this operation, call <a>GetParametersForImport</a>. Its response includes a public key and an import token. Use the public key to encrypt the key material. Then, submit the import token from the same <code>GetParametersForImport</code> response.</p> <p>When calling this operation, you must specify the following values:</p> <ul> <li> <p>The key ID or key ARN of a CMK with no key material. Its <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>To create a CMK with no key material, call <a>CreateKey</a> and set the value of its <code>Origin</code> parameter to <code>EXTERNAL</code>. To get the <code>Origin</code> of a CMK, call <a>DescribeKey</a>.)</p> </li> <li> <p>The encrypted key material. To get the public key to encrypt the key material, call <a>GetParametersForImport</a>.</p> </li> <li> <p>The import token that <a>GetParametersForImport</a> returned. You must use a public key and token from the same <code>GetParametersForImport</code> response.</p> </li> <li> <p>Whether the key material expires and if so, when. If you set an expiration date, AWS KMS deletes the key material from the CMK on the specified date, and the CMK becomes unusable. To use the CMK again, you must reimport the same key material. The only way to change an expiration date is by reimporting the same key material and specifying a new expiration date. </p> </li> </ul> <p>When this operation is successful, the key state of the CMK changes from <code>PendingImport</code> to <code>Enabled</code>, and you can use the CMK.</p> <p>If this operation fails, use the exception to help determine the problem. If the error is related to the key material, the import token, or wrapping key, use <a>GetParametersForImport</a> to get a new public key and import token for the CMK and repeat the import procedure. For help, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#importing-keys-overview">How To Import Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ImportKeyMaterial</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>DeleteImportedKeyMaterial</a> </p> </li> <li> <p> <a>GetParametersForImport</a> </p> </li> </ul></p>
    async fn import_key_material(
        &self,
        input: ImportKeyMaterialRequest,
    ) -> Result<ImportKeyMaterialResponse, RusotoError<ImportKeyMaterialError>>;

    /// <p><p>Gets a list of aliases in the caller&#39;s AWS account and region. For more information about aliases, see <a>CreateAlias</a>.</p> <p>By default, the <code>ListAliases</code> operation returns all aliases in the account and region. To get only the aliases associated with a particular customer master key (CMK), use the <code>KeyId</code> parameter.</p> <p>The <code>ListAliases</code> response can include aliases that you created and associated with your customer managed CMKs, and aliases that AWS created and associated with AWS managed CMKs in your account. You can recognize AWS aliases because their names have the format <code>aws/&lt;service-name&gt;</code>, such as <code>aws/dynamodb</code>.</p> <p>The response might also include aliases that have no <code>TargetKeyId</code> field. These are predefined aliases that AWS has created but has not yet associated with a CMK. Aliases that AWS creates in your account, including predefined aliases, do not count against your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html#aliases-limit">AWS KMS aliases quota</a>.</p> <p> <b>Cross-account use</b>: No. <code>ListAliases</code> does not return aliases in other AWS accounts.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListAliases</a> (IAM policy)</p> <p>For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html#alias-access">Controlling access to aliases</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> </ul></p>
    async fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> Result<ListAliasesResponse, RusotoError<ListAliasesError>>;

    /// <p><p>Gets a list of all grants for the specified customer master key (CMK). </p> <p>You must specify the CMK in all requests. You can filter the grant list by grant ID or grantee principal.</p> <note> <p>The <code>GranteePrincipal</code> field in the <code>ListGrants</code> response usually contains the user or role designated as the grantee principal in the grant. However, when the grantee principal in the grant is an AWS service, the <code>GranteePrincipal</code> field contains the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#principal-services">service principal</a>, which might represent several different grantee principals.</p> </note> <p> <b>Cross-account use</b>: Yes. To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListGrants</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateGrant</a> </p> </li> <li> <p> <a>ListRetirableGrants</a> </p> </li> <li> <p> <a>RetireGrant</a> </p> </li> <li> <p> <a>RevokeGrant</a> </p> </li> </ul></p>
    async fn list_grants(
        &self,
        input: ListGrantsRequest,
    ) -> Result<ListGrantsResponse, RusotoError<ListGrantsError>>;

    /// <p><p>Gets the names of the key policies that are attached to a customer master key (CMK). This operation is designed to get policy names that you can use in a <a>GetKeyPolicy</a> operation. However, the only valid policy name is <code>default</code>. </p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListKeyPolicies</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>GetKeyPolicy</a> </p> </li> <li> <p> <a>PutKeyPolicy</a> </p> </li> </ul></p>
    async fn list_key_policies(
        &self,
        input: ListKeyPoliciesRequest,
    ) -> Result<ListKeyPoliciesResponse, RusotoError<ListKeyPoliciesError>>;

    /// <p><p>Gets a list of all customer master keys (CMKs) in the caller&#39;s AWS account and Region.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListKeys</a> (IAM policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>DescribeKey</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>ListResourceTags</a> </p> </li> </ul></p>
    async fn list_keys(
        &self,
        input: ListKeysRequest,
    ) -> Result<ListKeysResponse, RusotoError<ListKeysError>>;

    /// <p><p>Returns all tags on the specified customer master key (CMK).</p> <p>For general information about tags, including the format and syntax, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS resources</a> in the <i>Amazon Web Services General Reference</i>. For information about using tags in AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging keys</a>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListResourceTags</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>ReplicateKey</a> </p> </li> <li> <p> <a>TagResource</a> </p> </li> <li> <p> <a>UntagResource</a> </p> </li> </ul></p>
    async fn list_resource_tags(
        &self,
        input: ListResourceTagsRequest,
    ) -> Result<ListResourceTagsResponse, RusotoError<ListResourceTagsError>>;

    /// <p><p>Returns information about all grants in the AWS account and Region that have the specified retiring principal. For more information about grants, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>You can specify any principal in your AWS account. The grants that are returned include grants for CMKs in your AWS account and other AWS accounts.</p> <p>You might use this operation to determine which grants you may retire. To retire a grant, use the <a>RetireGrant</a> operation.</p> <p> <b>Cross-account use</b>: You must specify a principal in your AWS account. However, this operation can return grants in any AWS account. You do not need <code>kms:ListRetirableGrants</code> permission (or any other additional permission) in any AWS account other than your own.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListRetirableGrants</a> (IAM policy) in your AWS account.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateGrant</a> </p> </li> <li> <p> <a>ListGrants</a> </p> </li> <li> <p> <a>RetireGrant</a> </p> </li> <li> <p> <a>RevokeGrant</a> </p> </li> </ul></p>
    async fn list_retirable_grants(
        &self,
        input: ListRetirableGrantsRequest,
    ) -> Result<ListGrantsResponse, RusotoError<ListRetirableGrantsError>>;

    /// <p>Attaches a key policy to the specified customer master key (CMK). </p> <p>For more information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key Policies</a> in the <i>AWS Key Management Service Developer Guide</i>. For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>IAM User Guide</i> </i>. For examples of adding a key policy in multiple programming languages, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/programming-key-policies.html#put-policy">Setting a key policy</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:PutKeyPolicy</a> (key policy)</p> <p> <b>Related operations</b>: <a>GetKeyPolicy</a> </p>
    async fn put_key_policy(
        &self,
        input: PutKeyPolicyRequest,
    ) -> Result<(), RusotoError<PutKeyPolicyError>>;

    /// <p><p>Decrypts ciphertext and then reencrypts it entirely within AWS KMS. You can use this operation to change the customer master key (CMK) under which data is encrypted, such as when you <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotate-keys-manually">manually rotate</a> a CMK or change the CMK that protects a ciphertext. You can also use it to reencrypt ciphertext under the same CMK, such as to change the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">encryption context</a> of a ciphertext.</p> <p>The <code>ReEncrypt</code> operation can decrypt ciphertext that was encrypted by using an AWS KMS CMK in an AWS KMS operation, such as <a>Encrypt</a> or <a>GenerateDataKey</a>. It can also decrypt ciphertext that was encrypted by using the public key of an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#asymmetric-cmks">asymmetric CMK</a> outside of AWS KMS. However, it cannot decrypt ciphertext produced by other libraries, such as the <a href="https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/">AWS Encryption SDK</a> or <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 client-side encryption</a>. These libraries return a ciphertext format that is incompatible with AWS KMS.</p> <p>When you use the <code>ReEncrypt</code> operation, you need to provide information for the decrypt operation and the subsequent encrypt operation.</p> <ul> <li> <p>If your ciphertext was encrypted under an asymmetric CMK, you must use the <code>SourceKeyId</code> parameter to identify the CMK that encrypted the ciphertext. You must also supply the encryption algorithm that was used. This information is required to decrypt the data.</p> </li> <li> <p>If your ciphertext was encrypted under a symmetric CMK, the <code>SourceKeyId</code> parameter is optional. AWS KMS can get this information from metadata that it adds to the symmetric ciphertext blob. This feature adds durability to your implementation by ensuring that authorized users can decrypt ciphertext decades after it was encrypted, even if they&#39;ve lost track of the CMK ID. However, specifying the source CMK is always recommended as a best practice. When you use the <code>SourceKeyId</code> parameter to specify a CMK, AWS KMS uses only the CMK you specify. If the ciphertext was encrypted under a different CMK, the <code>ReEncrypt</code> operation fails. This practice ensures that you use the CMK that you intend.</p> </li> <li> <p>To reencrypt the data, you must use the <code>DestinationKeyId</code> parameter specify the CMK that re-encrypts the data after it is decrypted. You can select a symmetric or asymmetric CMK. If the destination CMK is an asymmetric CMK, you must also provide the encryption algorithm. The algorithm that you choose must be compatible with the CMK.</p> <important> <p>When you use an asymmetric CMK to encrypt or reencrypt data, be sure to record the CMK and encryption algorithm that you choose. You will be required to provide the same CMK and encryption algorithm when you decrypt the data. If the CMK and algorithm do not match the values used to encrypt the data, the decrypt operation fails.</p> <p>You are not required to supply the CMK ID and encryption algorithm when you decrypt with symmetric CMKs because AWS KMS stores this information in the ciphertext blob. AWS KMS cannot store metadata in ciphertext generated with asymmetric keys. The standard format for asymmetric key ciphertext does not include configurable fields.</p> </important> </li> </ul> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. The source CMK and destination CMK can be in different AWS accounts. Either or both CMKs can be in a different account than the caller.</p> <p> <b>Required permissions</b>:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ReEncryptFrom</a> permission on the source CMK (key policy)</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ReEncryptTo</a> permission on the destination CMK (key policy)</p> </li> </ul> <p>To permit reencryption from or to a CMK, include the <code>&quot;kms:ReEncrypt*&quot;</code> permission in your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">key policy</a>. This permission is automatically included in the key policy when you use the console to create a CMK. But you must include it manually when you create a CMK programmatically or when you use the <a>PutKeyPolicy</a> operation to set a key policy.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> </ul></p>
    async fn re_encrypt(
        &self,
        input: ReEncryptRequest,
    ) -> Result<ReEncryptResponse, RusotoError<ReEncryptError>>;

    /// <p><p>Replicates a multi-Region key into the specified Region. This operation creates a multi-Region replica key based on a multi-Region primary key in a different Region of the same AWS partition. You can create multiple replicas of a primary key, but each must be in a different Region. To create a multi-Region primary key, use the <a>CreateKey</a> operation.</p> <p>This operation supports <i>multi-Region keys</i>, an AWS KMS feature that lets you create multiple interoperable CMKs in different AWS Regions. Because these CMKs have the same key ID, key material, and other metadata, you can use them to encrypt data in one AWS Region and decrypt it in a different AWS Region without making a cross-Region call or exposing the plaintext data. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>A <i>replica key</i> is a fully-functional CMK that can be used independently of its primary and peer replica keys. A primary key and its replica keys share properties that make them interoperable. They have the same <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-id">key ID</a> and key material. They also have the same <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-spec">key spec</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-usage">key usage</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-origin">key material origin</a>, and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic key rotation status</a>. AWS KMS automatically synchronizes these shared properties among related multi-Region keys. All other properties of a replica key can differ, including its <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">key policy</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">tags</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html">aliases</a>, and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">key state</a>. AWS KMS pricing and quotas for CMKs apply to each primary key and replica key.</p> <p>When this operation completes, the new replica key has a transient key state of <code>Creating</code>. This key state changes to <code>Enabled</code> (or <code>PendingImport</code>) after a few seconds when the process of creating the new replica key is complete. While the key state is <code>Creating</code>, you can manage key, but you cannot yet use it in cryptographic operations. If you are creating and using the replica key programmatically, retry on <code>KMSInvalidStateException</code> or call <code>DescribeKey</code> to check its <code>KeyState</code> value before using it. For details about the <code>Creating</code> key state, see <a href="kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The AWS CloudTrail log of a <code>ReplicateKey</code> operation records a <code>ReplicateKey</code> operation in the primary key&#39;s Region and a <a>CreateKey</a> operation in the replica key&#39;s Region.</p> <p>If you replicate a multi-Region primary key with imported key material, the replica key is created with no key material. You must import the same key material that you imported into the primary key. For details, see <a href="kms/latest/developerguide/multi-region-keys-import.html">Importing key material into multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To convert a replica key to a primary key, use the <a>UpdatePrimaryRegion</a> operation.</p> <note> <p> <code>ReplicateKey</code> uses different default values for the <code>KeyPolicy</code> and <code>Tags</code> parameters than those used in the AWS KMS console. For details, see the parameter descriptions.</p> </note> <p> <b>Cross-account use</b>: No. You cannot use this operation to create a CMK in a different AWS account. </p> <p> <b>Required permissions</b>: </p> <ul> <li> <p> <code>kms:ReplicateKey</code> on the primary CMK (in the primary CMK&#39;s Region). Include this permission in the primary CMK&#39;s key policy.</p> </li> <li> <p> <code>kms:CreateKey</code> in an IAM policy in the replica Region.</p> </li> <li> <p>To use the <code>Tags</code> parameter, <code>kms:TagResource</code> in an IAM policy in the replica Region.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>UpdatePrimaryRegion</a> </p> </li> </ul></p>
    async fn replicate_key(
        &self,
        input: ReplicateKeyRequest,
    ) -> Result<ReplicateKeyResponse, RusotoError<ReplicateKeyError>>;

    /// <p><p>Deletes a grant. Typically, you retire a grant when you no longer need its permissions. To identify the grant to retire, use a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">grant token</a>, or both the grant ID and a key identifier (key ID or key ARN) of the customer master key (CMK). The <a>CreateGrant</a> operation returns both values.</p> <p>This operation can be called by the <i>retiring principal</i> for a grant, by the <i>grantee principal</i> if the grant allows the <code>RetireGrant</code> operation, and by the AWS account (root user) in which the grant is created. It can also be called by principals to whom permission for retiring a grant is delegated. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#grant-delete">Retiring and revoking grants</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>For detailed information about grants, including grant terminology, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Using grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>. For examples of working with grants in several programming languages, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/programming-grants.html">Programming grants</a>.</p> <p> <b>Cross-account use</b>: Yes. You can retire a grant on a CMK in a different AWS account.</p> <p> <b>Required permissions:</b>:Permission to retire a grant is determined primarily by the grant. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#grant-delete">Retiring and revoking grants</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateGrant</a> </p> </li> <li> <p> <a>ListGrants</a> </p> </li> <li> <p> <a>ListRetirableGrants</a> </p> </li> <li> <p> <a>RevokeGrant</a> </p> </li> </ul></p>
    async fn retire_grant(
        &self,
        input: RetireGrantRequest,
    ) -> Result<(), RusotoError<RetireGrantError>>;

    /// <p><p>Deletes the specified grant. You revoke a grant to terminate the permissions that the grant allows. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/managing-grants.html#grant-delete">Retiring and revoking grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>When you create, retire, or revoke a grant, there might be a brief delay, usually less than five minutes, until the grant is available throughout AWS KMS. This state is known as <i>eventual consistency</i>. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-eventual-consistency">Eventual consistency</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>. </p> <p> <b>Cross-account use</b>: Yes. To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:RevokeGrant</a> (key policy).</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateGrant</a> </p> </li> <li> <p> <a>ListGrants</a> </p> </li> <li> <p> <a>ListRetirableGrants</a> </p> </li> <li> <p> <a>RetireGrant</a> </p> </li> </ul></p>
    async fn revoke_grant(
        &self,
        input: RevokeGrantRequest,
    ) -> Result<(), RusotoError<RevokeGrantError>>;

    /// <p><p>Schedules the deletion of a customer master key (CMK). By default, AWS KMS applies a waiting period of 30 days, but you can specify a waiting period of 7-30 days. When this operation is successful, the key state of the CMK changes to <code>PendingDeletion</code> and the key can&#39;t be used in any cryptographic operations. It remains in this state for the duration of the waiting period. Before the waiting period ends, you can use <a>CancelKeyDeletion</a> to cancel the deletion of the CMK. After the waiting period ends, AWS KMS deletes the CMK, its key material, and all AWS KMS data associated with it, including all aliases that refer to it.</p> <important> <p>Deleting a CMK is a destructive and potentially dangerous operation. When a CMK is deleted, all data that was encrypted under the CMK is unrecoverable. (The only exception is a multi-Region replica key.) To prevent the use of a CMK without deleting it, use <a>DisableKey</a>. </p> </important> <p>If you schedule deletion of a CMK from a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, when the waiting period expires, <code>ScheduleKeyDeletion</code> deletes the CMK from AWS KMS. Then AWS KMS makes a best effort to delete the key material from the associated AWS CloudHSM cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>You can schedule the deletion of a multi-Region primary key and its replica keys at any time. However, AWS KMS will not delete a multi-Region primary key with existing replica keys. If you schedule the deletion of a primary key with replicas, its key state changes to <code>PendingReplicaDeletion</code> and it cannot be replicated or used in cryptographic operations. This status can continue indefinitely. When the last of its replicas keys is deleted (not just scheduled), the key state of the primary key changes to <code>PendingDeletion</code> and its waiting period (<code>PendingWindowInDays</code>) begins. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-delete.html">Deleting multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>. </p> <p>For more information about scheduling a CMK for deletion, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: kms:ScheduleKeyDeletion (key policy)</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CancelKeyDeletion</a> </p> </li> <li> <p> <a>DisableKey</a> </p> </li> </ul></p>
    async fn schedule_key_deletion(
        &self,
        input: ScheduleKeyDeletionRequest,
    ) -> Result<ScheduleKeyDeletionResponse, RusotoError<ScheduleKeyDeletionError>>;

    /// <p>Creates a <a href="https://en.wikipedia.org/wiki/Digital_signature">digital signature</a> for a message or message digest by using the private key in an asymmetric CMK. To verify the signature, use the <a>Verify</a> operation, or use the public key in the same asymmetric CMK outside of AWS KMS. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Digital signatures are generated and verified by using asymmetric key pair, such as an RSA or ECC pair that is represented by an asymmetric customer master key (CMK). The key owner (or an authorized user) uses their private key to sign a message. Anyone with the public key can verify that the message was signed with that particular private key and that the message hasn't changed since it was signed. </p> <p>To use the <code>Sign</code> operation, provide the following information:</p> <ul> <li> <p>Use the <code>KeyId</code> parameter to identify an asymmetric CMK with a <code>KeyUsage</code> value of <code>SIGN_VERIFY</code>. To get the <code>KeyUsage</code> value of a CMK, use the <a>DescribeKey</a> operation. The caller must have <code>kms:Sign</code> permission on the CMK.</p> </li> <li> <p>Use the <code>Message</code> parameter to specify the message or message digest to sign. You can submit messages of up to 4096 bytes. To sign a larger message, generate a hash digest of the message, and then provide the hash digest in the <code>Message</code> parameter. To indicate whether the message is a full message or a digest, use the <code>MessageType</code> parameter.</p> </li> <li> <p>Choose a signing algorithm that is compatible with the CMK. </p> </li> </ul> <important> <p>When signing a message, be sure to record the CMK and the signing algorithm. This information is required to verify the signature.</p> </important> <p>To verify the signature that this operation generates, use the <a>Verify</a> operation. Or use the <a>GetPublicKey</a> operation to download the public key and then use the public key to verify the signature outside of AWS KMS. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:Sign</a> (key policy)</p> <p> <b>Related operations</b>: <a>Verify</a> </p>
    async fn sign(&self, input: SignRequest) -> Result<SignResponse, RusotoError<SignError>>;

    /// <p><p>Adds or edits tags on a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a>.</p> <note> <p>Tagging or untagging a CMK can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>Each tag consists of a tag key and a tag value, both of which are case-sensitive strings. The tag value can be an empty (null) string. To add a tag, specify a new tag key and a tag value. To edit a tag, specify an existing tag key and a new tag value.</p> <p>You can use this operation to tag a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a>, but you cannot tag an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMK</a>, an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-owned-cmk">AWS owned CMK</a>, a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#keystore-concept">custom key store</a>, or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#alias-concept">alias</a>.</p> <p>You can also add tags to a CMK while creating it (<a>CreateKey</a>) or replicating it (<a>ReplicateKey</a>).</p> <p>For information about using tags in AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging keys</a>. For general information about tags, including the format and syntax, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS resources</a> in the <i>Amazon Web Services General Reference</i>. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account. </p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> (key policy)</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>ListResourceTags</a> </p> </li> <li> <p> <a>ReplicateKey</a> </p> </li> <li> <p> <a>UntagResource</a> </p> </li> </ul></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p><p>Deletes tags from a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a>. To delete a tag, specify the tag key and the CMK.</p> <note> <p>Tagging or untagging a CMK can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>When it succeeds, the <code>UntagResource</code> operation doesn&#39;t return any output. Also, if the specified tag key isn&#39;t found on the CMK, it doesn&#39;t throw an exception or return a response. To confirm that the operation worked, use the <a>ListResourceTags</a> operation.</p> <p>For information about using tags in AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging keys</a>. For general information about tags, including the format and syntax, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS resources</a> in the <i>Amazon Web Services General Reference</i>. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UntagResource</a> (key policy)</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>ListResourceTags</a> </p> </li> <li> <p> <a>ReplicateKey</a> </p> </li> <li> <p> <a>TagResource</a> </p> </li> </ul></p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p><p>Associates an existing AWS KMS alias with a different customer master key (CMK). Each alias is associated with only one CMK at a time, although a CMK can have multiple aliases. The alias and the CMK must be in the same AWS account and Region.</p> <note> <p>Adding, deleting, or updating an alias can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>The current and new CMK must be the same type (both symmetric or both asymmetric), and they must have the same key usage (<code>ENCRYPT<em>DECRYPT</code> or <code>SIGN</em>VERIFY</code>). This restriction prevents errors in code that uses aliases. If you must assign an alias to a different type of CMK, use <a>DeleteAlias</a> to delete the old alias and <a>CreateAlias</a> to create a new alias.</p> <p>You cannot use <code>UpdateAlias</code> to change an alias name. To change an alias name, use <a>DeleteAlias</a> to delete the old alias and <a>CreateAlias</a> to create a new alias.</p> <p>Because an alias is not a property of a CMK, you can create, update, and delete the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs in the account, use the <a>ListAliases</a> operation. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account. </p> <p> <b>Required permissions</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UpdateAlias</a> on the alias (IAM policy).</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UpdateAlias</a> on the current CMK (key policy).</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UpdateAlias</a> on the new CMK (key policy).</p> </li> </ul> <p>For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html#alias-access">Controlling access to aliases</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> </ul></p>
    async fn update_alias(
        &self,
        input: UpdateAliasRequest,
    ) -> Result<(), RusotoError<UpdateAliasError>>;

    /// <p><p>Changes the properties of a custom key store. Use the <code>CustomKeyStoreId</code> parameter to identify the custom key store you want to edit. Use the remaining parameters to change the properties of the custom key store.</p> <p>You can only update a custom key store that is disconnected. To disconnect the custom key store, use <a>DisconnectCustomKeyStore</a>. To reconnect the custom key store after the update completes, use <a>ConnectCustomKeyStore</a>. To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>Use the parameters of <code>UpdateCustomKeyStore</code> to edit your keystore settings.</p> <ul> <li> <p>Use the <b>NewCustomKeyStoreName</b> parameter to change the friendly name of the custom key store to the value that you specify.</p> <p> </p> </li> <li> <p>Use the <b>KeyStorePassword</b> parameter tell AWS KMS the current password of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user (CU)</a> in the associated AWS CloudHSM cluster. You can use this parameter to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-password">fix connection failures</a> that occur when AWS KMS cannot log into the associated cluster because the <code>kmsuser</code> password has changed. This value does not change the password in the AWS CloudHSM cluster.</p> <p> </p> </li> <li> <p>Use the <b>CloudHsmClusterId</b> parameter to associate the custom key store with a different, but related, AWS CloudHSM cluster. You can use this parameter to repair a custom key store if its AWS CloudHSM cluster becomes corrupted or is deleted, or when you need to create or restore a cluster from a backup. </p> </li> </ul> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account. </p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UpdateCustomKeyStore</a> (IAM policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ConnectCustomKeyStore</a> </p> </li> <li> <p> <a>CreateCustomKeyStore</a> </p> </li> <li> <p> <a>DeleteCustomKeyStore</a> </p> </li> <li> <p> <a>DescribeCustomKeyStores</a> </p> </li> <li> <p> <a>DisconnectCustomKeyStore</a> </p> </li> </ul></p>
    async fn update_custom_key_store(
        &self,
        input: UpdateCustomKeyStoreRequest,
    ) -> Result<UpdateCustomKeyStoreResponse, RusotoError<UpdateCustomKeyStoreError>>;

    /// <p><p>Updates the description of a customer master key (CMK). To see the description of a CMK, use <a>DescribeKey</a>. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account. </p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UpdateKeyDescription</a> (key policy)</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>DescribeKey</a> </p> </li> </ul></p>
    async fn update_key_description(
        &self,
        input: UpdateKeyDescriptionRequest,
    ) -> Result<(), RusotoError<UpdateKeyDescriptionError>>;

    /// <p><p>Changes the primary key of a multi-Region key. </p> <p>This operation changes the replica key in the specified Region to a primary key and changes the former primary key to a replica key. For example, suppose you have a primary key in <code>us-east-1</code> and a replica key in <code>eu-west-2</code>. If you run <code>UpdatePrimaryRegion</code> with a <code>PrimaryRegion</code> value of <code>eu-west-2</code>, the primary key is now the key in <code>eu-west-2</code>, and the key in <code>us-east-1</code> becomes a replica key. For details, see </p> <p>This operation supports <i>multi-Region keys</i>, an AWS KMS feature that lets you create multiple interoperable CMKs in different AWS Regions. Because these CMKs have the same key ID, key material, and other metadata, you can use them to encrypt data in one AWS Region and decrypt it in a different AWS Region without making a cross-Region call or exposing the plaintext data. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The <i>primary key</i> of a multi-Region key is the source for properties that are always shared by primary and replica keys, including the key material, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-id">key ID</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-spec">key spec</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-usage">key usage</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-origin">key material origin</a>, and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic key rotation</a>. It&#39;s the only key that can be replicated. You cannot <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_ScheduleKeyDeletion.html">delete the primary key</a> until all replicas are deleted.</p> <p>The key ID and primary Region that you specify uniquely identify the replica key that will become the primary key. The primary Region must already have a replica key. This operation does not create a CMK in the specified Region. To find the replica keys, use the <a>DescribeKey</a> operation on the primary key or any replica key. To create a replica key, use the <a>ReplicateKey</a> operation.</p> <p>You can run this operation while using the affected multi-Region keys in cryptographic operations. This operation should not delay, interrupt, or cause failures in cryptographic operations. </p> <p>Even after this operation completes, the process of updating the primary Region might still be in progress for a few more seconds. Operations such as <code>DescribeKey</code> might display both the old and new primary keys as replicas. The old and new primary keys have a transient key state of <code>Updating</code>. The original key state is restored when the update is complete. While the key state is <code>Updating</code>, you can use the keys in cryptographic operations, but you cannot replicate the new primary key or perform certain management operations, such as enabling or disabling these keys. For details about the <code>Updating</code> key state, see <a href="kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>This operation does not return any output. To verify that primary key is changed, use the <a>DescribeKey</a> operation.</p> <p> <b>Cross-account use</b>: No. You cannot use this operation in a different AWS account. </p> <p> <b>Required permissions</b>: </p> <ul> <li> <p> <code>kms:UpdatePrimaryRegion</code> on the current primary CMK (in the primary CMK&#39;s Region). Include this permission primary CMK&#39;s key policy.</p> </li> <li> <p> <code>kms:UpdatePrimaryRegion</code> on the current replica CMK (in the replica CMK&#39;s Region). Include this permission in the replica CMK&#39;s key policy.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>ReplicateKey</a> </p> </li> </ul></p>
    async fn update_primary_region(
        &self,
        input: UpdatePrimaryRegionRequest,
    ) -> Result<(), RusotoError<UpdatePrimaryRegionError>>;

    /// <p>Verifies a digital signature that was generated by the <a>Sign</a> operation. </p> <p/> <p>Verification confirms that an authorized user signed the message with the specified CMK and signing algorithm, and the message hasn't changed since it was signed. If the signature is verified, the value of the <code>SignatureValid</code> field in the response is <code>True</code>. If the signature verification fails, the <code>Verify</code> operation fails with an <code>KMSInvalidSignatureException</code> exception.</p> <p>A digital signature is generated by using the private key in an asymmetric CMK. The signature is verified by using the public key in the same asymmetric CMK. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To verify a digital signature, you can use the <code>Verify</code> operation. Specify the same asymmetric CMK, message, and signing algorithm that were used to produce the signature.</p> <p>You can also verify the digital signature by using the public key of the CMK outside of AWS KMS. Use the <a>GetPublicKey</a> operation to download the public key in the asymmetric CMK and then use the public key to verify the signature outside of AWS KMS. The advantage of using the <code>Verify</code> operation is that it is performed within AWS KMS. As a result, it's easy to call, the operation is performed within the FIPS boundary, it is logged in AWS CloudTrail, and you can use key policy and IAM policy to determine who is authorized to use the CMK to verify signatures.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter. </p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:Verify</a> (key policy)</p> <p> <b>Related operations</b>: <a>Sign</a> </p>
    async fn verify(
        &self,
        input: VerifyRequest,
    ) -> Result<VerifyResponse, RusotoError<VerifyError>>;
}
/// A client for the KMS API.
#[derive(Clone)]
pub struct KmsClient {
    client: Client,
    region: region::Region,
}

impl KmsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KmsClient {
        KmsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KmsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        KmsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> KmsClient {
        KmsClient { client, region }
    }
}

#[async_trait]
impl Kms for KmsClient {
    /// <p>Cancels the deletion of a customer master key (CMK). When this operation succeeds, the key state of the CMK is <code>Disabled</code>. To enable the CMK, use <a>EnableKey</a>. </p> <p>For more information about scheduling and canceling deletion of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CancelKeyDeletion</a> (key policy)</p> <p> <b>Related operations</b>: <a>ScheduleKeyDeletion</a> </p>
    async fn cancel_key_deletion(
        &self,
        input: CancelKeyDeletionRequest,
    ) -> Result<CancelKeyDeletionResponse, RusotoError<CancelKeyDeletionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.CancelKeyDeletion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CancelKeyDeletionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CancelKeyDeletionResponse, _>()
    }

    /// <p><p>Connects or reconnects a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> to its associated AWS CloudHSM cluster.</p> <p>The custom key store must be connected before you can create customer master keys (CMKs) in the key store or use the CMKs it contains. You can disconnect and reconnect a custom key store at any time.</p> <p>To connect a custom key store, its associated AWS CloudHSM cluster must have at least one active HSM. To get the number of active HSMs in a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation. To add HSMs to the cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation. Also, the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user</a> (CU) must not be logged into the cluster. This prevents AWS KMS from using this account to log in.</p> <p>The connection process can take an extended amount of time to complete; up to 20 minutes. This operation starts the connection process, but it does not wait for it to complete. When it succeeds, this operation quickly returns an HTTP 200 response and a JSON object with no properties. However, this response does not indicate that the custom key store is connected. To get the connection state of the custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>During the connection process, AWS KMS finds the AWS CloudHSM cluster that is associated with the custom key store, creates the connection infrastructure, connects to the cluster, logs into the AWS CloudHSM client as the <code>kmsuser</code> CU, and rotates its password.</p> <p>The <code>ConnectCustomKeyStore</code> operation might fail for various reasons. To find the reason, use the <a>DescribeCustomKeyStores</a> operation and see the <code>ConnectionErrorCode</code> in the response. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>To fix the failure, use the <a>DisconnectCustomKeyStore</a> operation to disconnect the custom key store, correct the error, use the <a>UpdateCustomKeyStore</a> operation if necessary, and then use <code>ConnectCustomKeyStore</code> again.</p> <p>If you are having trouble connecting or disconnecting a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ConnectCustomKeyStore</a> (IAM policy)</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateCustomKeyStore</a> </p> </li> <li> <p> <a>DeleteCustomKeyStore</a> </p> </li> <li> <p> <a>DescribeCustomKeyStores</a> </p> </li> <li> <p> <a>DisconnectCustomKeyStore</a> </p> </li> <li> <p> <a>UpdateCustomKeyStore</a> </p> </li> </ul></p>
    async fn connect_custom_key_store(
        &self,
        input: ConnectCustomKeyStoreRequest,
    ) -> Result<ConnectCustomKeyStoreResponse, RusotoError<ConnectCustomKeyStoreError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.ConnectCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ConnectCustomKeyStoreError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ConnectCustomKeyStoreResponse, _>()
    }

    /// <p><p>Creates a friendly name for a customer master key (CMK). </p> <note> <p>Adding, deleting, or updating an alias can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>You can use an alias to identify a CMK in the AWS KMS console, in the <a>DescribeKey</a> operation and in <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a>, such as <a>Encrypt</a> and <a>GenerateDataKey</a>. You can also change the CMK that&#39;s associated with the alias (<a>UpdateAlias</a>) or delete the alias (<a>DeleteAlias</a>) at any time. These operations don&#39;t affect the underlying CMK. </p> <p>You can associate the alias with any customer managed CMK in the same AWS Region. Each alias is associated with only one CMK at a time, but a CMK can have multiple aliases. A valid CMK is required. You can&#39;t create an alias without a CMK.</p> <p>The alias must be unique in the account and Region, but you can have aliases with the same name in different Regions. For detailed information about aliases, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html">Using aliases</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>This operation does not return a response. To get the alias that you created, use the <a>ListAliases</a> operation.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on an alias in a different AWS account.</p> <p> <b>Required permissions</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CreateAlias</a> on the alias (IAM policy).</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CreateAlias</a> on the CMK (key policy).</p> </li> </ul> <p>For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html#alias-access">Controlling access to aliases</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> </ul></p>
    async fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> Result<(), RusotoError<CreateAliasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.CreateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateAliasError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Creates a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> that is associated with an <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/clusters.html">AWS CloudHSM cluster</a> that you own and manage.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>Before you create the custom key store, you must assemble the required elements, including an AWS CloudHSM cluster that fulfills the requirements for a custom key store. For details about the required elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>When the operation completes successfully, it returns the ID of the new custom key store. Before you can use your new custom key store, you need to use the <a>ConnectCustomKeyStore</a> operation to connect the new key store to its AWS CloudHSM cluster. Even if you are not going to use your custom key store immediately, you might want to connect it to verify that all settings are correct and then disconnect it until you are ready to use it.</p> <p>For help with failures, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CreateCustomKeyStore</a> (IAM policy).</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ConnectCustomKeyStore</a> </p> </li> <li> <p> <a>DeleteCustomKeyStore</a> </p> </li> <li> <p> <a>DescribeCustomKeyStores</a> </p> </li> <li> <p> <a>DisconnectCustomKeyStore</a> </p> </li> <li> <p> <a>UpdateCustomKeyStore</a> </p> </li> </ul></p>
    async fn create_custom_key_store(
        &self,
        input: CreateCustomKeyStoreRequest,
    ) -> Result<CreateCustomKeyStoreResponse, RusotoError<CreateCustomKeyStoreError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.CreateCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateCustomKeyStoreError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateCustomKeyStoreResponse, _>()
    }

    /// <p><p>Adds a grant to a customer master key (CMK). </p> <p>A <i>grant</i> is a policy instrument that allows AWS principals to use AWS KMS customer master keys (CMKs) in cryptographic operations. It also can allow them to view a CMK (<a>DescribeKey</a>) and create and manage grants. When authorizing access to a CMK, grants are considered along with key policies and IAM policies. Grants are often used for temporary permissions because you can create one, use its permissions, and delete it without changing your key policies or IAM policies. </p> <p>For detailed information about grants, including grant terminology, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Using grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>. For examples of working with grants in several programming languages, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/programming-grants.html">Programming grants</a>.</p> <p>The <code>CreateGrant</code> operation returns a <code>GrantToken</code> and a <code>GrantId</code>.</p> <ul> <li> <p>When you create, retire, or revoke a grant, there might be a brief delay, usually less than five minutes, until the grant is available throughout AWS KMS. This state is known as <i>eventual consistency</i>. Once the grant has achieved eventual consistency, the grantee principal can use the permissions in the grant without identifying the grant. </p> <p>However, to use the permissions in the grant immediately, use the <code>GrantToken</code> that <code>CreateGrant</code> returns. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </li> <li> <p>The <code>CreateGrant</code> operation also returns a <code>GrantId</code>. You can use the <code>GrantId</code> and a key identifier to identify the grant in the <a>RetireGrant</a> and <a>RevokeGrant</a> operations. To find the grant ID, use the <a>ListGrants</a> or <a>ListRetirableGrants</a> operations.</p> </li> </ul> <p>For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>. For more information about grants, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter. </p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CreateGrant</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ListGrants</a> </p> </li> <li> <p> <a>ListRetirableGrants</a> </p> </li> <li> <p> <a>RetireGrant</a> </p> </li> <li> <p> <a>RevokeGrant</a> </p> </li> </ul></p>
    async fn create_grant(
        &self,
        input: CreateGrantRequest,
    ) -> Result<CreateGrantResponse, RusotoError<CreateGrantError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.CreateGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateGrantError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateGrantResponse, _>()
    }

    /// <p><p>Creates a unique customer managed <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master-keys">customer master key</a> (CMK) in your AWS account and Region.</p> <p>You can use the <code>CreateKey</code> operation to create symmetric or asymmetric CMKs.</p> <ul> <li> <p> <b>Symmetric CMKs</b> contain a 256-bit symmetric key that never leaves AWS KMS unencrypted. To use the CMK, you must call AWS KMS. You can use a symmetric CMK to encrypt and decrypt small amounts of data, but they are typically used to generate <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data keys</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-key-pairs">data keys pairs</a>. For details, see <a>GenerateDataKey</a> and <a>GenerateDataKeyPair</a>.</p> </li> <li> <p> <b>Asymmetric CMKs</b> can contain an RSA key pair or an Elliptic Curve (ECC) key pair. The private key in an asymmetric CMK never leaves AWS KMS unencrypted. However, you can use the <a>GetPublicKey</a> operation to download the public key so it can be used outside of AWS KMS. CMKs with RSA key pairs can be used to encrypt or decrypt data or sign and verify messages (but not both). CMKs with ECC key pairs can be used only to sign and verify messages.</p> </li> </ul> <p>For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To create different types of CMKs, use the following guidance:</p> <dl> <dt>Asymmetric CMKs</dt> <dd> <p>To create an asymmetric CMK, use the <code>CustomerMasterKeySpec</code> parameter to specify the type of key material in the CMK. Then, use the <code>KeyUsage</code> parameter to determine whether the CMK will be used to encrypt and decrypt or sign and verify. You can&#39;t change these properties after the CMK is created.</p> <p> </p> </dd> <dt>Symmetric CMKs</dt> <dd> <p>When creating a symmetric CMK, you don&#39;t need to specify the <code>CustomerMasterKeySpec</code> or <code>KeyUsage</code> parameters. The default value for <code>CustomerMasterKeySpec</code>, <code>SYMMETRIC<em>DEFAULT</code>, and the default value for <code>KeyUsage</code>, <code>ENCRYPT</em>DECRYPT</code>, are the only valid values for symmetric CMKs. </p> <p> </p> </dd> <dt>Multi-Region primary keys</dt> <dt>Imported key material</dt> <dd> <p>To create a multi-Region <i>primary key</i> in the local AWS Region, use the <code>MultiRegion</code> parameter with a value of <code>True</code>. To create a multi-Region <i>replica key</i>, that is, a CMK with the same key ID and key material as a primary key, but in a different AWS Region, use the <a>ReplicateKey</a> operation. To change a replica key to a primary key, and its primary key to a replica key, use the <a>UpdatePrimaryRegion</a> operation.</p> <p>This operation supports <i>multi-Region keys</i>, an AWS KMS feature that lets you create multiple interoperable CMKs in different AWS Regions. Because these CMKs have the same key ID, key material, and other metadata, you can use them to encrypt data in one AWS Region and decrypt it in a different AWS Region without making a cross-Region call or exposing the plaintext data. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You can create symmetric and asymmetric multi-Region keys and multi-Region keys with imported key material. You cannot create multi-Region keys in a custom key store.</p> <p> </p> </dd> <dd> <p>To import your own key material, begin by creating a symmetric CMK with no key material. To do this, use the <code>Origin</code> parameter of <code>CreateKey</code> with a value of <code>EXTERNAL</code>. Next, use <a>GetParametersForImport</a> operation to get a public key and import token, and use the public key to encrypt your key material. Then, use <a>ImportKeyMaterial</a> with your import token to import the key material. For step-by-step instructions, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>. You cannot import the key material into an asymmetric CMK.</p> <p>To create a multi-Region primary key with imported key material, use the <code>Origin</code> parameter of <code>CreateKey</code> with a value of <code>EXTERNAL</code> and the <code>MultiRegion</code> parameter with a value of <code>True</code>. To create replicas of the multi-Region primary key, use the <a>ReplicateKey</a> operation. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> </p> </dd> <dt>Custom key store</dt> <dd> <p>To create a symmetric CMK in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, use the <code>CustomKeyStoreId</code> parameter to specify the custom key store. You must also use the <code>Origin</code> parameter with a value of <code>AWS_CLOUDHSM</code>. The AWS CloudHSM cluster that is associated with the custom key store must have at least two active HSMs in different Availability Zones in the AWS Region. </p> <p>You cannot create an asymmetric CMK or a multi-Region CMK in a custom key store. For information about custom key stores in AWS KMS see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Using Custom Key Stores</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </dd> </dl> <p> <b>Cross-account use</b>: No. You cannot use this operation to create a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:CreateKey</a> (IAM policy). To use the <code>Tags</code> parameter, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> (IAM policy). For examples and information about related permissions, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/iam-policies.html#iam-policy-example-create-key">Allow a user to create CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>DescribeKey</a> </p> </li> <li> <p> <a>ListKeys</a> </p> </li> <li> <p> <a>ScheduleKeyDeletion</a> </p> </li> </ul></p>
    async fn create_key(
        &self,
        input: CreateKeyRequest,
    ) -> Result<CreateKeyResponse, RusotoError<CreateKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.CreateKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateKeyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateKeyResponse, _>()
    }

    /// <p><p>Decrypts ciphertext that was encrypted by a AWS KMS customer master key (CMK) using any of the following operations:</p> <ul> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>GenerateDataKeyPairWithoutPlaintext</a> </p> </li> </ul> <p>You can use this operation to decrypt ciphertext that was encrypted under a symmetric or asymmetric CMK. When the CMK is asymmetric, you must specify the CMK and the encryption algorithm that was used to encrypt the ciphertext. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The Decrypt operation also decrypts ciphertext that was encrypted outside of AWS KMS by the public key in an AWS KMS asymmetric CMK. However, it cannot decrypt ciphertext produced by other libraries, such as the <a href="https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/">AWS Encryption SDK</a> or <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 client-side encryption</a>. These libraries return a ciphertext format that is incompatible with AWS KMS.</p> <p>If the ciphertext was encrypted under a symmetric CMK, the <code>KeyId</code> parameter is optional. AWS KMS can get this information from metadata that it adds to the symmetric ciphertext blob. This feature adds durability to your implementation by ensuring that authorized users can decrypt ciphertext decades after it was encrypted, even if they&#39;ve lost track of the CMK ID. However, specifying the CMK is always recommended as a best practice. When you use the <code>KeyId</code> parameter to specify a CMK, AWS KMS only uses the CMK you specify. If the ciphertext was encrypted under a different CMK, the <code>Decrypt</code> operation fails. This practice ensures that you use the CMK that you intend.</p> <p>Whenever possible, use key policies to give users permission to call the <code>Decrypt</code> operation on a particular CMK, instead of using IAM policies. Otherwise, you might create an IAM user policy that gives the user <code>Decrypt</code> permission on all CMKs. This user could decrypt ciphertext that was encrypted by CMKs in other accounts if the key policy for the cross-account CMK permits it. If you must use an IAM policy for <code>Decrypt</code> permissions, limit the user to particular CMKs or particular trusted accounts. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/iam-policies.html#iam-policies-best-practices">Best practices for IAM policies</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. You can decrypt a ciphertext using a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:Decrypt</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>ReEncrypt</a> </p> </li> </ul></p>
    async fn decrypt(
        &self,
        input: DecryptRequest,
    ) -> Result<DecryptResponse, RusotoError<DecryptError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.Decrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DecryptError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DecryptResponse, _>()
    }

    /// <p><p>Deletes the specified alias. </p> <note> <p>Adding, deleting, or updating an alias can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation. </p> <p>Each CMK can have multiple aliases. To change the alias of a CMK, use <a>DeleteAlias</a> to delete the current alias and <a>CreateAlias</a> to create a new alias. To associate an existing alias with a different customer master key (CMK), call <a>UpdateAlias</a>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on an alias in a different AWS account.</p> <p> <b>Required permissions</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DeleteAlias</a> on the alias (IAM policy).</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DeleteAlias</a> on the CMK (key policy).</p> </li> </ul> <p>For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html#alias-access">Controlling access to aliases</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> </ul></p>
    async fn delete_alias(
        &self,
        input: DeleteAliasRequest,
    ) -> Result<(), RusotoError<DeleteAliasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.DeleteAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAliasError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. This operation does not delete the AWS CloudHSM cluster that is associated with the custom key store, or affect any users or keys in the cluster.</p> <p>The custom key store that you delete cannot contain any AWS KMS <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">customer master keys (CMKs)</a>. Before deleting the key store, verify that you will never need to use any of the CMKs in the key store for any <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a>. Then, use <a>ScheduleKeyDeletion</a> to delete the AWS KMS customer master keys (CMKs) from the key store. When the scheduled waiting period expires, the <code>ScheduleKeyDeletion</code> operation deletes the CMKs. Then it makes a best effort to delete the key material from the associated cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>After all CMKs are deleted from AWS KMS, use <a>DisconnectCustomKeyStore</a> to disconnect the key store from AWS KMS. Then, you can delete the custom key store.</p> <p>Instead of deleting the custom key store, consider using <a>DisconnectCustomKeyStore</a> to disconnect it from AWS KMS. While the key store is disconnected, you cannot create or use the CMKs in the key store. But, you do not need to delete CMKs and you can reconnect a disconnected custom key store at any time.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DeleteCustomKeyStore</a> (IAM policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ConnectCustomKeyStore</a> </p> </li> <li> <p> <a>CreateCustomKeyStore</a> </p> </li> <li> <p> <a>DescribeCustomKeyStores</a> </p> </li> <li> <p> <a>DisconnectCustomKeyStore</a> </p> </li> <li> <p> <a>UpdateCustomKeyStore</a> </p> </li> </ul></p>
    async fn delete_custom_key_store(
        &self,
        input: DeleteCustomKeyStoreRequest,
    ) -> Result<DeleteCustomKeyStoreResponse, RusotoError<DeleteCustomKeyStoreError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.DeleteCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteCustomKeyStoreError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteCustomKeyStoreResponse, _>()
    }

    /// <p><p>Deletes key material that you previously imported. This operation makes the specified customer master key (CMK) unusable. For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>. </p> <p>When the specified CMK is in the <code>PendingDeletion</code> state, this operation does not change the CMK&#39;s state. Otherwise, it changes the CMK&#39;s state to <code>PendingImport</code>.</p> <p>After you delete key material, you can use <a>ImportKeyMaterial</a> to reimport the same key material into the CMK.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DeleteImportedKeyMaterial</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>GetParametersForImport</a> </p> </li> <li> <p> <a>ImportKeyMaterial</a> </p> </li> </ul></p>
    async fn delete_imported_key_material(
        &self,
        input: DeleteImportedKeyMaterialRequest,
    ) -> Result<(), RusotoError<DeleteImportedKeyMaterialError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.DeleteImportedKeyMaterial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteImportedKeyMaterialError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Gets information about <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key stores</a> in the account and Region.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>By default, this operation returns information about all custom key stores in the account and Region. To get only information about a particular custom key store, use either the <code>CustomKeyStoreName</code> or <code>CustomKeyStoreId</code> parameter (but not both).</p> <p>To determine whether the custom key store is connected to its AWS CloudHSM cluster, use the <code>ConnectionState</code> element in the response. If an attempt to connect the custom key store failed, the <code>ConnectionState</code> value is <code>FAILED</code> and the <code>ConnectionErrorCode</code> element in the response indicates the cause of the failure. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>Custom key stores have a <code>DISCONNECTED</code> connection state if the key store has never been connected or you use the <a>DisconnectCustomKeyStore</a> operation to disconnect it. If your custom key store state is <code>CONNECTED</code> but you are having trouble using it, make sure that its associated AWS CloudHSM cluster is active and contains the minimum number of HSMs required for the operation, if any.</p> <p> For help repairing your custom key store, see the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting Custom Key Stores</a> topic in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DescribeCustomKeyStores</a> (IAM policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ConnectCustomKeyStore</a> </p> </li> <li> <p> <a>CreateCustomKeyStore</a> </p> </li> <li> <p> <a>DeleteCustomKeyStore</a> </p> </li> <li> <p> <a>DisconnectCustomKeyStore</a> </p> </li> <li> <p> <a>UpdateCustomKeyStore</a> </p> </li> </ul></p>
    async fn describe_custom_key_stores(
        &self,
        input: DescribeCustomKeyStoresRequest,
    ) -> Result<DescribeCustomKeyStoresResponse, RusotoError<DescribeCustomKeyStoresError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.DescribeCustomKeyStores");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeCustomKeyStoresError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeCustomKeyStoresResponse, _>()
    }

    /// <p><p>Provides detailed information about a customer master key (CMK). You can run <code>DescribeKey</code> on a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a> or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMK</a>.</p> <p>This detailed information includes the key ARN, creation date (and deletion date, if applicable), the key state, and the origin and expiration date (if any) of the key material. For CMKs in custom key stores, it includes information about the custom key store, such as the key store ID and the AWS CloudHSM cluster ID. It includes fields, like <code>KeySpec</code>, that help you distinguish symmetric from asymmetric CMKs. It also provides information that is particularly important to asymmetric CMKs, such as the key usage (encryption or signing) and the encryption algorithms or signing algorithms that the CMK supports.</p> <p> <code>DescribeKey</code> does not return the following information:</p> <ul> <li> <p>Aliases associated with the CMK. To get this information, use <a>ListAliases</a>.</p> </li> <li> <p>Whether automatic key rotation is enabled on the CMK. To get this information, use <a>GetKeyRotationStatus</a>. Also, some key states prevent a CMK from being automatically rotated. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotate-keys-how-it-works">How Automatic Key Rotation Works</a> in <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p>Tags on the CMK. To get this information, use <a>ListResourceTags</a>.</p> </li> <li> <p>Key policies and grants on the CMK. To get this information, use <a>GetKeyPolicy</a> and <a>ListGrants</a>.</p> </li> </ul> <p>If you call the <code>DescribeKey</code> operation on a <i>predefined AWS alias</i>, that is, an AWS alias with no key ID, AWS KMS creates an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">AWS managed CMK</a>. Then, it associates the alias with the new CMK, and returns the <code>KeyId</code> and <code>Arn</code> of the new CMK in the response.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DescribeKey</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>GetKeyPolicy</a> </p> </li> <li> <p> <a>GetKeyRotationStatus</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>ListGrants</a> </p> </li> <li> <p> <a>ListKeys</a> </p> </li> <li> <p> <a>ListResourceTags</a> </p> </li> <li> <p> <a>ListRetirableGrants</a> </p> </li> </ul></p>
    async fn describe_key(
        &self,
        input: DescribeKeyRequest,
    ) -> Result<DescribeKeyResponse, RusotoError<DescribeKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.DescribeKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeKeyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeKeyResponse, _>()
    }

    /// <p>Sets the state of a customer master key (CMK) to disabled. This change temporarily prevents use of the CMK for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a>. </p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DisableKey</a> (key policy)</p> <p> <b>Related operations</b>: <a>EnableKey</a> </p>
    async fn disable_key(
        &self,
        input: DisableKeyRequest,
    ) -> Result<(), RusotoError<DisableKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.DisableKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisableKeyError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Disables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified symmetric customer master key (CMK).</p> <p> You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#asymmetric-cmks">asymmetric CMKs</a>, CMKs with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html#mrk-replica-key">multi-Region keys</a>, set the property on the primary key. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DisableKeyRotation</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>EnableKeyRotation</a> </p> </li> <li> <p> <a>GetKeyRotationStatus</a> </p> </li> </ul></p>
    async fn disable_key_rotation(
        &self,
        input: DisableKeyRotationRequest,
    ) -> Result<(), RusotoError<DisableKeyRotationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.DisableKeyRotation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisableKeyRotationError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Disconnects the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> from its associated AWS CloudHSM cluster. While a custom key store is disconnected, you can manage the custom key store and its customer master keys (CMKs), but you cannot create or use CMKs in the custom key store. You can reconnect the custom key store at any time.</p> <note> <p>While a custom key store is disconnected, all attempts to create customer master keys (CMKs) in the custom key store or to use existing CMKs in <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a> will fail. This action can prevent users from storing and accessing sensitive data.</p> </note> <p/> <p>To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation. To reconnect a custom key store, use the <a>ConnectCustomKeyStore</a> operation.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DisconnectCustomKeyStore</a> (IAM policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ConnectCustomKeyStore</a> </p> </li> <li> <p> <a>CreateCustomKeyStore</a> </p> </li> <li> <p> <a>DeleteCustomKeyStore</a> </p> </li> <li> <p> <a>DescribeCustomKeyStores</a> </p> </li> <li> <p> <a>UpdateCustomKeyStore</a> </p> </li> </ul></p>
    async fn disconnect_custom_key_store(
        &self,
        input: DisconnectCustomKeyStoreRequest,
    ) -> Result<DisconnectCustomKeyStoreResponse, RusotoError<DisconnectCustomKeyStoreError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.DisconnectCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisconnectCustomKeyStoreError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisconnectCustomKeyStoreResponse, _>()
    }

    /// <p>Sets the key state of a customer master key (CMK) to enabled. This allows you to use the CMK for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a>. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:EnableKey</a> (key policy)</p> <p> <b>Related operations</b>: <a>DisableKey</a> </p>
    async fn enable_key(&self, input: EnableKeyRequest) -> Result<(), RusotoError<EnableKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.EnableKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, EnableKeyError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Enables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified symmetric customer master key (CMK).</p> <p>You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#asymmetric-cmks">asymmetric CMKs</a>, CMKs with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html#mrk-replica-key">multi-Region keys</a>, set the property on the primary key.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:EnableKeyRotation</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>DisableKeyRotation</a> </p> </li> <li> <p> <a>GetKeyRotationStatus</a> </p> </li> </ul></p>
    async fn enable_key_rotation(
        &self,
        input: EnableKeyRotationRequest,
    ) -> Result<(), RusotoError<EnableKeyRotationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.EnableKeyRotation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, EnableKeyRotationError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Encrypts plaintext into ciphertext by using a customer master key (CMK). The <code>Encrypt</code> operation has two primary use cases:</p> <ul> <li> <p>You can encrypt small amounts of arbitrary data, such as a personal identifier or database password, or other sensitive information. </p> </li> <li> <p>You can use the <code>Encrypt</code> operation to move encrypted data from one AWS Region to another. For example, in Region A, generate a data key and use the plaintext key to encrypt your data. Then, in Region A, use the <code>Encrypt</code> operation to encrypt the plaintext data key under a CMK in Region B. Now, you can move the encrypted data and the encrypted data key to Region B. When necessary, you can decrypt the encrypted data key and the encrypted data entirely within in Region B.</p> </li> </ul> <p>You don&#39;t need to use the <code>Encrypt</code> operation to encrypt a data key. The <a>GenerateDataKey</a> and <a>GenerateDataKeyPair</a> operations return a plaintext data key and an encrypted copy of that data key.</p> <p>When you encrypt data, you must specify a symmetric or asymmetric CMK to use in the encryption operation. The CMK must have a <code>KeyUsage</code> value of <code>ENCRYPT<em>DECRYPT.</code> To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation. </p> <p>If you use a symmetric CMK, you can use an encryption context to add additional security to your encryption operation. If you specify an <code>EncryptionContext</code> when encrypting data, you must specify the same encryption context (a case-sensitive exact match) when decrypting the data. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt</em>context&quot;&gt;Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>If you specify an asymmetric CMK, you must also specify the encryption algorithm. The algorithm must be compatible with the CMK type.</p> <important> <p>When you use an asymmetric CMK to encrypt or reencrypt data, be sure to record the CMK and encryption algorithm that you choose. You will be required to provide the same CMK and encryption algorithm when you decrypt the data. If the CMK and algorithm do not match the values used to encrypt the data, the decrypt operation fails.</p> <p>You are not required to supply the CMK ID and encryption algorithm when you decrypt with symmetric CMKs because AWS KMS stores this information in the ciphertext blob. AWS KMS cannot store metadata in ciphertext generated with asymmetric keys. The standard format for asymmetric key ciphertext does not include configurable fields.</p> </important> <p>The maximum size of the data that you can encrypt varies with the type of CMK and the encryption algorithm that you choose.</p> <ul> <li> <p>Symmetric CMKs</p> <ul> <li> <p> <code>SYMMETRIC<em>DEFAULT</code>: 4096 bytes</p> </li> </ul> </li> <li> <p> <code>RSA</em>2048</code> </p> <ul> <li> <p> <code>RSAES<em>OAEP</em>SHA<em>1</code>: 214 bytes</p> </li> <li> <p> <code>RSAES</em>OAEP<em>SHA</em>256</code>: 190 bytes</p> </li> </ul> </li> <li> <p> <code>RSA<em>3072</code> </p> <ul> <li> <p> <code>RSAES</em>OAEP<em>SHA</em>1</code>: 342 bytes</p> </li> <li> <p> <code>RSAES<em>OAEP</em>SHA<em>256</code>: 318 bytes</p> </li> </ul> </li> <li> <p> <code>RSA</em>4096</code> </p> <ul> <li> <p> <code>RSAES<em>OAEP</em>SHA<em>1</code>: 470 bytes</p> </li> <li> <p> <code>RSAES</em>OAEP<em>SHA</em>256</code>: 446 bytes</p> </li> </ul> </li> </ul> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:Encrypt</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> </ul></p>
    async fn encrypt(
        &self,
        input: EncryptRequest,
    ) -> Result<EncryptResponse, RusotoError<EncryptError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.Encrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, EncryptError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<EncryptResponse, _>()
    }

    /// <p><p>Generates a unique symmetric data key for client-side encryption. This operation returns a plaintext copy of the data key and a copy that is encrypted under a customer master key (CMK) that you specify. You can use the plaintext key to encrypt your data outside of AWS KMS and store the encrypted data key with the encrypted data.</p> <p> <code>GenerateDataKey</code> returns a unique data key for each request. The bytes in the plaintext key are not related to the caller or the CMK.</p> <p>To generate a data key, specify the symmetric CMK that will be used to encrypt the data key. You cannot use an asymmetric CMK to generate data keys. To get the type of your CMK, use the <a>DescribeKey</a> operation. You must also specify the length of the data key. Use either the <code>KeySpec</code> or <code>NumberOfBytes</code> parameters (but not both). For 128-bit and 256-bit data keys, use the <code>KeySpec</code> parameter. </p> <p>To get only an encrypted copy of the data key, use <a>GenerateDataKeyWithoutPlaintext</a>. To generate an asymmetric data key pair, use the <a>GenerateDataKeyPair</a> or <a>GenerateDataKeyPairWithoutPlaintext</a> operation. To get a cryptographically secure random byte string, use <a>GenerateRandom</a>.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>How to use your data key</b> </p> <p>We recommend that you use the following pattern to encrypt data locally in your application. You can write your own code or use a client-side encryption library, such as the <a href="https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/">AWS Encryption SDK</a>, the <a href="https://docs.aws.amazon.com/dynamodb-encryption-client/latest/devguide/">Amazon DynamoDB Encryption Client</a>, or <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 client-side encryption</a> to do these tasks for you.</p> <p>To encrypt data outside of AWS KMS:</p> <ol> <li> <p>Use the <code>GenerateDataKey</code> operation to get a data key.</p> </li> <li> <p>Use the plaintext data key (in the <code>Plaintext</code> field of the response) to encrypt your data outside of AWS KMS. Then erase the plaintext data key from memory.</p> </li> <li> <p>Store the encrypted data key (in the <code>CiphertextBlob</code> field of the response) with the encrypted data.</p> </li> </ol> <p>To decrypt data outside of AWS KMS:</p> <ol> <li> <p>Use the <a>Decrypt</a> operation to decrypt the encrypted data key. The operation returns a plaintext copy of the data key.</p> </li> <li> <p>Use the plaintext data key to decrypt data outside of AWS KMS, then erase the plaintext data key from memory.</p> </li> </ol> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GenerateDataKey</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>GenerateDataKeyPairWithoutPlaintext</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> </ul></p>
    async fn generate_data_key(
        &self,
        input: GenerateDataKeyRequest,
    ) -> Result<GenerateDataKeyResponse, RusotoError<GenerateDataKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.GenerateDataKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GenerateDataKeyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GenerateDataKeyResponse, _>()
    }

    /// <p><p>Generates a unique asymmetric data key pair. The <code>GenerateDataKeyPair</code> operation returns a plaintext public key, a plaintext private key, and a copy of the private key that is encrypted under the symmetric CMK you specify. You can use the data key pair to perform asymmetric cryptography outside of AWS KMS.</p> <p> <code>GenerateDataKeyPair</code> returns a unique data key pair for each request. The bytes in the keys are not related to the caller or the CMK that is used to encrypt the private key.</p> <p>You can use the public key that <code>GenerateDataKeyPair</code> returns to encrypt data or verify a signature outside of AWS KMS. Then, store the encrypted private key with the data. When you are ready to decrypt data or sign a message, you can use the <a>Decrypt</a> operation to decrypt the encrypted private key.</p> <p>To generate a data key pair, you must specify a symmetric customer master key (CMK) to encrypt the private key in a data key pair. You cannot use an asymmetric CMK or a CMK in a custom key store. To get the type and origin of your CMK, use the <a>DescribeKey</a> operation. </p> <p>If you are using the data key pair to encrypt data, or for any operation where you don&#39;t immediately need a private key, consider using the <a>GenerateDataKeyPairWithoutPlaintext</a> operation. <code>GenerateDataKeyPairWithoutPlaintext</code> returns a plaintext public key and an encrypted private key, but omits the plaintext private key that you need only to decrypt ciphertext or sign a message. Later, when you need to decrypt the data or sign a message, use the <a>Decrypt</a> operation to decrypt the encrypted private key in the data key pair.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GenerateDataKeyPair</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPairWithoutPlaintext</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> </ul></p>
    async fn generate_data_key_pair(
        &self,
        input: GenerateDataKeyPairRequest,
    ) -> Result<GenerateDataKeyPairResponse, RusotoError<GenerateDataKeyPairError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.GenerateDataKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GenerateDataKeyPairError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GenerateDataKeyPairResponse, _>()
    }

    /// <p><p>Generates a unique asymmetric data key pair. The <code>GenerateDataKeyPairWithoutPlaintext</code> operation returns a plaintext public key and a copy of the private key that is encrypted under the symmetric CMK you specify. Unlike <a>GenerateDataKeyPair</a>, this operation does not return a plaintext private key. </p> <p>To generate a data key pair, you must specify a symmetric customer master key (CMK) to encrypt the private key in the data key pair. You cannot use an asymmetric CMK or a CMK in a custom key store. To get the type and origin of your CMK, use the <code>KeySpec</code> field in the <a>DescribeKey</a> response.</p> <p>You can use the public key that <code>GenerateDataKeyPairWithoutPlaintext</code> returns to encrypt data or verify a signature outside of AWS KMS. Then, store the encrypted private key with the data. When you are ready to decrypt data or sign a message, you can use the <a>Decrypt</a> operation to decrypt the encrypted private key.</p> <p> <code>GenerateDataKeyPairWithoutPlaintext</code> returns a unique data key pair for each request. The bytes in the key are not related to the caller or CMK that is used to encrypt the private key.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GenerateDataKeyPairWithoutPlaintext</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> </ul></p>
    async fn generate_data_key_pair_without_plaintext(
        &self,
        input: GenerateDataKeyPairWithoutPlaintextRequest,
    ) -> Result<
        GenerateDataKeyPairWithoutPlaintextResponse,
        RusotoError<GenerateDataKeyPairWithoutPlaintextError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "TrentService.GenerateDataKeyPairWithoutPlaintext",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                GenerateDataKeyPairWithoutPlaintextError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GenerateDataKeyPairWithoutPlaintextResponse, _>()
    }

    /// <p><p>Generates a unique symmetric data key. This operation returns a data key that is encrypted under a customer master key (CMK) that you specify. To request an asymmetric data key pair, use the <a>GenerateDataKeyPair</a> or <a>GenerateDataKeyPairWithoutPlaintext</a> operations.</p> <p> <code>GenerateDataKeyWithoutPlaintext</code> is identical to the <a>GenerateDataKey</a> operation except that returns only the encrypted copy of the data key. This operation is useful for systems that need to encrypt data at some point, but not immediately. When you need to encrypt the data, you call the <a>Decrypt</a> operation on the encrypted copy of the key. </p> <p>It&#39;s also useful in distributed systems with different levels of trust. For example, you might store encrypted data in containers. One component of your system creates new containers and stores an encrypted data key with each container. Then, a different component puts the data into the containers. That component first decrypts the data key, uses the plaintext data key to encrypt data, puts the encrypted data into the container, and then destroys the plaintext data key. In this system, the component that creates the containers never sees the plaintext data key.</p> <p> <code>GenerateDataKeyWithoutPlaintext</code> returns a unique data key for each request. The bytes in the keys are not related to the caller or CMK that is used to encrypt the private key.</p> <p>To generate a data key, you must specify the symmetric customer master key (CMK) that is used to encrypt the data key. You cannot use an asymmetric CMK to generate a data key. To get the type of your CMK, use the <a>DescribeKey</a> operation.</p> <p>If the operation succeeds, you will find the encrypted copy of the data key in the <code>CiphertextBlob</code> field.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GenerateDataKeyWithoutPlaintext</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>GenerateDataKeyPairWithoutPlaintext</a> </p> </li> </ul></p>
    async fn generate_data_key_without_plaintext(
        &self,
        input: GenerateDataKeyWithoutPlaintextRequest,
    ) -> Result<
        GenerateDataKeyWithoutPlaintextResponse,
        RusotoError<GenerateDataKeyWithoutPlaintextError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "TrentService.GenerateDataKeyWithoutPlaintext",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GenerateDataKeyWithoutPlaintextError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GenerateDataKeyWithoutPlaintextResponse, _>()
    }

    /// <p>Returns a random byte string that is cryptographically secure.</p> <p>By default, the random byte string is generated in AWS KMS. To generate the byte string in the AWS CloudHSM cluster that is associated with a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, specify the custom key store ID.</p> <p>For more information about entropy and random number generation, see <a href="https://docs.aws.amazon.com/kms/latest/cryptographic-details/">AWS Key Management Service Cryptographic Details</a>.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GenerateRandom</a> (IAM policy)</p>
    async fn generate_random(
        &self,
        input: GenerateRandomRequest,
    ) -> Result<GenerateRandomResponse, RusotoError<GenerateRandomError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.GenerateRandom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GenerateRandomError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GenerateRandomResponse, _>()
    }

    /// <p>Gets a key policy attached to the specified customer master key (CMK).</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GetKeyPolicy</a> (key policy)</p> <p> <b>Related operations</b>: <a>PutKeyPolicy</a> </p>
    async fn get_key_policy(
        &self,
        input: GetKeyPolicyRequest,
    ) -> Result<GetKeyPolicyResponse, RusotoError<GetKeyPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.GetKeyPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetKeyPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetKeyPolicyResponse, _>()
    }

    /// <p><p>Gets a Boolean value that indicates whether <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> is enabled for the specified customer master key (CMK).</p> <p>You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#asymmetric-cmks">asymmetric CMKs</a>, CMKs with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html#mrk-replica-key">multi-Region keys</a>, set the property on the primary key. The key rotation status for these CMKs is always <code>false</code>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <ul> <li> <p>Disabled: The key rotation status does not change when you disable a CMK. However, while the CMK is disabled, AWS KMS does not rotate the backing key.</p> </li> <li> <p>Pending deletion: While a CMK is pending deletion, its key rotation status is <code>false</code> and AWS KMS does not rotate the backing key. If you cancel the deletion, the original key rotation status is restored.</p> </li> </ul> <p> <b>Cross-account use</b>: Yes. To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GetKeyRotationStatus</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>DisableKeyRotation</a> </p> </li> <li> <p> <a>EnableKeyRotation</a> </p> </li> </ul></p>
    async fn get_key_rotation_status(
        &self,
        input: GetKeyRotationStatusRequest,
    ) -> Result<GetKeyRotationStatusResponse, RusotoError<GetKeyRotationStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.GetKeyRotationStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetKeyRotationStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetKeyRotationStatusResponse, _>()
    }

    /// <p><p>Returns the items you need to import key material into a symmetric, customer managed customer master key (CMK). For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>This operation returns a public key and an import token. Use the public key to encrypt the symmetric key material. Store the import token to send with a subsequent <a>ImportKeyMaterial</a> request.</p> <p>You must specify the key ID of the symmetric CMK into which you will import key material. This CMK&#39;s <code>Origin</code> must be <code>EXTERNAL</code>. You must also specify the wrapping algorithm and type of wrapping key (public key) that you will use to encrypt the key material. You cannot perform this operation on an asymmetric CMK or on any CMK in a different AWS account.</p> <p>To import key material, you must use the public key and import token from the same response. These items are valid for 24 hours. The expiration date and time appear in the <code>GetParametersForImport</code> response. You cannot use an expired token in an <a>ImportKeyMaterial</a> request. If your key and token expire, send another <code>GetParametersForImport</code> request.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GetParametersForImport</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ImportKeyMaterial</a> </p> </li> <li> <p> <a>DeleteImportedKeyMaterial</a> </p> </li> </ul></p>
    async fn get_parameters_for_import(
        &self,
        input: GetParametersForImportRequest,
    ) -> Result<GetParametersForImportResponse, RusotoError<GetParametersForImportError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.GetParametersForImport");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetParametersForImportError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetParametersForImportResponse, _>()
    }

    /// <p>Returns the public key of an asymmetric CMK. Unlike the private key of a asymmetric CMK, which never leaves AWS KMS unencrypted, callers with <code>kms:GetPublicKey</code> permission can download the public key of an asymmetric CMK. You can share the public key to allow others to encrypt messages and verify signatures outside of AWS KMS. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You do not need to download the public key. Instead, you can use the public key within AWS KMS by calling the <a>Encrypt</a>, <a>ReEncrypt</a>, or <a>Verify</a> operations with the identifier of an asymmetric CMK. When you use the public key within AWS KMS, you benefit from the authentication, authorization, and logging that are part of every AWS KMS operation. You also reduce of risk of encrypting data that cannot be decrypted. These features are not effective outside of AWS KMS. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/download-public-key.html#download-public-key-considerations">Special Considerations for Downloading Public Keys</a>.</p> <p>To help you use the public key safely outside of AWS KMS, <code>GetPublicKey</code> returns important information about the public key in the response, including:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-CustomerMasterKeySpec">CustomerMasterKeySpec</a>: The type of key material in the public key, such as <code>RSA_4096</code> or <code>ECC_NIST_P521</code>.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-KeyUsage">KeyUsage</a>: Whether the key is used for encryption or signing.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-EncryptionAlgorithms">EncryptionAlgorithms</a> or <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-SigningAlgorithms">SigningAlgorithms</a>: A list of the encryption algorithms or the signing algorithms for the key.</p> </li> </ul> <p>Although AWS KMS cannot enforce these restrictions on external operations, it is crucial that you use this information to prevent the public key from being used improperly. For example, you can prevent a public signing key from being used encrypt data, or prevent a public key from being used with an encryption algorithm that is not supported by AWS KMS. You can also avoid errors, such as using the wrong signing algorithm in a verification operation.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GetPublicKey</a> (key policy)</p> <p> <b>Related operations</b>: <a>CreateKey</a> </p>
    async fn get_public_key(
        &self,
        input: GetPublicKeyRequest,
    ) -> Result<GetPublicKeyResponse, RusotoError<GetPublicKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.GetPublicKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetPublicKeyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetPublicKeyResponse, _>()
    }

    /// <p><p>Imports key material into an existing symmetric AWS KMS customer master key (CMK) that was created without key material. After you successfully import key material into a CMK, you can <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#reimport-key-material">reimport the same key material</a> into that CMK, but you cannot import different key material. </p> <p>You cannot perform this operation on an asymmetric CMK or on any CMK in a different AWS account. For more information about creating CMKs with no key material and then importing key material, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Before using this operation, call <a>GetParametersForImport</a>. Its response includes a public key and an import token. Use the public key to encrypt the key material. Then, submit the import token from the same <code>GetParametersForImport</code> response.</p> <p>When calling this operation, you must specify the following values:</p> <ul> <li> <p>The key ID or key ARN of a CMK with no key material. Its <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>To create a CMK with no key material, call <a>CreateKey</a> and set the value of its <code>Origin</code> parameter to <code>EXTERNAL</code>. To get the <code>Origin</code> of a CMK, call <a>DescribeKey</a>.)</p> </li> <li> <p>The encrypted key material. To get the public key to encrypt the key material, call <a>GetParametersForImport</a>.</p> </li> <li> <p>The import token that <a>GetParametersForImport</a> returned. You must use a public key and token from the same <code>GetParametersForImport</code> response.</p> </li> <li> <p>Whether the key material expires and if so, when. If you set an expiration date, AWS KMS deletes the key material from the CMK on the specified date, and the CMK becomes unusable. To use the CMK again, you must reimport the same key material. The only way to change an expiration date is by reimporting the same key material and specifying a new expiration date. </p> </li> </ul> <p>When this operation is successful, the key state of the CMK changes from <code>PendingImport</code> to <code>Enabled</code>, and you can use the CMK.</p> <p>If this operation fails, use the exception to help determine the problem. If the error is related to the key material, the import token, or wrapping key, use <a>GetParametersForImport</a> to get a new public key and import token for the CMK and repeat the import procedure. For help, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#importing-keys-overview">How To Import Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ImportKeyMaterial</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>DeleteImportedKeyMaterial</a> </p> </li> <li> <p> <a>GetParametersForImport</a> </p> </li> </ul></p>
    async fn import_key_material(
        &self,
        input: ImportKeyMaterialRequest,
    ) -> Result<ImportKeyMaterialResponse, RusotoError<ImportKeyMaterialError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.ImportKeyMaterial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ImportKeyMaterialError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ImportKeyMaterialResponse, _>()
    }

    /// <p><p>Gets a list of aliases in the caller&#39;s AWS account and region. For more information about aliases, see <a>CreateAlias</a>.</p> <p>By default, the <code>ListAliases</code> operation returns all aliases in the account and region. To get only the aliases associated with a particular customer master key (CMK), use the <code>KeyId</code> parameter.</p> <p>The <code>ListAliases</code> response can include aliases that you created and associated with your customer managed CMKs, and aliases that AWS created and associated with AWS managed CMKs in your account. You can recognize AWS aliases because their names have the format <code>aws/&lt;service-name&gt;</code>, such as <code>aws/dynamodb</code>.</p> <p>The response might also include aliases that have no <code>TargetKeyId</code> field. These are predefined aliases that AWS has created but has not yet associated with a CMK. Aliases that AWS creates in your account, including predefined aliases, do not count against your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html#aliases-limit">AWS KMS aliases quota</a>.</p> <p> <b>Cross-account use</b>: No. <code>ListAliases</code> does not return aliases in other AWS accounts.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListAliases</a> (IAM policy)</p> <p>For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html#alias-access">Controlling access to aliases</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> </ul></p>
    async fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> Result<ListAliasesResponse, RusotoError<ListAliasesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.ListAliases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAliasesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAliasesResponse, _>()
    }

    /// <p><p>Gets a list of all grants for the specified customer master key (CMK). </p> <p>You must specify the CMK in all requests. You can filter the grant list by grant ID or grantee principal.</p> <note> <p>The <code>GranteePrincipal</code> field in the <code>ListGrants</code> response usually contains the user or role designated as the grantee principal in the grant. However, when the grantee principal in the grant is an AWS service, the <code>GranteePrincipal</code> field contains the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#principal-services">service principal</a>, which might represent several different grantee principals.</p> </note> <p> <b>Cross-account use</b>: Yes. To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListGrants</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateGrant</a> </p> </li> <li> <p> <a>ListRetirableGrants</a> </p> </li> <li> <p> <a>RetireGrant</a> </p> </li> <li> <p> <a>RevokeGrant</a> </p> </li> </ul></p>
    async fn list_grants(
        &self,
        input: ListGrantsRequest,
    ) -> Result<ListGrantsResponse, RusotoError<ListGrantsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.ListGrants");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListGrantsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListGrantsResponse, _>()
    }

    /// <p><p>Gets the names of the key policies that are attached to a customer master key (CMK). This operation is designed to get policy names that you can use in a <a>GetKeyPolicy</a> operation. However, the only valid policy name is <code>default</code>. </p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListKeyPolicies</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>GetKeyPolicy</a> </p> </li> <li> <p> <a>PutKeyPolicy</a> </p> </li> </ul></p>
    async fn list_key_policies(
        &self,
        input: ListKeyPoliciesRequest,
    ) -> Result<ListKeyPoliciesResponse, RusotoError<ListKeyPoliciesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.ListKeyPolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListKeyPoliciesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListKeyPoliciesResponse, _>()
    }

    /// <p><p>Gets a list of all customer master keys (CMKs) in the caller&#39;s AWS account and Region.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListKeys</a> (IAM policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>DescribeKey</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>ListResourceTags</a> </p> </li> </ul></p>
    async fn list_keys(
        &self,
        input: ListKeysRequest,
    ) -> Result<ListKeysResponse, RusotoError<ListKeysError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.ListKeys");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListKeysError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListKeysResponse, _>()
    }

    /// <p><p>Returns all tags on the specified customer master key (CMK).</p> <p>For general information about tags, including the format and syntax, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS resources</a> in the <i>Amazon Web Services General Reference</i>. For information about using tags in AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging keys</a>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListResourceTags</a> (key policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>ReplicateKey</a> </p> </li> <li> <p> <a>TagResource</a> </p> </li> <li> <p> <a>UntagResource</a> </p> </li> </ul></p>
    async fn list_resource_tags(
        &self,
        input: ListResourceTagsRequest,
    ) -> Result<ListResourceTagsResponse, RusotoError<ListResourceTagsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.ListResourceTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListResourceTagsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListResourceTagsResponse, _>()
    }

    /// <p><p>Returns information about all grants in the AWS account and Region that have the specified retiring principal. For more information about grants, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>You can specify any principal in your AWS account. The grants that are returned include grants for CMKs in your AWS account and other AWS accounts.</p> <p>You might use this operation to determine which grants you may retire. To retire a grant, use the <a>RetireGrant</a> operation.</p> <p> <b>Cross-account use</b>: You must specify a principal in your AWS account. However, this operation can return grants in any AWS account. You do not need <code>kms:ListRetirableGrants</code> permission (or any other additional permission) in any AWS account other than your own.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ListRetirableGrants</a> (IAM policy) in your AWS account.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateGrant</a> </p> </li> <li> <p> <a>ListGrants</a> </p> </li> <li> <p> <a>RetireGrant</a> </p> </li> <li> <p> <a>RevokeGrant</a> </p> </li> </ul></p>
    async fn list_retirable_grants(
        &self,
        input: ListRetirableGrantsRequest,
    ) -> Result<ListGrantsResponse, RusotoError<ListRetirableGrantsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.ListRetirableGrants");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRetirableGrantsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListGrantsResponse, _>()
    }

    /// <p>Attaches a key policy to the specified customer master key (CMK). </p> <p>For more information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key Policies</a> in the <i>AWS Key Management Service Developer Guide</i>. For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>IAM User Guide</i> </i>. For examples of adding a key policy in multiple programming languages, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/programming-key-policies.html#put-policy">Setting a key policy</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:PutKeyPolicy</a> (key policy)</p> <p> <b>Related operations</b>: <a>GetKeyPolicy</a> </p>
    async fn put_key_policy(
        &self,
        input: PutKeyPolicyRequest,
    ) -> Result<(), RusotoError<PutKeyPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.PutKeyPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutKeyPolicyError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Decrypts ciphertext and then reencrypts it entirely within AWS KMS. You can use this operation to change the customer master key (CMK) under which data is encrypted, such as when you <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotate-keys-manually">manually rotate</a> a CMK or change the CMK that protects a ciphertext. You can also use it to reencrypt ciphertext under the same CMK, such as to change the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">encryption context</a> of a ciphertext.</p> <p>The <code>ReEncrypt</code> operation can decrypt ciphertext that was encrypted by using an AWS KMS CMK in an AWS KMS operation, such as <a>Encrypt</a> or <a>GenerateDataKey</a>. It can also decrypt ciphertext that was encrypted by using the public key of an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html#asymmetric-cmks">asymmetric CMK</a> outside of AWS KMS. However, it cannot decrypt ciphertext produced by other libraries, such as the <a href="https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/">AWS Encryption SDK</a> or <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 client-side encryption</a>. These libraries return a ciphertext format that is incompatible with AWS KMS.</p> <p>When you use the <code>ReEncrypt</code> operation, you need to provide information for the decrypt operation and the subsequent encrypt operation.</p> <ul> <li> <p>If your ciphertext was encrypted under an asymmetric CMK, you must use the <code>SourceKeyId</code> parameter to identify the CMK that encrypted the ciphertext. You must also supply the encryption algorithm that was used. This information is required to decrypt the data.</p> </li> <li> <p>If your ciphertext was encrypted under a symmetric CMK, the <code>SourceKeyId</code> parameter is optional. AWS KMS can get this information from metadata that it adds to the symmetric ciphertext blob. This feature adds durability to your implementation by ensuring that authorized users can decrypt ciphertext decades after it was encrypted, even if they&#39;ve lost track of the CMK ID. However, specifying the source CMK is always recommended as a best practice. When you use the <code>SourceKeyId</code> parameter to specify a CMK, AWS KMS uses only the CMK you specify. If the ciphertext was encrypted under a different CMK, the <code>ReEncrypt</code> operation fails. This practice ensures that you use the CMK that you intend.</p> </li> <li> <p>To reencrypt the data, you must use the <code>DestinationKeyId</code> parameter specify the CMK that re-encrypts the data after it is decrypted. You can select a symmetric or asymmetric CMK. If the destination CMK is an asymmetric CMK, you must also provide the encryption algorithm. The algorithm that you choose must be compatible with the CMK.</p> <important> <p>When you use an asymmetric CMK to encrypt or reencrypt data, be sure to record the CMK and encryption algorithm that you choose. You will be required to provide the same CMK and encryption algorithm when you decrypt the data. If the CMK and algorithm do not match the values used to encrypt the data, the decrypt operation fails.</p> <p>You are not required to supply the CMK ID and encryption algorithm when you decrypt with symmetric CMKs because AWS KMS stores this information in the ciphertext blob. AWS KMS cannot store metadata in ciphertext generated with asymmetric keys. The standard format for asymmetric key ciphertext does not include configurable fields.</p> </important> </li> </ul> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. The source CMK and destination CMK can be in different AWS accounts. Either or both CMKs can be in a different account than the caller.</p> <p> <b>Required permissions</b>:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ReEncryptFrom</a> permission on the source CMK (key policy)</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ReEncryptTo</a> permission on the destination CMK (key policy)</p> </li> </ul> <p>To permit reencryption from or to a CMK, include the <code>&quot;kms:ReEncrypt*&quot;</code> permission in your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">key policy</a>. This permission is automatically included in the key policy when you use the console to create a CMK. But you must include it manually when you create a CMK programmatically or when you use the <a>PutKeyPolicy</a> operation to set a key policy.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> </ul></p>
    async fn re_encrypt(
        &self,
        input: ReEncryptRequest,
    ) -> Result<ReEncryptResponse, RusotoError<ReEncryptError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.ReEncrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ReEncryptError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ReEncryptResponse, _>()
    }

    /// <p><p>Replicates a multi-Region key into the specified Region. This operation creates a multi-Region replica key based on a multi-Region primary key in a different Region of the same AWS partition. You can create multiple replicas of a primary key, but each must be in a different Region. To create a multi-Region primary key, use the <a>CreateKey</a> operation.</p> <p>This operation supports <i>multi-Region keys</i>, an AWS KMS feature that lets you create multiple interoperable CMKs in different AWS Regions. Because these CMKs have the same key ID, key material, and other metadata, you can use them to encrypt data in one AWS Region and decrypt it in a different AWS Region without making a cross-Region call or exposing the plaintext data. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>A <i>replica key</i> is a fully-functional CMK that can be used independently of its primary and peer replica keys. A primary key and its replica keys share properties that make them interoperable. They have the same <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-id">key ID</a> and key material. They also have the same <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-spec">key spec</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-usage">key usage</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-origin">key material origin</a>, and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic key rotation status</a>. AWS KMS automatically synchronizes these shared properties among related multi-Region keys. All other properties of a replica key can differ, including its <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">key policy</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">tags</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html">aliases</a>, and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">key state</a>. AWS KMS pricing and quotas for CMKs apply to each primary key and replica key.</p> <p>When this operation completes, the new replica key has a transient key state of <code>Creating</code>. This key state changes to <code>Enabled</code> (or <code>PendingImport</code>) after a few seconds when the process of creating the new replica key is complete. While the key state is <code>Creating</code>, you can manage key, but you cannot yet use it in cryptographic operations. If you are creating and using the replica key programmatically, retry on <code>KMSInvalidStateException</code> or call <code>DescribeKey</code> to check its <code>KeyState</code> value before using it. For details about the <code>Creating</code> key state, see <a href="kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The AWS CloudTrail log of a <code>ReplicateKey</code> operation records a <code>ReplicateKey</code> operation in the primary key&#39;s Region and a <a>CreateKey</a> operation in the replica key&#39;s Region.</p> <p>If you replicate a multi-Region primary key with imported key material, the replica key is created with no key material. You must import the same key material that you imported into the primary key. For details, see <a href="kms/latest/developerguide/multi-region-keys-import.html">Importing key material into multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To convert a replica key to a primary key, use the <a>UpdatePrimaryRegion</a> operation.</p> <note> <p> <code>ReplicateKey</code> uses different default values for the <code>KeyPolicy</code> and <code>Tags</code> parameters than those used in the AWS KMS console. For details, see the parameter descriptions.</p> </note> <p> <b>Cross-account use</b>: No. You cannot use this operation to create a CMK in a different AWS account. </p> <p> <b>Required permissions</b>: </p> <ul> <li> <p> <code>kms:ReplicateKey</code> on the primary CMK (in the primary CMK&#39;s Region). Include this permission in the primary CMK&#39;s key policy.</p> </li> <li> <p> <code>kms:CreateKey</code> in an IAM policy in the replica Region.</p> </li> <li> <p>To use the <code>Tags</code> parameter, <code>kms:TagResource</code> in an IAM policy in the replica Region.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>UpdatePrimaryRegion</a> </p> </li> </ul></p>
    async fn replicate_key(
        &self,
        input: ReplicateKeyRequest,
    ) -> Result<ReplicateKeyResponse, RusotoError<ReplicateKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.ReplicateKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ReplicateKeyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ReplicateKeyResponse, _>()
    }

    /// <p><p>Deletes a grant. Typically, you retire a grant when you no longer need its permissions. To identify the grant to retire, use a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">grant token</a>, or both the grant ID and a key identifier (key ID or key ARN) of the customer master key (CMK). The <a>CreateGrant</a> operation returns both values.</p> <p>This operation can be called by the <i>retiring principal</i> for a grant, by the <i>grantee principal</i> if the grant allows the <code>RetireGrant</code> operation, and by the AWS account (root user) in which the grant is created. It can also be called by principals to whom permission for retiring a grant is delegated. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#grant-delete">Retiring and revoking grants</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>For detailed information about grants, including grant terminology, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Using grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>. For examples of working with grants in several programming languages, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/programming-grants.html">Programming grants</a>.</p> <p> <b>Cross-account use</b>: Yes. You can retire a grant on a CMK in a different AWS account.</p> <p> <b>Required permissions:</b>:Permission to retire a grant is determined primarily by the grant. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#grant-delete">Retiring and revoking grants</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateGrant</a> </p> </li> <li> <p> <a>ListGrants</a> </p> </li> <li> <p> <a>ListRetirableGrants</a> </p> </li> <li> <p> <a>RevokeGrant</a> </p> </li> </ul></p>
    async fn retire_grant(
        &self,
        input: RetireGrantRequest,
    ) -> Result<(), RusotoError<RetireGrantError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.RetireGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RetireGrantError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes the specified grant. You revoke a grant to terminate the permissions that the grant allows. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/managing-grants.html#grant-delete">Retiring and revoking grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>When you create, retire, or revoke a grant, there might be a brief delay, usually less than five minutes, until the grant is available throughout AWS KMS. This state is known as <i>eventual consistency</i>. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-eventual-consistency">Eventual consistency</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>. </p> <p> <b>Cross-account use</b>: Yes. To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:RevokeGrant</a> (key policy).</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateGrant</a> </p> </li> <li> <p> <a>ListGrants</a> </p> </li> <li> <p> <a>ListRetirableGrants</a> </p> </li> <li> <p> <a>RetireGrant</a> </p> </li> </ul></p>
    async fn revoke_grant(
        &self,
        input: RevokeGrantRequest,
    ) -> Result<(), RusotoError<RevokeGrantError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.RevokeGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RevokeGrantError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Schedules the deletion of a customer master key (CMK). By default, AWS KMS applies a waiting period of 30 days, but you can specify a waiting period of 7-30 days. When this operation is successful, the key state of the CMK changes to <code>PendingDeletion</code> and the key can&#39;t be used in any cryptographic operations. It remains in this state for the duration of the waiting period. Before the waiting period ends, you can use <a>CancelKeyDeletion</a> to cancel the deletion of the CMK. After the waiting period ends, AWS KMS deletes the CMK, its key material, and all AWS KMS data associated with it, including all aliases that refer to it.</p> <important> <p>Deleting a CMK is a destructive and potentially dangerous operation. When a CMK is deleted, all data that was encrypted under the CMK is unrecoverable. (The only exception is a multi-Region replica key.) To prevent the use of a CMK without deleting it, use <a>DisableKey</a>. </p> </important> <p>If you schedule deletion of a CMK from a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, when the waiting period expires, <code>ScheduleKeyDeletion</code> deletes the CMK from AWS KMS. Then AWS KMS makes a best effort to delete the key material from the associated AWS CloudHSM cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>You can schedule the deletion of a multi-Region primary key and its replica keys at any time. However, AWS KMS will not delete a multi-Region primary key with existing replica keys. If you schedule the deletion of a primary key with replicas, its key state changes to <code>PendingReplicaDeletion</code> and it cannot be replicated or used in cryptographic operations. This status can continue indefinitely. When the last of its replicas keys is deleted (not just scheduled), the key state of the primary key changes to <code>PendingDeletion</code> and its waiting period (<code>PendingWindowInDays</code>) begins. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-delete.html">Deleting multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>. </p> <p>For more information about scheduling a CMK for deletion, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: kms:ScheduleKeyDeletion (key policy)</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CancelKeyDeletion</a> </p> </li> <li> <p> <a>DisableKey</a> </p> </li> </ul></p>
    async fn schedule_key_deletion(
        &self,
        input: ScheduleKeyDeletionRequest,
    ) -> Result<ScheduleKeyDeletionResponse, RusotoError<ScheduleKeyDeletionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.ScheduleKeyDeletion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ScheduleKeyDeletionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ScheduleKeyDeletionResponse, _>()
    }

    /// <p>Creates a <a href="https://en.wikipedia.org/wiki/Digital_signature">digital signature</a> for a message or message digest by using the private key in an asymmetric CMK. To verify the signature, use the <a>Verify</a> operation, or use the public key in the same asymmetric CMK outside of AWS KMS. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Digital signatures are generated and verified by using asymmetric key pair, such as an RSA or ECC pair that is represented by an asymmetric customer master key (CMK). The key owner (or an authorized user) uses their private key to sign a message. Anyone with the public key can verify that the message was signed with that particular private key and that the message hasn't changed since it was signed. </p> <p>To use the <code>Sign</code> operation, provide the following information:</p> <ul> <li> <p>Use the <code>KeyId</code> parameter to identify an asymmetric CMK with a <code>KeyUsage</code> value of <code>SIGN_VERIFY</code>. To get the <code>KeyUsage</code> value of a CMK, use the <a>DescribeKey</a> operation. The caller must have <code>kms:Sign</code> permission on the CMK.</p> </li> <li> <p>Use the <code>Message</code> parameter to specify the message or message digest to sign. You can submit messages of up to 4096 bytes. To sign a larger message, generate a hash digest of the message, and then provide the hash digest in the <code>Message</code> parameter. To indicate whether the message is a full message or a digest, use the <code>MessageType</code> parameter.</p> </li> <li> <p>Choose a signing algorithm that is compatible with the CMK. </p> </li> </ul> <important> <p>When signing a message, be sure to record the CMK and the signing algorithm. This information is required to verify the signature.</p> </important> <p>To verify the signature that this operation generates, use the <a>Verify</a> operation. Or use the <a>GetPublicKey</a> operation to download the public key and then use the public key to verify the signature outside of AWS KMS. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:Sign</a> (key policy)</p> <p> <b>Related operations</b>: <a>Verify</a> </p>
    async fn sign(&self, input: SignRequest) -> Result<SignResponse, RusotoError<SignError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.Sign");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SignError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<SignResponse, _>()
    }

    /// <p><p>Adds or edits tags on a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a>.</p> <note> <p>Tagging or untagging a CMK can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>Each tag consists of a tag key and a tag value, both of which are case-sensitive strings. The tag value can be an empty (null) string. To add a tag, specify a new tag key and a tag value. To edit a tag, specify an existing tag key and a new tag value.</p> <p>You can use this operation to tag a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a>, but you cannot tag an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMK</a>, an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-owned-cmk">AWS owned CMK</a>, a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#keystore-concept">custom key store</a>, or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#alias-concept">alias</a>.</p> <p>You can also add tags to a CMK while creating it (<a>CreateKey</a>) or replicating it (<a>ReplicateKey</a>).</p> <p>For information about using tags in AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging keys</a>. For general information about tags, including the format and syntax, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS resources</a> in the <i>Amazon Web Services General Reference</i>. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account. </p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> (key policy)</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>ListResourceTags</a> </p> </li> <li> <p> <a>ReplicateKey</a> </p> </li> <li> <p> <a>UntagResource</a> </p> </li> </ul></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes tags from a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a>. To delete a tag, specify the tag key and the CMK.</p> <note> <p>Tagging or untagging a CMK can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>When it succeeds, the <code>UntagResource</code> operation doesn&#39;t return any output. Also, if the specified tag key isn&#39;t found on the CMK, it doesn&#39;t throw an exception or return a response. To confirm that the operation worked, use the <a>ListResourceTags</a> operation.</p> <p>For information about using tags in AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging keys</a>. For general information about tags, including the format and syntax, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS resources</a> in the <i>Amazon Web Services General Reference</i>. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account.</p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UntagResource</a> (key policy)</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>ListResourceTags</a> </p> </li> <li> <p> <a>ReplicateKey</a> </p> </li> <li> <p> <a>TagResource</a> </p> </li> </ul></p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Associates an existing AWS KMS alias with a different customer master key (CMK). Each alias is associated with only one CMK at a time, although a CMK can have multiple aliases. The alias and the CMK must be in the same AWS account and Region.</p> <note> <p>Adding, deleting, or updating an alias can allow or deny permission to the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">Using ABAC in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> </note> <p>The current and new CMK must be the same type (both symmetric or both asymmetric), and they must have the same key usage (<code>ENCRYPT<em>DECRYPT</code> or <code>SIGN</em>VERIFY</code>). This restriction prevents errors in code that uses aliases. If you must assign an alias to a different type of CMK, use <a>DeleteAlias</a> to delete the old alias and <a>CreateAlias</a> to create a new alias.</p> <p>You cannot use <code>UpdateAlias</code> to change an alias name. To change an alias name, use <a>DeleteAlias</a> to delete the old alias and <a>CreateAlias</a> to create a new alias.</p> <p>Because an alias is not a property of a CMK, you can create, update, and delete the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs in the account, use the <a>ListAliases</a> operation. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account. </p> <p> <b>Required permissions</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UpdateAlias</a> on the alias (IAM policy).</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UpdateAlias</a> on the current CMK (key policy).</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UpdateAlias</a> on the new CMK (key policy).</p> </li> </ul> <p>For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html#alias-access">Controlling access to aliases</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> </ul></p>
    async fn update_alias(
        &self,
        input: UpdateAliasRequest,
    ) -> Result<(), RusotoError<UpdateAliasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.UpdateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateAliasError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Changes the properties of a custom key store. Use the <code>CustomKeyStoreId</code> parameter to identify the custom key store you want to edit. Use the remaining parameters to change the properties of the custom key store.</p> <p>You can only update a custom key store that is disconnected. To disconnect the custom key store, use <a>DisconnectCustomKeyStore</a>. To reconnect the custom key store after the update completes, use <a>ConnectCustomKeyStore</a>. To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>Use the parameters of <code>UpdateCustomKeyStore</code> to edit your keystore settings.</p> <ul> <li> <p>Use the <b>NewCustomKeyStoreName</b> parameter to change the friendly name of the custom key store to the value that you specify.</p> <p> </p> </li> <li> <p>Use the <b>KeyStorePassword</b> parameter tell AWS KMS the current password of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user (CU)</a> in the associated AWS CloudHSM cluster. You can use this parameter to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-password">fix connection failures</a> that occur when AWS KMS cannot log into the associated cluster because the <code>kmsuser</code> password has changed. This value does not change the password in the AWS CloudHSM cluster.</p> <p> </p> </li> <li> <p>Use the <b>CloudHsmClusterId</b> parameter to associate the custom key store with a different, but related, AWS CloudHSM cluster. You can use this parameter to repair a custom key store if its AWS CloudHSM cluster becomes corrupted or is deleted, or when you need to create or restore a cluster from a backup. </p> </li> </ul> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a custom key store in a different AWS account. </p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UpdateCustomKeyStore</a> (IAM policy)</p> <p> <b>Related operations:</b> </p> <ul> <li> <p> <a>ConnectCustomKeyStore</a> </p> </li> <li> <p> <a>CreateCustomKeyStore</a> </p> </li> <li> <p> <a>DeleteCustomKeyStore</a> </p> </li> <li> <p> <a>DescribeCustomKeyStores</a> </p> </li> <li> <p> <a>DisconnectCustomKeyStore</a> </p> </li> </ul></p>
    async fn update_custom_key_store(
        &self,
        input: UpdateCustomKeyStoreRequest,
    ) -> Result<UpdateCustomKeyStoreResponse, RusotoError<UpdateCustomKeyStoreError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.UpdateCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateCustomKeyStoreError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateCustomKeyStoreResponse, _>()
    }

    /// <p><p>Updates the description of a customer master key (CMK). To see the description of a CMK, use <a>DescribeKey</a>. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: No. You cannot perform this operation on a CMK in a different AWS account. </p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:UpdateKeyDescription</a> (key policy)</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>DescribeKey</a> </p> </li> </ul></p>
    async fn update_key_description(
        &self,
        input: UpdateKeyDescriptionRequest,
    ) -> Result<(), RusotoError<UpdateKeyDescriptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.UpdateKeyDescription");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateKeyDescriptionError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Changes the primary key of a multi-Region key. </p> <p>This operation changes the replica key in the specified Region to a primary key and changes the former primary key to a replica key. For example, suppose you have a primary key in <code>us-east-1</code> and a replica key in <code>eu-west-2</code>. If you run <code>UpdatePrimaryRegion</code> with a <code>PrimaryRegion</code> value of <code>eu-west-2</code>, the primary key is now the key in <code>eu-west-2</code>, and the key in <code>us-east-1</code> becomes a replica key. For details, see </p> <p>This operation supports <i>multi-Region keys</i>, an AWS KMS feature that lets you create multiple interoperable CMKs in different AWS Regions. Because these CMKs have the same key ID, key material, and other metadata, you can use them to encrypt data in one AWS Region and decrypt it in a different AWS Region without making a cross-Region call or exposing the plaintext data. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The <i>primary key</i> of a multi-Region key is the source for properties that are always shared by primary and replica keys, including the key material, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-id">key ID</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-spec">key spec</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-usage">key usage</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-origin">key material origin</a>, and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic key rotation</a>. It&#39;s the only key that can be replicated. You cannot <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_ScheduleKeyDeletion.html">delete the primary key</a> until all replicas are deleted.</p> <p>The key ID and primary Region that you specify uniquely identify the replica key that will become the primary key. The primary Region must already have a replica key. This operation does not create a CMK in the specified Region. To find the replica keys, use the <a>DescribeKey</a> operation on the primary key or any replica key. To create a replica key, use the <a>ReplicateKey</a> operation.</p> <p>You can run this operation while using the affected multi-Region keys in cryptographic operations. This operation should not delay, interrupt, or cause failures in cryptographic operations. </p> <p>Even after this operation completes, the process of updating the primary Region might still be in progress for a few more seconds. Operations such as <code>DescribeKey</code> might display both the old and new primary keys as replicas. The old and new primary keys have a transient key state of <code>Updating</code>. The original key state is restored when the update is complete. While the key state is <code>Updating</code>, you can use the keys in cryptographic operations, but you cannot replicate the new primary key or perform certain management operations, such as enabling or disabling these keys. For details about the <code>Updating</code> key state, see <a href="kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>This operation does not return any output. To verify that primary key is changed, use the <a>DescribeKey</a> operation.</p> <p> <b>Cross-account use</b>: No. You cannot use this operation in a different AWS account. </p> <p> <b>Required permissions</b>: </p> <ul> <li> <p> <code>kms:UpdatePrimaryRegion</code> on the current primary CMK (in the primary CMK&#39;s Region). Include this permission primary CMK&#39;s key policy.</p> </li> <li> <p> <code>kms:UpdatePrimaryRegion</code> on the current replica CMK (in the replica CMK&#39;s Region). Include this permission in the replica CMK&#39;s key policy.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateKey</a> </p> </li> <li> <p> <a>ReplicateKey</a> </p> </li> </ul></p>
    async fn update_primary_region(
        &self,
        input: UpdatePrimaryRegionRequest,
    ) -> Result<(), RusotoError<UpdatePrimaryRegionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.UpdatePrimaryRegion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdatePrimaryRegionError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Verifies a digital signature that was generated by the <a>Sign</a> operation. </p> <p/> <p>Verification confirms that an authorized user signed the message with the specified CMK and signing algorithm, and the message hasn't changed since it was signed. If the signature is verified, the value of the <code>SignatureValid</code> field in the response is <code>True</code>. If the signature verification fails, the <code>Verify</code> operation fails with an <code>KMSInvalidSignatureException</code> exception.</p> <p>A digital signature is generated by using the private key in an asymmetric CMK. The signature is verified by using the public key in the same asymmetric CMK. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To verify a digital signature, you can use the <code>Verify</code> operation. Specify the same asymmetric CMK, message, and signing algorithm that were used to produce the signature.</p> <p>You can also verify the digital signature by using the public key of the CMK outside of AWS KMS. Use the <a>GetPublicKey</a> operation to download the public key in the asymmetric CMK and then use the public key to verify the signature outside of AWS KMS. The advantage of using the <code>Verify</code> operation is that it is performed within AWS KMS. As a result, it's easy to call, the operation is performed within the FIPS boundary, it is logged in AWS CloudTrail, and you can use key policy and IAM policy to determine who is authorized to use the CMK to verify signatures.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key state: Effect on your CMK</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p> <b>Cross-account use</b>: Yes. To perform this operation with a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter. </p> <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:Verify</a> (key policy)</p> <p> <b>Related operations</b>: <a>Sign</a> </p>
    async fn verify(
        &self,
        input: VerifyRequest,
    ) -> Result<VerifyResponse, RusotoError<VerifyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TrentService.Verify");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, VerifyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<VerifyResponse, _>()
    }
}
