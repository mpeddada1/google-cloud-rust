// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.

#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::redundant_explicit_links)]

/// Workflow program to be executed by Workflows.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Workflow {
    /// The resource name of the workflow.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Description of the workflow provided by the user.
    /// Must be at most 1000 unicode characters long.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,

    /// Output only. State of the workflow deployment.
    pub state: crate::model::workflow::State,

    /// Output only. The revision of the workflow.
    /// A new revision of a workflow is created as a result of updating the
    /// following properties of a workflow:
    ///
    /// - [Service account][google.cloud.workflows.v1.Workflow.service_account]
    /// - [Workflow code to beexecuted][google.cloud.workflows.v1.Workflow.source_contents]
    /// The format is "000001-a4d", where the first six characters define
    /// the zero-padded revision ordinal number. They are followed by a hyphen and
    /// three hexadecimal random characters.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub revision_id: String,

    /// Output only. The timestamp for when the workflow was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<wkt::Timestamp>,

    /// Output only. The timestamp for when the workflow was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<wkt::Timestamp>,

    /// Output only. The timestamp for the latest revision of the workflow's
    /// creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_create_time: Option<wkt::Timestamp>,

    /// Labels associated with this workflow.
    /// Labels can contain at most 64 entries. Keys and values can be no longer
    /// than 63 characters and can only contain lowercase letters, numeric
    /// characters, underscores, and dashes. Label keys must start with a letter.
    /// International characters are allowed.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<String, String>,

    /// The service account associated with the latest workflow version.
    /// This service account represents the identity of the workflow and determines
    /// what permissions the workflow has.
    /// Format: projects/{project}/serviceAccounts/{account} or {account}
    ///
    /// Using `-` as a wildcard for the `{project}` or not providing one at all
    /// will infer the project from the account. The `{account}` value can be the
    /// `email` address or the `unique_id` of the service account.
    ///
    /// If not provided, workflow will use the project's default service account.
    /// Modifying this field for an existing workflow results in a new workflow
    /// revision.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub service_account: String,

    /// Optional. The resource name of a KMS crypto key used to encrypt or decrypt
    /// the data associated with the workflow.
    ///
    /// Format:
    /// projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}
    ///
    /// Using `-` as a wildcard for the `{project}` or not providing one at all
    /// will infer the project from the account.
    ///
    /// If not provided, data associated with the workflow will not be
    /// CMEK-encrypted.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub crypto_key_name: String,

    /// Output only. Error regarding the state of the workflow. For example, this
    /// field will have error details if the execution data is unavailable due to
    /// revoked KMS key permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_error: Option<crate::model::workflow::StateError>,

    /// Optional. Describes the level of platform logging to apply to calls and
    /// call responses during executions of this workflow. If both the workflow and
    /// the execution specify a logging level, the execution level takes
    /// precedence.
    pub call_log_level: crate::model::workflow::CallLogLevel,

    /// Optional. User-defined environment variables associated with this workflow
    /// revision. This map has a maximum length of 20. Each string can take up to
    /// 40KiB. Keys cannot be empty strings and cannot start with “GOOGLE” or
    /// “WORKFLOWS".
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub user_env_vars: std::collections::HashMap<String, String>,

    /// Required. Location of the workflow source code.
    /// Modifying this field for an existing workflow results in a new workflow
    /// revision.
    #[serde(flatten)]
    pub source_code: Option<crate::model::workflow::SourceCode>,
}

impl Workflow {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `description`.
    pub fn set_description<T: Into<String>>(mut self, v: T) -> Self {
        self.description = v.into();
        self
    }

    /// Sets the value of `state`.
    pub fn set_state<T: Into<crate::model::workflow::State>>(mut self, v: T) -> Self {
        self.state = v.into();
        self
    }

    /// Sets the value of `revision_id`.
    pub fn set_revision_id<T: Into<String>>(mut self, v: T) -> Self {
        self.revision_id = v.into();
        self
    }

