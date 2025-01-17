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

impl CostExplorerClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "ce", &self.region, request_uri);

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
/// <p> An unusual cost pattern. This consists of the detailed metadata and the current status of the anomaly object. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Anomaly {
    /// <p> The last day the anomaly is detected. </p>
    #[serde(rename = "anomalyEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_end_date: Option<String>,
    /// <p> The unique identifier for the anomaly. </p>
    #[serde(rename = "anomalyId")]
    pub anomaly_id: String,
    /// <p> The latest and maximum score for the anomaly. </p>
    #[serde(rename = "anomalyScore")]
    pub anomaly_score: AnomalyScore,
    /// <p> The first day the anomaly is detected. </p>
    #[serde(rename = "anomalyStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_start_date: Option<String>,
    /// <p> The dimension for the anomaly. For example, an AWS service in a service monitor. </p>
    #[serde(rename = "dimensionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_value: Option<String>,
    /// <p> The feedback value. </p>
    #[serde(rename = "feedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    /// <p> The dollar impact for the anomaly. </p>
    #[serde(rename = "impact")]
    pub impact: Impact,
    /// <p> The Amazon Resource Name (ARN) for the cost monitor that generated this anomaly. </p>
    #[serde(rename = "monitorArn")]
    pub monitor_arn: String,
    /// <p> The list of identified root causes for the anomaly. </p>
    #[serde(rename = "rootCauses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_causes: Option<Vec<RootCause>>,
}

/// <p> The time period for an anomaly. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AnomalyDateInterval {
    /// <p> The last date an anomaly was observed. </p>
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// <p> The first date an anomaly was observed. </p>
    #[serde(rename = "startDate")]
    pub start_date: String,
}

/// <p> This object continuously inspects your account's cost data for anomalies, based on <code>MonitorType</code> and <code>MonitorSpecification</code>. The content consists of detailed metadata and the current status of the monitor object. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AnomalyMonitor {
    /// <p> The date when the monitor was created. </p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p> The value for evaluated dimensions. </p>
    #[serde(rename = "dimensionalValueCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensional_value_count: Option<i64>,
    /// <p> The date when the monitor last evaluated for anomalies. </p>
    #[serde(rename = "lastEvaluatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_date: Option<String>,
    /// <p> The date when the monitor was last updated. </p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    /// <p> The Amazon Resource Name (ARN) value. </p>
    #[serde(rename = "monitorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
    /// <p> The dimensions to evaluate. </p>
    #[serde(rename = "monitorDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_dimension: Option<String>,
    /// <p> The name of the monitor. </p>
    #[serde(rename = "monitorName")]
    pub monitor_name: String,
    #[serde(rename = "monitorSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_specification: Option<Expression>,
    /// <p> The possible type values. </p>
    #[serde(rename = "monitorType")]
    pub monitor_type: String,
}

/// <p> Quantifies the anomaly. The higher score means that it is more anomalous. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnomalyScore {
    /// <p> The last observed score. </p>
    #[serde(rename = "currentScore")]
    pub current_score: f64,
    /// <p> The maximum score observed during the <code>AnomalyDateInterval</code>. </p>
    #[serde(rename = "maxScore")]
    pub max_score: f64,
}

/// <p> The association between a monitor, threshold, and list of subscribers used to deliver notifications about anomalies detected by a monitor that exceeds a threshold. The content consists of the detailed metadata and the current status of the <code>AnomalySubscription</code> object. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AnomalySubscription {
    /// <p> Your unique account identifier. </p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p> The frequency at which anomaly reports are sent over email. </p>
    #[serde(rename = "frequency")]
    pub frequency: String,
    /// <p> A list of cost anomaly monitors. </p>
    #[serde(rename = "monitorArnList")]
    pub monitor_arn_list: Vec<String>,
    /// <p> A list of subscribers to notify. </p>
    #[serde(rename = "subscribers")]
    pub subscribers: Vec<Subscriber>,
    /// <p> The <code>AnomalySubscription</code> Amazon Resource Name (ARN). </p>
    #[serde(rename = "subscriptionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_arn: Option<String>,
    /// <p> The name for the subscription. </p>
    #[serde(rename = "subscriptionName")]
    pub subscription_name: String,
    /// <p> The dollar value that triggers a notification if the threshold is exceeded. </p>
    #[serde(rename = "threshold")]
    pub threshold: f64,
}

/// <p>The structure of Cost Categories. This includes detailed metadata and the set of rules for the <code>CostCategory</code> object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CostCategory {
    /// <p> The unique identifier for your Cost Category. </p>
    #[serde(rename = "costCategoryArn")]
    pub cost_category_arn: String,
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p> The Cost Category's effective end date.</p>
    #[serde(rename = "effectiveEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_end: Option<String>,
    /// <p> The Cost Category's effective start date.</p>
    #[serde(rename = "effectiveStart")]
    pub effective_start: String,
    #[serde(rename = "name")]
    pub name: String,
    /// <p> The list of processing statuses for Cost Management products for a specific cost category. </p>
    #[serde(rename = "processingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<Vec<CostCategoryProcessingStatus>>,
    #[serde(rename = "ruleVersion")]
    pub rule_version: String,
    /// <p> Rules are processed in order. If there are multiple rules that match the line item, then the first rule to match is used to determine that Cost Category value. </p>
    #[serde(rename = "rules")]
    pub rules: Vec<CostCategoryRule>,
}

/// <p>When creating or updating a cost category, you can define the <code>CostCategoryRule</code> rule type as <code>INHERITED_VALUE</code>. This rule type adds the flexibility of defining a rule that dynamically inherits the cost category value from the dimension value defined by <code>CostCategoryInheritedValueDimension</code>. For example, if you wanted to dynamically group costs based on the value of a specific tag key, you would first choose an inherited value rule type, then choose the tag dimension and specify the tag key to use.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CostCategoryInheritedValueDimension {
    /// <p>The key to extract cost category values.</p>
    #[serde(rename = "dimensionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_key: Option<String>,
    /// <p>The name of dimension for which to group costs.</p> <p>If you specify <code>LINKED_ACCOUNT_NAME</code>, the cost category value will be based on account name. If you specify <code>TAG</code>, the cost category value will be based on the value of the specified tag key.</p>
    #[serde(rename = "dimensionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<String>,
}

/// <p> The list of processing statuses for Cost Management products for a specific cost category. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CostCategoryProcessingStatus {
    /// <p> The Cost Management product name of the applied status. </p>
    #[serde(rename = "component")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    /// <p> The process status for a specific cost category. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>A reference to a Cost Category containing only enough information to identify the Cost Category.</p> <p>You can use this information to retrieve the full Cost Category information using <code>DescribeCostCategory</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CostCategoryReference {
    /// <p> The unique identifier for your Cost Category. </p>
    #[serde(rename = "costCategoryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p> The Cost Category's effective end date.</p>
    #[serde(rename = "effectiveEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_end: Option<String>,
    /// <p> The Cost Category's effective start date.</p>
    #[serde(rename = "effectiveStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The number of rules associated with a specific Cost Category. </p>
    #[serde(rename = "numberOfRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_rules: Option<i64>,
    /// <p> The list of processing statuses for Cost Management products for a specific cost category. </p>
    #[serde(rename = "processingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<Vec<CostCategoryProcessingStatus>>,
    /// <p> A list of unique cost category values in a specific cost category. </p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Rules are processed in order. If there are multiple rules that match the line item, then the first rule to match is used to determine that Cost Category value.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CostCategoryRule {
    /// <p>The value the line item will be categorized as, if the line item contains the matched dimension.</p>
    #[serde(rename = "inheritedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_value: Option<CostCategoryInheritedValueDimension>,
    /// <p>An <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> object used to categorize costs. This supports dimensions, tags, and nested expressions. Currently the only dimensions supported are <code>LINKED_ACCOUNT</code>, <code>SERVICE_CODE</code>, <code>RECORD_TYPE</code>, and <code>LINKED_ACCOUNT_NAME</code>.</p> <p>Root level <code>OR</code> is not supported. We recommend that you create a separate rule instead.</p> <p> <code>RECORD_TYPE</code> is a dimension used for Cost Explorer APIs, and is also supported for Cost Category expressions. This dimension uses different terms, depending on whether you're using the console or API/JSON editor. For a detailed comparison, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/manage-cost-categories.html#cost-categories-terms">Term Comparisons</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Expression>,
    /// <p>You can define the <code>CostCategoryRule</code> rule type as either <code>REGULAR</code> or <code>INHERITED_VALUE</code>. The <code>INHERITED_VALUE</code> rule type adds the flexibility of defining a rule that dynamically inherits the cost category value from the dimension value defined by <code>CostCategoryInheritedValueDimension</code>. For example, if you wanted to dynamically group costs based on the value of a specific tag key, you would first choose an inherited value rule type, then choose the tag dimension and specify the tag key to use.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The Cost Categories values used for filtering the costs.</p> <p>If <code>Values</code> and <code>Key</code> are not specified, the <code>ABSENT</code> <code>MatchOption</code> is applied to all Cost Categories. That is, filtering on resources that are not mapped to any Cost Categories.</p> <p>If <code>Values</code> is provided and <code>Key</code> is not specified, the <code>ABSENT</code> <code>MatchOption</code> is applied to the Cost Categories <code>Key</code> only. That is, filtering on resources without the given Cost Categories key.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CostCategoryValues {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p> The match options that you can use to filter your results. MatchOptions is only applicable for actions related to cost category. The default values for <code>MatchOptions</code> is <code>EQUALS</code> and <code>CASE_SENSITIVE</code>. </p>
    #[serde(rename = "matchOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    /// <p>The specific value of the Cost Category.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>The amount of instance usage that a reservation covered.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Coverage {
    /// <p>The amount of cost that the reservation covered.</p>
    #[serde(rename = "coverageCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_cost: Option<CoverageCost>,
    /// <p>The amount of instance usage that the reservation covered, in hours.</p>
    #[serde(rename = "coverageHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_hours: Option<CoverageHours>,
    /// <p>The amount of instance usage that the reservation covered, in normalized units.</p>
    #[serde(rename = "coverageNormalizedUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_normalized_units: Option<CoverageNormalizedUnits>,
}

/// <p>Reservation coverage for a specified period, in hours.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CoverageByTime {
    /// <p>The groups of instances that the reservation covered.</p>
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ReservationCoverageGroup>>,
    /// <p>The period that this coverage was used over.</p>
    #[serde(rename = "timePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
    /// <p>The total reservation coverage, in hours.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Coverage>,
}

/// <p>How much it costs to run an instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CoverageCost {
    /// <p>How much an On-Demand Instance costs.</p>
    #[serde(rename = "onDemandCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_cost: Option<String>,
}

/// <p>How long a running instance either used a reservation or was On-Demand.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CoverageHours {
    /// <p>The percentage of instance hours that a reservation covered.</p>
    #[serde(rename = "coverageHoursPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_hours_percentage: Option<String>,
    /// <p>The number of instance running hours that On-Demand Instances covered.</p>
    #[serde(rename = "onDemandHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_hours: Option<String>,
    /// <p>The number of instance running hours that reservations covered.</p>
    #[serde(rename = "reservedHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_hours: Option<String>,
    /// <p>The total instance usage, in hours.</p>
    #[serde(rename = "totalRunningHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_running_hours: Option<String>,
}

/// <p>The amount of instance usage, in normalized units. Normalized units enable you to see your EC2 usage for multiple sizes of instances in a uniform way. For example, suppose you run an xlarge instance and a 2xlarge instance. If you run both instances for the same amount of time, the 2xlarge instance uses twice as much of your reservation as the xlarge instance, even though both instances show only one instance-hour. Using normalized units instead of instance-hours, the xlarge instance used 8 normalized units, and the 2xlarge instance used 16 normalized units.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ri-modifying.html">Modifying Reserved Instances</a> in the <i>Amazon Elastic Compute Cloud User Guide for Linux Instances</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CoverageNormalizedUnits {
    /// <p>The percentage of your used instance normalized units that a reservation covers.</p>
    #[serde(rename = "coverageNormalizedUnitsPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_normalized_units_percentage: Option<String>,
    /// <p>The number of normalized units that are covered by On-Demand Instances instead of a reservation.</p>
    #[serde(rename = "onDemandNormalizedUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_normalized_units: Option<String>,
    /// <p>The number of normalized units that a reservation covers.</p>
    #[serde(rename = "reservedNormalizedUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_normalized_units: Option<String>,
    /// <p>The total number of normalized units that you used.</p>
    #[serde(rename = "totalRunningNormalizedUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_running_normalized_units: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAnomalyMonitorRequest {
    /// <p> The cost anomaly detection monitor object that you want to create.</p>
    #[serde(rename = "anomalyMonitor")]
    pub anomaly_monitor: AnomalyMonitor,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAnomalyMonitorResponse {
    /// <p> The unique identifier of your newly created cost anomaly detection monitor.</p>
    #[serde(rename = "monitorArn")]
    pub monitor_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAnomalySubscriptionRequest {
    /// <p> The cost anomaly subscription object that you want to create. </p>
    #[serde(rename = "anomalySubscription")]
    pub anomaly_subscription: AnomalySubscription,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAnomalySubscriptionResponse {
    /// <p> The unique identifier of your newly created cost anomaly subscription. </p>
    #[serde(rename = "subscriptionArn")]
    pub subscription_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCostCategoryDefinitionRequest {
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ruleVersion")]
    pub rule_version: String,
    /// <p>The Cost Category rules used to categorize costs. For more information, see <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_CostCategoryRule.html">CostCategoryRule</a>.</p>
    #[serde(rename = "rules")]
    pub rules: Vec<CostCategoryRule>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCostCategoryDefinitionResponse {
    /// <p> The unique identifier for your newly created Cost Category. </p>
    #[serde(rename = "costCategoryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
    /// <p> The Cost Category's effective start date. </p>
    #[serde(rename = "effectiveStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start: Option<String>,
}

/// <p>Context about the current instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CurrentInstance {
    /// <p> The currency code that AWS used to calculate the costs for this instance.</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>The name you've given an instance. This field will show as blank if you haven't given the instance a name.</p>
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// <p> Current On-Demand cost of operating this instance on a monthly basis.</p>
    #[serde(rename = "monthlyCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_cost: Option<String>,
    /// <p> Number of hours during the lookback period billed at On-Demand rates.</p>
    #[serde(rename = "onDemandHoursInLookbackPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_hours_in_lookback_period: Option<String>,
    /// <p> Number of hours during the lookback period covered by reservations.</p>
    #[serde(rename = "reservationCoveredHoursInLookbackPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_covered_hours_in_lookback_period: Option<String>,
    /// <p> Details about the resource and utilization.</p>
    #[serde(rename = "resourceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<ResourceDetails>,
    /// <p>Resource ID of the current instance.</p>
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p> Utilization information of the current instance during the lookback period.</p>
    #[serde(rename = "resourceUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_utilization: Option<ResourceUtilization>,
    /// <p>Number of hours during the lookback period covered by Savings Plans.</p>
    #[serde(rename = "savingsPlansCoveredHoursInLookbackPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_covered_hours_in_lookback_period: Option<String>,
    /// <p>Cost allocation resource tags applied to the instance.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagValues>>,
    /// <p> The total number of hours the instance ran during the lookback period.</p>
    #[serde(rename = "totalRunningHoursInLookbackPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_running_hours_in_lookback_period: Option<String>,
}

/// <p>The time period of the request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DateInterval {
    /// <p>The end of the time period. The end date is exclusive. For example, if <code>end</code> is <code>2017-05-01</code>, AWS retrieves cost and usage data from the start date up to, but not including, <code>2017-05-01</code>.</p>
    #[serde(rename = "end")]
    pub end: String,
    /// <p>The beginning of the time period. The start date is inclusive. For example, if <code>start</code> is <code>2017-01-01</code>, AWS retrieves cost and usage data starting at <code>2017-01-01</code> up to the end date. The start date must be equal to or no later than the current date to avoid a validation error.</p>
    #[serde(rename = "start")]
    pub start: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAnomalyMonitorRequest {
    /// <p> The unique identifier of the cost anomaly monitor that you want to delete. </p>
    #[serde(rename = "monitorArn")]
    pub monitor_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAnomalyMonitorResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAnomalySubscriptionRequest {
    /// <p> The unique identifier of the cost anomaly subscription that you want to delete. </p>
    #[serde(rename = "subscriptionArn")]
    pub subscription_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAnomalySubscriptionResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCostCategoryDefinitionRequest {
    /// <p> The unique identifier for your Cost Category. </p>
    #[serde(rename = "costCategoryArn")]
    pub cost_category_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCostCategoryDefinitionResponse {
    /// <p> The unique identifier for your Cost Category. </p>
    #[serde(rename = "costCategoryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
    /// <p> The effective end date of the Cost Category as a result of deleting it. No costs after this date will be categorized by the deleted Cost Category. </p>
    #[serde(rename = "effectiveEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_end: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCostCategoryDefinitionRequest {
    /// <p> The unique identifier for your Cost Category. </p>
    #[serde(rename = "costCategoryArn")]
    pub cost_category_arn: String,
    /// <p> The date when the Cost Category was effective. </p>
    #[serde(rename = "effectiveOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_on: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCostCategoryDefinitionResponse {
    #[serde(rename = "costCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category: Option<CostCategory>,
}

/// <p>The metadata that you can use to filter and group your results. You can use <code>GetDimensionValues</code> to find specific values.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DimensionValues {
    /// <p>The names of the metadata types that you can use to filter and group your results. For example, <code>AZ</code> returns a list of Availability Zones.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The match options that you can use to filter your results. <code>MatchOptions</code> is only applicable for actions related to Cost Category. The default values for <code>MatchOptions</code> are <code>EQUALS</code> and <code>CASE_SENSITIVE</code>.</p>
    #[serde(rename = "matchOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    /// <p>The metadata values that you can use to filter and group your results. You can use <code>GetDimensionValues</code> to find specific values.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>The metadata of a specific type that you can use to filter and group your results. You can use <code>GetDimensionValues</code> to find specific values.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DimensionValuesWithAttributes {
    /// <p>The attribute that applies to a specific <code>Dimension</code>.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The value of a dimension with a specific attribute.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p> The field that contains a list of disk (local storage) metrics associated with the current instance. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DiskResourceUtilization {
    /// <p> The maximum read throughput operations per second. </p>
    #[serde(rename = "diskReadBytesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_read_bytes_per_second: Option<String>,
    /// <p> The maximum number of read operations per second. </p>
    #[serde(rename = "diskReadOpsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_read_ops_per_second: Option<String>,
    /// <p> The maximum write throughput operations per second. </p>
    #[serde(rename = "diskWriteBytesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_write_bytes_per_second: Option<String>,
    /// <p> The maximum number of write operations per second. </p>
    #[serde(rename = "diskWriteOpsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_write_ops_per_second: Option<String>,
}

/// <p> The EBS field that contains a list of EBS metrics associated with the current instance. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EBSResourceUtilization {
    /// <p> The maximum size of read operations per second </p>
    #[serde(rename = "ebsReadBytesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_read_bytes_per_second: Option<String>,
    /// <p> The maximum number of read operations per second. </p>
    #[serde(rename = "ebsReadOpsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_read_ops_per_second: Option<String>,
    /// <p> The maximum size of write operations per second. </p>
    #[serde(rename = "ebsWriteBytesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_write_bytes_per_second: Option<String>,
    /// <p> The maximum number of write operations per second. </p>
    #[serde(rename = "ebsWriteOpsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_write_ops_per_second: Option<String>,
}

/// <p>Details about the Amazon EC2 instances that AWS recommends that you purchase.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EC2InstanceDetails {
    /// <p>The Availability Zone of the recommended reservation.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>Whether the recommendation is for a current-generation instance. </p>
    #[serde(rename = "currentGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    /// <p>The instance family of the recommended reservation.</p>
    #[serde(rename = "family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The type of instance that AWS recommends.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The platform of the recommended reservation. The platform is the specific combination of operating system, license model, and software on an instance.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The AWS Region of the recommended reservation.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Whether the recommended reservation is size flexible.</p>
    #[serde(rename = "sizeFlexEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
    /// <p>Whether the recommended reservation is dedicated or shared.</p>
    #[serde(rename = "tenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
}

/// <p> Details on the Amazon EC2 Resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EC2ResourceDetails {
    /// <p> Hourly public On-Demand rate for the instance type.</p>
    #[serde(rename = "hourlyOnDemandRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_on_demand_rate: Option<String>,
    /// <p> The type of AWS instance.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p> Memory capacity of the AWS instance.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// <p> Network performance capacity of the AWS instance.</p>
    #[serde(rename = "networkPerformance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_performance: Option<String>,
    /// <p> The platform of the AWS instance. The platform is the specific combination of operating system, license model, and software on an instance.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p> The AWS Region of the instance.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p> The SKU of the product.</p>
    #[serde(rename = "sku")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// <p> The disk storage of the AWS instance (not EBS storage).</p>
    #[serde(rename = "storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    /// <p> Number of VCPU cores in the AWS instance type.</p>
    #[serde(rename = "vcpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu: Option<String>,
}

/// <p> Utilization metrics of the instance. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EC2ResourceUtilization {
    /// <p> The field that contains a list of disk (local storage) metrics associated with the current instance. </p>
    #[serde(rename = "diskResourceUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_resource_utilization: Option<DiskResourceUtilization>,
    /// <p> The EBS field that contains a list of EBS metrics associated with the current instance. </p>
    #[serde(rename = "eBSResourceUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_resource_utilization: Option<EBSResourceUtilization>,
    /// <p> Maximum observed or expected CPU utilization of the instance.</p>
    #[serde(rename = "maxCpuUtilizationPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cpu_utilization_percentage: Option<String>,
    /// <p> Maximum observed or expected memory utilization of the instance.</p>
    #[serde(rename = "maxMemoryUtilizationPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory_utilization_percentage: Option<String>,
    /// <p> Maximum observed or expected storage utilization of the instance (does not measure EBS storage).</p>
    #[serde(rename = "maxStorageUtilizationPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_utilization_percentage: Option<String>,
    /// <p> The network field that contains a list of network metrics associated with the current instance. </p>
    #[serde(rename = "networkResourceUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_resource_utilization: Option<NetworkResourceUtilization>,
}

/// <p>The Amazon EC2 hardware specifications that you want AWS to provide recommendations for.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EC2Specification {
    /// <p>Whether you want a recommendation for standard or convertible reservations.</p>
    #[serde(rename = "offeringClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_class: Option<String>,
}

