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

impl ComprehendMedicalClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "comprehendmedical", &self.region, request_uri);

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
/// <p> An extracted segment of the text that is an attribute of an entity, or otherwise related to an entity, such as the dosage of a medication taken. It contains information about the attribute such as id, begin and end offset within the input text, and the segment of the input text. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Attribute {
    /// <p> The 0-based character offset in the input text that shows where the attribute begins. The offset returns the UTF-8 code point in the string. </p>
    #[serde(rename = "beginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p> The category of attribute. </p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p> The 0-based character offset in the input text that shows where the attribute ends. The offset returns the UTF-8 code point in the string.</p>
    #[serde(rename = "endOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p> The numeric identifier for this attribute. This is a monotonically increasing id unique within this response rather than a global unique identifier. </p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p> The level of confidence that Amazon Comprehend Medical has that this attribute is correctly related to this entity. </p>
    #[serde(rename = "relationshipScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_score: Option<f32>,
    /// <p>The type of relationship between the entity and attribute. Type for the relationship is <code>OVERLAP</code>, indicating that the entity occurred at the same time as the <code>Date_Expression</code>. </p>
    #[serde(rename = "relationshipType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    /// <p> The level of confidence that Amazon Comprehend Medical has that the segment of text is correctly recognized as an attribute. </p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p> The segment of input text extracted as this attribute.</p>
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p> Contextual information for this attribute. </p>
    #[serde(rename = "traits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<Trait>>,
    /// <p> The type of attribute. </p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Provides information for filtering a list of detection jobs.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ComprehendMedicalAsyncJobFilter {
    /// <p>Filters on the name of the job.</p>
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Filters the list of jobs based on job status. Returns only jobs with the specified status.</p>
    #[serde(rename = "jobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted after the specified time. Jobs are returned in descending order, newest to oldest.</p>
    #[serde(rename = "submitTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted before the specified time. Jobs are returned in ascending order, oldest to newest.</p>
    #[serde(rename = "submitTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