    /// Sets the value of `create_time`.
    pub fn set_create_time<T: Into<Option<wkt::Timestamp>>>(mut self, v: T) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of `update_time`.
    pub fn set_update_time<T: Into<Option<wkt::Timestamp>>>(mut self, v: T) -> Self {
        self.update_time = v.into();
        self
    }

    /// Sets the value of `revision_create_time`.
    pub fn set_revision_create_time<T: Into<Option<wkt::Timestamp>>>(mut self, v: T) -> Self {
        self.revision_create_time = v.into();
        self
    }

    /// Sets the value of `labels`.
    pub fn set_labels<T: Into<std::collections::HashMap<String, String>>>(mut self, v: T) -> Self {
        self.labels = v.into();
        self
    }

    /// Sets the value of `service_account`.
    pub fn set_service_account<T: Into<String>>(mut self, v: T) -> Self {
        self.service_account = v.into();
        self
    }

    /// Sets the value of `crypto_key_name`.
    pub fn set_crypto_key_name<T: Into<String>>(mut self, v: T) -> Self {
        self.crypto_key_name = v.into();
        self
    }

    /// Sets the value of `state_error`.
    pub fn set_state_error<T: Into<Option<crate::model::workflow::StateError>>>(
        mut self,
        v: T,
    ) -> Self {
        self.state_error = v.into();
        self
    }

    /// Sets the value of `call_log_level`.
    pub fn set_call_log_level<T: Into<crate::model::workflow::CallLogLevel>>(
        mut self,
        v: T,
    ) -> Self {
        self.call_log_level = v.into();
        self
    }

    /// Sets the value of `user_env_vars`.
    pub fn set_user_env_vars<T: Into<std::collections::HashMap<String, String>>>(
        mut self,
        v: T,
    ) -> Self {
        self.user_env_vars = v.into();
        self
    }

    /// Sets the value of `source_code`.
    pub fn set_source_code<T: Into<Option<crate::model::workflow::SourceCode>>>(
        mut self,
        v: T,
    ) -> Self {
        self.source_code = v.into();
        self
    }
}

/// Defines additional types related to Workflow
pub mod workflow {

    /// Describes an error related to the current state of the workflow.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct StateError {
        /// Provides specifics about the error.
        #[serde(skip_serializing_if = "String::is_empty")]
        pub details: String,

        /// The type of this state error.
        #[serde(rename = "type")]
        pub r#type: crate::model::workflow::state_error::Type,
    }

    impl StateError {
        /// Sets the value of `details`.
        pub fn set_details<T: Into<String>>(mut self, v: T) -> Self {
            self.details = v.into();
            self
        }

        /// Sets the value of `r#type`.
        pub fn set_type<T: Into<crate::model::workflow::state_error::Type>>(
            mut self,
            v: T,
        ) -> Self {
            self.r#type = v.into();
            self
        }
    }

    /// Defines additional types related to StateError
    pub mod state_error {

        /// Describes the possibled types of a state error.
        #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
        pub struct Type(String);

        impl Type {
            /// Sets the enum value.
            pub fn set_value<T: Into<String>>(mut self, v: T) -> Self {
                self.0 = v.into();
                self
            }

            /// Gets the enum value.
            pub fn value(&self) -> &str {
                &self.0
            }
        }

        /// Useful constants to work with [Type](Type)
        pub mod r#type {

            /// No type specified.
            pub const TYPE_UNSPECIFIED: &str = "TYPE_UNSPECIFIED";

            /// Caused by an issue with KMS.
            pub const KMS_ERROR: &str = "KMS_ERROR";
        }
    }

    /// Describes the current state of workflow deployment.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct State(String);

    impl State {
        /// Sets the enum value.
        pub fn set_value<T: Into<String>>(mut self, v: T) -> Self {
            self.0 = v.into();
            self
        }

        /// Gets the enum value.
        pub fn value(&self) -> &str {
            &self.0
        }
    }

    /// Useful constants to work with [State](State)
    pub mod state {

        /// Invalid state.
        pub const STATE_UNSPECIFIED: &str = "STATE_UNSPECIFIED";

        /// The workflow has been deployed successfully and is serving.
        pub const ACTIVE: &str = "ACTIVE";

        /// Workflow data is unavailable. See the `state_error` field.
        pub const UNAVAILABLE: &str = "UNAVAILABLE";
    }

    /// Describes the level of platform logging to apply to calls and call
    /// responses during workflow executions.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct CallLogLevel(String);

    impl CallLogLevel {
        /// Sets the enum value.
        pub fn set_value<T: Into<String>>(mut self, v: T) -> Self {
            self.0 = v.into();
            self
        }

        /// Gets the enum value.
        pub fn value(&self) -> &str {
            &self.0
        }
    }

    /// Useful constants to work with [CallLogLevel](CallLogLevel)
    pub mod call_log_level {

        /// No call logging level specified.
        pub const CALL_LOG_LEVEL_UNSPECIFIED: &str = "CALL_LOG_LEVEL_UNSPECIFIED";

        /// Log all call steps within workflows, all call returns, and all exceptions
        /// raised.
        pub const LOG_ALL_CALLS: &str = "LOG_ALL_CALLS";

        /// Log only exceptions that are raised from call steps within workflows.
        pub const LOG_ERRORS_ONLY: &str = "LOG_ERRORS_ONLY";

        /// Explicitly log nothing.
        pub const LOG_NONE: &str = "LOG_NONE";
    }

    /// Required. Location of the workflow source code.
    /// Modifying this field for an existing workflow results in a new workflow
    /// revision.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub enum SourceCode {
        /// Workflow code to be executed. The size limit is 128KB.
        SourceContents { source_contents: String },
    }
}