/// <p>Details about the Amazon ES instances that AWS recommends that you purchase.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ESInstanceDetails {
    /// <p>Whether the recommendation is for a current-generation instance.</p>
    #[serde(rename = "currentGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    /// <p>The class of instance that AWS recommends.</p>
    #[serde(rename = "instanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// <p>The size of instance that AWS recommends.</p>
    #[serde(rename = "instanceSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_size: Option<String>,
    /// <p>The AWS Region of the recommended reservation.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Whether the recommended reservation is size flexible.</p>
    #[serde(rename = "sizeFlexEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

/// <p>Details about the Amazon ElastiCache instances that AWS recommends that you purchase.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ElastiCacheInstanceDetails {
    /// <p>Whether the recommendation is for a current generation instance.</p>
    #[serde(rename = "currentGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    /// <p>The instance family of the recommended reservation.</p>
    #[serde(rename = "family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The type of node that AWS recommends.</p>
    #[serde(rename = "nodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <p>The description of the recommended reservation.</p>
    #[serde(rename = "productDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// <p>The AWS Region of the recommended reservation.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Whether the recommended reservation is size flexible.</p>
    #[serde(rename = "sizeFlexEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

/// <p><p>Use <code>Expression</code> to filter by cost or by usage. There are two patterns: </p> <ul> <li> <p>Simple dimension values - You can set the dimension name and values for the filters that you plan to use. For example, you can filter for <code>REGION==us-east-1 OR REGION==us-west-1</code>. For <code>GetRightsizingRecommendation</code>, the Region is a full name (for example, <code>REGION==US East (N. Virginia)</code>. The <code>Expression</code> example looks like:</p> <p> <code>{ &quot;Dimensions&quot;: { &quot;Key&quot;: &quot;REGION&quot;, &quot;Values&quot;: [ &quot;us-east-1&quot;, “us-west-1” ] } }</code> </p> <p>The list of dimension values are OR&#39;d together to retrieve cost or usage data. You can create <code>Expression</code> and <code>DimensionValues</code> objects using either <code>with<em></code> methods or <code>set</em></code> methods in multiple lines. </p> </li> <li> <p>Compound dimension values with logical operations - You can use multiple <code>Expression</code> types and the logical operators <code>AND/OR/NOT</code> to create a list of one or more <code>Expression</code> objects. This allows you to filter on more advanced options. For example, you can filter on <code>((REGION == us-east-1 OR REGION == us-west-1) OR (TAG.Type == Type1)) AND (USAGE<em>TYPE != DataTransfer)</code>. The <code>Expression</code> for that looks like this:</p> <p> <code>{ &quot;And&quot;: [ {&quot;Or&quot;: [ {&quot;Dimensions&quot;: { &quot;Key&quot;: &quot;REGION&quot;, &quot;Values&quot;: [ &quot;us-east-1&quot;, &quot;us-west-1&quot; ] }}, {&quot;Tags&quot;: { &quot;Key&quot;: &quot;TagName&quot;, &quot;Values&quot;: [&quot;Value1&quot;] } } ]}, {&quot;Not&quot;: {&quot;Dimensions&quot;: { &quot;Key&quot;: &quot;USAGE</em>TYPE&quot;, &quot;Values&quot;: [&quot;DataTransfer&quot;] }}} ] } </code> </p> <note> <p>Because each <code>Expression</code> can have only one operator, the service returns an error if more than one is specified. The following example shows an <code>Expression</code> object that creates an error.</p> </note> <p> <code> { &quot;And&quot;: [ ... ], &quot;DimensionValues&quot;: { &quot;Dimension&quot;: &quot;USAGE<em>TYPE&quot;, &quot;Values&quot;: [ &quot;DataTransfer&quot; ] } } </code> </p> </li> </ul> <note> <p>For the <code>GetRightsizingRecommendation</code> action, a combination of OR and NOT is not supported. OR is not supported between different dimensions, or dimensions and tags. NOT operators aren&#39;t supported. Dimensions are also limited to <code>LINKED</em>ACCOUNT</code>, <code>REGION</code>, or <code>RIGHTSIZING<em>TYPE</code>.</p> <p>For the <code>GetReservationPurchaseRecommendation</code> action, only NOT is supported. AND and OR are not supported. Dimensions are limited to <code>LINKED</em>ACCOUNT</code>.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Expression {
    /// <p>Return results that match both <code>Dimension</code> objects.</p>
    #[serde(rename = "and")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Expression>>,
    /// <p>The filter based on <code>CostCategory</code> values.</p>
    #[serde(rename = "costCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_categories: Option<CostCategoryValues>,
    /// <p>The specific <code>Dimension</code> to use for <code>Expression</code>.</p>
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<DimensionValues>,
    /// <p>Return results that don't match a <code>Dimension</code> object.</p>
    #[serde(rename = "not")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Box<Option<Expression>>,
    /// <p>Return results that match either <code>Dimension</code> object.</p>
    #[serde(rename = "or")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Expression>>,
    /// <p>The specific <code>Tag</code> to use for <code>Expression</code>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagValues>,
}