/// <p>Provides information about a detection job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComprehendMedicalAsyncJobProperties {
    /// <p>The Amazon Resource Name (ARN) that gives Amazon Comprehend Medical read access to your input data.</p>
    #[serde(rename = "dataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    /// <p>The time that the detection job completed.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The date and time that job metadata is deleted from the server. Output files in your S3 bucket will not be deleted. After the metadata is deleted, the job will no longer appear in the results of the <code>ListEntitiesDetectionV2Job</code> or the <code>ListPHIDetectionJobs</code> operation.</p>
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    /// <p>The input data configuration that you supplied when you created the detection job.</p>
    #[serde(rename = "inputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>The identifier assigned to the detection job.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The name that you assigned to the detection job.</p>
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The current status of the detection job. If the status is <code>FAILED</code>, the <code>Message</code> field shows the reason for the failure.</p>
    #[serde(rename = "jobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The AWS Key Management Service key, if any, used to encrypt the output files. </p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The language code of the input documents.</p>
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The path to the file that describes the results of a batch job.</p>
    #[serde(rename = "manifestFilePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_file_path: Option<String>,
    /// <p>A description of the status of a job.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The version of the model used to analyze the documents. The version number looks like X.X.X. You can use this information to track the model used for a particular batch of documents.</p>
    #[serde(rename = "modelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    /// <p>The output data configuration that you supplied when you created the detection job.</p>
    #[serde(rename = "outputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    /// <p>The time that the detection job was submitted for processing.</p>
    #[serde(rename = "submitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEntitiesDetectionV2JobRequest {
    /// <p>The identifier that Amazon Comprehend Medical generated for the job. The <code>StartEntitiesDetectionV2Job</code> operation returns this identifier in its response.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEntitiesDetectionV2JobResponse {
    /// <p>An object that contains the properties associated with a detection job.</p>
    #[serde(rename = "comprehendMedicalAsyncJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties: Option<ComprehendMedicalAsyncJobProperties>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeICD10CMInferenceJobRequest {
    /// <p>The identifier that Amazon Comprehend Medical generated for the job. <code>The StartICD10CMInferenceJob</code> operation returns this identifier in its response.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeICD10CMInferenceJobResponse {
    /// <p>An object that contains the properties associated with a detection job.</p>
    #[serde(rename = "comprehendMedicalAsyncJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties: Option<ComprehendMedicalAsyncJobProperties>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePHIDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend Medical generated for the job. The <code>StartPHIDetectionJob</code> operation returns this identifier in its response.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePHIDetectionJobResponse {
    /// <p>An object that contains the properties associated with a detection job.</p>
    #[serde(rename = "comprehendMedicalAsyncJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties: Option<ComprehendMedicalAsyncJobProperties>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRxNormInferenceJobRequest {
    /// <p>The identifier that Amazon Comprehend Medical generated for the job. The StartRxNormInferenceJob operation returns this identifier in its response.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRxNormInferenceJobResponse {
    /// <p>An object that contains the properties associated with a detection job.</p>
    #[serde(rename = "comprehendMedicalAsyncJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties: Option<ComprehendMedicalAsyncJobProperties>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectEntitiesRequest {
    /// <p> A UTF-8 text string containing the clinical content being examined for entities. Each string must contain fewer than 20,000 bytes of characters.</p>
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectEntitiesResponse {
    /// <p> The collection of medical entities extracted from the input text and their associated information. For each entity, the response provides the entity text, the entity category, where the entity text begins and ends, and the level of confidence that Amazon Comprehend Medical has in the detection and analysis. Attributes and traits of the entity are also returned.</p>
    #[serde(rename = "entities")]
    pub entities: Vec<Entity>,
    /// <p>The version of the model used to analyze the documents. The version number looks like X.X.X. You can use this information to track the model used for a particular batch of documents.</p>
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    /// <p> If the result of the previous request to <code>DetectEntities</code> was truncated, include the <code>PaginationToken</code> to fetch the next page of entities.</p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p> Attributes extracted from the input text that we were unable to relate to an entity.</p>
    #[serde(rename = "unmappedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmapped_attributes: Option<Vec<UnmappedAttribute>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectEntitiesV2Request {
    /// <p>A UTF-8 string containing the clinical content being examined for entities. Each string must contain fewer than 20,000 bytes of characters.</p>
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectEntitiesV2Response {
    /// <p>The collection of medical entities extracted from the input text and their associated information. For each entity, the response provides the entity text, the entity category, where the entity text begins and ends, and the level of confidence in the detection and analysis. Attributes and traits of the entity are also returned.</p>
    #[serde(rename = "entities")]
    pub entities: Vec<Entity>,
    /// <p>The version of the model used to analyze the documents. The version number looks like X.X.X. You can use this information to track the model used for a particular batch of documents.</p>
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    /// <p>If the result to the <code>DetectEntitiesV2</code> operation was truncated, include the <code>PaginationToken</code> to fetch the next page of entities.</p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>Attributes extracted from the input text that couldn't be related to an entity.</p>
    #[serde(rename = "unmappedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmapped_attributes: Option<Vec<UnmappedAttribute>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectPHIRequest {
    /// <p> A UTF-8 text string containing the clinical content being examined for PHI entities. Each string must contain fewer than 20,000 bytes of characters.</p>
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectPHIResponse {
    /// <p> The collection of PHI entities extracted from the input text and their associated information. For each entity, the response provides the entity text, the entity category, where the entity text begins and ends, and the level of confidence that Amazon Comprehend Medical has in its detection. </p>
    #[serde(rename = "entities")]
    pub entities: Vec<Entity>,
    /// <p>The version of the model used to analyze the documents. The version number looks like X.X.X. You can use this information to track the model used for a particular batch of documents.</p>
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    /// <p> If the result of the previous request to <code>DetectPHI</code> was truncated, include the <code>PaginationToken</code> to fetch the next page of PHI entities. </p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

/// <p> Provides information about an extracted medical entity.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Entity {
    /// <p> The extracted attributes that relate to this entity.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p> The 0-based character offset in the input text that shows where the entity begins. The offset returns the UTF-8 code point in the string. </p>
    #[serde(rename = "beginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p> The category of the entity.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p> The 0-based character offset in the input text that shows where the entity ends. The offset returns the UTF-8 code point in the string. </p>
    #[serde(rename = "endOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p> The numeric identifier for the entity. This is a monotonically increasing id unique within this response rather than a global unique identifier. </p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p>The level of confidence that Amazon Comprehend Medical has in the accuracy of the detection.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p> The segment of input text extracted as this entity.</p>
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>Contextual information for the entity.</p>
    #[serde(rename = "traits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<Trait>>,
    /// <p> Describes the specific type of entity with category of entities.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The detected attributes that relate to an entity. This includes an extracted segment of the text that is an attribute of an entity, or otherwise related to an entity. InferICD10CM detects the following attributes: <code>Direction</code>, <code>System, Organ or Site</code>, and <code>Acuity</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ICD10CMAttribute {
    /// <p>The 0-based character offset in the input text that shows where the attribute begins. The offset returns the UTF-8 code point in the string.</p>
    #[serde(rename = "beginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p>The category of attribute. Can be either of <code>DX_NAME</code> or <code>TIME_EXPRESSION</code>.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The 0-based character offset in the input text that shows where the attribute ends. The offset returns the UTF-8 code point in the string.</p>
    #[serde(rename = "endOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p>The numeric identifier for this attribute. This is a monotonically increasing id unique within this response rather than a global unique identifier.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p>The level of confidence that Amazon Comprehend Medical has that this attribute is correctly related to this entity.</p>
    #[serde(rename = "relationshipScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_score: Option<f32>,
    /// <p>The type of relationship between the entity and attribute. Type for the relationship can be either of <code>OVERLAP</code> or <code>SYSTEM_ORGAN_SITE</code>.</p>
    #[serde(rename = "relationshipType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    /// <p>The level of confidence that Amazon Comprehend Medical has that the segment of text is correctly recognized as an attribute.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p>The segment of input text which contains the detected attribute.</p>
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>The contextual information for the attribute. The traits recognized by InferICD10CM are <code>DIAGNOSIS</code>, <code>SIGN</code>, <code>SYMPTOM</code>, and <code>NEGATION</code>.</p>
    #[serde(rename = "traits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<ICD10CMTrait>>,
    /// <p>The type of attribute. InferICD10CM detects entities of the type <code>DX_NAME</code>. </p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p> The ICD-10-CM concepts that the entity could refer to, along with a score indicating the likelihood of the match.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ICD10CMConcept {
    /// <p>The ICD-10-CM code that identifies the concept found in the knowledge base from the Centers for Disease Control.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The long description of the ICD-10-CM code in the ontology.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The level of confidence that Amazon Comprehend Medical has that the entity is accurately linked to an ICD-10-CM concept.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

/// <p>The collection of medical entities extracted from the input text and their associated information. For each entity, the response provides the entity text, the entity category, where the entity text begins and ends, and the level of confidence that Amazon Comprehend Medical has in the detection and analysis. Attributes and traits of the entity are also returned. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ICD10CMEntity {
    /// <p>The detected attributes that relate to the entity. An extracted segment of the text that is an attribute of an entity, or otherwise related to an entity, such as the nature of a medical condition.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ICD10CMAttribute>>,
    /// <p>The 0-based character offset in the input text that shows where the entity begins. The offset returns the UTF-8 code point in the string.</p>
    #[serde(rename = "beginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p> The category of the entity. InferICD10CM detects entities in the <code>MEDICAL_CONDITION</code> category. </p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The 0-based character offset in the input text that shows where the entity ends. The offset returns the UTF-8 code point in the string.</p>
    #[serde(rename = "endOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p>The ICD-10-CM concepts that the entity could refer to, along with a score indicating the likelihood of the match.</p>
    #[serde(rename = "iCD10CMConcepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icd10cm_concepts: Option<Vec<ICD10CMConcept>>,
    /// <p>The numeric identifier for the entity. This is a monotonically increasing id unique within this response rather than a global unique identifier.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p>The level of confidence that Amazon Comprehend Medical has in the accuracy of the detection.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p>The segment of input text that is matched to the detected entity.</p>
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>Provides Contextual information for the entity. The traits recognized by InferICD10CM are <code>DIAGNOSIS</code>, <code>SIGN</code>, <code>SYMPTOM</code>, and <code>NEGATION.</code> </p>
    #[serde(rename = "traits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<ICD10CMTrait>>,
    /// <p>Describes the specific type of entity with category of entities. InferICD10CM detects entities of the type <code>DX_NAME</code> and <code>TIME_EXPRESSION</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contextual information for the entity. The traits recognized by InferICD10CM are <code>DIAGNOSIS</code>, <code>SIGN</code>, <code>SYMPTOM</code>, and <code>NEGATION</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ICD10CMTrait {
    /// <p>Provides a name or contextual description about the trait.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The level of confidence that Amazon Comprehend Medical has that the segment of text is correctly recognized as a trait.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InferICD10CMRequest {
    /// <p>The input text used for analysis. The input for InferICD10CM is a string from 1 to 10000 characters.</p>
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InferICD10CMResponse {
    /// <p>The medical conditions detected in the text linked to ICD-10-CM concepts. If the action is successful, the service sends back an HTTP 200 response, as well as the entities detected.</p>
    #[serde(rename = "entities")]
    pub entities: Vec<ICD10CMEntity>,
    /// <p>The version of the model used to analyze the documents, in the format <i>n</i>.<i>n</i>.<i>n</i> You can use this information to track the model used for a particular batch of documents.</p>
    #[serde(rename = "modelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    /// <p>If the result of the previous request to <code>InferICD10CM</code> was truncated, include the <code>PaginationToken</code> to fetch the next page of medical condition entities. </p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InferRxNormRequest {
    /// <p>The input text used for analysis. The input for InferRxNorm is a string from 1 to 10000 characters.</p>
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InferRxNormResponse {
    /// <p>The medication entities detected in the text linked to RxNorm concepts. If the action is successful, the service sends back an HTTP 200 response, as well as the entities detected.</p>
    #[serde(rename = "entities")]
    pub entities: Vec<RxNormEntity>,
    /// <p>The version of the model used to analyze the documents, in the format <i>n</i>.<i>n</i>.<i>n</i> You can use this information to track the model used for a particular batch of documents.</p>
    #[serde(rename = "modelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    /// <p>If the result of the previous request to <code>InferRxNorm</code> was truncated, include the <code>PaginationToken</code> to fetch the next page of medication entities.</p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

/// <p>The input properties for an entities detection job. This includes the name of the S3 bucket and the path to the files to be analyzed. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputDataConfig {
    /// <p>The URI of the S3 bucket that contains the input data. The bucket must be in the same region as the API endpoint that you are calling.</p> <p>Each file in the document collection must be less than 40 KB. You can store a maximum of 30 GB in the bucket.</p>
    #[serde(rename = "s3Bucket")]
    pub s3_bucket: String,
    /// <p>The path to the input data files in the S3 bucket.</p>
    #[serde(rename = "s3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEntitiesDetectionV2JobsRequest {
    /// <p>Filters the jobs that are returned. You can filter jobs based on their names, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ComprehendMedicalAsyncJobFilter>,
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEntitiesDetectionV2JobsResponse {
    /// <p>A list containing the properties of each job returned.</p>
    #[serde(rename = "comprehendMedicalAsyncJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties_list:
        Option<Vec<ComprehendMedicalAsyncJobProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListICD10CMInferenceJobsRequest {
    /// <p>Filters the jobs that are returned. You can filter jobs based on their names, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ComprehendMedicalAsyncJobFilter>,
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListICD10CMInferenceJobsResponse {
    /// <p>A list containing the properties of each job that is returned.</p>
    #[serde(rename = "comprehendMedicalAsyncJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties_list:
        Option<Vec<ComprehendMedicalAsyncJobProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPHIDetectionJobsRequest {
    /// <p>Filters the jobs that are returned. You can filter jobs based on their names, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ComprehendMedicalAsyncJobFilter>,
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPHIDetectionJobsResponse {
    /// <p>A list containing the properties of each job returned.</p>
    #[serde(rename = "comprehendMedicalAsyncJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties_list:
        Option<Vec<ComprehendMedicalAsyncJobProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRxNormInferenceJobsRequest {
    /// <p>Filters the jobs that are returned. You can filter jobs based on their names, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ComprehendMedicalAsyncJobFilter>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRxNormInferenceJobsResponse {
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    #[serde(rename = "comprehendMedicalAsyncJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties_list:
        Option<Vec<ComprehendMedicalAsyncJobProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The output properties for a detection job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputDataConfig {
    /// <p>When you use the <code>OutputDataConfig</code> object with asynchronous operations, you specify the Amazon S3 location where you want to write the output data. The URI must be in the same region as the API endpoint that you are calling. The location is used as the prefix for the actual location of the output.</p>
    #[serde(rename = "s3Bucket")]
    pub s3_bucket: String,
    /// <p>The path to the output data files in the S3 bucket. Amazon Comprehend Medical creates an output directory using the job ID so that the output from one job does not overwrite the output of another.</p>
    #[serde(rename = "s3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
}

/// <p>The extracted attributes that relate to this entity. The attributes recognized by InferRxNorm are <code>DOSAGE</code>, <code>DURATION</code>, <code>FORM</code>, <code>FREQUENCY</code>, <code>RATE</code>, <code>ROUTE_OR_MODE</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RxNormAttribute {
    /// <p>The 0-based character offset in the input text that shows where the attribute begins. The offset returns the UTF-8 code point in the string.</p>
    #[serde(rename = "beginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p>The 0-based character offset in the input text that shows where the attribute ends. The offset returns the UTF-8 code point in the string.</p>
    #[serde(rename = "endOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p>The numeric identifier for this attribute. This is a monotonically increasing id unique within this response rather than a global unique identifier.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p>The level of confidence that Amazon Comprehend Medical has that the attribute is accurately linked to an entity.</p>
    #[serde(rename = "relationshipScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_score: Option<f32>,
    /// <p>The level of confidence that Comprehend Medical has that the segment of text is correctly recognized as an attribute.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p>The segment of input text which corresponds to the detected attribute.</p>
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>Contextual information for the attribute. InferRxNorm recognizes the trait <code>NEGATION</code> for attributes, i.e. that the patient is not taking a specific dose or form of a medication.</p>
    #[serde(rename = "traits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<RxNormTrait>>,
    /// <p>The type of attribute. The types of attributes recognized by InferRxNorm are <code>BRAND_NAME</code> and <code>GENERIC_NAME</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The RxNorm concept that the entity could refer to, along with a score indicating the likelihood of the match.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RxNormConcept {
    /// <p>RxNorm concept ID, also known as the RxCUI.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The description of the RxNorm concept.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The level of confidence that Amazon Comprehend Medical has that the entity is accurately linked to the reported RxNorm concept.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

/// <p>The collection of medical entities extracted from the input text and their associated information. For each entity, the response provides the entity text, the entity category, where the entity text begins and ends, and the level of confidence that Amazon Comprehend Medical has in the detection and analysis. Attributes and traits of the entity are also returned. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RxNormEntity {
    /// <p>The extracted attributes that relate to the entity. The attributes recognized by InferRxNorm are <code>DOSAGE</code>, <code>DURATION</code>, <code>FORM</code>, <code>FREQUENCY</code>, <code>RATE</code>, <code>ROUTE_OR_MODE</code>, and <code>STRENGTH</code>.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<RxNormAttribute>>,
    /// <p>The 0-based character offset in the input text that shows where the entity begins. The offset returns the UTF-8 code point in the string.</p>
    #[serde(rename = "beginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p>The category of the entity. The recognized categories are <code>GENERIC</code> or <code>BRAND_NAME</code>.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The 0-based character offset in the input text that shows where the entity ends. The offset returns the UTF-8 code point in the string.</p>
    #[serde(rename = "endOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p>The numeric identifier for the entity. This is a monotonically increasing id unique within this response rather than a global unique identifier.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p> The RxNorm concepts that the entity could refer to, along with a score indicating the likelihood of the match.</p>
    #[serde(rename = "rxNormConcepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_norm_concepts: Option<Vec<RxNormConcept>>,
    /// <p>The level of confidence that Amazon Comprehend Medical has in the accuracy of the detected entity.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p>The segment of input text extracted from which the entity was detected.</p>
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p> Contextual information for the entity.</p>
    #[serde(rename = "traits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<RxNormTrait>>,
    /// <p> Describes the specific type of entity. For InferRxNorm, the recognized entity type is <code>MEDICATION</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The contextual information for the entity. InferRxNorm recognizes the trait <code>NEGATION</code>, which is any indication that the patient is not taking a medication. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RxNormTrait {
    /// <p>Provides a name or contextual description about the trait.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The level of confidence that Amazon Comprehend Medical has in the accuracy of the detected trait.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartEntitiesDetectionV2JobRequest {
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend Medical generates one.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend Medical read access to your input data. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions-med.html#auth-role-permissions-med"> Role-Based Permissions Required for Asynchronous Operations</a>.</p>
    #[serde(rename = "dataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "inputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>An AWS Key Management Service key to encrypt your output files. If you do not specify a key, the files are written in plain text.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The language of the input documents. All documents must be in the same language.</p>
    #[serde(rename = "languageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "outputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartEntitiesDetectionV2JobResponse {
    /// <p>The identifier generated for the job. To get the status of a job, use this identifier with the <code>DescribeEntitiesDetectionV2Job</code> operation.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartICD10CMInferenceJobRequest {
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend Medical generates one.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend Medical read access to your input data. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions-med.html#auth-role-permissions-med"> Role-Based Permissions Required for Asynchronous Operations</a>.</p>
    #[serde(rename = "dataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "inputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>An AWS Key Management Service key to encrypt your output files. If you do not specify a key, the files are written in plain text.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The language of the input documents. All documents must be in the same language.</p>
    #[serde(rename = "languageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "outputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartICD10CMInferenceJobResponse {
    /// <p>The identifier generated for the job. To get the status of a job, use this identifier with the <code>StartICD10CMInferenceJob</code> operation.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartPHIDetectionJobRequest {
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend Medical generates one.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend Medical read access to your input data. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions-med.html#auth-role-permissions-med"> Role-Based Permissions Required for Asynchronous Operations</a>.</p>
    #[serde(rename = "dataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "inputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>An AWS Key Management Service key to encrypt your output files. If you do not specify a key, the files are written in plain text.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The language of the input documents. All documents must be in the same language.</p>
    #[serde(rename = "languageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "outputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartPHIDetectionJobResponse {
    /// <p>The identifier generated for the job. To get the status of a job, use this identifier with the <code>DescribePHIDetectionJob</code> operation.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartRxNormInferenceJobRequest {
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend Medical generates one.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend Medical read access to your input data. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions-med.html#auth-role-permissions-med"> Role-Based Permissions Required for Asynchronous Operations</a>.</p>
    #[serde(rename = "dataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "inputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>An AWS Key Management Service key to encrypt your output files. If you do not specify a key, the files are written in plain text.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The language of the input documents. All documents must be in the same language.</p>
    #[serde(rename = "languageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "outputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartRxNormInferenceJobResponse {
    /// <p>The identifier of the job.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopEntitiesDetectionV2JobRequest {
    /// <p>The identifier of the medical entities job to stop.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopEntitiesDetectionV2JobResponse {
    /// <p>The identifier of the medical entities detection job that was stopped.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopICD10CMInferenceJobRequest {
    /// <p>The identifier of the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopICD10CMInferenceJobResponse {
    /// <p>The identifier generated for the job. To get the status of job, use this identifier with the <code>DescribeICD10CMInferenceJob</code> operation.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopPHIDetectionJobRequest {
    /// <p>The identifier of the PHI detection job to stop.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopPHIDetectionJobResponse {
    /// <p>The identifier of the PHI detection job that was stopped.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopRxNormInferenceJobRequest {
    /// <p>The identifier of the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopRxNormInferenceJobResponse {
    /// <p>The identifier generated for the job. To get the status of job, use this identifier with the <code>DescribeRxNormInferenceJob</code> operation.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p> Provides contextual information about the extracted entity. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Trait {
    /// <p> Provides a name or contextual description about the trait. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The level of confidence that Amazon Comprehend Medical has in the accuracy of this trait.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

/// <p> An attribute that we extracted, but were unable to relate to an entity. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnmappedAttribute {
    /// <p> The specific attribute that has been extracted but not mapped to an entity. </p>
    #[serde(rename = "attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<Attribute>,
    /// <p> The type of the attribute, could be one of the following values: "MEDICATION", "MEDICAL_CONDITION", "ANATOMY", "TEST_AND_TREATMENT_PROCEDURE" or "PROTECTED_HEALTH_INFORMATION". </p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Errors returned by DescribeEntitiesDetectionV2Job
#[derive(Debug, PartialEq)]
pub enum DescribeEntitiesDetectionV2JobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DescribeEntitiesDetectionV2JobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEntitiesDetectionV2JobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeEntitiesDetectionV2JobError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeEntitiesDetectionV2JobError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeEntitiesDetectionV2JobError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeEntitiesDetectionV2JobError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEntitiesDetectionV2JobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEntitiesDetectionV2JobError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEntitiesDetectionV2JobError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEntitiesDetectionV2JobError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEntitiesDetectionV2JobError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeEntitiesDetectionV2JobError {}
/// Errors returned by DescribeICD10CMInferenceJob
#[derive(Debug, PartialEq)]
pub enum DescribeICD10CMInferenceJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DescribeICD10CMInferenceJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeICD10CMInferenceJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeICD10CMInferenceJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeICD10CMInferenceJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeICD10CMInferenceJobError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeICD10CMInferenceJobError::TooManyRequests(
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
impl fmt::Display for DescribeICD10CMInferenceJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeICD10CMInferenceJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeICD10CMInferenceJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeICD10CMInferenceJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeICD10CMInferenceJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeICD10CMInferenceJobError {}
/// Errors returned by DescribePHIDetectionJob
#[derive(Debug, PartialEq)]
pub enum DescribePHIDetectionJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DescribePHIDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePHIDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribePHIDetectionJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribePHIDetectionJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePHIDetectionJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribePHIDetectionJobError::TooManyRequests(
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
impl fmt::Display for DescribePHIDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePHIDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribePHIDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribePHIDetectionJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribePHIDetectionJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePHIDetectionJobError {}
/// Errors returned by DescribeRxNormInferenceJob
#[derive(Debug, PartialEq)]
pub enum DescribeRxNormInferenceJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DescribeRxNormInferenceJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRxNormInferenceJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeRxNormInferenceJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeRxNormInferenceJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRxNormInferenceJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeRxNormInferenceJobError::TooManyRequests(
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
impl fmt::Display for DescribeRxNormInferenceJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRxNormInferenceJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeRxNormInferenceJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeRxNormInferenceJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeRxNormInferenceJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRxNormInferenceJobError {}
/// Errors returned by DetectEntities
#[derive(Debug, PartialEq)]
pub enum DetectEntitiesError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The input text was not in valid UTF-8 character encoding. Check your text then retry your request.</p>
    InvalidEncoding(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> The Amazon Comprehend Medical service is temporarily unavailable. Please wait and then retry your request. </p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DetectEntitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectEntitiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectEntitiesError::InternalServer(err.msg))
                }
                "InvalidEncodingException" => {
                    return RusotoError::Service(DetectEntitiesError::InvalidEncoding(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectEntitiesError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DetectEntitiesError::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectEntitiesError::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DetectEntitiesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DetectEntitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectEntitiesError::InternalServer(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::InvalidEncoding(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectEntitiesError {}
/// Errors returned by DetectEntitiesV2
#[derive(Debug, PartialEq)]
pub enum DetectEntitiesV2Error {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The input text was not in valid UTF-8 character encoding. Check your text then retry your request.</p>
    InvalidEncoding(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> The Amazon Comprehend Medical service is temporarily unavailable. Please wait and then retry your request. </p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DetectEntitiesV2Error {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectEntitiesV2Error> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::InternalServer(err.msg))
                }
                "InvalidEncodingException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::InvalidEncoding(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DetectEntitiesV2Error {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectEntitiesV2Error::InternalServer(ref cause) => write!(f, "{}", cause),
            DetectEntitiesV2Error::InvalidEncoding(ref cause) => write!(f, "{}", cause),
            DetectEntitiesV2Error::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DetectEntitiesV2Error::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DetectEntitiesV2Error::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            DetectEntitiesV2Error::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectEntitiesV2Error {}
/// Errors returned by DetectPHI
#[derive(Debug, PartialEq)]
pub enum DetectPHIError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The input text was not in valid UTF-8 character encoding. Check your text then retry your request.</p>
    InvalidEncoding(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> The Amazon Comprehend Medical service is temporarily unavailable. Please wait and then retry your request. </p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DetectPHIError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectPHIError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectPHIError::InternalServer(err.msg))
                }
                "InvalidEncodingException" => {
                    return RusotoError::Service(DetectPHIError::InvalidEncoding(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectPHIError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DetectPHIError::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectPHIError::TextSizeLimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DetectPHIError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DetectPHIError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectPHIError::InternalServer(ref cause) => write!(f, "{}", cause),
            DetectPHIError::InvalidEncoding(ref cause) => write!(f, "{}", cause),
            DetectPHIError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DetectPHIError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DetectPHIError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            DetectPHIError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectPHIError {}
/// Errors returned by InferICD10CM
#[derive(Debug, PartialEq)]
pub enum InferICD10CMError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The input text was not in valid UTF-8 character encoding. Check your text then retry your request.</p>
    InvalidEncoding(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> The Amazon Comprehend Medical service is temporarily unavailable. Please wait and then retry your request. </p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl InferICD10CMError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InferICD10CMError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(InferICD10CMError::InternalServer(err.msg))
                }
                "InvalidEncodingException" => {
                    return RusotoError::Service(InferICD10CMError::InvalidEncoding(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(InferICD10CMError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(InferICD10CMError::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(InferICD10CMError::TextSizeLimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(InferICD10CMError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InferICD10CMError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InferICD10CMError::InternalServer(ref cause) => write!(f, "{}", cause),
            InferICD10CMError::InvalidEncoding(ref cause) => write!(f, "{}", cause),
            InferICD10CMError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            InferICD10CMError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            InferICD10CMError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            InferICD10CMError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InferICD10CMError {}
/// Errors returned by InferRxNorm
#[derive(Debug, PartialEq)]
pub enum InferRxNormError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The input text was not in valid UTF-8 character encoding. Check your text then retry your request.</p>
    InvalidEncoding(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> The Amazon Comprehend Medical service is temporarily unavailable. Please wait and then retry your request. </p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl InferRxNormError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InferRxNormError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(InferRxNormError::InternalServer(err.msg))
                }
                "InvalidEncodingException" => {
                    return RusotoError::Service(InferRxNormError::InvalidEncoding(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(InferRxNormError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(InferRxNormError::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(InferRxNormError::TextSizeLimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(InferRxNormError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InferRxNormError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InferRxNormError::InternalServer(ref cause) => write!(f, "{}", cause),
            InferRxNormError::InvalidEncoding(ref cause) => write!(f, "{}", cause),
            InferRxNormError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            InferRxNormError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            InferRxNormError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            InferRxNormError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InferRxNormError {}
/// Errors returned by ListEntitiesDetectionV2Jobs
#[derive(Debug, PartialEq)]
pub enum ListEntitiesDetectionV2JobsError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl ListEntitiesDetectionV2JobsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListEntitiesDetectionV2JobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListEntitiesDetectionV2JobsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListEntitiesDetectionV2JobsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEntitiesDetectionV2JobsError::TooManyRequests(
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
impl fmt::Display for ListEntitiesDetectionV2JobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEntitiesDetectionV2JobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListEntitiesDetectionV2JobsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListEntitiesDetectionV2JobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEntitiesDetectionV2JobsError {}
/// Errors returned by ListICD10CMInferenceJobs
#[derive(Debug, PartialEq)]
pub enum ListICD10CMInferenceJobsError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl ListICD10CMInferenceJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListICD10CMInferenceJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListICD10CMInferenceJobsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListICD10CMInferenceJobsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListICD10CMInferenceJobsError::TooManyRequests(
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
impl fmt::Display for ListICD10CMInferenceJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListICD10CMInferenceJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListICD10CMInferenceJobsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListICD10CMInferenceJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListICD10CMInferenceJobsError {}
/// Errors returned by ListPHIDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListPHIDetectionJobsError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl ListPHIDetectionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPHIDetectionJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListPHIDetectionJobsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListPHIDetectionJobsError::InvalidRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListPHIDetectionJobsError::TooManyRequests(
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
impl fmt::Display for ListPHIDetectionJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPHIDetectionJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListPHIDetectionJobsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListPHIDetectionJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPHIDetectionJobsError {}
/// Errors returned by ListRxNormInferenceJobs
#[derive(Debug, PartialEq)]
pub enum ListRxNormInferenceJobsError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl ListRxNormInferenceJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRxNormInferenceJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListRxNormInferenceJobsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListRxNormInferenceJobsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListRxNormInferenceJobsError::TooManyRequests(
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
impl fmt::Display for ListRxNormInferenceJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRxNormInferenceJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListRxNormInferenceJobsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListRxNormInferenceJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRxNormInferenceJobsError {}
/// Errors returned by StartEntitiesDetectionV2Job
#[derive(Debug, PartialEq)]
pub enum StartEntitiesDetectionV2JobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl StartEntitiesDetectionV2JobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartEntitiesDetectionV2JobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartEntitiesDetectionV2JobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartEntitiesDetectionV2JobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        StartEntitiesDetectionV2JobError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartEntitiesDetectionV2JobError::TooManyRequests(
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
impl fmt::Display for StartEntitiesDetectionV2JobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartEntitiesDetectionV2JobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartEntitiesDetectionV2JobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartEntitiesDetectionV2JobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartEntitiesDetectionV2JobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartEntitiesDetectionV2JobError {}
/// Errors returned by StartICD10CMInferenceJob
#[derive(Debug, PartialEq)]
pub enum StartICD10CMInferenceJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl StartICD10CMInferenceJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartICD10CMInferenceJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartICD10CMInferenceJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartICD10CMInferenceJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartICD10CMInferenceJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartICD10CMInferenceJobError::TooManyRequests(
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
impl fmt::Display for StartICD10CMInferenceJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartICD10CMInferenceJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartICD10CMInferenceJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartICD10CMInferenceJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartICD10CMInferenceJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartICD10CMInferenceJobError {}
/// Errors returned by StartPHIDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartPHIDetectionJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl StartPHIDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartPHIDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartPHIDetectionJobError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartPHIDetectionJobError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartPHIDetectionJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartPHIDetectionJobError::TooManyRequests(
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
impl fmt::Display for StartPHIDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartPHIDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartPHIDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartPHIDetectionJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartPHIDetectionJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartPHIDetectionJobError {}
/// Errors returned by StartRxNormInferenceJob
#[derive(Debug, PartialEq)]
pub enum StartRxNormInferenceJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl StartRxNormInferenceJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartRxNormInferenceJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartRxNormInferenceJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartRxNormInferenceJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartRxNormInferenceJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartRxNormInferenceJobError::TooManyRequests(
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
impl fmt::Display for StartRxNormInferenceJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartRxNormInferenceJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartRxNormInferenceJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartRxNormInferenceJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartRxNormInferenceJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartRxNormInferenceJobError {}
/// Errors returned by StopEntitiesDetectionV2Job
#[derive(Debug, PartialEq)]
pub enum StopEntitiesDetectionV2JobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
}

impl StopEntitiesDetectionV2JobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StopEntitiesDetectionV2JobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopEntitiesDetectionV2JobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopEntitiesDetectionV2JobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopEntitiesDetectionV2JobError::ResourceNotFound(
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
impl fmt::Display for StopEntitiesDetectionV2JobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopEntitiesDetectionV2JobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopEntitiesDetectionV2JobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopEntitiesDetectionV2JobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopEntitiesDetectionV2JobError {}
/// Errors returned by StopICD10CMInferenceJob
#[derive(Debug, PartialEq)]
pub enum StopICD10CMInferenceJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
}

impl StopICD10CMInferenceJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopICD10CMInferenceJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopICD10CMInferenceJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopICD10CMInferenceJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopICD10CMInferenceJobError::ResourceNotFound(
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
impl fmt::Display for StopICD10CMInferenceJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopICD10CMInferenceJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopICD10CMInferenceJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopICD10CMInferenceJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopICD10CMInferenceJobError {}
/// Errors returned by StopPHIDetectionJob
#[derive(Debug, PartialEq)]
pub enum StopPHIDetectionJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
}

impl StopPHIDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopPHIDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopPHIDetectionJobError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopPHIDetectionJobError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopPHIDetectionJobError::ResourceNotFound(
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
impl fmt::Display for StopPHIDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopPHIDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopPHIDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopPHIDetectionJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopPHIDetectionJobError {}
/// Errors returned by StopRxNormInferenceJob
#[derive(Debug, PartialEq)]
pub enum StopRxNormInferenceJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
}

impl StopRxNormInferenceJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopRxNormInferenceJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopRxNormInferenceJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopRxNormInferenceJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopRxNormInferenceJobError::ResourceNotFound(
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
impl fmt::Display for StopRxNormInferenceJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopRxNormInferenceJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopRxNormInferenceJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopRxNormInferenceJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopRxNormInferenceJobError {}
/// Trait representing the capabilities of the ComprehendMedical API. ComprehendMedical clients implement this trait.
#[async_trait]
pub trait ComprehendMedical {
    /// <p>Gets the properties associated with a medical entities detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_entities_detection_v2_job(
        &self,
        input: DescribeEntitiesDetectionV2JobRequest,
    ) -> Result<
        DescribeEntitiesDetectionV2JobResponse,
        RusotoError<DescribeEntitiesDetectionV2JobError>,
    >;

    /// <p>Gets the properties associated with an InferICD10CM job. Use this operation to get the status of an inference job.</p>
    async fn describe_icd10cm_inference_job(
        &self,
        input: DescribeICD10CMInferenceJobRequest,
    ) -> Result<DescribeICD10CMInferenceJobResponse, RusotoError<DescribeICD10CMInferenceJobError>>;

    /// <p>Gets the properties associated with a protected health information (PHI) detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_phi_detection_job(
        &self,
        input: DescribePHIDetectionJobRequest,
    ) -> Result<DescribePHIDetectionJobResponse, RusotoError<DescribePHIDetectionJobError>>;

    /// <p>Gets the properties associated with an InferRxNorm job. Use this operation to get the status of an inference job.</p>
    async fn describe_rx_norm_inference_job(
        &self,
        input: DescribeRxNormInferenceJobRequest,
    ) -> Result<DescribeRxNormInferenceJobResponse, RusotoError<DescribeRxNormInferenceJobError>>;

    /// <p>The <code>DetectEntities</code> operation is deprecated. You should use the <a>DetectEntitiesV2</a> operation instead.</p> <p> Inspects the clinical text for a variety of medical entities and returns specific information about them such as entity category, location, and confidence score on that information .</p>
    async fn detect_entities(
        &self,
        input: DetectEntitiesRequest,
    ) -> Result<DetectEntitiesResponse, RusotoError<DetectEntitiesError>>;

    /// <p>Inspects the clinical text for a variety of medical entities and returns specific information about them such as entity category, location, and confidence score on that information. Amazon Comprehend Medical only detects medical entities in English language texts.</p> <p>The <code>DetectEntitiesV2</code> operation replaces the <a>DetectEntities</a> operation. This new action uses a different model for determining the entities in your medical text and changes the way that some entities are returned in the output. You should use the <code>DetectEntitiesV2</code> operation in all new applications.</p> <p>The <code>DetectEntitiesV2</code> operation returns the <code>Acuity</code> and <code>Direction</code> entities as attributes instead of types. </p>
    async fn detect_entities_v2(
        &self,
        input: DetectEntitiesV2Request,
    ) -> Result<DetectEntitiesV2Response, RusotoError<DetectEntitiesV2Error>>;

    /// <p> Inspects the clinical text for protected health information (PHI) entities and returns the entity category, location, and confidence score for each entity. Amazon Comprehend Medical only detects entities in English language texts.</p>
    async fn detect_phi(
        &self,
        input: DetectPHIRequest,
    ) -> Result<DetectPHIResponse, RusotoError<DetectPHIError>>;

    /// <p>InferICD10CM detects medical conditions as entities listed in a patient record and links those entities to normalized concept identifiers in the ICD-10-CM knowledge base from the Centers for Disease Control. Amazon Comprehend Medical only detects medical entities in English language texts. </p>
    async fn infer_icd10cm(
        &self,
        input: InferICD10CMRequest,
    ) -> Result<InferICD10CMResponse, RusotoError<InferICD10CMError>>;

    /// <p>InferRxNorm detects medications as entities listed in a patient record and links to the normalized concept identifiers in the RxNorm database from the National Library of Medicine. Amazon Comprehend Medical only detects medical entities in English language texts. </p>
    async fn infer_rx_norm(
        &self,
        input: InferRxNormRequest,
    ) -> Result<InferRxNormResponse, RusotoError<InferRxNormError>>;

    /// <p>Gets a list of medical entity detection jobs that you have submitted.</p>
    async fn list_entities_detection_v2_jobs(
        &self,
        input: ListEntitiesDetectionV2JobsRequest,
    ) -> Result<ListEntitiesDetectionV2JobsResponse, RusotoError<ListEntitiesDetectionV2JobsError>>;

    /// <p>Gets a list of InferICD10CM jobs that you have submitted.</p>
    async fn list_icd10cm_inference_jobs(
        &self,
        input: ListICD10CMInferenceJobsRequest,
    ) -> Result<ListICD10CMInferenceJobsResponse, RusotoError<ListICD10CMInferenceJobsError>>;

    /// <p>Gets a list of protected health information (PHI) detection jobs that you have submitted.</p>
    async fn list_phi_detection_jobs(
        &self,
        input: ListPHIDetectionJobsRequest,
    ) -> Result<ListPHIDetectionJobsResponse, RusotoError<ListPHIDetectionJobsError>>;

    /// <p>Gets a list of InferRxNorm jobs that you have submitted.</p>
    async fn list_rx_norm_inference_jobs(
        &self,
        input: ListRxNormInferenceJobsRequest,
    ) -> Result<ListRxNormInferenceJobsResponse, RusotoError<ListRxNormInferenceJobsError>>;

    /// <p>Starts an asynchronous medical entity detection job for a collection of documents. Use the <code>DescribeEntitiesDetectionV2Job</code> operation to track the status of a job.</p>
    async fn start_entities_detection_v2_job(
        &self,
        input: StartEntitiesDetectionV2JobRequest,
    ) -> Result<StartEntitiesDetectionV2JobResponse, RusotoError<StartEntitiesDetectionV2JobError>>;

    /// <p>Starts an asynchronous job to detect medical conditions and link them to the ICD-10-CM ontology. Use the <code>DescribeICD10CMInferenceJob</code> operation to track the status of a job.</p>
    async fn start_icd10cm_inference_job(
        &self,
        input: StartICD10CMInferenceJobRequest,
    ) -> Result<StartICD10CMInferenceJobResponse, RusotoError<StartICD10CMInferenceJobError>>;

    /// <p>Starts an asynchronous job to detect protected health information (PHI). Use the <code>DescribePHIDetectionJob</code> operation to track the status of a job.</p>
    async fn start_phi_detection_job(
        &self,
        input: StartPHIDetectionJobRequest,
    ) -> Result<StartPHIDetectionJobResponse, RusotoError<StartPHIDetectionJobError>>;

    /// <p>Starts an asynchronous job to detect medication entities and link them to the RxNorm ontology. Use the <code>DescribeRxNormInferenceJob</code> operation to track the status of a job.</p>
    async fn start_rx_norm_inference_job(
        &self,
        input: StartRxNormInferenceJobRequest,
    ) -> Result<StartRxNormInferenceJobResponse, RusotoError<StartRxNormInferenceJobError>>;

    /// <p>Stops a medical entities detection job in progress.</p>
    async fn stop_entities_detection_v2_job(
        &self,
        input: StopEntitiesDetectionV2JobRequest,
    ) -> Result<StopEntitiesDetectionV2JobResponse, RusotoError<StopEntitiesDetectionV2JobError>>;

    /// <p>Stops an InferICD10CM inference job in progress.</p>
    async fn stop_icd10cm_inference_job(
        &self,
        input: StopICD10CMInferenceJobRequest,
    ) -> Result<StopICD10CMInferenceJobResponse, RusotoError<StopICD10CMInferenceJobError>>;

    /// <p>Stops a protected health information (PHI) detection job in progress.</p>
    async fn stop_phi_detection_job(
        &self,
        input: StopPHIDetectionJobRequest,
    ) -> Result<StopPHIDetectionJobResponse, RusotoError<StopPHIDetectionJobError>>;

    /// <p>Stops an InferRxNorm inference job in progress.</p>
    async fn stop_rx_norm_inference_job(
        &self,
        input: StopRxNormInferenceJobRequest,
    ) -> Result<StopRxNormInferenceJobResponse, RusotoError<StopRxNormInferenceJobError>>;
}
/// A client for the ComprehendMedical API.
#[derive(Clone)]
pub struct ComprehendMedicalClient {
    client: Client,
    region: region::Region,
}

impl ComprehendMedicalClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ComprehendMedicalClient {
        ComprehendMedicalClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ComprehendMedicalClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ComprehendMedicalClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ComprehendMedicalClient {
        ComprehendMedicalClient { client, region }
    }
}

#[async_trait]
impl ComprehendMedical for ComprehendMedicalClient {
    /// <p>Gets the properties associated with a medical entities detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_entities_detection_v2_job(
        &self,
        input: DescribeEntitiesDetectionV2JobRequest,
    ) -> Result<
        DescribeEntitiesDetectionV2JobResponse,
        RusotoError<DescribeEntitiesDetectionV2JobError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.DescribeEntitiesDetectionV2Job",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEntitiesDetectionV2JobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEntitiesDetectionV2JobResponse, _>()
    }

    /// <p>Gets the properties associated with an InferICD10CM job. Use this operation to get the status of an inference job.</p>
    async fn describe_icd10cm_inference_job(
        &self,
        input: DescribeICD10CMInferenceJobRequest,
    ) -> Result<DescribeICD10CMInferenceJobResponse, RusotoError<DescribeICD10CMInferenceJobError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.DescribeICD10CMInferenceJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeICD10CMInferenceJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeICD10CMInferenceJobResponse, _>()
    }

    /// <p>Gets the properties associated with a protected health information (PHI) detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_phi_detection_job(
        &self,
        input: DescribePHIDetectionJobRequest,
    ) -> Result<DescribePHIDetectionJobResponse, RusotoError<DescribePHIDetectionJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.DescribePHIDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribePHIDetectionJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribePHIDetectionJobResponse, _>()
    }

    /// <p>Gets the properties associated with an InferRxNorm job. Use this operation to get the status of an inference job.</p>
    async fn describe_rx_norm_inference_job(
        &self,
        input: DescribeRxNormInferenceJobRequest,
    ) -> Result<DescribeRxNormInferenceJobResponse, RusotoError<DescribeRxNormInferenceJobError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.DescribeRxNormInferenceJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeRxNormInferenceJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeRxNormInferenceJobResponse, _>()
    }

    /// <p>The <code>DetectEntities</code> operation is deprecated. You should use the <a>DetectEntitiesV2</a> operation instead.</p> <p> Inspects the clinical text for a variety of medical entities and returns specific information about them such as entity category, location, and confidence score on that information .</p>
    async fn detect_entities(
        &self,
        input: DetectEntitiesRequest,
    ) -> Result<DetectEntitiesResponse, RusotoError<DetectEntitiesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "ComprehendMedical_20181030.DetectEntities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DetectEntitiesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DetectEntitiesResponse, _>()
    }

    /// <p>Inspects the clinical text for a variety of medical entities and returns specific information about them such as entity category, location, and confidence score on that information. Amazon Comprehend Medical only detects medical entities in English language texts.</p> <p>The <code>DetectEntitiesV2</code> operation replaces the <a>DetectEntities</a> operation. This new action uses a different model for determining the entities in your medical text and changes the way that some entities are returned in the output. You should use the <code>DetectEntitiesV2</code> operation in all new applications.</p> <p>The <code>DetectEntitiesV2</code> operation returns the <code>Acuity</code> and <code>Direction</code> entities as attributes instead of types. </p>
    async fn detect_entities_v2(
        &self,
        input: DetectEntitiesV2Request,
    ) -> Result<DetectEntitiesV2Response, RusotoError<DetectEntitiesV2Error>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.DetectEntitiesV2",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DetectEntitiesV2Error::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DetectEntitiesV2Response, _>()
    }

    /// <p> Inspects the clinical text for protected health information (PHI) entities and returns the entity category, location, and confidence score for each entity. Amazon Comprehend Medical only detects entities in English language texts.</p>
    async fn detect_phi(
        &self,
        input: DetectPHIRequest,
    ) -> Result<DetectPHIResponse, RusotoError<DetectPHIError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "ComprehendMedical_20181030.DetectPHI");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DetectPHIError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DetectPHIResponse, _>()
    }

    /// <p>InferICD10CM detects medical conditions as entities listed in a patient record and links those entities to normalized concept identifiers in the ICD-10-CM knowledge base from the Centers for Disease Control. Amazon Comprehend Medical only detects medical entities in English language texts. </p>
    async fn infer_icd10cm(
        &self,
        input: InferICD10CMRequest,
    ) -> Result<InferICD10CMResponse, RusotoError<InferICD10CMError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "ComprehendMedical_20181030.InferICD10CM");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, InferICD10CMError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<InferICD10CMResponse, _>()
    }

    /// <p>InferRxNorm detects medications as entities listed in a patient record and links to the normalized concept identifiers in the RxNorm database from the National Library of Medicine. Amazon Comprehend Medical only detects medical entities in English language texts. </p>
    async fn infer_rx_norm(
        &self,
        input: InferRxNormRequest,
    ) -> Result<InferRxNormResponse, RusotoError<InferRxNormError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "ComprehendMedical_20181030.InferRxNorm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, InferRxNormError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<InferRxNormResponse, _>()
    }

    /// <p>Gets a list of medical entity detection jobs that you have submitted.</p>
    async fn list_entities_detection_v2_jobs(
        &self,
        input: ListEntitiesDetectionV2JobsRequest,
    ) -> Result<ListEntitiesDetectionV2JobsResponse, RusotoError<ListEntitiesDetectionV2JobsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.ListEntitiesDetectionV2Jobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListEntitiesDetectionV2JobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListEntitiesDetectionV2JobsResponse, _>()
    }

    /// <p>Gets a list of InferICD10CM jobs that you have submitted.</p>
    async fn list_icd10cm_inference_jobs(
        &self,
        input: ListICD10CMInferenceJobsRequest,
    ) -> Result<ListICD10CMInferenceJobsResponse, RusotoError<ListICD10CMInferenceJobsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.ListICD10CMInferenceJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListICD10CMInferenceJobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListICD10CMInferenceJobsResponse, _>()
    }

    /// <p>Gets a list of protected health information (PHI) detection jobs that you have submitted.</p>
    async fn list_phi_detection_jobs(
        &self,
        input: ListPHIDetectionJobsRequest,
    ) -> Result<ListPHIDetectionJobsResponse, RusotoError<ListPHIDetectionJobsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.ListPHIDetectionJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListPHIDetectionJobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListPHIDetectionJobsResponse, _>()
    }

    /// <p>Gets a list of InferRxNorm jobs that you have submitted.</p>
    async fn list_rx_norm_inference_jobs(
        &self,
        input: ListRxNormInferenceJobsRequest,
    ) -> Result<ListRxNormInferenceJobsResponse, RusotoError<ListRxNormInferenceJobsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.ListRxNormInferenceJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRxNormInferenceJobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListRxNormInferenceJobsResponse, _>()
    }

    /// <p>Starts an asynchronous medical entity detection job for a collection of documents. Use the <code>DescribeEntitiesDetectionV2Job</code> operation to track the status of a job.</p>
    async fn start_entities_detection_v2_job(
        &self,
        input: StartEntitiesDetectionV2JobRequest,
    ) -> Result<StartEntitiesDetectionV2JobResponse, RusotoError<StartEntitiesDetectionV2JobError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StartEntitiesDetectionV2Job",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartEntitiesDetectionV2JobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartEntitiesDetectionV2JobResponse, _>()
    }

    /// <p>Starts an asynchronous job to detect medical conditions and link them to the ICD-10-CM ontology. Use the <code>DescribeICD10CMInferenceJob</code> operation to track the status of a job.</p>
    async fn start_icd10cm_inference_job(
        &self,
        input: StartICD10CMInferenceJobRequest,
    ) -> Result<StartICD10CMInferenceJobResponse, RusotoError<StartICD10CMInferenceJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StartICD10CMInferenceJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartICD10CMInferenceJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartICD10CMInferenceJobResponse, _>()
    }

    /// <p>Starts an asynchronous job to detect protected health information (PHI). Use the <code>DescribePHIDetectionJob</code> operation to track the status of a job.</p>
    async fn start_phi_detection_job(
        &self,
        input: StartPHIDetectionJobRequest,
    ) -> Result<StartPHIDetectionJobResponse, RusotoError<StartPHIDetectionJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StartPHIDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartPHIDetectionJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartPHIDetectionJobResponse, _>()
    }

    /// <p>Starts an asynchronous job to detect medication entities and link them to the RxNorm ontology. Use the <code>DescribeRxNormInferenceJob</code> operation to track the status of a job.</p>
    async fn start_rx_norm_inference_job(
        &self,
        input: StartRxNormInferenceJobRequest,
    ) -> Result<StartRxNormInferenceJobResponse, RusotoError<StartRxNormInferenceJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StartRxNormInferenceJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartRxNormInferenceJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartRxNormInferenceJobResponse, _>()
    }

    /// <p>Stops a medical entities detection job in progress.</p>
    async fn stop_entities_detection_v2_job(
        &self,
        input: StopEntitiesDetectionV2JobRequest,
    ) -> Result<StopEntitiesDetectionV2JobResponse, RusotoError<StopEntitiesDetectionV2JobError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StopEntitiesDetectionV2Job",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopEntitiesDetectionV2JobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StopEntitiesDetectionV2JobResponse, _>()
    }

    /// <p>Stops an InferICD10CM inference job in progress.</p>
    async fn stop_icd10cm_inference_job(
        &self,
        input: StopICD10CMInferenceJobRequest,
    ) -> Result<StopICD10CMInferenceJobResponse, RusotoError<StopICD10CMInferenceJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StopICD10CMInferenceJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopICD10CMInferenceJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StopICD10CMInferenceJobResponse, _>()
    }

    /// <p>Stops a protected health information (PHI) detection job in progress.</p>
    async fn stop_phi_detection_job(
        &self,
        input: StopPHIDetectionJobRequest,
    ) -> Result<StopPHIDetectionJobResponse, RusotoError<StopPHIDetectionJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StopPHIDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopPHIDetectionJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StopPHIDetectionJobResponse, _>()
    }

    /// <p>Stops an InferRxNorm inference job in progress.</p>
    async fn stop_rx_norm_inference_job(
        &self,
        input: StopRxNormInferenceJobRequest,
    ) -> Result<StopRxNormInferenceJobResponse, RusotoError<StopRxNormInferenceJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StopRxNormInferenceJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopRxNormInferenceJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StopRxNormInferenceJobResponse, _>()
    }
}