/// Request for the
/// [ListWorkflows][google.cloud.workflows.v1.Workflows.ListWorkflows]
/// method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListWorkflowsRequest {
    /// Required. Project and location from which the workflows should be listed.
    /// Format: projects/{project}/locations/{location}
    #[serde(skip_serializing_if = "String::is_empty")]
    pub parent: String,

    /// Maximum number of workflows to return per call. The service might return
    /// fewer than this value even if not at the end of the collection. If a value
    /// is not specified, a default value of 500 is used. The maximum permitted
    /// value is 1000 and values greater than 1000 are coerced down to 1000.
    pub page_size: i32,

    /// A page token, received from a previous `ListWorkflows` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListWorkflows` must
    /// match the call that provided the page token.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub page_token: String,

    /// Filter to restrict results to specific workflows.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub filter: String,

    /// Comma-separated list of fields that specify the order of the results.
    /// Default sorting order for a field is ascending. To specify descending order
    /// for a field, append a "desc" suffix.
    /// If not specified, the results are returned in an unspecified order.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub order_by: String,
}

impl ListWorkflowsRequest {
    /// Sets the value of `parent`.
    pub fn set_parent<T: Into<String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }

    /// Sets the value of `order_by`.
    pub fn set_order_by<T: Into<String>>(mut self, v: T) -> Self {
        self.order_by = v.into();
        self
    }
}

/// Response for the
/// [ListWorkflows][google.cloud.workflows.v1.Workflows.ListWorkflows]
/// method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListWorkflowsResponse {
    /// The workflows that match the request.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub workflows: Vec<crate::model::Workflow>,

    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_page_token: String,

    /// Unreachable resources.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unreachable: Vec<String>,
}

impl ListWorkflowsResponse {
    /// Sets the value of `workflows`.
    pub fn set_workflows<T: Into<Vec<crate::model::Workflow>>>(mut self, v: T) -> Self {
        self.workflows = v.into();
        self
    }

    /// Sets the value of `next_page_token`.
    pub fn set_next_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of `unreachable`.
    pub fn set_unreachable<T: Into<Vec<String>>>(mut self, v: T) -> Self {
        self.unreachable = v.into();
        self
    }
}

#[cfg(feature = "unstable-stream")]
impl gax::paginator::PageableResponse for ListWorkflowsResponse {
    type PageItem = crate::model::Workflow;

    fn items(self) -> Vec<Self::PageItem> {
        self.workflows
    }

    fn next_page_token(&self) -> String {
        gax::paginator::extract_token(&self.next_page_token)
    }
}