/// <p>The forecast created for your query.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ForecastResult {
    /// <p>The mean value of the forecast.</p>
    #[serde(rename = "meanValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_value: Option<String>,
    /// <p>The lower limit for the prediction interval. </p>
    #[serde(rename = "predictionIntervalLowerBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval_lower_bound: Option<String>,
    /// <p>The upper limit for the prediction interval. </p>
    #[serde(rename = "predictionIntervalUpperBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval_upper_bound: Option<String>,
    /// <p>The period of time that the forecast covers.</p>
    #[serde(rename = "timePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAnomaliesRequest {
    /// <p>Assigns the start and end dates for retrieving cost anomalies. The returned anomaly object will have an <code>AnomalyEndDate</code> in the specified time range. </p>
    #[serde(rename = "dateInterval")]
    pub date_interval: AnomalyDateInterval,
    /// <p>Filters anomaly results by the feedback field on the anomaly object. </p>
    #[serde(rename = "feedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    /// <p> The number of entries a paginated response contains. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Retrieves all of the cost anomalies detected for a specific cost anomaly monitor Amazon Resource Name (ARN). </p>
    #[serde(rename = "monitorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
    /// <p> The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size. </p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Filters anomaly results by the total impact field on the anomaly object. For example, you can filter anomalies <code>GREATER_THAN 200.00</code> to retrieve anomalies, with an estimated dollar impact greater than 200. </p>
    #[serde(rename = "totalImpact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_impact: Option<TotalImpactFilter>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAnomaliesResponse {
    /// <p> A list of cost anomalies. </p>
    #[serde(rename = "anomalies")]
    pub anomalies: Vec<Anomaly>,
    /// <p> The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size. </p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAnomalyMonitorsRequest {
    /// <p> The number of entries a paginated response contains. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> A list of cost anomaly monitor ARNs. </p>
    #[serde(rename = "monitorArnList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn_list: Option<Vec<String>>,
    /// <p> The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size. </p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAnomalyMonitorsResponse {
    /// <p> A list of cost anomaly monitors that includes the detailed metadata for each monitor. </p>
    #[serde(rename = "anomalyMonitors")]
    pub anomaly_monitors: Vec<AnomalyMonitor>,
    /// <p> The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size. </p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAnomalySubscriptionsRequest {
    /// <p> The number of entries a paginated response contains. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> Cost anomaly monitor ARNs. </p>
    #[serde(rename = "monitorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
    /// <p> The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size. </p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p> A list of cost anomaly subscription ARNs. </p>
    #[serde(rename = "subscriptionArnList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_arn_list: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAnomalySubscriptionsResponse {
    /// <p> A list of cost anomaly subscriptions that includes the detailed metadata for each one. </p>
    #[serde(rename = "anomalySubscriptions")]
    pub anomaly_subscriptions: Vec<AnomalySubscription>,
    /// <p> The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size. </p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCostAndUsageRequest {
    /// <p>Filters AWS costs by different dimensions. For example, you can specify <code>SERVICE</code> and <code>LINKED_ACCOUNT</code> and get the costs that are associated with that account's usage of that service. You can nest <code>Expression</code> objects to define any combination of dimension filters. For more information, see <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a>. </p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>Sets the AWS cost granularity to <code>MONTHLY</code> or <code>DAILY</code>, or <code>HOURLY</code>. If <code>Granularity</code> isn't set, the response object doesn't include the <code>Granularity</code>, either <code>MONTHLY</code> or <code>DAILY</code>, or <code>HOURLY</code>. </p>
    #[serde(rename = "granularity")]
    pub granularity: String,
    /// <p>You can group AWS costs using up to two different groups, either dimensions, tag keys, cost categories, or any two group by types.</p> <p>When you group by tag key, you get all tag values, including empty strings.</p> <p>Valid values are <code>AZ</code>, <code>INSTANCE_TYPE</code>, <code>LEGAL_ENTITY_NAME</code>, <code>LINKED_ACCOUNT</code>, <code>OPERATION</code>, <code>PLATFORM</code>, <code>PURCHASE_TYPE</code>, <code>SERVICE</code>, <code>TAGS</code>, <code>TENANCY</code>, <code>RECORD_TYPE</code>, and <code>USAGE_TYPE</code>.</p>
    #[serde(rename = "groupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    /// <p>Which metrics are returned in the query. For more information about blended and unblended rates, see <a href="http://aws.amazon.com/premiumsupport/knowledge-center/blended-rates-intro/">Why does the "blended" annotation appear on some line items in my bill?</a>. </p> <p>Valid values are <code>AmortizedCost</code>, <code>BlendedCost</code>, <code>NetAmortizedCost</code>, <code>NetUnblendedCost</code>, <code>NormalizedUsageAmount</code>, <code>UnblendedCost</code>, and <code>UsageQuantity</code>. </p> <note> <p>If you return the <code>UsageQuantity</code> metric, the service aggregates all usage numbers without taking into account the units. For example, if you aggregate <code>usageQuantity</code> across all of Amazon EC2, the results aren't meaningful because Amazon EC2 compute hours and data transfer are measured in different units (for example, hours vs. GB). To get more meaningful <code>UsageQuantity</code> metrics, filter by <code>UsageType</code> or <code>UsageTypeGroups</code>. </p> </note> <p> <code>Metrics</code> is required for <code>GetCostAndUsage</code> requests.</p>
    #[serde(rename = "metrics")]
    pub metrics: Vec<String>,
    /// <p>The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Sets the start and end dates for retrieving AWS costs. The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>.</p>
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCostAndUsageResponse {
    /// <p>The attributes that apply to a specific dimension value. For example, if the value is a linked account, the attribute is that account name.</p>
    #[serde(rename = "dimensionValueAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_value_attributes: Option<Vec<DimensionValuesWithAttributes>>,
    /// <p>The groups that are specified by the <code>Filter</code> or <code>GroupBy</code> parameters in the request.</p>
    #[serde(rename = "groupDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_definitions: Option<Vec<GroupDefinition>>,
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The time period that is covered by the results in the response.</p>
    #[serde(rename = "resultsByTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_by_time: Option<Vec<ResultByTime>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCostAndUsageWithResourcesRequest {
    /// <p>Filters Amazon Web Services costs by different dimensions. For example, you can specify <code>SERVICE</code> and <code>LINKED_ACCOUNT</code> and get the costs that are associated with that account's usage of that service. You can nest <code>Expression</code> objects to define any combination of dimension filters. For more information, see <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a>. </p> <p>The <code>GetCostAndUsageWithResources</code> operation requires that you either group by or filter by a <code>ResourceId</code>. It requires the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> <code>"SERVICE = Amazon Elastic Compute Cloud - Compute"</code> in the filter.</p>
    #[serde(rename = "filter")]
    pub filter: Expression,
    /// <p>Sets the AWS cost granularity to <code>MONTHLY</code>, <code>DAILY</code>, or <code>HOURLY</code>. If <code>Granularity</code> isn't set, the response object doesn't include the <code>Granularity</code>, <code>MONTHLY</code>, <code>DAILY</code>, or <code>HOURLY</code>. </p>
    #[serde(rename = "granularity")]
    pub granularity: String,
    /// <p>You can group Amazon Web Services costs using up to two different groups: <code>DIMENSION</code>, <code>TAG</code>, <code>COST_CATEGORY</code>.</p>
    #[serde(rename = "groupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    /// <p>Which metrics are returned in the query. For more information about blended and unblended rates, see <a href="http://aws.amazon.com/premiumsupport/knowledge-center/blended-rates-intro/">Why does the "blended" annotation appear on some line items in my bill?</a>. </p> <p>Valid values are <code>AmortizedCost</code>, <code>BlendedCost</code>, <code>NetAmortizedCost</code>, <code>NetUnblendedCost</code>, <code>NormalizedUsageAmount</code>, <code>UnblendedCost</code>, and <code>UsageQuantity</code>. </p> <note> <p>If you return the <code>UsageQuantity</code> metric, the service aggregates all usage numbers without taking the units into account. For example, if you aggregate <code>usageQuantity</code> across all of Amazon EC2, the results aren't meaningful because Amazon EC2 compute hours and data transfer are measured in different units (for example, hours vs. GB). To get more meaningful <code>UsageQuantity</code> metrics, filter by <code>UsageType</code> or <code>UsageTypeGroups</code>. </p> </note> <p> <code>Metrics</code> is required for <code>GetCostAndUsageWithResources</code> requests.</p>
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    /// <p>The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Sets the start and end dates for retrieving Amazon Web Services costs. The range must be within the last 14 days (the start date cannot be earlier than 14 days ago). The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>.</p>
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCostAndUsageWithResourcesResponse {
    /// <p>The attributes that apply to a specific dimension value. For example, if the value is a linked account, the attribute is that account name.</p>
    #[serde(rename = "dimensionValueAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_value_attributes: Option<Vec<DimensionValuesWithAttributes>>,
    /// <p>The groups that are specified by the <code>Filter</code> or <code>GroupBy</code> parameters in the request.</p>
    #[serde(rename = "groupDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_definitions: Option<Vec<GroupDefinition>>,
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The time period that is covered by the results in the response.</p>
    #[serde(rename = "resultsByTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_by_time: Option<Vec<ResultByTime>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCostCategoriesRequest {
    #[serde(rename = "costCategoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_name: Option<String>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>This field is only used when <code>SortBy</code> is provided in the request.</p> <p>The maximum number of objects that to be returned for this request. If <code>MaxResults</code> is not specified with <code>SortBy</code>, the request will return 1000 results as the default value for this parameter.</p> <p>For <code>GetCostCategories</code>, MaxResults has an upper limit of 1000.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the number of objects that are still available for retrieval exceeds the limit, AWS returns a NextPageToken value in the response. To retrieve the next batch of objects, provide the NextPageToken from the prior call in your next request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The value that you want to search the filter values for.</p> <p>If you do not specify a <code>CostCategoryName</code>, <code>SearchString</code> will be used to filter Cost Category names that match the <code>SearchString</code> pattern. If you do specifiy a <code>CostCategoryName</code>, <code>SearchString</code> will be used to filter Cost Category values that match the <code>SearchString</code> pattern.</p>
    #[serde(rename = "searchString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    /// <p>The value by which you want to sort the data.</p> <p>The key represents cost and usage metrics. The following values are supported:</p> <ul> <li> <p> <code>BlendedCost</code> </p> </li> <li> <p> <code>UnblendedCost</code> </p> </li> <li> <p> <code>AmortizedCost</code> </p> </li> <li> <p> <code>NetAmortizedCost</code> </p> </li> <li> <p> <code>NetUnblendedCost</code> </p> </li> <li> <p> <code>UsageQuantity</code> </p> </li> <li> <p> <code>NormalizedUsageAmount</code> </p> </li> </ul> <p>Supported values for <code>SortOrder</code> are <code>ASCENDING</code> or <code>DESCENDING</code>.</p> <p>When using <code>SortBy</code>, <code>NextPageToken</code> and <code>SearchString</code> are not supported.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Vec<SortDefinition>>,
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCostCategoriesResponse {
    /// <p>The names of the Cost Categories.</p>
    #[serde(rename = "costCategoryNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_names: Option<Vec<String>>,
    /// <p>The Cost Category values.</p> <p> <code>CostCategoryValues</code> are not returned if <code>CostCategoryName</code> is not specified in the request. </p>
    #[serde(rename = "costCategoryValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_values: Option<Vec<String>>,
    /// <p>If the number of objects that are still available for retrieval exceeds the limit, AWS returns a NextPageToken value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The number of objects returned.</p>
    #[serde(rename = "returnSize")]
    pub return_size: i64,
    /// <p>The total number of objects.</p>
    #[serde(rename = "totalSize")]
    pub total_size: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCostForecastRequest {
    /// <p><p>The filters that you want to use to filter your forecast. The <code>GetCostForecast</code> API supports filtering by the following dimensions:</p> <ul> <li> <p> <code>AZ</code> </p> </li> <li> <p> <code>INSTANCE<em>TYPE</code> </p> </li> <li> <p> <code>LINKED</em>ACCOUNT</code> </p> </li> <li> <p> <code>LINKED<em>ACCOUNT</em>NAME</code> </p> </li> <li> <p> <code>OPERATION</code> </p> </li> <li> <p> <code>PURCHASE<em>TYPE</code> </p> </li> <li> <p> <code>REGION</code> </p> </li> <li> <p> <code>SERVICE</code> </p> </li> <li> <p> <code>USAGE</em>TYPE</code> </p> </li> <li> <p> <code>USAGE<em>TYPE</em>GROUP</code> </p> </li> <li> <p> <code>RECORD<em>TYPE</code> </p> </li> <li> <p> <code>OPERATING</em>SYSTEM</code> </p> </li> <li> <p> <code>TENANCY</code> </p> </li> <li> <p> <code>SCOPE</code> </p> </li> <li> <p> <code>PLATFORM</code> </p> </li> <li> <p> <code>SUBSCRIPTION<em>ID</code> </p> </li> <li> <p> <code>LEGAL</em>ENTITY<em>NAME</code> </p> </li> <li> <p> <code>DEPLOYMENT</em>OPTION</code> </p> </li> <li> <p> <code>DATABASE<em>ENGINE</code> </p> </li> <li> <p> <code>INSTANCE</em>TYPE<em>FAMILY</code> </p> </li> <li> <p> <code>BILLING</em>ENTITY</code> </p> </li> <li> <p> <code>RESERVATION<em>ID</code> </p> </li> <li> <p> <code>SAVINGS</em>PLAN_ARN</code> </p> </li> </ul></p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>How granular you want the forecast to be. You can get 3 months of <code>DAILY</code> forecasts or 12 months of <code>MONTHLY</code> forecasts.</p> <p>The <code>GetCostForecast</code> operation supports only <code>DAILY</code> and <code>MONTHLY</code> granularities.</p>
    #[serde(rename = "granularity")]
    pub granularity: String,
    /// <p><p>Which metric Cost Explorer uses to create your forecast. For more information about blended and unblended rates, see <a href="http://aws.amazon.com/premiumsupport/knowledge-center/blended-rates-intro/">Why does the &quot;blended&quot; annotation appear on some line items in my bill?</a>. </p> <p>Valid values for a <code>GetCostForecast</code> call are the following:</p> <ul> <li> <p>AMORTIZED<em>COST</p> </li> <li> <p>BLENDED</em>COST</p> </li> <li> <p>NET<em>AMORTIZED</em>COST</p> </li> <li> <p>NET<em>UNBLENDED</em>COST</p> </li> <li> <p>UNBLENDED_COST</p> </li> </ul></p>
    #[serde(rename = "metric")]
    pub metric: String,
    /// <p>Cost Explorer always returns the mean forecast as a single point. You can request a prediction interval around the mean by specifying a confidence level. The higher the confidence level, the more confident Cost Explorer is about the actual value falling in the prediction interval. Higher confidence levels result in wider prediction intervals.</p>
    #[serde(rename = "predictionIntervalLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval_level: Option<i64>,
    /// <p>The period of time that you want the forecast to cover. The start date must be equal to or no later than the current date to avoid a validation error.</p>
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCostForecastResponse {
    /// <p>The forecasts for your query, in order. For <code>DAILY</code> forecasts, this is a list of days. For <code>MONTHLY</code> forecasts, this is a list of months.</p>
    #[serde(rename = "forecastResultsByTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_results_by_time: Option<Vec<ForecastResult>>,
    /// <p>How much you are forecasted to spend over the forecast period, in <code>USD</code>.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<MetricValue>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDimensionValuesRequest {
    /// <p><p>The context for the call to <code>GetDimensionValues</code>. This can be <code>RESERVATIONS</code> or <code>COST<em>AND</em>USAGE</code>. The default value is <code>COST<em>AND</em>USAGE</code>. If the context is set to <code>RESERVATIONS</code>, the resulting dimension values can be used in the <code>GetReservationUtilization</code> operation. If the context is set to <code>COST<em>AND</em>USAGE</code>, the resulting dimension values can be used in the <code>GetCostAndUsage</code> operation.</p> <p>If you set the context to <code>COST<em>AND</em>USAGE</code>, you can use the following dimensions for searching:</p> <ul> <li> <p>AZ - The Availability Zone. An example is <code>us-east-1a</code>.</p> </li> <li> <p>DATABASE<em>ENGINE - The Amazon Relational Database Service database. Examples are Aurora or MySQL.</p> </li> <li> <p>INSTANCE</em>TYPE - The type of Amazon EC2 instance. An example is <code>m4.xlarge</code>.</p> </li> <li> <p>LEGAL<em>ENTITY</em>NAME - The name of the organization that sells you AWS services, such as Amazon Web Services.</p> </li> <li> <p>LINKED<em>ACCOUNT - The description in the attribute map that includes the full name of the member account. The value field contains the AWS ID of the member account.</p> </li> <li> <p>OPERATING</em>SYSTEM - The operating system. Examples are Windows or Linux.</p> </li> <li> <p>OPERATION - The action performed. Examples include <code>RunInstance</code> and <code>CreateBucket</code>.</p> </li> <li> <p>PLATFORM - The Amazon EC2 operating system. Examples are Windows or Linux.</p> </li> <li> <p>PURCHASE<em>TYPE - The reservation type of the purchase to which this usage is related. Examples include On-Demand Instances and Standard Reserved Instances.</p> </li> <li> <p>SERVICE - The AWS service such as Amazon DynamoDB.</p> </li> <li> <p>USAGE</em>TYPE - The type of usage. An example is DataTransfer-In-Bytes. The response for the <code>GetDimensionValues</code> operation includes a unit attribute. Examples include GB and Hrs.</p> </li> <li> <p>USAGE<em>TYPE</em>GROUP - The grouping of common usage types. An example is Amazon EC2: CloudWatch – Alarms. The response for this operation includes a unit attribute.</p> </li> <li> <p>REGION - The AWS Region.</p> </li> <li> <p>RECORD<em>TYPE - The different types of charges such as RI fees, usage costs, tax refunds, and credits.</p> </li> <li> <p>RESOURCE</em>ID - The unique identifier of the resource. ResourceId is an opt-in feature only available for last 14 days for EC2-Compute Service.</p> </li> </ul> <p>If you set the context to <code>RESERVATIONS</code>, you can use the following dimensions for searching:</p> <ul> <li> <p>AZ - The Availability Zone. An example is <code>us-east-1a</code>.</p> </li> <li> <p>CACHE<em>ENGINE - The Amazon ElastiCache operating system. Examples are Windows or Linux.</p> </li> <li> <p>DEPLOYMENT</em>OPTION - The scope of Amazon Relational Database Service deployments. Valid values are <code>SingleAZ</code> and <code>MultiAZ</code>.</p> </li> <li> <p>INSTANCE<em>TYPE - The type of Amazon EC2 instance. An example is <code>m4.xlarge</code>.</p> </li> <li> <p>LINKED</em>ACCOUNT - The description in the attribute map that includes the full name of the member account. The value field contains the AWS ID of the member account.</p> </li> <li> <p>PLATFORM - The Amazon EC2 operating system. Examples are Windows or Linux.</p> </li> <li> <p>REGION - The AWS Region.</p> </li> <li> <p>SCOPE (Utilization only) - The scope of a Reserved Instance (RI). Values are regional or a single Availability Zone.</p> </li> <li> <p>TAG (Coverage only) - The tags that are associated with a Reserved Instance (RI).</p> </li> <li> <p>TENANCY - The tenancy of a resource. Examples are shared or dedicated.</p> </li> </ul> <p>If you set the context to <code>SAVINGS<em>PLANS</code>, you can use the following dimensions for searching:</p> <ul> <li> <p>SAVINGS</em>PLANS<em>TYPE - Type of Savings Plans (EC2 Instance or Compute)</p> </li> <li> <p>PAYMENT</em>OPTION - Payment option for the given Savings Plans (for example, All Upfront)</p> </li> <li> <p>REGION - The AWS Region.</p> </li> <li> <p>INSTANCE<em>TYPE</em>FAMILY - The family of instances (For example, <code>m5</code>)</p> </li> <li> <p>LINKED<em>ACCOUNT - The description in the attribute map that includes the full name of the member account. The value field contains the AWS ID of the member account.</p> </li> <li> <p>SAVINGS</em>PLAN_ARN - The unique identifier for your Savings Plan</p> </li> </ul></p>
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// <p>The name of the dimension. Each <code>Dimension</code> is available for a different <code>Context</code>. For more information, see <code>Context</code>. </p>
    #[serde(rename = "dimension")]
    pub dimension: String,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>This field is only used when SortBy is provided in the request. The maximum number of objects that to be returned for this request. If MaxResults is not specified with SortBy, the request will return 1000 results as the default value for this parameter.</p> <p>For <code>GetDimensionValues</code>, MaxResults has an upper limit of 1000.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The value that you want to search the filter values for.</p>
    #[serde(rename = "searchString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    /// <p>The value by which you want to sort the data.</p> <p>The key represents cost and usage metrics. The following values are supported:</p> <ul> <li> <p> <code>BlendedCost</code> </p> </li> <li> <p> <code>UnblendedCost</code> </p> </li> <li> <p> <code>AmortizedCost</code> </p> </li> <li> <p> <code>NetAmortizedCost</code> </p> </li> <li> <p> <code>NetUnblendedCost</code> </p> </li> <li> <p> <code>UsageQuantity</code> </p> </li> <li> <p> <code>NormalizedUsageAmount</code> </p> </li> </ul> <p>Supported values for <code>SortOrder</code> are <code>ASCENDING</code> or <code>DESCENDING</code>.</p> <p>When you specify a <code>SortBy</code> paramater, the context must be <code>COST_AND_USAGE</code>. Further, when using <code>SortBy</code>, <code>NextPageToken</code> and <code>SearchString</code> are not supported.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Vec<SortDefinition>>,
    /// <p>The start and end dates for retrieving the dimension values. The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>.</p>
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDimensionValuesResponse {
    /// <p><p>The filters that you used to filter your request. Some dimensions are available only for a specific context.</p> <p>If you set the context to <code>COST<em>AND</em>USAGE</code>, you can use the following dimensions for searching:</p> <ul> <li> <p>AZ - The Availability Zone. An example is <code>us-east-1a</code>.</p> </li> <li> <p>DATABASE<em>ENGINE - The Amazon Relational Database Service database. Examples are Aurora or MySQL.</p> </li> <li> <p>INSTANCE</em>TYPE - The type of Amazon EC2 instance. An example is <code>m4.xlarge</code>.</p> </li> <li> <p>LEGAL<em>ENTITY</em>NAME - The name of the organization that sells you AWS services, such as Amazon Web Services.</p> </li> <li> <p>LINKED<em>ACCOUNT - The description in the attribute map that includes the full name of the member account. The value field contains the AWS ID of the member account.</p> </li> <li> <p>OPERATING</em>SYSTEM - The operating system. Examples are Windows or Linux.</p> </li> <li> <p>OPERATION - The action performed. Examples include <code>RunInstance</code> and <code>CreateBucket</code>.</p> </li> <li> <p>PLATFORM - The Amazon EC2 operating system. Examples are Windows or Linux.</p> </li> <li> <p>PURCHASE<em>TYPE - The reservation type of the purchase to which this usage is related. Examples include On-Demand Instances and Standard Reserved Instances.</p> </li> <li> <p>SERVICE - The AWS service such as Amazon DynamoDB.</p> </li> <li> <p>USAGE</em>TYPE - The type of usage. An example is DataTransfer-In-Bytes. The response for the <code>GetDimensionValues</code> operation includes a unit attribute. Examples include GB and Hrs.</p> </li> <li> <p>USAGE<em>TYPE</em>GROUP - The grouping of common usage types. An example is Amazon EC2: CloudWatch – Alarms. The response for this operation includes a unit attribute.</p> </li> <li> <p>RECORD<em>TYPE - The different types of charges such as RI fees, usage costs, tax refunds, and credits.</p> </li> <li> <p>RESOURCE</em>ID - The unique identifier of the resource. ResourceId is an opt-in feature only available for last 14 days for EC2-Compute Service.</p> </li> </ul> <p>If you set the context to <code>RESERVATIONS</code>, you can use the following dimensions for searching:</p> <ul> <li> <p>AZ - The Availability Zone. An example is <code>us-east-1a</code>.</p> </li> <li> <p>CACHE<em>ENGINE - The Amazon ElastiCache operating system. Examples are Windows or Linux.</p> </li> <li> <p>DEPLOYMENT</em>OPTION - The scope of Amazon Relational Database Service deployments. Valid values are <code>SingleAZ</code> and <code>MultiAZ</code>.</p> </li> <li> <p>INSTANCE<em>TYPE - The type of Amazon EC2 instance. An example is <code>m4.xlarge</code>.</p> </li> <li> <p>LINKED</em>ACCOUNT - The description in the attribute map that includes the full name of the member account. The value field contains the AWS ID of the member account.</p> </li> <li> <p>PLATFORM - The Amazon EC2 operating system. Examples are Windows or Linux.</p> </li> <li> <p>REGION - The AWS Region.</p> </li> <li> <p>SCOPE (Utilization only) - The scope of a Reserved Instance (RI). Values are regional or a single Availability Zone.</p> </li> <li> <p>TAG (Coverage only) - The tags that are associated with a Reserved Instance (RI).</p> </li> <li> <p>TENANCY - The tenancy of a resource. Examples are shared or dedicated.</p> </li> </ul> <p>If you set the context to <code>SAVINGS<em>PLANS</code>, you can use the following dimensions for searching:</p> <ul> <li> <p>SAVINGS</em>PLANS<em>TYPE - Type of Savings Plans (EC2 Instance or Compute)</p> </li> <li> <p>PAYMENT</em>OPTION - Payment option for the given Savings Plans (for example, All Upfront)</p> </li> <li> <p>REGION - The AWS Region.</p> </li> <li> <p>INSTANCE<em>TYPE</em>FAMILY - The family of instances (For example, <code>m5</code>)</p> </li> <li> <p>LINKED<em>ACCOUNT - The description in the attribute map that includes the full name of the member account. The value field contains the AWS ID of the member account.</p> </li> <li> <p>SAVINGS</em>PLAN_ARN - The unique identifier for your Savings Plan</p> </li> </ul></p>
    #[serde(rename = "dimensionValues")]
    pub dimension_values: Vec<DimensionValuesWithAttributes>,
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The number of results that AWS returned at one time.</p>
    #[serde(rename = "returnSize")]
    pub return_size: i64,
    /// <p>The total number of search results.</p>
    #[serde(rename = "totalSize")]
    pub total_size: i64,
}

/// <p>You can use the following request parameters to query for how much of your instance usage a reservation covered.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetReservationCoverageRequest {
    /// <p>Filters utilization data by dimensions. You can filter by the following dimensions:</p> <ul> <li> <p>AZ</p> </li> <li> <p>CACHE_ENGINE</p> </li> <li> <p>DATABASE_ENGINE</p> </li> <li> <p>DEPLOYMENT_OPTION</p> </li> <li> <p>INSTANCE_TYPE</p> </li> <li> <p>LINKED_ACCOUNT</p> </li> <li> <p>OPERATING_SYSTEM</p> </li> <li> <p>PLATFORM</p> </li> <li> <p>REGION</p> </li> <li> <p>SERVICE</p> </li> <li> <p>TAG</p> </li> <li> <p>TENANCY</p> </li> </ul> <p> <code>GetReservationCoverage</code> uses the same <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> object as the other operations, but only <code>AND</code> is supported among each dimension. You can nest only one level deep. If there are multiple values for a dimension, they are OR'd together.</p> <p>If you don't provide a <code>SERVICE</code> filter, Cost Explorer defaults to EC2.</p> <p>Cost category is also supported.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>The granularity of the AWS cost data for the reservation. Valid values are <code>MONTHLY</code> and <code>DAILY</code>.</p> <p>If <code>GroupBy</code> is set, <code>Granularity</code> can't be set. If <code>Granularity</code> isn't set, the response object doesn't include <code>Granularity</code>, either <code>MONTHLY</code> or <code>DAILY</code>.</p> <p>The <code>GetReservationCoverage</code> operation supports only <code>DAILY</code> and <code>MONTHLY</code> granularities.</p>
    #[serde(rename = "granularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    /// <p><p>You can group the data by the following attributes:</p> <ul> <li> <p>AZ</p> </li> <li> <p>CACHE<em>ENGINE</p> </li> <li> <p>DATABASE</em>ENGINE</p> </li> <li> <p>DEPLOYMENT<em>OPTION</p> </li> <li> <p>INSTANCE</em>TYPE</p> </li> <li> <p>LINKED<em>ACCOUNT</p> </li> <li> <p>OPERATING</em>SYSTEM</p> </li> <li> <p>PLATFORM</p> </li> <li> <p>REGION</p> </li> <li> <p>TENANCY</p> </li> </ul></p>
    #[serde(rename = "groupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    /// <p>The maximum number of objects that you returned for this request. If more objects are available, in the response, AWS provides a NextPageToken value that you can use in a subsequent call to get the next batch of objects.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The measurement that you want your reservation coverage reported in.</p> <p>Valid values are <code>Hour</code>, <code>Unit</code>, and <code>Cost</code>. You can use multiple values in a request.</p>
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    /// <p>The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The value by which you want to sort the data.</p> <p>The following values are supported for <code>Key</code>:</p> <ul> <li> <p> <code>OnDemandCost</code> </p> </li> <li> <p> <code>CoverageHoursPercentage</code> </p> </li> <li> <p> <code>OnDemandHours</code> </p> </li> <li> <p> <code>ReservedHours</code> </p> </li> <li> <p> <code>TotalRunningHours</code> </p> </li> <li> <p> <code>CoverageNormalizedUnitsPercentage</code> </p> </li> <li> <p> <code>OnDemandNormalizedUnits</code> </p> </li> <li> <p> <code>ReservedNormalizedUnits</code> </p> </li> <li> <p> <code>TotalRunningNormalizedUnits</code> </p> </li> <li> <p> <code>Time</code> </p> </li> </ul> <p>Supported values for <code>SortOrder</code> are <code>ASCENDING</code> or <code>DESCENDING</code>.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortDefinition>,
    /// <p>The start and end dates of the period that you want to retrieve data about reservation coverage for. You can retrieve data for a maximum of 13 months: the last 12 months and the current month. The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>. </p>
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetReservationCoverageResponse {
    /// <p>The amount of time that your reservations covered.</p>
    #[serde(rename = "coveragesByTime")]
    pub coverages_by_time: Vec<CoverageByTime>,
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The total amount of instance usage that a reservation covered.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Coverage>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetReservationPurchaseRecommendationRequest {
    /// <p>The account ID that is associated with the recommendation. </p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The account scope that you want your recommendations for. Amazon Web Services calculates recommendations including the management account and member accounts if the value is set to <code>PAYER</code>. If the value is <code>LINKED</code>, recommendations are calculated for individual member accounts only.</p>
    #[serde(rename = "accountScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>The number of previous days that you want AWS to consider when it calculates your recommendations.</p>
    #[serde(rename = "lookbackPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_days: Option<String>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The number of recommendations that you want returned in a single response object.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The reservation purchase option that you want recommendations for.</p>
    #[serde(rename = "paymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    /// <p>The specific service that you want recommendations for.</p>
    #[serde(rename = "service")]
    pub service: String,
    /// <p>The hardware specifications for the service instances that you want recommendations for, such as standard or convertible Amazon EC2 instances.</p>
    #[serde(rename = "serviceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
    /// <p>The reservation term that you want recommendations for.</p>
    #[serde(rename = "termInYears")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_in_years: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetReservationPurchaseRecommendationResponse {
    /// <p>Information about this specific recommendation call, such as the time stamp for when Cost Explorer generated this recommendation.</p>
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ReservationPurchaseRecommendationMetadata>,
    /// <p>The pagination token for the next set of retrievable results.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Recommendations for reservations to purchase.</p>
    #[serde(rename = "recommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<ReservationPurchaseRecommendation>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetReservationUtilizationRequest {
    /// <p>Filters utilization data by dimensions. You can filter by the following dimensions:</p> <ul> <li> <p>AZ</p> </li> <li> <p>CACHE_ENGINE</p> </li> <li> <p>DEPLOYMENT_OPTION</p> </li> <li> <p>INSTANCE_TYPE</p> </li> <li> <p>LINKED_ACCOUNT</p> </li> <li> <p>OPERATING_SYSTEM</p> </li> <li> <p>PLATFORM</p> </li> <li> <p>REGION</p> </li> <li> <p>SERVICE</p> </li> <li> <p>SCOPE</p> </li> <li> <p>TENANCY</p> </li> </ul> <p> <code>GetReservationUtilization</code> uses the same <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> object as the other operations, but only <code>AND</code> is supported among each dimension, and nesting is supported up to only one level deep. If there are multiple values for a dimension, they are OR'd together.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>If <code>GroupBy</code> is set, <code>Granularity</code> can't be set. If <code>Granularity</code> isn't set, the response object doesn't include <code>Granularity</code>, either <code>MONTHLY</code> or <code>DAILY</code>. If both <code>GroupBy</code> and <code>Granularity</code> aren't set, <code>GetReservationUtilization</code> defaults to <code>DAILY</code>.</p> <p>The <code>GetReservationUtilization</code> operation supports only <code>DAILY</code> and <code>MONTHLY</code> granularities.</p>
    #[serde(rename = "granularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    /// <p>Groups only by <code>SUBSCRIPTION_ID</code>. Metadata is included.</p>
    #[serde(rename = "groupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    /// <p>The maximum number of objects that you returned for this request. If more objects are available, in the response, AWS provides a NextPageToken value that you can use in a subsequent call to get the next batch of objects.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The value by which you want to sort the data.</p> <p>The following values are supported for <code>Key</code>:</p> <ul> <li> <p> <code>UtilizationPercentage</code> </p> </li> <li> <p> <code>UtilizationPercentageInUnits</code> </p> </li> <li> <p> <code>PurchasedHours</code> </p> </li> <li> <p> <code>PurchasedUnits</code> </p> </li> <li> <p> <code>TotalActualHours</code> </p> </li> <li> <p> <code>TotalActualUnits</code> </p> </li> <li> <p> <code>UnusedHours</code> </p> </li> <li> <p> <code>UnusedUnits</code> </p> </li> <li> <p> <code>OnDemandCostOfRIHoursUsed</code> </p> </li> <li> <p> <code>NetRISavings</code> </p> </li> <li> <p> <code>TotalPotentialRISavings</code> </p> </li> <li> <p> <code>AmortizedUpfrontFee</code> </p> </li> <li> <p> <code>AmortizedRecurringFee</code> </p> </li> <li> <p> <code>TotalAmortizedFee</code> </p> </li> <li> <p> <code>RICostForUnusedHours</code> </p> </li> <li> <p> <code>RealizedSavings</code> </p> </li> <li> <p> <code>UnrealizedSavings</code> </p> </li> </ul> <p>Supported values for <code>SortOrder</code> are <code>ASCENDING</code> or <code>DESCENDING</code>.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortDefinition>,
    /// <p>Sets the start and end dates for retrieving RI utilization. The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>. </p>
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetReservationUtilizationResponse {
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The total amount of time that you used your RIs.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<ReservationAggregates>,
    /// <p>The amount of time that you used your RIs.</p>
    #[serde(rename = "utilizationsByTime")]
    pub utilizations_by_time: Vec<UtilizationByTime>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRightsizingRecommendationRequest {
    /// <p> Enables you to customize recommendations across two attributes. You can choose to view recommendations for instances within the same instance families or across different instance families. You can also choose to view your estimated savings associated with recommendations with consideration of existing Savings Plans or RI benefits, or neither. </p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RightsizingRecommendationConfiguration>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The number of recommendations that you want returned in a single response object.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The specific service that you want recommendations for. The only valid value for <code>GetRightsizingRecommendation</code> is "<code>AmazonEC2</code>".</p>
    #[serde(rename = "service")]
    pub service: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRightsizingRecommendationResponse {
    /// <p> Enables you to customize recommendations across two attributes. You can choose to view recommendations for instances within the same instance families or across different instance families. You can also choose to view your estimated savings associated with recommendations with consideration of existing Savings Plans or RI benefits, or neither. </p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RightsizingRecommendationConfiguration>,
    /// <p>Information regarding this specific recommendation set.</p>
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RightsizingRecommendationMetadata>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Recommendations to rightsize resources.</p>
    #[serde(rename = "rightsizingRecommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rightsizing_recommendations: Option<Vec<RightsizingRecommendation>>,
    /// <p>Summary of this recommendation set.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<RightsizingRecommendationSummary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSavingsPlansCoverageRequest {
    /// <p>Filters Savings Plans coverage data by dimensions. You can filter data for Savings Plans usage with the following dimensions:</p> <ul> <li> <p> <code>LINKED_ACCOUNT</code> </p> </li> <li> <p> <code>REGION</code> </p> </li> <li> <p> <code>SERVICE</code> </p> </li> <li> <p> <code>INSTANCE_FAMILY</code> </p> </li> </ul> <p> <code>GetSavingsPlansCoverage</code> uses the same <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> object as the other operations, but only <code>AND</code> is supported among each dimension. If there are multiple values for a dimension, they are OR'd together.</p> <p>Cost category is also supported.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>The granularity of the Amazon Web Services cost data for your Savings Plans. <code>Granularity</code> can't be set if <code>GroupBy</code> is set.</p> <p>The <code>GetSavingsPlansCoverage</code> operation supports only <code>DAILY</code> and <code>MONTHLY</code> granularities.</p>
    #[serde(rename = "granularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    /// <p>You can group the data using the attributes <code>INSTANCE_FAMILY</code>, <code>REGION</code>, or <code>SERVICE</code>.</p>
    #[serde(rename = "groupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    /// <p>The number of items to be returned in a response. The default is <code>20</code>, with a minimum value of <code>1</code>.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The measurement that you want your Savings Plans coverage reported in. The only valid value is <code>SpendCoveredBySavingsPlans</code>.</p>
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    /// <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The value by which you want to sort the data.</p> <p>The following values are supported for <code>Key</code>:</p> <ul> <li> <p> <code>SpendCoveredBySavingsPlan</code> </p> </li> <li> <p> <code>OnDemandCost</code> </p> </li> <li> <p> <code>CoveragePercentage</code> </p> </li> <li> <p> <code>TotalCost</code> </p> </li> <li> <p> <code>InstanceFamily</code> </p> </li> <li> <p> <code>Region</code> </p> </li> <li> <p> <code>Service</code> </p> </li> </ul> <p>Supported values for <code>SortOrder</code> are <code>ASCENDING</code> or <code>DESCENDING</code>.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortDefinition>,
    /// <p>The time period that you want the usage and costs for. The <code>Start</code> date must be within 13 months. The <code>End</code> date must be after the <code>Start</code> date, and before the current date. Future dates can't be used as an <code>End</code> date.</p>
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSavingsPlansCoverageResponse {
    /// <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The amount of spend that your Savings Plans covered.</p>
    #[serde(rename = "savingsPlansCoverages")]
    pub savings_plans_coverages: Vec<SavingsPlansCoverage>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSavingsPlansPurchaseRecommendationRequest {
    /// <p>The account scope that you want your recommendations for. Amazon Web Services calculates recommendations including the management account and member accounts if the value is set to <code>PAYER</code>. If the value is <code>LINKED</code>, recommendations are calculated for individual member accounts only.</p>
    #[serde(rename = "accountScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    /// <p>You can filter your recommendations by Account ID with the <code>LINKED_ACCOUNT</code> dimension. To filter your recommendations by Account ID, specify <code>Key</code> as <code>LINKED_ACCOUNT</code> and <code>Value</code> as the comma-separated Acount ID(s) for which you want to see Savings Plans purchase recommendations.</p> <p>For GetSavingsPlansPurchaseRecommendation, the <code>Filter</code> does not include <code>CostCategories</code> or <code>Tags</code>. It only includes <code>Dimensions</code>. With <code>Dimensions</code>, <code>Key</code> must be <code>LINKED_ACCOUNT</code> and <code>Value</code> can be a single Account ID or multiple comma-separated Account IDs for which you want to see Savings Plans Purchase Recommendations. <code>AND</code> and <code>OR</code> operators are not supported.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>The lookback period used to generate the recommendation.</p>
    #[serde(rename = "lookbackPeriodInDays")]
    pub lookback_period_in_days: String,
    /// <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The number of recommendations that you want returned in a single response object.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The payment option used to generate these recommendations.</p>
    #[serde(rename = "paymentOption")]
    pub payment_option: String,
    /// <p>The Savings Plans recommendation type requested.</p>
    #[serde(rename = "savingsPlansType")]
    pub savings_plans_type: String,
    /// <p>The savings plan recommendation term used to generate these recommendations.</p>
    #[serde(rename = "termInYears")]
    pub term_in_years: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSavingsPlansPurchaseRecommendationResponse {
    /// <p>Information regarding this specific recommendation set.</p>
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SavingsPlansPurchaseRecommendationMetadata>,
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Contains your request parameters, Savings Plan Recommendations Summary, and Details.</p>
    #[serde(rename = "savingsPlansPurchaseRecommendation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_purchase_recommendation: Option<SavingsPlansPurchaseRecommendation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSavingsPlansUtilizationDetailsRequest {
    /// <p>The data type.</p>
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<Vec<String>>,
    /// <p>Filters Savings Plans utilization coverage data for active Savings Plans dimensions. You can filter data with the following dimensions:</p> <ul> <li> <p> <code>LINKED_ACCOUNT</code> </p> </li> <li> <p> <code>SAVINGS_PLAN_ARN</code> </p> </li> <li> <p> <code>REGION</code> </p> </li> <li> <p> <code>PAYMENT_OPTION</code> </p> </li> <li> <p> <code>INSTANCE_TYPE_FAMILY</code> </p> </li> </ul> <p> <code>GetSavingsPlansUtilizationDetails</code> uses the same <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> object as the other operations, but only <code>AND</code> is supported among each dimension.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>The number of items to be returned in a response. The default is <code>20</code>, with a minimum value of <code>1</code>.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The value by which you want to sort the data.</p> <p>The following values are supported for <code>Key</code>:</p> <ul> <li> <p> <code>UtilizationPercentage</code> </p> </li> <li> <p> <code>TotalCommitment</code> </p> </li> <li> <p> <code>UsedCommitment</code> </p> </li> <li> <p> <code>UnusedCommitment</code> </p> </li> <li> <p> <code>NetSavings</code> </p> </li> <li> <p> <code>AmortizedRecurringCommitment</code> </p> </li> <li> <p> <code>AmortizedUpfrontCommitment</code> </p> </li> </ul> <p>Supported values for <code>SortOrder</code> are <code>ASCENDING</code> or <code>DESCENDING</code>.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortDefinition>,
    /// <p>The time period that you want the usage and costs for. The <code>Start</code> date must be within 13 months. The <code>End</code> date must be after the <code>Start</code> date, and before the current date. Future dates can't be used as an <code>End</code> date.</p>
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSavingsPlansUtilizationDetailsResponse {
    /// <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Retrieves a single daily or monthly Savings Plans utilization rate and details for your account.</p>
    #[serde(rename = "savingsPlansUtilizationDetails")]
    pub savings_plans_utilization_details: Vec<SavingsPlansUtilizationDetail>,
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
    /// <p>The total Savings Plans utilization, regardless of time period.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<SavingsPlansUtilizationAggregates>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSavingsPlansUtilizationRequest {
    /// <p>Filters Savings Plans utilization coverage data for active Savings Plans dimensions. You can filter data with the following dimensions:</p> <ul> <li> <p> <code>LINKED_ACCOUNT</code> </p> </li> <li> <p> <code>SAVINGS_PLAN_ARN</code> </p> </li> <li> <p> <code>SAVINGS_PLANS_TYPE</code> </p> </li> <li> <p> <code>REGION</code> </p> </li> <li> <p> <code>PAYMENT_OPTION</code> </p> </li> <li> <p> <code>INSTANCE_TYPE_FAMILY</code> </p> </li> </ul> <p> <code>GetSavingsPlansUtilization</code> uses the same <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> object as the other operations, but only <code>AND</code> is supported among each dimension.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>The granularity of the Amazon Web Services utillization data for your Savings Plans.</p> <p>The <code>GetSavingsPlansUtilization</code> operation supports only <code>DAILY</code> and <code>MONTHLY</code> granularities.</p>
    #[serde(rename = "granularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    /// <p>The value by which you want to sort the data.</p> <p>The following values are supported for <code>Key</code>:</p> <ul> <li> <p> <code>UtilizationPercentage</code> </p> </li> <li> <p> <code>TotalCommitment</code> </p> </li> <li> <p> <code>UsedCommitment</code> </p> </li> <li> <p> <code>UnusedCommitment</code> </p> </li> <li> <p> <code>NetSavings</code> </p> </li> </ul> <p>Supported values for <code>SortOrder</code> are <code>ASCENDING</code> or <code>DESCENDING</code>.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortDefinition>,
    /// <p>The time period that you want the usage and costs for. The <code>Start</code> date must be within 13 months. The <code>End</code> date must be after the <code>Start</code> date, and before the current date. Future dates can't be used as an <code>End</code> date.</p>
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSavingsPlansUtilizationResponse {
    /// <p>The amount of cost/commitment you used your Savings Plans. This allows you to specify date ranges.</p>
    #[serde(rename = "savingsPlansUtilizationsByTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_utilizations_by_time: Option<Vec<SavingsPlansUtilizationByTime>>,
    /// <p>The total amount of cost/commitment that you used your Savings Plans, regardless of date ranges.</p>
    #[serde(rename = "total")]
    pub total: SavingsPlansUtilizationAggregates,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTagsRequest {
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>This field is only used when SortBy is provided in the request. The maximum number of objects that to be returned for this request. If MaxResults is not specified with SortBy, the request will return 1000 results as the default value for this parameter.</p> <p>For <code>GetTags</code>, MaxResults has an upper limit of 1000.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The value that you want to search for.</p>
    #[serde(rename = "searchString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    /// <p>The value by which you want to sort the data.</p> <p>The key represents cost and usage metrics. The following values are supported:</p> <ul> <li> <p> <code>BlendedCost</code> </p> </li> <li> <p> <code>UnblendedCost</code> </p> </li> <li> <p> <code>AmortizedCost</code> </p> </li> <li> <p> <code>NetAmortizedCost</code> </p> </li> <li> <p> <code>NetUnblendedCost</code> </p> </li> <li> <p> <code>UsageQuantity</code> </p> </li> <li> <p> <code>NormalizedUsageAmount</code> </p> </li> </ul> <p>Supported values for <code>SortOrder</code> are <code>ASCENDING</code> or <code>DESCENDING</code>.</p> <p>When using <code>SortBy</code>, <code>NextPageToken</code> and <code>SearchString</code> are not supported.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Vec<SortDefinition>>,
    /// <p>The key of the tag that you want to return values for.</p>
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// <p>The start and end dates for retrieving the dimension values. The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>.</p>
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTagsResponse {
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The number of query results that AWS returns at a time.</p>
    #[serde(rename = "returnSize")]
    pub return_size: i64,
    /// <p>The tags that match your request.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// <p>The total number of query results.</p>
    #[serde(rename = "totalSize")]
    pub total_size: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUsageForecastRequest {
    /// <p><p>The filters that you want to use to filter your forecast. The <code>GetUsageForecast</code> API supports filtering by the following dimensions:</p> <ul> <li> <p> <code>AZ</code> </p> </li> <li> <p> <code>INSTANCE<em>TYPE</code> </p> </li> <li> <p> <code>LINKED</em>ACCOUNT</code> </p> </li> <li> <p> <code>LINKED<em>ACCOUNT</em>NAME</code> </p> </li> <li> <p> <code>OPERATION</code> </p> </li> <li> <p> <code>PURCHASE<em>TYPE</code> </p> </li> <li> <p> <code>REGION</code> </p> </li> <li> <p> <code>SERVICE</code> </p> </li> <li> <p> <code>USAGE</em>TYPE</code> </p> </li> <li> <p> <code>USAGE<em>TYPE</em>GROUP</code> </p> </li> <li> <p> <code>RECORD<em>TYPE</code> </p> </li> <li> <p> <code>OPERATING</em>SYSTEM</code> </p> </li> <li> <p> <code>TENANCY</code> </p> </li> <li> <p> <code>SCOPE</code> </p> </li> <li> <p> <code>PLATFORM</code> </p> </li> <li> <p> <code>SUBSCRIPTION<em>ID</code> </p> </li> <li> <p> <code>LEGAL</em>ENTITY<em>NAME</code> </p> </li> <li> <p> <code>DEPLOYMENT</em>OPTION</code> </p> </li> <li> <p> <code>DATABASE<em>ENGINE</code> </p> </li> <li> <p> <code>INSTANCE</em>TYPE<em>FAMILY</code> </p> </li> <li> <p> <code>BILLING</em>ENTITY</code> </p> </li> <li> <p> <code>RESERVATION<em>ID</code> </p> </li> <li> <p> <code>SAVINGS</em>PLAN_ARN</code> </p> </li> </ul></p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>How granular you want the forecast to be. You can get 3 months of <code>DAILY</code> forecasts or 12 months of <code>MONTHLY</code> forecasts.</p> <p>The <code>GetUsageForecast</code> operation supports only <code>DAILY</code> and <code>MONTHLY</code> granularities.</p>
    #[serde(rename = "granularity")]
    pub granularity: String,
    /// <p><p>Which metric Cost Explorer uses to create your forecast.</p> <p>Valid values for a <code>GetUsageForecast</code> call are the following:</p> <ul> <li> <p>USAGE<em>QUANTITY</p> </li> <li> <p>NORMALIZED</em>USAGE_AMOUNT</p> </li> </ul></p>
    #[serde(rename = "metric")]
    pub metric: String,
    /// <p>Cost Explorer always returns the mean forecast as a single point. You can request a prediction interval around the mean by specifying a confidence level. The higher the confidence level, the more confident Cost Explorer is about the actual value falling in the prediction interval. Higher confidence levels result in wider prediction intervals.</p>
    #[serde(rename = "predictionIntervalLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval_level: Option<i64>,
    /// <p>The start and end dates of the period that you want to retrieve usage forecast for. The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>. The start date must be equal to or later than the current date to avoid a validation error.</p>
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUsageForecastResponse {
    /// <p>The forecasts for your query, in order. For <code>DAILY</code> forecasts, this is a list of days. For <code>MONTHLY</code> forecasts, this is a list of months.</p>
    #[serde(rename = "forecastResultsByTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_results_by_time: Option<Vec<ForecastResult>>,
    /// <p>How much you're forecasted to use over the forecast period.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<MetricValue>,
}

/// <p>One level of grouped data in the results.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Group {
    /// <p>The keys that are included in this group.</p>
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
    /// <p>The metrics that are included in this group.</p>
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, MetricValue>>,
}

/// <p>Represents a group when you specify a group by criteria or in the response to a query with a specific grouping.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GroupDefinition {
    /// <p>The string that represents a key for a specified group.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The string that represents the type of group.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p> The anomaly's dollar value. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Impact {
    /// <p> The maximum dollar value observed for an anomaly. </p>
    #[serde(rename = "maxImpact")]
    pub max_impact: f64,
    /// <p> The cumulative dollar value observed for an anomaly. </p>
    #[serde(rename = "totalImpact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_impact: Option<f64>,
}

/// <p>Details about the instances that AWS recommends that you purchase.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceDetails {
    /// <p>The Amazon EC2 instances that AWS recommends that you purchase.</p>
    #[serde(rename = "eC2InstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_details: Option<EC2InstanceDetails>,
    /// <p>The Amazon ES instances that AWS recommends that you purchase.</p>
    #[serde(rename = "eSInstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_instance_details: Option<ESInstanceDetails>,
    /// <p>The ElastiCache instances that AWS recommends that you purchase.</p>
    #[serde(rename = "elastiCacheInstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasti_cache_instance_details: Option<ElastiCacheInstanceDetails>,
    /// <p>The Amazon RDS instances that AWS recommends that you purchase.</p>
    #[serde(rename = "rDSInstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_instance_details: Option<RDSInstanceDetails>,
    /// <p>The Amazon Redshift instances that AWS recommends that you purchase.</p>
    #[serde(rename = "redshiftInstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_instance_details: Option<RedshiftInstanceDetails>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCostCategoryDefinitionsRequest {
    /// <p> The date when the Cost Category was effective. </p>
    #[serde(rename = "effectiveOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_on: Option<String>,
    /// <p> The number of entries a paginated response contains. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCostCategoryDefinitionsResponse {
    /// <p> A reference to a Cost Category containing enough information to identify the Cost Category. </p>
    #[serde(rename = "costCategoryReferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_references: Option<Vec<CostCategoryReference>>,
    /// <p> The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The aggregated value for a metric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MetricValue {
    /// <p>The actual number that represents the metric.</p>
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// <p>The unit that the metric is given in.</p>
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p> Details on the modification recommendation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyRecommendationDetail {
    /// <p>Identifies whether this instance type is the AWS default recommendation.</p>
    #[serde(rename = "targetInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_instances: Option<Vec<TargetInstance>>,
}

/// <p> The network field that contains a list of network metrics associated with the current instance. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkResourceUtilization {
    /// <p> The network ingress throughput utilization measured in Bytes per second. </p>
    #[serde(rename = "networkInBytesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_in_bytes_per_second: Option<String>,
    /// <p> The network outgress throughput utilization measured in Bytes per second. </p>
    #[serde(rename = "networkOutBytesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_out_bytes_per_second: Option<String>,
    /// <p> The network ingress packets measured in packets per second. </p>
    #[serde(rename = "networkPacketsInPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_packets_in_per_second: Option<String>,
    /// <p> The network outgress packets measured in packets per second. </p>
    #[serde(rename = "networkPacketsOutPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_packets_out_per_second: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ProvideAnomalyFeedbackRequest {
    /// <p> A cost anomaly ID. </p>
    #[serde(rename = "anomalyId")]
    pub anomaly_id: String,
    /// <p>Describes whether the cost anomaly was a planned activity or you considered it an anomaly. </p>
    #[serde(rename = "feedback")]
    pub feedback: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvideAnomalyFeedbackResponse {
    /// <p> The ID of the modified cost anomaly. </p>
    #[serde(rename = "anomalyId")]
    pub anomaly_id: String,
}

/// <p>Details about the Amazon RDS instances that AWS recommends that you purchase.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RDSInstanceDetails {
    /// <p>Whether the recommendation is for a current-generation instance. </p>
    #[serde(rename = "currentGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    /// <p>The database edition that the recommended reservation supports.</p>
    #[serde(rename = "databaseEdition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_edition: Option<String>,
    /// <p>The database engine that the recommended reservation supports.</p>
    #[serde(rename = "databaseEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_engine: Option<String>,
    /// <p>Whether the recommendation is for a reservation in a single Availability Zone or a reservation with a backup in a second Availability Zone.</p>
    #[serde(rename = "deploymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_option: Option<String>,
    /// <p>The instance family of the recommended reservation.</p>
    #[serde(rename = "family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The type of instance that AWS recommends.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The license model that the recommended reservation supports.</p>
    #[serde(rename = "licenseModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    /// <p>The AWS Region of the recommended reservation.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Whether the recommended reservation is size flexible.</p>
    #[serde(rename = "sizeFlexEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

/// <p>Details about the Amazon Redshift instances that AWS recommends that you purchase.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RedshiftInstanceDetails {
    /// <p>Whether the recommendation is for a current-generation instance.</p>
    #[serde(rename = "currentGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    /// <p>The instance family of the recommended reservation.</p>
    #[serde(rename = "family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The type of node that AWS recommends.</p>
    #[serde(rename = "nodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <p>The AWS Region of the recommended reservation.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Whether the recommended reservation is size flexible.</p>
    #[serde(rename = "sizeFlexEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

/// <p>The aggregated numbers for your reservation usage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservationAggregates {
    /// <p>The monthly cost of your reservation, amortized over the reservation period.</p>
    #[serde(rename = "amortizedRecurringFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_recurring_fee: Option<String>,
    /// <p>The upfront cost of your reservation, amortized over the reservation period.</p>
    #[serde(rename = "amortizedUpfrontFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_upfront_fee: Option<String>,
    /// <p>How much you saved due to purchasing and utilizing reservation. AWS calculates this by subtracting <code>TotalAmortizedFee</code> from <code>OnDemandCostOfRIHoursUsed</code>.</p>
    #[serde(rename = "netRISavings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_ri_savings: Option<String>,
    /// <p>How much your reservation would cost if charged On-Demand rates.</p>
    #[serde(rename = "onDemandCostOfRIHoursUsed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_cost_of_ri_hours_used: Option<String>,
    /// <p>How many reservation hours that you purchased.</p>
    #[serde(rename = "purchasedHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_hours: Option<String>,
    /// <p>How many Amazon EC2 reservation hours that you purchased, converted to normalized units. Normalized units are available only for Amazon EC2 usage after November 11, 2017.</p>
    #[serde(rename = "purchasedUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_units: Option<String>,
    /// <p>The cost of unused hours for your reservation.</p>
    #[serde(rename = "rICostForUnusedHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ri_cost_for_unused_hours: Option<String>,
    /// <p>The realized savings due to purchasing and using a reservation.</p>
    #[serde(rename = "realizedSavings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realized_savings: Option<String>,
    /// <p>The total number of reservation hours that you used.</p>
    #[serde(rename = "totalActualHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_actual_hours: Option<String>,
    /// <p>The total number of Amazon EC2 reservation hours that you used, converted to normalized units. Normalized units are available only for Amazon EC2 usage after November 11, 2017.</p>
    #[serde(rename = "totalActualUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_actual_units: Option<String>,
    /// <p>The total cost of your reservation, amortized over the reservation period.</p>
    #[serde(rename = "totalAmortizedFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amortized_fee: Option<String>,
    /// <p>How much you could save if you use your entire reservation.</p>
    #[serde(rename = "totalPotentialRISavings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_potential_ri_savings: Option<String>,
    /// <p>The unrealized savings due to purchasing and using a reservation.</p>
    #[serde(rename = "unrealizedSavings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrealized_savings: Option<String>,
    /// <p>The number of reservation hours that you didn't use.</p>
    #[serde(rename = "unusedHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_hours: Option<String>,
    /// <p>The number of Amazon EC2 reservation hours that you didn't use, converted to normalized units. Normalized units are available only for Amazon EC2 usage after November 11, 2017.</p>
    #[serde(rename = "unusedUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_units: Option<String>,
    /// <p>The percentage of reservation time that you used.</p>
    #[serde(rename = "utilizationPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_percentage: Option<String>,
    /// <p>The percentage of Amazon EC2 reservation time that you used, converted to normalized units. Normalized units are available only for Amazon EC2 usage after November 11, 2017.</p>
    #[serde(rename = "utilizationPercentageInUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_percentage_in_units: Option<String>,
}

/// <p>A group of reservations that share a set of attributes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservationCoverageGroup {
    /// <p>The attributes for this group of reservations.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>How much instance usage this group of reservations covered.</p>
    #[serde(rename = "coverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<Coverage>,
}

/// <p>A specific reservation that AWS recommends for purchase.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservationPurchaseRecommendation {
    /// <p>The account scope that AWS recommends that you purchase this instance for. For example, you can purchase this reservation for an entire organization in AWS Organizations.</p>
    #[serde(rename = "accountScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    /// <p>How many days of previous usage that AWS considers when making this recommendation.</p>
    #[serde(rename = "lookbackPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_days: Option<String>,
    /// <p>The payment option for the reservation. For example, <code>AllUpfront</code> or <code>NoUpfront</code>.</p>
    #[serde(rename = "paymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    /// <p>Details about the recommended purchases.</p>
    #[serde(rename = "recommendationDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_details: Option<Vec<ReservationPurchaseRecommendationDetail>>,
    /// <p>A summary about the recommended purchase.</p>
    #[serde(rename = "recommendationSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_summary: Option<ReservationPurchaseRecommendationSummary>,
    /// <p>Hardware specifications for the service that you want recommendations for.</p>
    #[serde(rename = "serviceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
    /// <p>The term of the reservation that you want recommendations for, in years.</p>
    #[serde(rename = "termInYears")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_in_years: Option<String>,
}

/// <p>Details about your recommended reservation purchase.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservationPurchaseRecommendationDetail {
    /// <p>The account that this RI recommendation is for.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The average number of normalized units that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "averageNormalizedUnitsUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_normalized_units_used_per_hour: Option<String>,
    /// <p>The average number of instances that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "averageNumberOfInstancesUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_number_of_instances_used_per_hour: Option<String>,
    /// <p>The average utilization of your instances. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "averageUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<String>,
    /// <p>The currency code that AWS used to calculate the costs for this instance.</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>How long AWS estimates that it takes for this instance to start saving you money, in months.</p>
    #[serde(rename = "estimatedBreakEvenInMonths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_break_even_in_months: Option<String>,
    /// <p>How much AWS estimates that you spend on On-Demand Instances in a month.</p>
    #[serde(rename = "estimatedMonthlyOnDemandCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_on_demand_cost: Option<String>,
    /// <p>How much AWS estimates that this specific recommendation could save you in a month.</p>
    #[serde(rename = "estimatedMonthlySavingsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_amount: Option<String>,
    /// <p>How much AWS estimates that this specific recommendation could save you in a month, as a percentage of your overall costs.</p>
    #[serde(rename = "estimatedMonthlySavingsPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_percentage: Option<String>,
    /// <p>How much AWS estimates that you would have spent for all usage during the specified historical period if you had a reservation.</p>
    #[serde(rename = "estimatedReservationCostForLookbackPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_reservation_cost_for_lookback_period: Option<String>,
    /// <p>Details about the instances that AWS recommends that you purchase.</p>
    #[serde(rename = "instanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_details: Option<InstanceDetails>,
    /// <p>The maximum number of normalized units that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "maximumNormalizedUnitsUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_normalized_units_used_per_hour: Option<String>,
    /// <p>The maximum number of instances that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "maximumNumberOfInstancesUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_number_of_instances_used_per_hour: Option<String>,
    /// <p>The minimum number of normalized units that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "minimumNormalizedUnitsUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_normalized_units_used_per_hour: Option<String>,
    /// <p>The minimum number of instances that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "minimumNumberOfInstancesUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_number_of_instances_used_per_hour: Option<String>,
    /// <p>The number of normalized units that AWS recommends that you purchase.</p>
    #[serde(rename = "recommendedNormalizedUnitsToPurchase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_normalized_units_to_purchase: Option<String>,
    /// <p>The number of instances that AWS recommends that you purchase.</p>
    #[serde(rename = "recommendedNumberOfInstancesToPurchase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_number_of_instances_to_purchase: Option<String>,
    /// <p>How much purchasing this instance costs you on a monthly basis.</p>
    #[serde(rename = "recurringStandardMonthlyCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_standard_monthly_cost: Option<String>,
    /// <p>How much purchasing this instance costs you upfront.</p>
    #[serde(rename = "upfrontCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_cost: Option<String>,
}

/// <p>Information about this specific recommendation, such as the timestamp for when AWS made a specific recommendation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservationPurchaseRecommendationMetadata {
    /// <p>The timestamp for when AWS made this recommendation.</p>
    #[serde(rename = "generationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_timestamp: Option<String>,
    /// <p>The ID for this specific recommendation.</p>
    #[serde(rename = "recommendationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

/// <p>A summary about this recommendation, such as the currency code, the amount that AWS estimates that you could save, and the total amount of reservation to purchase.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservationPurchaseRecommendationSummary {
    /// <p>The currency code used for this recommendation.</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>The total amount that AWS estimates that this recommendation could save you in a month.</p>
    #[serde(rename = "totalEstimatedMonthlySavingsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_estimated_monthly_savings_amount: Option<String>,
    /// <p>The total amount that AWS estimates that this recommendation could save you in a month, as a percentage of your costs.</p>
    #[serde(rename = "totalEstimatedMonthlySavingsPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_estimated_monthly_savings_percentage: Option<String>,
}

/// <p>A group of reservations that share a set of attributes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservationUtilizationGroup {
    /// <p>The attributes for this group of reservations.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The key for a specific reservation attribute.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>How much you used this group of reservations.</p>
    #[serde(rename = "utilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization: Option<ReservationAggregates>,
    /// <p>The value of a specific reservation attribute.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Details on the resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceDetails {
    /// <p>Details on the Amazon EC2 resource.</p>
    #[serde(rename = "eC2ResourceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_resource_details: Option<EC2ResourceDetails>,
}

/// <p>Resource utilization of current resource. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceUtilization {
    /// <p>Utilization of current Amazon EC2 instance. </p>
    #[serde(rename = "eC2ResourceUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_resource_utilization: Option<EC2ResourceUtilization>,
}

/// <p>The result that is associated with a time period.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResultByTime {
    /// <p>Whether the result is estimated.</p>
    #[serde(rename = "estimated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated: Option<bool>,
    /// <p>The groups that this time period includes.</p>
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    /// <p>The time period that the result covers.</p>
    #[serde(rename = "timePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
    /// <p>The total amount of cost or usage accrued during the time period.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<::std::collections::HashMap<String, MetricValue>>,
}

/// <p>Recommendations to rightsize resources.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RightsizingRecommendation {
    /// <p>The account that this recommendation is for.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p> Context regarding the current instance.</p>
    #[serde(rename = "currentInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_instance: Option<CurrentInstance>,
    /// <p> The list of possible reasons why the recommendation is generated such as under or over utilization of specific metrics (for example, CPU, Memory, Network). </p>
    #[serde(rename = "findingReasonCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_reason_codes: Option<Vec<String>>,
    /// <p> Details for modification recommendations. </p>
    #[serde(rename = "modifyRecommendationDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_recommendation_detail: Option<ModifyRecommendationDetail>,
    /// <p>Recommendation to either terminate or modify the resource.</p>
    #[serde(rename = "rightsizingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rightsizing_type: Option<String>,
    /// <p>Details for termination recommendations.</p>
    #[serde(rename = "terminateRecommendationDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_recommendation_detail: Option<TerminateRecommendationDetail>,
}

/// <p> Enables you to customize recommendations across two attributes. You can choose to view recommendations for instances within the same instance families or across different instance families. You can also choose to view your estimated savings associated with recommendations with consideration of existing Savings Plans or RI benefits, or neither. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RightsizingRecommendationConfiguration {
    /// <p> The option to consider RI or Savings Plans discount benefits in your savings calculation. The default value is <code>TRUE</code>. </p>
    #[serde(rename = "benefitsConsidered")]
    pub benefits_considered: bool,
    /// <p> The option to see recommendations within the same instance family, or recommendations for instances across other families. The default value is <code>SAME_INSTANCE_FAMILY</code>. </p>
    #[serde(rename = "recommendationTarget")]
    pub recommendation_target: String,
}

/// <p>Metadata for this recommendation set.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RightsizingRecommendationMetadata {
    /// <p>Additional metadata that may be applicable to the recommendation.</p>
    #[serde(rename = "additionalMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metadata: Option<String>,
    /// <p> The timestamp for when AWS made this recommendation.</p>
    #[serde(rename = "generationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_timestamp: Option<String>,
    /// <p> How many days of previous usage that AWS considers when making this recommendation.</p>
    #[serde(rename = "lookbackPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_days: Option<String>,
    /// <p> The ID for this specific recommendation.</p>
    #[serde(rename = "recommendationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

/// <p> Summary of rightsizing recommendations </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RightsizingRecommendationSummary {
    /// <p> Estimated total savings resulting from modifications, on a monthly basis.</p>
    #[serde(rename = "estimatedTotalMonthlySavingsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_total_monthly_savings_amount: Option<String>,
    /// <p> The currency code that AWS used to calculate the savings.</p>
    #[serde(rename = "savingsCurrencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_currency_code: Option<String>,
    /// <p> Savings percentage based on the recommended modifications, relative to the total On-Demand costs associated with these instances.</p>
    #[serde(rename = "savingsPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_percentage: Option<String>,
    /// <p> Total number of instance recommendations.</p>
    #[serde(rename = "totalRecommendationCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_recommendation_count: Option<String>,
}

/// <p> The combination of AWS service, linked account, Region, and usage type where a cost anomaly is observed. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RootCause {
    /// <p> The linked account value associated with the cost anomaly. </p>
    #[serde(rename = "linkedAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_account: Option<String>,
    /// <p> The AWS Region associated with the cost anomaly. </p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p> The AWS service name associated with the cost anomaly. </p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// <p> The <code>UsageType</code> value associated with the cost anomaly. </p>
    #[serde(rename = "usageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}

/// <p>The amortized amount of Savings Plans purchased in a specific account during a specific time interval.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansAmortizedCommitment {
    /// <p>The amortized amount of your Savings Plans commitment that was purchased with either a <code>Partial</code> or a <code>NoUpfront</code>.</p>
    #[serde(rename = "amortizedRecurringCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_recurring_commitment: Option<String>,
    /// <p>The amortized amount of your Savings Plans commitment that was purchased with an <code>Upfront</code> or <code>PartialUpfront</code> Savings Plans.</p>
    #[serde(rename = "amortizedUpfrontCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_upfront_commitment: Option<String>,
    /// <p>The total amortized amount of your Savings Plans commitment, regardless of your Savings Plans purchase method. </p>
    #[serde(rename = "totalAmortizedCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amortized_commitment: Option<String>,
}

/// <p>The amount of Savings Plans eligible usage that is covered by Savings Plans. All calculations consider the On-Demand equivalent of your Savings Plans usage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansCoverage {
    /// <p>The attribute that applies to a specific <code>Dimension</code>.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The amount of Savings Plans eligible usage that the Savings Plans covered.</p>
    #[serde(rename = "coverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<SavingsPlansCoverageData>,
    #[serde(rename = "timePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
}

/// <p>Specific coverage percentage, On-Demand costs, and spend covered by Savings Plans, and total Savings Plans costs for an account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansCoverageData {
    /// <p>The percentage of your existing Savings Plans covered usage, divided by all of your eligible Savings Plans usage in an account(or set of accounts).</p>
    #[serde(rename = "coveragePercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_percentage: Option<String>,
    /// <p>The cost of your AWS usage at the public On-Demand rate.</p>
    #[serde(rename = "onDemandCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_cost: Option<String>,
    /// <p>The amount of your AWS usage that is covered by a Savings Plans.</p>
    #[serde(rename = "spendCoveredBySavingsPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_covered_by_savings_plans: Option<String>,
    /// <p>The total cost of your AWS usage, regardless of your purchase option.</p>
    #[serde(rename = "totalCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<String>,
}

/// <p>Attribute details on a specific Savings Plan.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansDetails {
    /// <p>A group of instance types that Savings Plans applies to.</p>
    #[serde(rename = "instanceFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_family: Option<String>,
    /// <p>The unique ID used to distinguish Savings Plans from one another.</p>
    #[serde(rename = "offeringId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    /// <p>A collection of AWS resources in a geographic area. Each AWS Region is isolated and independent of the other Regions.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// <p>Contains your request parameters, Savings Plan Recommendations Summary, and Details.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansPurchaseRecommendation {
    /// <p>The account scope that you want your recommendations for. Amazon Web Services calculates recommendations including the management account and member accounts if the value is set to <code>PAYER</code>. If the value is <code>LINKED</code>, recommendations are calculated for individual member accounts only.</p>
    #[serde(rename = "accountScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    /// <p>The lookback period in days, used to generate the recommendation.</p>
    #[serde(rename = "lookbackPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_days: Option<String>,
    /// <p>The payment option used to generate the recommendation.</p>
    #[serde(rename = "paymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    /// <p>Details for the Savings Plans we recommend that you purchase to cover existing Savings Plans eligible workloads.</p>
    #[serde(rename = "savingsPlansPurchaseRecommendationDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_purchase_recommendation_details:
        Option<Vec<SavingsPlansPurchaseRecommendationDetail>>,
    /// <p>Summary metrics for your Savings Plans Recommendations. </p>
    #[serde(rename = "savingsPlansPurchaseRecommendationSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_purchase_recommendation_summary:
        Option<SavingsPlansPurchaseRecommendationSummary>,
    /// <p>The requested Savings Plans recommendation type.</p>
    #[serde(rename = "savingsPlansType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_type: Option<String>,
    /// <p>The Savings Plans recommendation term in years, used to generate the recommendation.</p>
    #[serde(rename = "termInYears")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_in_years: Option<String>,
}

/// <p>Details for your recommended Savings Plans.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansPurchaseRecommendationDetail {
    /// <p>The <code>AccountID</code> the recommendation is generated for.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The currency code AWS used to generate the recommendations and present potential savings.</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>The average value of hourly On-Demand spend over the lookback period of the applicable usage type.</p>
    #[serde(rename = "currentAverageHourlyOnDemandSpend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_average_hourly_on_demand_spend: Option<String>,
    /// <p>The highest value of hourly On-Demand spend over the lookback period of the applicable usage type.</p>
    #[serde(rename = "currentMaximumHourlyOnDemandSpend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_maximum_hourly_on_demand_spend: Option<String>,
    /// <p>The lowest value of hourly On-Demand spend over the lookback period of the applicable usage type.</p>
    #[serde(rename = "currentMinimumHourlyOnDemandSpend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_minimum_hourly_on_demand_spend: Option<String>,
    /// <p>The estimated utilization of the recommended Savings Plans.</p>
    #[serde(rename = "estimatedAverageUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_average_utilization: Option<String>,
    /// <p>The estimated monthly savings amount, based on the recommended Savings Plans.</p>
    #[serde(rename = "estimatedMonthlySavingsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_amount: Option<String>,
    /// <p>The remaining On-Demand cost estimated to not be covered by the recommended Savings Plans, over the length of the lookback period.</p>
    #[serde(rename = "estimatedOnDemandCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on_demand_cost: Option<String>,
    /// <p> The estimated On-Demand costs you would expect with no additional commitment, based on your usage of the selected time period and the Savings Plans you own. </p>
    #[serde(rename = "estimatedOnDemandCostWithCurrentCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on_demand_cost_with_current_commitment: Option<String>,
    /// <p>The estimated return on investment based on the recommended Savings Plans purchased. This is calculated as <code>estimatedSavingsAmount</code>/ <code>estimatedSPCost</code>*100.</p>
    #[serde(rename = "estimatedROI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_roi: Option<String>,
    /// <p>The cost of the recommended Savings Plans over the length of the lookback period.</p>
    #[serde(rename = "estimatedSPCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_sp_cost: Option<String>,
    /// <p>The estimated savings amount based on the recommended Savings Plans over the length of the lookback period.</p>
    #[serde(rename = "estimatedSavingsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_amount: Option<String>,
    /// <p>The estimated savings percentage relative to the total cost of applicable On-Demand usage over the lookback period.</p>
    #[serde(rename = "estimatedSavingsPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_percentage: Option<String>,
    /// <p>The recommended hourly commitment level for the Savings Plans type, and configuration based on the usage during the lookback period.</p>
    #[serde(rename = "hourlyCommitmentToPurchase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_commitment_to_purchase: Option<String>,
    /// <p>Details for your recommended Savings Plans.</p>
    #[serde(rename = "savingsPlansDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_details: Option<SavingsPlansDetails>,
    /// <p>The upfront cost of the recommended Savings Plans, based on the selected payment option.</p>
    #[serde(rename = "upfrontCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_cost: Option<String>,
}

/// <p>Metadata about your Savings Plans Purchase Recommendations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansPurchaseRecommendationMetadata {
    /// <p>Additional metadata that may be applicable to the recommendation.</p>
    #[serde(rename = "additionalMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metadata: Option<String>,
    /// <p>The timestamp showing when the recommendations were generated.</p>
    #[serde(rename = "generationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_timestamp: Option<String>,
    /// <p>The unique identifier for the recommendation set.</p>
    #[serde(rename = "recommendationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

/// <p>Summary metrics for your Savings Plans Purchase Recommendations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansPurchaseRecommendationSummary {
    /// <p>The currency code AWS used to generate the recommendations and present potential savings.</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>The current total on demand spend of the applicable usage types over the lookback period.</p>
    #[serde(rename = "currentOnDemandSpend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_on_demand_spend: Option<String>,
    /// <p>The recommended Savings Plans cost on a daily (24 hourly) basis.</p>
    #[serde(rename = "dailyCommitmentToPurchase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_commitment_to_purchase: Option<String>,
    /// <p>The estimated monthly savings amount, based on the recommended Savings Plans purchase.</p>
    #[serde(rename = "estimatedMonthlySavingsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_amount: Option<String>,
    /// <p> The estimated On-Demand costs you would expect with no additional commitment, based on your usage of the selected time period and the Savings Plans you own. </p>
    #[serde(rename = "estimatedOnDemandCostWithCurrentCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on_demand_cost_with_current_commitment: Option<String>,
    /// <p>The estimated return on investment based on the recommended Savings Plans and estimated savings.</p>
    #[serde(rename = "estimatedROI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_roi: Option<String>,
    /// <p>The estimated total savings over the lookback period, based on the purchase of the recommended Savings Plans.</p>
    #[serde(rename = "estimatedSavingsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_amount: Option<String>,
    /// <p>The estimated savings relative to the total cost of On-Demand usage, over the lookback period. This is calculated as <code>estimatedSavingsAmount</code>/ <code>CurrentOnDemandSpend</code>*100.</p>
    #[serde(rename = "estimatedSavingsPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_percentage: Option<String>,
    /// <p>The estimated total cost of the usage after purchasing the recommended Savings Plans. This is a sum of the cost of Savings Plans during this term, and the remaining On-Demand usage.</p>
    #[serde(rename = "estimatedTotalCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_total_cost: Option<String>,
    /// <p>The recommended hourly commitment based on the recommendation parameters.</p>
    #[serde(rename = "hourlyCommitmentToPurchase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_commitment_to_purchase: Option<String>,
    /// <p>The aggregate number of Savings Plans recommendations that exist for your account.</p>
    #[serde(rename = "totalRecommendationCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_recommendation_count: Option<String>,
}