/// Request for the
/// [GetWorkflow][google.cloud.workflows.v1.Workflows.GetWorkflow] method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetWorkflowRequest {
    /// Required. Name of the workflow for which information should be retrieved.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Optional. The revision of the workflow to retrieve. If the revision_id is
    /// empty, the latest revision is retrieved.
    /// The format is "000001-a4d", where the first six characters define
    /// the zero-padded decimal revision number. They are followed by a hyphen and
    /// three hexadecimal characters.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub revision_id: String,
}

impl GetWorkflowRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `revision_id`.
    pub fn set_revision_id<T: Into<String>>(mut self, v: T) -> Self {
        self.revision_id = v.into();
        self
    }
}

/// Request for the
/// [CreateWorkflow][google.cloud.workflows.v1.Workflows.CreateWorkflow]
/// method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateWorkflowRequest {
    /// Required. Project and location in which the workflow should be created.
    /// Format:  projects/{project}/locations/{location}
    #[serde(skip_serializing_if = "String::is_empty")]
    pub parent: String,

    /// Required. Workflow to be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<crate::model::Workflow>,

    /// Required. The ID of the workflow to be created. It has to fulfill the
    /// following requirements:
    ///
    /// * Must contain only letters, numbers, underscores and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-64 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the customer project and location.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub workflow_id: String,
}

impl CreateWorkflowRequest {
    /// Sets the value of `parent`.
    pub fn set_parent<T: Into<String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of `workflow`.
    pub fn set_workflow<T: Into<Option<crate::model::Workflow>>>(mut self, v: T) -> Self {
        self.workflow = v.into();
        self
    }

    /// Sets the value of `workflow_id`.
    pub fn set_workflow_id<T: Into<String>>(mut self, v: T) -> Self {
        self.workflow_id = v.into();
        self
    }
}

/// Request for the
/// [DeleteWorkflow][google.cloud.workflows.v1.Workflows.DeleteWorkflow]
/// method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteWorkflowRequest {
    /// Required. Name of the workflow to be deleted.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
}

impl DeleteWorkflowRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

/// Request for the
/// [UpdateWorkflow][google.cloud.workflows.v1.Workflows.UpdateWorkflow]
/// method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct UpdateWorkflowRequest {
    /// Required. Workflow to be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<crate::model::Workflow>,

    /// List of fields to be updated. If not present, the entire workflow
    /// will be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mask: Option<wkt::FieldMask>,
}

impl UpdateWorkflowRequest {
    /// Sets the value of `workflow`.
    pub fn set_workflow<T: Into<Option<crate::model::Workflow>>>(mut self, v: T) -> Self {
        self.workflow = v.into();
        self
    }

    /// Sets the value of `update_mask`.
    pub fn set_update_mask<T: Into<Option<wkt::FieldMask>>>(mut self, v: T) -> Self {
        self.update_mask = v.into();
        self
    }
}

/// Represents the metadata of the long-running operation.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<wkt::Timestamp>,

    /// The time the operation finished running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<wkt::Timestamp>,

    /// Server-defined resource path for the target of the operation.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub target: String,

    /// Name of the verb executed by the operation.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub verb: String,

    /// API version used to start the operation.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub api_version: String,
}

impl OperationMetadata {
    /// Sets the value of `create_time`.
    pub fn set_create_time<T: Into<Option<wkt::Timestamp>>>(mut self, v: T) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of `end_time`.
    pub fn set_end_time<T: Into<Option<wkt::Timestamp>>>(mut self, v: T) -> Self {
        self.end_time = v.into();
        self
    }

    /// Sets the value of `target`.
    pub fn set_target<T: Into<String>>(mut self, v: T) -> Self {
        self.target = v.into();
        self
    }

    /// Sets the value of `verb`.
    pub fn set_verb<T: Into<String>>(mut self, v: T) -> Self {
        self.verb = v.into();
        self
    }

    /// Sets the value of `api_version`.
    pub fn set_api_version<T: Into<String>>(mut self, v: T) -> Self {
        self.api_version = v.into();
        self
    }
}