/// <p>The amount of savings you're accumulating, against the public On-Demand rate of the usage accrued in an account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansSavings {
    /// <p>The savings amount that you are accumulating for the usage that is covered by a Savings Plans, when compared to the On-Demand equivalent of the same usage.</p>
    #[serde(rename = "netSavings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_savings: Option<String>,
    /// <p>How much the amount that the usage would have cost if it was accrued at the On-Demand rate.</p>
    #[serde(rename = "onDemandCostEquivalent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_cost_equivalent: Option<String>,
}

/// <p>The measurement of how well you are using your existing Savings Plans.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansUtilization {
    /// <p>The total amount of Savings Plans commitment that's been purchased in an account (or set of accounts).</p>
    #[serde(rename = "totalCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_commitment: Option<String>,
    /// <p>The amount of your Savings Plans commitment that was not consumed from Savings Plans eligible usage in a specific period.</p>
    #[serde(rename = "unusedCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_commitment: Option<String>,
    /// <p>The amount of your Savings Plans commitment that was consumed from Savings Plans eligible usage in a specific period.</p>
    #[serde(rename = "usedCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_commitment: Option<String>,
    /// <p>The amount of <code>UsedCommitment</code> divided by the <code>TotalCommitment</code> for your Savings Plans.</p>
    #[serde(rename = "utilizationPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_percentage: Option<String>,
}

/// <p>The aggregated utilization metrics for your Savings Plans usage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansUtilizationAggregates {
    /// <p>The total amortized commitment for a Savings Plans. This includes the sum of the upfront and recurring Savings Plans fees.</p>
    #[serde(rename = "amortizedCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_commitment: Option<SavingsPlansAmortizedCommitment>,
    /// <p>The amount saved by using existing Savings Plans. Savings returns both net savings from Savings Plans, as well as the <code>onDemandCostEquivalent</code> of the Savings Plans when considering the utilization rate.</p>
    #[serde(rename = "savings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings: Option<SavingsPlansSavings>,
    /// <p>A ratio of your effectiveness of using existing Savings Plans to apply to workloads that are Savings Plans eligible.</p>
    #[serde(rename = "utilization")]
    pub utilization: SavingsPlansUtilization,
}

/// <p>The amount of Savings Plans utilization, in hours.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansUtilizationByTime {
    /// <p>The total amortized commitment for a Savings Plans. This includes the sum of the upfront and recurring Savings Plans fees.</p>
    #[serde(rename = "amortizedCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_commitment: Option<SavingsPlansAmortizedCommitment>,
    /// <p>The amount saved by using existing Savings Plans. Savings returns both net savings from Savings Plans as well as the <code>onDemandCostEquivalent</code> of the Savings Plans when considering the utilization rate.</p>
    #[serde(rename = "savings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings: Option<SavingsPlansSavings>,
    #[serde(rename = "timePeriod")]
    pub time_period: DateInterval,
    /// <p>A ratio of your effectiveness of using existing Savings Plans to apply to workloads that are Savings Plans eligible.</p>
    #[serde(rename = "utilization")]
    pub utilization: SavingsPlansUtilization,
}

/// <p>A single daily or monthly Savings Plans utilization rate, and details for your account. A management account in an organization have access to member accounts. You can use <code>GetDimensionValues</code> to determine the possible dimension values. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlansUtilizationDetail {
    /// <p>The total amortized commitment for a Savings Plans. Includes the sum of the upfront and recurring Savings Plans fees.</p>
    #[serde(rename = "amortizedCommitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_commitment: Option<SavingsPlansAmortizedCommitment>,
    /// <p>The attribute that applies to a specific <code>Dimension</code>.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The amount saved by using existing Savings Plans. Savings returns both net savings from savings plans as well as the <code>onDemandCostEquivalent</code> of the Savings Plans when considering the utilization rate.</p>
    #[serde(rename = "savings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings: Option<SavingsPlansSavings>,
    /// <p>The unique Amazon Resource Name (ARN) for a particular Savings Plan.</p>
    #[serde(rename = "savingsPlanArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_arn: Option<String>,
    /// <p>A ratio of your effectiveness of using existing Savings Plans to apply to workloads that are Savings Plans eligible.</p>
    #[serde(rename = "utilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization: Option<SavingsPlansUtilization>,
}

/// <p>Hardware specifications for the service that you want recommendations for.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServiceSpecification {
    /// <p>The Amazon EC2 hardware specifications that you want AWS to provide recommendations for.</p>
    #[serde(rename = "eC2Specification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_specification: Option<EC2Specification>,
}

/// <p>The details of how to sort the data.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SortDefinition {
    /// <p>The key by which to sort the data.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The order in which to sort the data.</p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// <p> The recipient of <code>AnomalySubscription</code> notifications. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Subscriber {
    /// <p> The email address or SNS Amazon Resource Name (ARN), depending on the <code>Type</code>. </p>
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p> Indicates if the subscriber accepts the notifications. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The notification delivery channel. </p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The values that are available for a tag.</p> <p>If <code>Values</code> and <code>Key</code> are not specified, the <code>ABSENT</code> <code>MatchOption</code> is applied to all tags. That is, filtering on resources with no tags.</p> <p>If <code>Values</code> is provided and <code>Key</code> is not specified, the <code>ABSENT</code> <code>MatchOption</code> is applied to the tag <code>Key</code> only. That is, filtering on resources without the given tag key.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TagValues {
    /// <p>The key for the tag.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The match options that you can use to filter your results. <code>MatchOptions</code> is only applicable for actions related to Cost Category. The default values for <code>MatchOptions</code> are <code>EQUALS</code> and <code>CASE_SENSITIVE</code>.</p>
    #[serde(rename = "matchOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    /// <p>The specific value of the tag.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p> Details on recommended instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TargetInstance {
    /// <p> The currency code that AWS used to calculate the costs for this instance.</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p> Indicates whether this recommendation is the defaulted AWS recommendation.</p>
    #[serde(rename = "defaultTargetInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_target_instance: Option<bool>,
    /// <p> Expected cost to operate this instance type on a monthly basis.</p>
    #[serde(rename = "estimatedMonthlyCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_cost: Option<String>,
    /// <p> Estimated savings resulting from modification, on a monthly basis.</p>
    #[serde(rename = "estimatedMonthlySavings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings: Option<String>,
    /// <p> Expected utilization metrics for target instance type.</p>
    #[serde(rename = "expectedResourceUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_resource_utilization: Option<ResourceUtilization>,
    /// <p> Explains the actions you might need to take in order to successfully migrate your workloads from the current instance type to the recommended instance type. </p>
    #[serde(rename = "platformDifferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_differences: Option<Vec<String>>,
    /// <p> Details on the target instance type. </p>
    #[serde(rename = "resourceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<ResourceDetails>,
}

/// <p> Details on termination recommendation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TerminateRecommendationDetail {
    /// <p> The currency code that AWS used to calculate the costs for this instance.</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p> Estimated savings resulting from modification, on a monthly basis.</p>
    #[serde(rename = "estimatedMonthlySavings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings: Option<String>,
}

/// <p> Filters cost anomalies based on the total impact. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TotalImpactFilter {
    /// <p> The upper bound dollar value used in the filter. </p>
    #[serde(rename = "endValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_value: Option<f64>,
    /// <p> The comparing value used in the filter. </p>
    #[serde(rename = "numericOperator")]
    pub numeric_operator: String,
    /// <p> The lower bound dollar value used in the filter. </p>
    #[serde(rename = "startValue")]
    pub start_value: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAnomalyMonitorRequest {
    /// <p> Cost anomaly monitor Amazon Resource Names (ARNs). </p>
    #[serde(rename = "monitorArn")]
    pub monitor_arn: String,
    /// <p> The new name for the cost anomaly monitor. </p>
    #[serde(rename = "monitorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAnomalyMonitorResponse {
    /// <p> A cost anomaly monitor ARN. </p>
    #[serde(rename = "monitorArn")]
    pub monitor_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAnomalySubscriptionRequest {
    /// <p> The update to the frequency value at which subscribers will receive notifications. </p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// <p> A list of cost anomaly monitor ARNs. </p>
    #[serde(rename = "monitorArnList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn_list: Option<Vec<String>>,
    /// <p> The update to the subscriber list. </p>
    #[serde(rename = "subscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<Subscriber>>,
    /// <p> A cost anomaly subscription Amazon Resource Name (ARN). </p>
    #[serde(rename = "subscriptionArn")]
    pub subscription_arn: String,
    /// <p> The subscription's new name. </p>
    #[serde(rename = "subscriptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    /// <p> The update to the threshold value for receiving notifications. </p>
    #[serde(rename = "threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAnomalySubscriptionResponse {
    /// <p> A cost anomaly subscription ARN. </p>
    #[serde(rename = "subscriptionArn")]
    pub subscription_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCostCategoryDefinitionRequest {
    /// <p>The unique identifier for your Cost Category.</p>
    #[serde(rename = "costCategoryArn")]
    pub cost_category_arn: String,
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "ruleVersion")]
    pub rule_version: String,
    /// <p>The <code>Expression</code> object used to categorize costs. For more information, see <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_CostCategoryRule.html">CostCategoryRule </a>. </p>
    #[serde(rename = "rules")]
    pub rules: Vec<CostCategoryRule>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCostCategoryDefinitionResponse {
    /// <p> The unique identifier for your Cost Category. </p>
    #[serde(rename = "costCategoryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
    /// <p> The Cost Category's effective start date. </p>
    #[serde(rename = "effectiveStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start: Option<String>,
}

/// <p>The amount of utilization, in hours.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UtilizationByTime {
    /// <p>The groups that this utilization result uses.</p>
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ReservationUtilizationGroup>>,
    /// <p>The period of time that this utilization was used for.</p>
    #[serde(rename = "timePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
    /// <p>The total number of reservation hours that were used.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<ReservationAggregates>,
}

/// Errors returned by CreateAnomalyMonitor
#[derive(Debug, PartialEq)]
pub enum CreateAnomalyMonitorError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl CreateAnomalyMonitorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAnomalyMonitorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAnomalyMonitorError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAnomalyMonitorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAnomalyMonitorError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAnomalyMonitorError {}
/// Errors returned by CreateAnomalySubscription
#[derive(Debug, PartialEq)]
pub enum CreateAnomalySubscriptionError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>The cost anomaly monitor does not exist for the account. </p>
    UnknownMonitor(String),
}

impl CreateAnomalySubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAnomalySubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAnomalySubscriptionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "UnknownMonitorException" => {
                    return RusotoError::Service(CreateAnomalySubscriptionError::UnknownMonitor(
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
impl fmt::Display for CreateAnomalySubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAnomalySubscriptionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateAnomalySubscriptionError::UnknownMonitor(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAnomalySubscriptionError {}
/// Errors returned by CreateCostCategoryDefinition
#[derive(Debug, PartialEq)]
pub enum CreateCostCategoryDefinitionError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p> You've reached the limit on the number of resources you can create, or exceeded the size of an individual resource. </p>
    ServiceQuotaExceeded(String),
}

impl CreateCostCategoryDefinitionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateCostCategoryDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(CreateCostCategoryDefinitionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(
                        CreateCostCategoryDefinitionError::ServiceQuotaExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCostCategoryDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCostCategoryDefinitionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateCostCategoryDefinitionError::ServiceQuotaExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateCostCategoryDefinitionError {}
/// Errors returned by DeleteAnomalyMonitor
#[derive(Debug, PartialEq)]
pub enum DeleteAnomalyMonitorError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>The cost anomaly monitor does not exist for the account. </p>
    UnknownMonitor(String),
}

impl DeleteAnomalyMonitorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAnomalyMonitorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteAnomalyMonitorError::LimitExceeded(err.msg))
                }
                "UnknownMonitorException" => {
                    return RusotoError::Service(DeleteAnomalyMonitorError::UnknownMonitor(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAnomalyMonitorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAnomalyMonitorError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteAnomalyMonitorError::UnknownMonitor(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAnomalyMonitorError {}
/// Errors returned by DeleteAnomalySubscription
#[derive(Debug, PartialEq)]
pub enum DeleteAnomalySubscriptionError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>The cost anomaly subscription does not exist for the account. </p>
    UnknownSubscription(String),
}

impl DeleteAnomalySubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAnomalySubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteAnomalySubscriptionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "UnknownSubscriptionException" => {
                    return RusotoError::Service(
                        DeleteAnomalySubscriptionError::UnknownSubscription(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAnomalySubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAnomalySubscriptionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteAnomalySubscriptionError::UnknownSubscription(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteAnomalySubscriptionError {}
/// Errors returned by DeleteCostCategoryDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteCostCategoryDefinitionError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p> The specified ARN in the request doesn't exist. </p>
    ResourceNotFound(String),
}

impl DeleteCostCategoryDefinitionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteCostCategoryDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteCostCategoryDefinitionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteCostCategoryDefinitionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteCostCategoryDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCostCategoryDefinitionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteCostCategoryDefinitionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteCostCategoryDefinitionError {}
/// Errors returned by DescribeCostCategoryDefinition
#[derive(Debug, PartialEq)]
pub enum DescribeCostCategoryDefinitionError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p> The specified ARN in the request doesn't exist. </p>
    ResourceNotFound(String),
}

impl DescribeCostCategoryDefinitionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeCostCategoryDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(
                        DescribeCostCategoryDefinitionError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeCostCategoryDefinitionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCostCategoryDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCostCategoryDefinitionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeCostCategoryDefinitionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeCostCategoryDefinitionError {}
/// Errors returned by GetAnomalies
#[derive(Debug, PartialEq)]
pub enum GetAnomaliesError {
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl GetAnomaliesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAnomaliesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetAnomaliesError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetAnomaliesError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAnomaliesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAnomaliesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetAnomaliesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAnomaliesError {}
/// Errors returned by GetAnomalyMonitors
#[derive(Debug, PartialEq)]
pub enum GetAnomalyMonitorsError {
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>The cost anomaly monitor does not exist for the account. </p>
    UnknownMonitor(String),
}

impl GetAnomalyMonitorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAnomalyMonitorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetAnomalyMonitorsError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetAnomalyMonitorsError::LimitExceeded(err.msg))
                }
                "UnknownMonitorException" => {
                    return RusotoError::Service(GetAnomalyMonitorsError::UnknownMonitor(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAnomalyMonitorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAnomalyMonitorsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetAnomalyMonitorsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetAnomalyMonitorsError::UnknownMonitor(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAnomalyMonitorsError {}
/// Errors returned by GetAnomalySubscriptions
#[derive(Debug, PartialEq)]
pub enum GetAnomalySubscriptionsError {
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>The cost anomaly subscription does not exist for the account. </p>
    UnknownSubscription(String),
}

impl GetAnomalySubscriptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAnomalySubscriptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetAnomalySubscriptionsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetAnomalySubscriptionsError::LimitExceeded(
                        err.msg,
                    ))
                }
                "UnknownSubscriptionException" => {
                    return RusotoError::Service(GetAnomalySubscriptionsError::UnknownSubscription(
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
impl fmt::Display for GetAnomalySubscriptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAnomalySubscriptionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetAnomalySubscriptionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetAnomalySubscriptionsError::UnknownSubscription(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAnomalySubscriptionsError {}
/// Errors returned by GetCostAndUsage
#[derive(Debug, PartialEq)]
pub enum GetCostAndUsageError {
    /// <p>The requested report expired. Update the date interval and try again.</p>
    BillExpiration(String),
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>Your request parameters changed between pages. Try again with the old parameters or without a pagination token.</p>
    RequestChanged(String),
}

impl GetCostAndUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCostAndUsageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BillExpirationException" => {
                    return RusotoError::Service(GetCostAndUsageError::BillExpiration(err.msg))
                }
                "DataUnavailableException" => {
                    return RusotoError::Service(GetCostAndUsageError::DataUnavailable(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetCostAndUsageError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetCostAndUsageError::LimitExceeded(err.msg))
                }
                "RequestChangedException" => {
                    return RusotoError::Service(GetCostAndUsageError::RequestChanged(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCostAndUsageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCostAndUsageError::BillExpiration(ref cause) => write!(f, "{}", cause),
            GetCostAndUsageError::DataUnavailable(ref cause) => write!(f, "{}", cause),
            GetCostAndUsageError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetCostAndUsageError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetCostAndUsageError::RequestChanged(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCostAndUsageError {}
/// Errors returned by GetCostAndUsageWithResources
#[derive(Debug, PartialEq)]
pub enum GetCostAndUsageWithResourcesError {
    /// <p>The requested report expired. Update the date interval and try again.</p>
    BillExpiration(String),
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>Your request parameters changed between pages. Try again with the old parameters or without a pagination token.</p>
    RequestChanged(String),
}

impl GetCostAndUsageWithResourcesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCostAndUsageWithResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BillExpirationException" => {
                    return RusotoError::Service(GetCostAndUsageWithResourcesError::BillExpiration(
                        err.msg,
                    ))
                }
                "DataUnavailableException" => {
                    return RusotoError::Service(
                        GetCostAndUsageWithResourcesError::DataUnavailable(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetCostAndUsageWithResourcesError::InvalidNextToken(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetCostAndUsageWithResourcesError::LimitExceeded(
                        err.msg,
                    ))
                }
                "RequestChangedException" => {
                    return RusotoError::Service(GetCostAndUsageWithResourcesError::RequestChanged(
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
impl fmt::Display for GetCostAndUsageWithResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCostAndUsageWithResourcesError::BillExpiration(ref cause) => write!(f, "{}", cause),
            GetCostAndUsageWithResourcesError::DataUnavailable(ref cause) => write!(f, "{}", cause),
            GetCostAndUsageWithResourcesError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCostAndUsageWithResourcesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetCostAndUsageWithResourcesError::RequestChanged(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCostAndUsageWithResourcesError {}
/// Errors returned by GetCostCategories
#[derive(Debug, PartialEq)]
pub enum GetCostCategoriesError {
    /// <p>The requested report expired. Update the date interval and try again.</p>
    BillExpiration(String),
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>Your request parameters changed between pages. Try again with the old parameters or without a pagination token.</p>
    RequestChanged(String),
}

impl GetCostCategoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCostCategoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BillExpirationException" => {
                    return RusotoError::Service(GetCostCategoriesError::BillExpiration(err.msg))
                }
                "DataUnavailableException" => {
                    return RusotoError::Service(GetCostCategoriesError::DataUnavailable(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetCostCategoriesError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetCostCategoriesError::LimitExceeded(err.msg))
                }
                "RequestChangedException" => {
                    return RusotoError::Service(GetCostCategoriesError::RequestChanged(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCostCategoriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCostCategoriesError::BillExpiration(ref cause) => write!(f, "{}", cause),
            GetCostCategoriesError::DataUnavailable(ref cause) => write!(f, "{}", cause),
            GetCostCategoriesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetCostCategoriesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetCostCategoriesError::RequestChanged(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCostCategoriesError {}
/// Errors returned by GetCostForecast
#[derive(Debug, PartialEq)]
pub enum GetCostForecastError {
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl GetCostForecastError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCostForecastError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DataUnavailableException" => {
                    return RusotoError::Service(GetCostForecastError::DataUnavailable(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetCostForecastError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCostForecastError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCostForecastError::DataUnavailable(ref cause) => write!(f, "{}", cause),
            GetCostForecastError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCostForecastError {}
/// Errors returned by GetDimensionValues
#[derive(Debug, PartialEq)]
pub enum GetDimensionValuesError {
    /// <p>The requested report expired. Update the date interval and try again.</p>
    BillExpiration(String),
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>Your request parameters changed between pages. Try again with the old parameters or without a pagination token.</p>
    RequestChanged(String),
}

impl GetDimensionValuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDimensionValuesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BillExpirationException" => {
                    return RusotoError::Service(GetDimensionValuesError::BillExpiration(err.msg))
                }
                "DataUnavailableException" => {
                    return RusotoError::Service(GetDimensionValuesError::DataUnavailable(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetDimensionValuesError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetDimensionValuesError::LimitExceeded(err.msg))
                }
                "RequestChangedException" => {
                    return RusotoError::Service(GetDimensionValuesError::RequestChanged(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDimensionValuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDimensionValuesError::BillExpiration(ref cause) => write!(f, "{}", cause),
            GetDimensionValuesError::DataUnavailable(ref cause) => write!(f, "{}", cause),
            GetDimensionValuesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetDimensionValuesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetDimensionValuesError::RequestChanged(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDimensionValuesError {}
/// Errors returned by GetReservationCoverage
#[derive(Debug, PartialEq)]
pub enum GetReservationCoverageError {
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl GetReservationCoverageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetReservationCoverageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DataUnavailableException" => {
                    return RusotoError::Service(GetReservationCoverageError::DataUnavailable(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetReservationCoverageError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetReservationCoverageError::LimitExceeded(
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
impl fmt::Display for GetReservationCoverageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetReservationCoverageError::DataUnavailable(ref cause) => write!(f, "{}", cause),
            GetReservationCoverageError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetReservationCoverageError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetReservationCoverageError {}
/// Errors returned by GetReservationPurchaseRecommendation
#[derive(Debug, PartialEq)]
pub enum GetReservationPurchaseRecommendationError {
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl GetReservationPurchaseRecommendationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetReservationPurchaseRecommendationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DataUnavailableException" => {
                    return RusotoError::Service(
                        GetReservationPurchaseRecommendationError::DataUnavailable(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetReservationPurchaseRecommendationError::InvalidNextToken(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        GetReservationPurchaseRecommendationError::LimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetReservationPurchaseRecommendationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetReservationPurchaseRecommendationError::DataUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetReservationPurchaseRecommendationError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetReservationPurchaseRecommendationError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetReservationPurchaseRecommendationError {}
/// Errors returned by GetReservationUtilization
#[derive(Debug, PartialEq)]
pub enum GetReservationUtilizationError {
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl GetReservationUtilizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetReservationUtilizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DataUnavailableException" => {
                    return RusotoError::Service(GetReservationUtilizationError::DataUnavailable(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetReservationUtilizationError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetReservationUtilizationError::LimitExceeded(
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
impl fmt::Display for GetReservationUtilizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetReservationUtilizationError::DataUnavailable(ref cause) => write!(f, "{}", cause),
            GetReservationUtilizationError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetReservationUtilizationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetReservationUtilizationError {}
/// Errors returned by GetRightsizingRecommendation
#[derive(Debug, PartialEq)]
pub enum GetRightsizingRecommendationError {
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl GetRightsizingRecommendationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRightsizingRecommendationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetRightsizingRecommendationError::InvalidNextToken(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetRightsizingRecommendationError::LimitExceeded(
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
impl fmt::Display for GetRightsizingRecommendationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRightsizingRecommendationError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRightsizingRecommendationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRightsizingRecommendationError {}
/// Errors returned by GetSavingsPlansCoverage
#[derive(Debug, PartialEq)]
pub enum GetSavingsPlansCoverageError {
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl GetSavingsPlansCoverageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSavingsPlansCoverageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DataUnavailableException" => {
                    return RusotoError::Service(GetSavingsPlansCoverageError::DataUnavailable(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetSavingsPlansCoverageError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetSavingsPlansCoverageError::LimitExceeded(
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
impl fmt::Display for GetSavingsPlansCoverageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSavingsPlansCoverageError::DataUnavailable(ref cause) => write!(f, "{}", cause),
            GetSavingsPlansCoverageError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetSavingsPlansCoverageError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSavingsPlansCoverageError {}
/// Errors returned by GetSavingsPlansPurchaseRecommendation
#[derive(Debug, PartialEq)]
pub enum GetSavingsPlansPurchaseRecommendationError {
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl GetSavingsPlansPurchaseRecommendationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetSavingsPlansPurchaseRecommendationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetSavingsPlansPurchaseRecommendationError::InvalidNextToken(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        GetSavingsPlansPurchaseRecommendationError::LimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSavingsPlansPurchaseRecommendationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSavingsPlansPurchaseRecommendationError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetSavingsPlansPurchaseRecommendationError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetSavingsPlansPurchaseRecommendationError {}
/// Errors returned by GetSavingsPlansUtilization
#[derive(Debug, PartialEq)]
pub enum GetSavingsPlansUtilizationError {
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl GetSavingsPlansUtilizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetSavingsPlansUtilizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DataUnavailableException" => {
                    return RusotoError::Service(GetSavingsPlansUtilizationError::DataUnavailable(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetSavingsPlansUtilizationError::LimitExceeded(
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
impl fmt::Display for GetSavingsPlansUtilizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSavingsPlansUtilizationError::DataUnavailable(ref cause) => write!(f, "{}", cause),
            GetSavingsPlansUtilizationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSavingsPlansUtilizationError {}
/// Errors returned by GetSavingsPlansUtilizationDetails
#[derive(Debug, PartialEq)]
pub enum GetSavingsPlansUtilizationDetailsError {
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl GetSavingsPlansUtilizationDetailsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetSavingsPlansUtilizationDetailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DataUnavailableException" => {
                    return RusotoError::Service(
                        GetSavingsPlansUtilizationDetailsError::DataUnavailable(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetSavingsPlansUtilizationDetailsError::InvalidNextToken(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        GetSavingsPlansUtilizationDetailsError::LimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSavingsPlansUtilizationDetailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSavingsPlansUtilizationDetailsError::DataUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetSavingsPlansUtilizationDetailsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetSavingsPlansUtilizationDetailsError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetSavingsPlansUtilizationDetailsError {}
/// Errors returned by GetTags
#[derive(Debug, PartialEq)]
pub enum GetTagsError {
    /// <p>The requested report expired. Update the date interval and try again.</p>
    BillExpiration(String),
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>Your request parameters changed between pages. Try again with the old parameters or without a pagination token.</p>
    RequestChanged(String),
}

impl GetTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BillExpirationException" => {
                    return RusotoError::Service(GetTagsError::BillExpiration(err.msg))
                }
                "DataUnavailableException" => {
                    return RusotoError::Service(GetTagsError::DataUnavailable(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetTagsError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetTagsError::LimitExceeded(err.msg))
                }
                "RequestChangedException" => {
                    return RusotoError::Service(GetTagsError::RequestChanged(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTagsError::BillExpiration(ref cause) => write!(f, "{}", cause),
            GetTagsError::DataUnavailable(ref cause) => write!(f, "{}", cause),
            GetTagsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetTagsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetTagsError::RequestChanged(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTagsError {}
/// Errors returned by GetUsageForecast
#[derive(Debug, PartialEq)]
pub enum GetUsageForecastError {
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>Cost Explorer was unable to identify the usage unit. Provide <code>UsageType/UsageTypeGroup</code> filter selections that contain matching units, for example: <code>hours</code>.</p>
    UnresolvableUsageUnit(String),
}

impl GetUsageForecastError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUsageForecastError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DataUnavailableException" => {
                    return RusotoError::Service(GetUsageForecastError::DataUnavailable(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetUsageForecastError::LimitExceeded(err.msg))
                }
                "UnresolvableUsageUnitException" => {
                    return RusotoError::Service(GetUsageForecastError::UnresolvableUsageUnit(
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
impl fmt::Display for GetUsageForecastError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUsageForecastError::DataUnavailable(ref cause) => write!(f, "{}", cause),
            GetUsageForecastError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetUsageForecastError::UnresolvableUsageUnit(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUsageForecastError {}
/// Errors returned by ListCostCategoryDefinitions
#[derive(Debug, PartialEq)]
pub enum ListCostCategoryDefinitionsError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl ListCostCategoryDefinitionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListCostCategoryDefinitionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(ListCostCategoryDefinitionsError::LimitExceeded(
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
impl fmt::Display for ListCostCategoryDefinitionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCostCategoryDefinitionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListCostCategoryDefinitionsError {}
/// Errors returned by ProvideAnomalyFeedback
#[derive(Debug, PartialEq)]
pub enum ProvideAnomalyFeedbackError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
}

impl ProvideAnomalyFeedbackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ProvideAnomalyFeedbackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(ProvideAnomalyFeedbackError::LimitExceeded(
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
impl fmt::Display for ProvideAnomalyFeedbackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProvideAnomalyFeedbackError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ProvideAnomalyFeedbackError {}
/// Errors returned by UpdateAnomalyMonitor
#[derive(Debug, PartialEq)]
pub enum UpdateAnomalyMonitorError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>The cost anomaly monitor does not exist for the account. </p>
    UnknownMonitor(String),
}

impl UpdateAnomalyMonitorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAnomalyMonitorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateAnomalyMonitorError::LimitExceeded(err.msg))
                }
                "UnknownMonitorException" => {
                    return RusotoError::Service(UpdateAnomalyMonitorError::UnknownMonitor(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAnomalyMonitorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAnomalyMonitorError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateAnomalyMonitorError::UnknownMonitor(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAnomalyMonitorError {}
/// Errors returned by UpdateAnomalySubscription
#[derive(Debug, PartialEq)]
pub enum UpdateAnomalySubscriptionError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>The cost anomaly monitor does not exist for the account. </p>
    UnknownMonitor(String),
    /// <p>The cost anomaly subscription does not exist for the account. </p>
    UnknownSubscription(String),
}

impl UpdateAnomalySubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAnomalySubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateAnomalySubscriptionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "UnknownMonitorException" => {
                    return RusotoError::Service(UpdateAnomalySubscriptionError::UnknownMonitor(
                        err.msg,
                    ))
                }
                "UnknownSubscriptionException" => {
                    return RusotoError::Service(
                        UpdateAnomalySubscriptionError::UnknownSubscription(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAnomalySubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAnomalySubscriptionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateAnomalySubscriptionError::UnknownMonitor(ref cause) => write!(f, "{}", cause),
            UpdateAnomalySubscriptionError::UnknownSubscription(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateAnomalySubscriptionError {}
/// Errors returned by UpdateCostCategoryDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateCostCategoryDefinitionError {
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p> The specified ARN in the request doesn't exist. </p>
    ResourceNotFound(String),
    /// <p> You've reached the limit on the number of resources you can create, or exceeded the size of an individual resource. </p>
    ServiceQuotaExceeded(String),
}

impl UpdateCostCategoryDefinitionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateCostCategoryDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateCostCategoryDefinitionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateCostCategoryDefinitionError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(
                        UpdateCostCategoryDefinitionError::ServiceQuotaExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateCostCategoryDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCostCategoryDefinitionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateCostCategoryDefinitionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCostCategoryDefinitionError::ServiceQuotaExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateCostCategoryDefinitionError {}
/// Trait representing the capabilities of the AWS Cost Explorer API. AWS Cost Explorer clients implement this trait.
#[async_trait]
pub trait CostExplorer {
    /// <p>Creates a new cost anomaly detection monitor with the requested type and monitor specification. </p>
    async fn create_anomaly_monitor(
        &self,
        input: CreateAnomalyMonitorRequest,
    ) -> Result<CreateAnomalyMonitorResponse, RusotoError<CreateAnomalyMonitorError>>;

    /// <p>Adds a subscription to a cost anomaly detection monitor. You can use each subscription to define subscribers with email or SNS notifications. Email subscribers can set a dollar threshold and a time frequency for receiving notifications. </p>
    async fn create_anomaly_subscription(
        &self,
        input: CreateAnomalySubscriptionRequest,
    ) -> Result<CreateAnomalySubscriptionResponse, RusotoError<CreateAnomalySubscriptionError>>;

    /// <p>Creates a new Cost Category with the requested name and rules.</p>
    async fn create_cost_category_definition(
        &self,
        input: CreateCostCategoryDefinitionRequest,
    ) -> Result<CreateCostCategoryDefinitionResponse, RusotoError<CreateCostCategoryDefinitionError>>;

    /// <p>Deletes a cost anomaly monitor. </p>
    async fn delete_anomaly_monitor(
        &self,
        input: DeleteAnomalyMonitorRequest,
    ) -> Result<DeleteAnomalyMonitorResponse, RusotoError<DeleteAnomalyMonitorError>>;

    /// <p>Deletes a cost anomaly subscription. </p>
    async fn delete_anomaly_subscription(
        &self,
        input: DeleteAnomalySubscriptionRequest,
    ) -> Result<DeleteAnomalySubscriptionResponse, RusotoError<DeleteAnomalySubscriptionError>>;

    /// <p>Deletes a Cost Category. Expenses from this month going forward will no longer be categorized with this Cost Category.</p>
    async fn delete_cost_category_definition(
        &self,
        input: DeleteCostCategoryDefinitionRequest,
    ) -> Result<DeleteCostCategoryDefinitionResponse, RusotoError<DeleteCostCategoryDefinitionError>>;

    /// <p>Returns the name, ARN, rules, definition, and effective dates of a Cost Category that's defined in the account.</p> <p>You have the option to use <code>EffectiveOn</code> to return a Cost Category that is active on a specific date. If there is no <code>EffectiveOn</code> specified, you’ll see a Cost Category that is effective on the current date. If Cost Category is still effective, <code>EffectiveEnd</code> is omitted in the response. </p>
    async fn describe_cost_category_definition(
        &self,
        input: DescribeCostCategoryDefinitionRequest,
    ) -> Result<
        DescribeCostCategoryDefinitionResponse,
        RusotoError<DescribeCostCategoryDefinitionError>,
    >;

    /// <p>Retrieves all of the cost anomalies detected on your account, during the time period specified by the <code>DateInterval</code> object. </p>
    async fn get_anomalies(
        &self,
        input: GetAnomaliesRequest,
    ) -> Result<GetAnomaliesResponse, RusotoError<GetAnomaliesError>>;

    /// <p>Retrieves the cost anomaly monitor definitions for your account. You can filter using a list of cost anomaly monitor Amazon Resource Names (ARNs). </p>
    async fn get_anomaly_monitors(
        &self,
        input: GetAnomalyMonitorsRequest,
    ) -> Result<GetAnomalyMonitorsResponse, RusotoError<GetAnomalyMonitorsError>>;

    /// <p>Retrieves the cost anomaly subscription objects for your account. You can filter using a list of cost anomaly monitor Amazon Resource Names (ARNs). </p>
    async fn get_anomaly_subscriptions(
        &self,
        input: GetAnomalySubscriptionsRequest,
    ) -> Result<GetAnomalySubscriptionsResponse, RusotoError<GetAnomalySubscriptionsError>>;

    /// <p>Retrieves cost and usage metrics for your account. You can specify which cost and usage-related metric, such as <code>BlendedCosts</code> or <code>UsageQuantity</code>, that you want the request to return. You can also filter and group your data by various dimensions, such as <code>SERVICE</code> or <code>AZ</code>, in a specific time range. For a complete list of valid dimensions, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_GetDimensionValues.html">GetDimensionValues</a> operation. Management account in an organization in AWS Organizations have access to all member accounts.</p> <p>For information about filter limitations, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-limits.html">Quotas and restrictions</a> in the <i>Billing and Cost Management User Guide</i>.</p>
    async fn get_cost_and_usage(
        &self,
        input: GetCostAndUsageRequest,
    ) -> Result<GetCostAndUsageResponse, RusotoError<GetCostAndUsageError>>;

    /// <p><p>Retrieves cost and usage metrics with resources for your account. You can specify which cost and usage-related metric, such as <code>BlendedCosts</code> or <code>UsageQuantity</code>, that you want the request to return. You can also filter and group your data by various dimensions, such as <code>SERVICE</code> or <code>AZ</code>, in a specific time range. For a complete list of valid dimensions, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_GetDimensionValues.html">GetDimensionValues</a> operation. Management account in an organization in AWS Organizations have access to all member accounts. This API is currently available for the Amazon Elastic Compute Cloud – Compute service only.</p> <note> <p>This is an opt-in only feature. You can enable this feature from the Cost Explorer Settings page. For information on how to access the Settings page, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/ce-access.html">Controlling Access for Cost Explorer</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> </note></p>
    async fn get_cost_and_usage_with_resources(
        &self,
        input: GetCostAndUsageWithResourcesRequest,
    ) -> Result<GetCostAndUsageWithResourcesResponse, RusotoError<GetCostAndUsageWithResourcesError>>;

    /// <p><p>Retrieves an array of Cost Category names and values incurred cost.</p> <note> <p>If some Cost Category names and values are not associated with any cost, they will not be returned by this API.</p> </note></p>
    async fn get_cost_categories(
        &self,
        input: GetCostCategoriesRequest,
    ) -> Result<GetCostCategoriesResponse, RusotoError<GetCostCategoriesError>>;

    /// <p>Retrieves a forecast for how much Amazon Web Services predicts that you will spend over the forecast time period that you select, based on your past costs. </p>
    async fn get_cost_forecast(
        &self,
        input: GetCostForecastRequest,
    ) -> Result<GetCostForecastResponse, RusotoError<GetCostForecastError>>;

    /// <p>Retrieves all available filter values for a specified filter over a period of time. You can search the dimension values for an arbitrary string. </p>
    async fn get_dimension_values(
        &self,
        input: GetDimensionValuesRequest,
    ) -> Result<GetDimensionValuesResponse, RusotoError<GetDimensionValuesError>>;

    /// <p>Retrieves the reservation coverage for your account. This enables you to see how much of your Amazon Elastic Compute Cloud, Amazon ElastiCache, Amazon Relational Database Service, or Amazon Redshift usage is covered by a reservation. An organization's management account can see the coverage of the associated member accounts. This supports dimensions, Cost Categories, and nested expressions. For any time period, you can filter data about reservation usage by the following dimensions:</p> <ul> <li> <p>AZ</p> </li> <li> <p>CACHE_ENGINE</p> </li> <li> <p>DATABASE_ENGINE</p> </li> <li> <p>DEPLOYMENT_OPTION</p> </li> <li> <p>INSTANCE_TYPE</p> </li> <li> <p>LINKED_ACCOUNT</p> </li> <li> <p>OPERATING_SYSTEM</p> </li> <li> <p>PLATFORM</p> </li> <li> <p>REGION</p> </li> <li> <p>SERVICE</p> </li> <li> <p>TAG</p> </li> <li> <p>TENANCY</p> </li> </ul> <p>To determine valid values for a dimension, use the <code>GetDimensionValues</code> operation. </p>
    async fn get_reservation_coverage(
        &self,
        input: GetReservationCoverageRequest,
    ) -> Result<GetReservationCoverageResponse, RusotoError<GetReservationCoverageError>>;

    /// <p>Gets recommendations for which reservations to purchase. These recommendations could help you reduce your costs. Reservations provide a discounted hourly rate (up to 75%) compared to On-Demand pricing.</p> <p>AWS generates your recommendations by identifying your On-Demand usage during a specific time period and collecting your usage into categories that are eligible for a reservation. After AWS has these categories, it simulates every combination of reservations in each category of usage to identify the best number of each type of RI to purchase to maximize your estimated savings. </p> <p>For example, AWS automatically aggregates your Amazon EC2 Linux, shared tenancy, and c4 family usage in the US West (Oregon) Region and recommends that you buy size-flexible regional reservations to apply to the c4 family usage. AWS recommends the smallest size instance in an instance family. This makes it easier to purchase a size-flexible RI. AWS also shows the equal number of normalized units so that you can purchase any instance size that you want. For this example, your RI recommendation would be for <code>c4.large</code> because that is the smallest size instance in the c4 instance family.</p>
    async fn get_reservation_purchase_recommendation(
        &self,
        input: GetReservationPurchaseRecommendationRequest,
    ) -> Result<
        GetReservationPurchaseRecommendationResponse,
        RusotoError<GetReservationPurchaseRecommendationError>,
    >;

    /// <p>Retrieves the reservation utilization for your account. Management account in an organization have access to member accounts. You can filter data by dimensions in a time period. You can use <code>GetDimensionValues</code> to determine the possible dimension values. Currently, you can group only by <code>SUBSCRIPTION_ID</code>. </p>
    async fn get_reservation_utilization(
        &self,
        input: GetReservationUtilizationRequest,
    ) -> Result<GetReservationUtilizationResponse, RusotoError<GetReservationUtilizationError>>;

    /// <p>Creates recommendations that help you save cost by identifying idle and underutilized Amazon EC2 instances.</p> <p>Recommendations are generated to either downsize or terminate instances, along with providing savings detail and metrics. For details on calculation and function, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/ce-rightsizing.html">Optimizing Your Cost with Rightsizing Recommendations</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    async fn get_rightsizing_recommendation(
        &self,
        input: GetRightsizingRecommendationRequest,
    ) -> Result<GetRightsizingRecommendationResponse, RusotoError<GetRightsizingRecommendationError>>;

    /// <p>Retrieves the Savings Plans covered for your account. This enables you to see how much of your cost is covered by a Savings Plan. An organization’s management account can see the coverage of the associated member accounts. This supports dimensions, Cost Categories, and nested expressions. For any time period, you can filter data for Savings Plans usage with the following dimensions:</p> <ul> <li> <p> <code>LINKED_ACCOUNT</code> </p> </li> <li> <p> <code>REGION</code> </p> </li> <li> <p> <code>SERVICE</code> </p> </li> <li> <p> <code>INSTANCE_FAMILY</code> </p> </li> </ul> <p>To determine valid values for a dimension, use the <code>GetDimensionValues</code> operation.</p>
    async fn get_savings_plans_coverage(
        &self,
        input: GetSavingsPlansCoverageRequest,
    ) -> Result<GetSavingsPlansCoverageResponse, RusotoError<GetSavingsPlansCoverageError>>;

    /// <p>Retrieves your request parameters, Savings Plan Recommendations Summary and Details. </p>
    async fn get_savings_plans_purchase_recommendation(
        &self,
        input: GetSavingsPlansPurchaseRecommendationRequest,
    ) -> Result<
        GetSavingsPlansPurchaseRecommendationResponse,
        RusotoError<GetSavingsPlansPurchaseRecommendationError>,
    >;

    /// <p><p>Retrieves the Savings Plans utilization for your account across date ranges with daily or monthly granularity. Management account in an organization have access to member accounts. You can use <code>GetDimensionValues</code> in <code>SAVINGS_PLANS</code> to determine the possible dimension values.</p> <note> <p>You cannot group by any dimension values for <code>GetSavingsPlansUtilization</code>.</p> </note></p>
    async fn get_savings_plans_utilization(
        &self,
        input: GetSavingsPlansUtilizationRequest,
    ) -> Result<GetSavingsPlansUtilizationResponse, RusotoError<GetSavingsPlansUtilizationError>>;

    /// <p><p>Retrieves attribute data along with aggregate utilization and savings data for a given time period. This doesn&#39;t support granular or grouped data (daily/monthly) in response. You can&#39;t retrieve data by dates in a single response similar to <code>GetSavingsPlanUtilization</code>, but you have the option to make multiple calls to <code>GetSavingsPlanUtilizationDetails</code> by providing individual dates. You can use <code>GetDimensionValues</code> in <code>SAVINGS_PLANS</code> to determine the possible dimension values.</p> <note> <p> <code>GetSavingsPlanUtilizationDetails</code> internally groups data by <code>SavingsPlansArn</code>.</p> </note></p>
    async fn get_savings_plans_utilization_details(
        &self,
        input: GetSavingsPlansUtilizationDetailsRequest,
    ) -> Result<
        GetSavingsPlansUtilizationDetailsResponse,
        RusotoError<GetSavingsPlansUtilizationDetailsError>,
    >;

    /// <p>Queries for available tag keys and tag values for a specified period. You can search the tag values for an arbitrary string. </p>
    async fn get_tags(
        &self,
        input: GetTagsRequest,
    ) -> Result<GetTagsResponse, RusotoError<GetTagsError>>;

    /// <p>Retrieves a forecast for how much Amazon Web Services predicts that you will use over the forecast time period that you select, based on your past usage. </p>
    async fn get_usage_forecast(
        &self,
        input: GetUsageForecastRequest,
    ) -> Result<GetUsageForecastResponse, RusotoError<GetUsageForecastError>>;

    /// <p>Returns the name, ARN, <code>NumberOfRules</code> and effective dates of all Cost Categories defined in the account. You have the option to use <code>EffectiveOn</code> to return a list of Cost Categories that were active on a specific date. If there is no <code>EffectiveOn</code> specified, you’ll see Cost Categories that are effective on the current date. If Cost Category is still effective, <code>EffectiveEnd</code> is omitted in the response. <code>ListCostCategoryDefinitions</code> supports pagination. The request can have a <code>MaxResults</code> range up to 100.</p>
    async fn list_cost_category_definitions(
        &self,
        input: ListCostCategoryDefinitionsRequest,
    ) -> Result<ListCostCategoryDefinitionsResponse, RusotoError<ListCostCategoryDefinitionsError>>;

    /// <p>Modifies the feedback property of a given cost anomaly. </p>
    async fn provide_anomaly_feedback(
        &self,
        input: ProvideAnomalyFeedbackRequest,
    ) -> Result<ProvideAnomalyFeedbackResponse, RusotoError<ProvideAnomalyFeedbackError>>;

    /// <p>Updates an existing cost anomaly monitor. The changes made are applied going forward, and does not change anomalies detected in the past. </p>
    async fn update_anomaly_monitor(
        &self,
        input: UpdateAnomalyMonitorRequest,
    ) -> Result<UpdateAnomalyMonitorResponse, RusotoError<UpdateAnomalyMonitorError>>;

    /// <p> Updates an existing cost anomaly monitor subscription. </p>
    async fn update_anomaly_subscription(
        &self,
        input: UpdateAnomalySubscriptionRequest,
    ) -> Result<UpdateAnomalySubscriptionResponse, RusotoError<UpdateAnomalySubscriptionError>>;

    /// <p>Updates an existing Cost Category. Changes made to the Cost Category rules will be used to categorize the current month’s expenses and future expenses. This won’t change categorization for the previous months.</p>
    async fn update_cost_category_definition(
        &self,
        input: UpdateCostCategoryDefinitionRequest,
    ) -> Result<UpdateCostCategoryDefinitionResponse, RusotoError<UpdateCostCategoryDefinitionError>>;
}
/// A client for the AWS Cost Explorer API.
#[derive(Clone)]
pub struct CostExplorerClient {
    client: Client,
    region: region::Region,
}

impl CostExplorerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CostExplorerClient {
        CostExplorerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CostExplorerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CostExplorerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CostExplorerClient {
        CostExplorerClient { client, region }
    }
}

#[async_trait]
impl CostExplorer for CostExplorerClient {
    /// <p>Creates a new cost anomaly detection monitor with the requested type and monitor specification. </p>
    async fn create_anomaly_monitor(
        &self,
        input: CreateAnomalyMonitorRequest,
    ) -> Result<CreateAnomalyMonitorResponse, RusotoError<CreateAnomalyMonitorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.CreateAnomalyMonitor",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateAnomalyMonitorError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateAnomalyMonitorResponse, _>()
    }

    /// <p>Adds a subscription to a cost anomaly detection monitor. You can use each subscription to define subscribers with email or SNS notifications. Email subscribers can set a dollar threshold and a time frequency for receiving notifications. </p>
    async fn create_anomaly_subscription(
        &self,
        input: CreateAnomalySubscriptionRequest,
    ) -> Result<CreateAnomalySubscriptionResponse, RusotoError<CreateAnomalySubscriptionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.CreateAnomalySubscription",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateAnomalySubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateAnomalySubscriptionResponse, _>()
    }

    /// <p>Creates a new Cost Category with the requested name and rules.</p>
    async fn create_cost_category_definition(
        &self,
        input: CreateCostCategoryDefinitionRequest,
    ) -> Result<CreateCostCategoryDefinitionResponse, RusotoError<CreateCostCategoryDefinitionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.CreateCostCategoryDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateCostCategoryDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateCostCategoryDefinitionResponse, _>()
    }

    /// <p>Deletes a cost anomaly monitor. </p>
    async fn delete_anomaly_monitor(
        &self,
        input: DeleteAnomalyMonitorRequest,
    ) -> Result<DeleteAnomalyMonitorResponse, RusotoError<DeleteAnomalyMonitorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.DeleteAnomalyMonitor",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAnomalyMonitorError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteAnomalyMonitorResponse, _>()
    }

    /// <p>Deletes a cost anomaly subscription. </p>
    async fn delete_anomaly_subscription(
        &self,
        input: DeleteAnomalySubscriptionRequest,
    ) -> Result<DeleteAnomalySubscriptionResponse, RusotoError<DeleteAnomalySubscriptionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.DeleteAnomalySubscription",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAnomalySubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteAnomalySubscriptionResponse, _>()
    }

    /// <p>Deletes a Cost Category. Expenses from this month going forward will no longer be categorized with this Cost Category.</p>
    async fn delete_cost_category_definition(
        &self,
        input: DeleteCostCategoryDefinitionRequest,
    ) -> Result<DeleteCostCategoryDefinitionResponse, RusotoError<DeleteCostCategoryDefinitionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.DeleteCostCategoryDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteCostCategoryDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteCostCategoryDefinitionResponse, _>()
    }

    /// <p>Returns the name, ARN, rules, definition, and effective dates of a Cost Category that's defined in the account.</p> <p>You have the option to use <code>EffectiveOn</code> to return a Cost Category that is active on a specific date. If there is no <code>EffectiveOn</code> specified, you’ll see a Cost Category that is effective on the current date. If Cost Category is still effective, <code>EffectiveEnd</code> is omitted in the response. </p>
    async fn describe_cost_category_definition(
        &self,
        input: DescribeCostCategoryDefinitionRequest,
    ) -> Result<
        DescribeCostCategoryDefinitionResponse,
        RusotoError<DescribeCostCategoryDefinitionError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.DescribeCostCategoryDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeCostCategoryDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeCostCategoryDefinitionResponse, _>()
    }

    /// <p>Retrieves all of the cost anomalies detected on your account, during the time period specified by the <code>DateInterval</code> object. </p>
    async fn get_anomalies(
        &self,
        input: GetAnomaliesRequest,
    ) -> Result<GetAnomaliesResponse, RusotoError<GetAnomaliesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSInsightsIndexService.GetAnomalies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAnomaliesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetAnomaliesResponse, _>()
    }

    /// <p>Retrieves the cost anomaly monitor definitions for your account. You can filter using a list of cost anomaly monitor Amazon Resource Names (ARNs). </p>
    async fn get_anomaly_monitors(
        &self,
        input: GetAnomalyMonitorsRequest,
    ) -> Result<GetAnomalyMonitorsResponse, RusotoError<GetAnomalyMonitorsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSInsightsIndexService.GetAnomalyMonitors");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAnomalyMonitorsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetAnomalyMonitorsResponse, _>()
    }

    /// <p>Retrieves the cost anomaly subscription objects for your account. You can filter using a list of cost anomaly monitor Amazon Resource Names (ARNs). </p>
    async fn get_anomaly_subscriptions(
        &self,
        input: GetAnomalySubscriptionsRequest,
    ) -> Result<GetAnomalySubscriptionsResponse, RusotoError<GetAnomalySubscriptionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetAnomalySubscriptions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAnomalySubscriptionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetAnomalySubscriptionsResponse, _>()
    }

    /// <p>Retrieves cost and usage metrics for your account. You can specify which cost and usage-related metric, such as <code>BlendedCosts</code> or <code>UsageQuantity</code>, that you want the request to return. You can also filter and group your data by various dimensions, such as <code>SERVICE</code> or <code>AZ</code>, in a specific time range. For a complete list of valid dimensions, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_GetDimensionValues.html">GetDimensionValues</a> operation. Management account in an organization in AWS Organizations have access to all member accounts.</p> <p>For information about filter limitations, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-limits.html">Quotas and restrictions</a> in the <i>Billing and Cost Management User Guide</i>.</p>
    async fn get_cost_and_usage(
        &self,
        input: GetCostAndUsageRequest,
    ) -> Result<GetCostAndUsageResponse, RusotoError<GetCostAndUsageError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSInsightsIndexService.GetCostAndUsage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetCostAndUsageError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetCostAndUsageResponse, _>()
    }

    /// <p><p>Retrieves cost and usage metrics with resources for your account. You can specify which cost and usage-related metric, such as <code>BlendedCosts</code> or <code>UsageQuantity</code>, that you want the request to return. You can also filter and group your data by various dimensions, such as <code>SERVICE</code> or <code>AZ</code>, in a specific time range. For a complete list of valid dimensions, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_GetDimensionValues.html">GetDimensionValues</a> operation. Management account in an organization in AWS Organizations have access to all member accounts. This API is currently available for the Amazon Elastic Compute Cloud – Compute service only.</p> <note> <p>This is an opt-in only feature. You can enable this feature from the Cost Explorer Settings page. For information on how to access the Settings page, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/ce-access.html">Controlling Access for Cost Explorer</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> </note></p>
    async fn get_cost_and_usage_with_resources(
        &self,
        input: GetCostAndUsageWithResourcesRequest,
    ) -> Result<GetCostAndUsageWithResourcesResponse, RusotoError<GetCostAndUsageWithResourcesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetCostAndUsageWithResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetCostAndUsageWithResourcesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetCostAndUsageWithResourcesResponse, _>()
    }

    /// <p><p>Retrieves an array of Cost Category names and values incurred cost.</p> <note> <p>If some Cost Category names and values are not associated with any cost, they will not be returned by this API.</p> </note></p>
    async fn get_cost_categories(
        &self,
        input: GetCostCategoriesRequest,
    ) -> Result<GetCostCategoriesResponse, RusotoError<GetCostCategoriesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSInsightsIndexService.GetCostCategories");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetCostCategoriesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetCostCategoriesResponse, _>()
    }

    /// <p>Retrieves a forecast for how much Amazon Web Services predicts that you will spend over the forecast time period that you select, based on your past costs. </p>
    async fn get_cost_forecast(
        &self,
        input: GetCostForecastRequest,
    ) -> Result<GetCostForecastResponse, RusotoError<GetCostForecastError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSInsightsIndexService.GetCostForecast");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetCostForecastError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetCostForecastResponse, _>()
    }

    /// <p>Retrieves all available filter values for a specified filter over a period of time. You can search the dimension values for an arbitrary string. </p>
    async fn get_dimension_values(
        &self,
        input: GetDimensionValuesRequest,
    ) -> Result<GetDimensionValuesResponse, RusotoError<GetDimensionValuesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSInsightsIndexService.GetDimensionValues");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetDimensionValuesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetDimensionValuesResponse, _>()
    }

    /// <p>Retrieves the reservation coverage for your account. This enables you to see how much of your Amazon Elastic Compute Cloud, Amazon ElastiCache, Amazon Relational Database Service, or Amazon Redshift usage is covered by a reservation. An organization's management account can see the coverage of the associated member accounts. This supports dimensions, Cost Categories, and nested expressions. For any time period, you can filter data about reservation usage by the following dimensions:</p> <ul> <li> <p>AZ</p> </li> <li> <p>CACHE_ENGINE</p> </li> <li> <p>DATABASE_ENGINE</p> </li> <li> <p>DEPLOYMENT_OPTION</p> </li> <li> <p>INSTANCE_TYPE</p> </li> <li> <p>LINKED_ACCOUNT</p> </li> <li> <p>OPERATING_SYSTEM</p> </li> <li> <p>PLATFORM</p> </li> <li> <p>REGION</p> </li> <li> <p>SERVICE</p> </li> <li> <p>TAG</p> </li> <li> <p>TENANCY</p> </li> </ul> <p>To determine valid values for a dimension, use the <code>GetDimensionValues</code> operation. </p>
    async fn get_reservation_coverage(
        &self,
        input: GetReservationCoverageRequest,
    ) -> Result<GetReservationCoverageResponse, RusotoError<GetReservationCoverageError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetReservationCoverage",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetReservationCoverageError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetReservationCoverageResponse, _>()
    }

    /// <p>Gets recommendations for which reservations to purchase. These recommendations could help you reduce your costs. Reservations provide a discounted hourly rate (up to 75%) compared to On-Demand pricing.</p> <p>AWS generates your recommendations by identifying your On-Demand usage during a specific time period and collecting your usage into categories that are eligible for a reservation. After AWS has these categories, it simulates every combination of reservations in each category of usage to identify the best number of each type of RI to purchase to maximize your estimated savings. </p> <p>For example, AWS automatically aggregates your Amazon EC2 Linux, shared tenancy, and c4 family usage in the US West (Oregon) Region and recommends that you buy size-flexible regional reservations to apply to the c4 family usage. AWS recommends the smallest size instance in an instance family. This makes it easier to purchase a size-flexible RI. AWS also shows the equal number of normalized units so that you can purchase any instance size that you want. For this example, your RI recommendation would be for <code>c4.large</code> because that is the smallest size instance in the c4 instance family.</p>
    async fn get_reservation_purchase_recommendation(
        &self,
        input: GetReservationPurchaseRecommendationRequest,
    ) -> Result<
        GetReservationPurchaseRecommendationResponse,
        RusotoError<GetReservationPurchaseRecommendationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetReservationPurchaseRecommendation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                GetReservationPurchaseRecommendationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetReservationPurchaseRecommendationResponse, _>()
    }

    /// <p>Retrieves the reservation utilization for your account. Management account in an organization have access to member accounts. You can filter data by dimensions in a time period. You can use <code>GetDimensionValues</code> to determine the possible dimension values. Currently, you can group only by <code>SUBSCRIPTION_ID</code>. </p>
    async fn get_reservation_utilization(
        &self,
        input: GetReservationUtilizationRequest,
    ) -> Result<GetReservationUtilizationResponse, RusotoError<GetReservationUtilizationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetReservationUtilization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetReservationUtilizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetReservationUtilizationResponse, _>()
    }

    /// <p>Creates recommendations that help you save cost by identifying idle and underutilized Amazon EC2 instances.</p> <p>Recommendations are generated to either downsize or terminate instances, along with providing savings detail and metrics. For details on calculation and function, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/ce-rightsizing.html">Optimizing Your Cost with Rightsizing Recommendations</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    async fn get_rightsizing_recommendation(
        &self,
        input: GetRightsizingRecommendationRequest,
    ) -> Result<GetRightsizingRecommendationResponse, RusotoError<GetRightsizingRecommendationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetRightsizingRecommendation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRightsizingRecommendationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetRightsizingRecommendationResponse, _>()
    }

    /// <p>Retrieves the Savings Plans covered for your account. This enables you to see how much of your cost is covered by a Savings Plan. An organization’s management account can see the coverage of the associated member accounts. This supports dimensions, Cost Categories, and nested expressions. For any time period, you can filter data for Savings Plans usage with the following dimensions:</p> <ul> <li> <p> <code>LINKED_ACCOUNT</code> </p> </li> <li> <p> <code>REGION</code> </p> </li> <li> <p> <code>SERVICE</code> </p> </li> <li> <p> <code>INSTANCE_FAMILY</code> </p> </li> </ul> <p>To determine valid values for a dimension, use the <code>GetDimensionValues</code> operation.</p>
    async fn get_savings_plans_coverage(
        &self,
        input: GetSavingsPlansCoverageRequest,
    ) -> Result<GetSavingsPlansCoverageResponse, RusotoError<GetSavingsPlansCoverageError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetSavingsPlansCoverage",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetSavingsPlansCoverageError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetSavingsPlansCoverageResponse, _>()
    }

    /// <p>Retrieves your request parameters, Savings Plan Recommendations Summary and Details. </p>
    async fn get_savings_plans_purchase_recommendation(
        &self,
        input: GetSavingsPlansPurchaseRecommendationRequest,
    ) -> Result<
        GetSavingsPlansPurchaseRecommendationResponse,
        RusotoError<GetSavingsPlansPurchaseRecommendationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetSavingsPlansPurchaseRecommendation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                GetSavingsPlansPurchaseRecommendationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetSavingsPlansPurchaseRecommendationResponse, _>()
    }

    /// <p><p>Retrieves the Savings Plans utilization for your account across date ranges with daily or monthly granularity. Management account in an organization have access to member accounts. You can use <code>GetDimensionValues</code> in <code>SAVINGS_PLANS</code> to determine the possible dimension values.</p> <note> <p>You cannot group by any dimension values for <code>GetSavingsPlansUtilization</code>.</p> </note></p>
    async fn get_savings_plans_utilization(
        &self,
        input: GetSavingsPlansUtilizationRequest,
    ) -> Result<GetSavingsPlansUtilizationResponse, RusotoError<GetSavingsPlansUtilizationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetSavingsPlansUtilization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetSavingsPlansUtilizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetSavingsPlansUtilizationResponse, _>()
    }

    /// <p><p>Retrieves attribute data along with aggregate utilization and savings data for a given time period. This doesn&#39;t support granular or grouped data (daily/monthly) in response. You can&#39;t retrieve data by dates in a single response similar to <code>GetSavingsPlanUtilization</code>, but you have the option to make multiple calls to <code>GetSavingsPlanUtilizationDetails</code> by providing individual dates. You can use <code>GetDimensionValues</code> in <code>SAVINGS_PLANS</code> to determine the possible dimension values.</p> <note> <p> <code>GetSavingsPlanUtilizationDetails</code> internally groups data by <code>SavingsPlansArn</code>.</p> </note></p>
    async fn get_savings_plans_utilization_details(
        &self,
        input: GetSavingsPlansUtilizationDetailsRequest,
    ) -> Result<
        GetSavingsPlansUtilizationDetailsResponse,
        RusotoError<GetSavingsPlansUtilizationDetailsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetSavingsPlansUtilizationDetails",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                GetSavingsPlansUtilizationDetailsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetSavingsPlansUtilizationDetailsResponse, _>()
    }

    /// <p>Queries for available tag keys and tag values for a specified period. You can search the tag values for an arbitrary string. </p>
    async fn get_tags(
        &self,
        input: GetTagsRequest,
    ) -> Result<GetTagsResponse, RusotoError<GetTagsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSInsightsIndexService.GetTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetTagsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetTagsResponse, _>()
    }

    /// <p>Retrieves a forecast for how much Amazon Web Services predicts that you will use over the forecast time period that you select, based on your past usage. </p>
    async fn get_usage_forecast(
        &self,
        input: GetUsageForecastRequest,
    ) -> Result<GetUsageForecastResponse, RusotoError<GetUsageForecastError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSInsightsIndexService.GetUsageForecast");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetUsageForecastError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetUsageForecastResponse, _>()
    }

    /// <p>Returns the name, ARN, <code>NumberOfRules</code> and effective dates of all Cost Categories defined in the account. You have the option to use <code>EffectiveOn</code> to return a list of Cost Categories that were active on a specific date. If there is no <code>EffectiveOn</code> specified, you’ll see Cost Categories that are effective on the current date. If Cost Category is still effective, <code>EffectiveEnd</code> is omitted in the response. <code>ListCostCategoryDefinitions</code> supports pagination. The request can have a <code>MaxResults</code> range up to 100.</p>
    async fn list_cost_category_definitions(
        &self,
        input: ListCostCategoryDefinitionsRequest,
    ) -> Result<ListCostCategoryDefinitionsResponse, RusotoError<ListCostCategoryDefinitionsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.ListCostCategoryDefinitions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListCostCategoryDefinitionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListCostCategoryDefinitionsResponse, _>()
    }

    /// <p>Modifies the feedback property of a given cost anomaly. </p>
    async fn provide_anomaly_feedback(
        &self,
        input: ProvideAnomalyFeedbackRequest,
    ) -> Result<ProvideAnomalyFeedbackResponse, RusotoError<ProvideAnomalyFeedbackError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.ProvideAnomalyFeedback",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ProvideAnomalyFeedbackError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ProvideAnomalyFeedbackResponse, _>()
    }

    /// <p>Updates an existing cost anomaly monitor. The changes made are applied going forward, and does not change anomalies detected in the past. </p>
    async fn update_anomaly_monitor(
        &self,
        input: UpdateAnomalyMonitorRequest,
    ) -> Result<UpdateAnomalyMonitorResponse, RusotoError<UpdateAnomalyMonitorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.UpdateAnomalyMonitor",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateAnomalyMonitorError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateAnomalyMonitorResponse, _>()
    }

    /// <p> Updates an existing cost anomaly monitor subscription. </p>
    async fn update_anomaly_subscription(
        &self,
        input: UpdateAnomalySubscriptionRequest,
    ) -> Result<UpdateAnomalySubscriptionResponse, RusotoError<UpdateAnomalySubscriptionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.UpdateAnomalySubscription",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateAnomalySubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateAnomalySubscriptionResponse, _>()
    }

    /// <p>Updates an existing Cost Category. Changes made to the Cost Category rules will be used to categorize the current month’s expenses and future expenses. This won’t change categorization for the previous months.</p>
    async fn update_cost_category_definition(
        &self,
        input: UpdateCostCategoryDefinitionRequest,
    ) -> Result<UpdateCostCategoryDefinitionResponse, RusotoError<UpdateCostCategoryDefinitionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.UpdateCostCategoryDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateCostCategoryDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateCostCategoryDefinitionResponse, _>()
    }
}
