// Copyright 2024 Google LLC
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

/// Describes the cause of the error with structured details.
/// Example of an error when contacting the "pubsub.googleapis.com" API when it
/// is not enabled:
/// ```norust
/// { "reason": "API_DISABLED"
///   "domain": "googleapis.com"
///   "metadata": {
///     "resource": "projects/123",
///     "service": "pubsub.googleapis.com"
///   }
/// }
/// ```
/// This response indicates that the pubsub.googleapis.com API is not enabled.
/// Example of an error that is returned when attempting to create a Spanner
/// instance in a region that is out of stock:
/// ```norust
/// { "reason": "STOCKOUT"
///   "domain": "spanner.googleapis.com",
///   "metadata": {
///     "availableRegions": "us-central1,us-east2"
///   }
/// }
/// ```
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ErrorInfo {
    /// The reason of the error. This is a constant value that identifies the
    /// proximate cause of the error. Error reasons are unique within a particular
    /// domain of errors. This should be at most 63 characters and match a
    /// regular expression of `[A-Z][A-Z0-9_]+[A-Z0-9]`, which represents
    /// UPPER_SNAKE_CASE.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// The logical grouping to which the "reason" belongs. The error domain
    /// is typically the registered service name of the tool or product that
    /// generates the error. Example: "pubsub.googleapis.com". If the error is
    /// generated by some common infrastructure, the error domain must be a
    /// globally unique value that identifies the infrastructure. For Google API
    /// infrastructure, the error domain is "googleapis.com".
    #[serde(skip_serializing_if = "String::is_empty")]
    pub domain: String,

    /// Additional structured details about this error.
    /// Keys must match a regular expression of `[a-z][a-zA-Z0-9-_]+` but should
    /// ideally be lowerCamelCase. Also, they must be limited to 64 characters in
    /// length. When identifying the current value of an exceeded limit, the units
    /// should be contained in the key, not the value.  For example, rather than
    /// `{"instanceLimit": "100/request"}`, should be returned as,
    /// `{"instanceLimitPerRequest": "100"}`, if the client exceeds the number of
    /// instances that can be created in a single (batch) request.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
}

impl ErrorInfo {
    /// Sets the value of `reason`.
    pub fn set_reason<T: Into<String>>(mut self, v: T) -> Self {
        self.reason = v.into();
        self
    }

    /// Sets the value of `domain`.
    pub fn set_domain<T: Into<String>>(mut self, v: T) -> Self {
        self.domain = v.into();
        self
    }

    /// Sets the value of `metadata`.
    pub fn set_metadata<T: Into<std::collections::HashMap<String, String>>>(
        mut self,
        v: T,
    ) -> Self {
        self.metadata = v.into();
        self
    }
}

/// Describes when the clients can retry a failed request. Clients could ignore
/// the recommendation here or retry when this information is missing from error
/// responses.
/// It's always recommended that clients should use exponential backoff when
/// retrying.
/// Clients should wait until `retry_delay` amount of time has passed since
/// receiving the error response before retrying.  If retrying requests also
/// fail, clients should use an exponential backoff scheme to gradually increase
/// the delay between retries based on `retry_delay`, until either a maximum
/// number of retries have been reached or a maximum retry delay cap has been
/// reached.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct RetryInfo {
    /// Clients should wait at least this long between retrying the same request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay: Option<wkt::Duration>,
}

impl RetryInfo {
    /// Sets the value of `retry_delay`.
    pub fn set_retry_delay<T: Into<Option<wkt::Duration>>>(mut self, v: T) -> Self {
        self.retry_delay = v.into();
        self
    }
}

/// Describes additional debugging info.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DebugInfo {
    /// The stack trace entries indicating where the error occurred.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stack_entries: Vec<String>,

    /// Additional debugging information provided by the server.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub detail: String,
}

impl DebugInfo {
    /// Sets the value of `stack_entries`.
    pub fn set_stack_entries<T: Into<Vec<String>>>(mut self, v: T) -> Self {
        self.stack_entries = v.into();
        self
    }

    /// Sets the value of `detail`.
    pub fn set_detail<T: Into<String>>(mut self, v: T) -> Self {
        self.detail = v.into();
        self
    }
}

/// Describes how a quota check failed.
/// For example if a daily limit was exceeded for the calling project,
/// a service could respond with a QuotaFailure detail containing the project
/// id and the description of the quota limit that was exceeded.  If the
/// calling project hasn't enabled the service in the developer console, then
/// a service could respond with the project id and set `service_disabled`
/// to true.
/// Also see RetryInfo and Help types for other details about handling a
/// quota failure.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct QuotaFailure {
    /// Describes all quota violations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub violations: Vec<crate::model::quota_failure::Violation>,
}

impl QuotaFailure {
    /// Sets the value of `violations`.
    pub fn set_violations<T: Into<Vec<crate::model::quota_failure::Violation>>>(
        mut self,
        v: T,
    ) -> Self {
        self.violations = v.into();
        self
    }
}

/// Defines additional types related to QuotaFailure
pub mod quota_failure {

    /// A message type used to describe a single quota violation.  For example, a
    /// daily quota or a custom quota that was exceeded.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct Violation {
        /// The subject on which the quota check failed.
        /// For example, "clientip:<ip address of client>" or "project:<Google
        /// developer project id>".
        #[serde(skip_serializing_if = "String::is_empty")]
        pub subject: String,

        /// A description of how the quota check failed. Clients can use this
        /// description to find more about the quota configuration in the service's
        /// public documentation, or find the relevant quota limit to adjust through
        /// developer console.
        /// For example: "Service disabled" or "Daily Limit for read operations
        /// exceeded".
        #[serde(skip_serializing_if = "String::is_empty")]
        pub description: String,
    }

    impl Violation {
        /// Sets the value of `subject`.
        pub fn set_subject<T: Into<String>>(mut self, v: T) -> Self {
            self.subject = v.into();
            self
        }

        /// Sets the value of `description`.
        pub fn set_description<T: Into<String>>(mut self, v: T) -> Self {
            self.description = v.into();
            self
        }
    }
}

/// Describes what preconditions have failed.
/// For example, if an RPC failed because it required the Terms of Service to be
/// acknowledged, it could list the terms of service violation in the
/// PreconditionFailure message.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct PreconditionFailure {
    /// Describes all precondition violations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub violations: Vec<crate::model::precondition_failure::Violation>,
}

impl PreconditionFailure {
    /// Sets the value of `violations`.
    pub fn set_violations<T: Into<Vec<crate::model::precondition_failure::Violation>>>(
        mut self,
        v: T,
    ) -> Self {
        self.violations = v.into();
        self
    }
}

/// Defines additional types related to PreconditionFailure
pub mod precondition_failure {

    /// A message type used to describe a single precondition failure.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct Violation {
        /// The type of PreconditionFailure. We recommend using a service-specific
        /// enum type to define the supported precondition violation subjects. For
        /// example, "TOS" for "Terms of Service violation".
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "String::is_empty")]
        pub r#type: String,

        /// The subject, relative to the type, that failed.
        /// For example, "google.com/cloud" relative to the "TOS" type would indicate
        /// which terms of service is being referenced.
        #[serde(skip_serializing_if = "String::is_empty")]
        pub subject: String,

        /// A description of how the precondition failed. Developers can use this
        /// description to understand how to fix the failure.
        /// For example: "Terms of service not accepted".
        #[serde(skip_serializing_if = "String::is_empty")]
        pub description: String,
    }

    impl Violation {
        /// Sets the value of `r#type`.
        pub fn set_type<T: Into<String>>(mut self, v: T) -> Self {
            self.r#type = v.into();
            self
        }

        /// Sets the value of `subject`.
        pub fn set_subject<T: Into<String>>(mut self, v: T) -> Self {
            self.subject = v.into();
            self
        }

        /// Sets the value of `description`.
        pub fn set_description<T: Into<String>>(mut self, v: T) -> Self {
            self.description = v.into();
            self
        }
    }
}

/// Describes violations in a client request. This error type focuses on the
/// syntactic aspects of the request.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct BadRequest {
    /// Describes all violations in a client request.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub field_violations: Vec<crate::model::bad_request::FieldViolation>,
}

impl BadRequest {
    /// Sets the value of `field_violations`.
    pub fn set_field_violations<T: Into<Vec<crate::model::bad_request::FieldViolation>>>(
        mut self,
        v: T,
    ) -> Self {
        self.field_violations = v.into();
        self
    }
}

/// Defines additional types related to BadRequest
pub mod bad_request {

    /// A message type used to describe a single bad request field.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct FieldViolation {
        /// A path that leads to a field in the request body. The value will be a
        /// sequence of dot-separated identifiers that identify a protocol buffer
        /// field.
        /// Consider the following:
        /// ```norust
        /// message CreateContactRequest {
        ///   message EmailAddress {
        ///     enum Type {
        ///       TYPE_UNSPECIFIED = 0;
        ///       HOME = 1;
        ///       WORK = 2;
        ///     }
        ///
        ///     optional string email = 1;
        ///     repeated EmailType type = 2;
        ///   }
        ///
        ///   string full_name = 1;
        ///   repeated EmailAddress email_addresses = 2;
        /// }
        /// ```
        /// In this example, in proto `field` could take one of the following values:
        /// In JSON, the same values are represented as:
        #[serde(skip_serializing_if = "String::is_empty")]
        pub field: String,

        /// A description of why the request element is bad.
        #[serde(skip_serializing_if = "String::is_empty")]
        pub description: String,

        /// The reason of the field-level error. This is a constant value that
        /// identifies the proximate cause of the field-level error. It should
        /// uniquely identify the type of the FieldViolation within the scope of the
        /// google.rpc.ErrorInfo.domain. This should be at most 63
        /// characters and match a regular expression of `[A-Z][A-Z0-9_]+[A-Z0-9]`,
        /// which represents UPPER_SNAKE_CASE.
        #[serde(skip_serializing_if = "String::is_empty")]
        pub reason: String,

        /// Provides a localized error message for field-level errors that is safe to
        /// return to the API consumer.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub localized_message: Option<crate::model::LocalizedMessage>,
    }

    impl FieldViolation {
        /// Sets the value of `field`.
        pub fn set_field<T: Into<String>>(mut self, v: T) -> Self {
            self.field = v.into();
            self
        }

        /// Sets the value of `description`.
        pub fn set_description<T: Into<String>>(mut self, v: T) -> Self {
            self.description = v.into();
            self
        }

        /// Sets the value of `reason`.
        pub fn set_reason<T: Into<String>>(mut self, v: T) -> Self {
            self.reason = v.into();
            self
        }

        /// Sets the value of `localized_message`.
        pub fn set_localized_message<T: Into<Option<crate::model::LocalizedMessage>>>(
            mut self,
            v: T,
        ) -> Self {
            self.localized_message = v.into();
            self
        }
    }
}

/// Contains metadata about the request that clients can attach when filing a bug
/// or providing other forms of feedback.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct RequestInfo {
    /// An opaque string that should only be interpreted by the service generating
    /// it. For example, it can be used to identify requests in the service's logs.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub request_id: String,

    /// Any data that was used to serve this request. For example, an encrypted
    /// stack trace that can be sent back to the service provider for debugging.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub serving_data: String,
}

impl RequestInfo {
    /// Sets the value of `request_id`.
    pub fn set_request_id<T: Into<String>>(mut self, v: T) -> Self {
        self.request_id = v.into();
        self
    }

    /// Sets the value of `serving_data`.
    pub fn set_serving_data<T: Into<String>>(mut self, v: T) -> Self {
        self.serving_data = v.into();
        self
    }
}

/// Describes the resource that is being accessed.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ResourceInfo {
    /// A name for the type of resource being accessed, e.g. "sql table",
    /// "cloud storage bucket", "file", "Google calendar"; or the type URL
    /// of the resource: e.g. "type.googleapis.com/google.pubsub.v1.Topic".
    #[serde(skip_serializing_if = "String::is_empty")]
    pub resource_type: String,

    /// The name of the resource being accessed.  For example, a shared calendar
    /// name: "example.com_4fghdhgsrgh@group.calendar.google.com", if the current
    /// error is
    /// [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED].
    #[serde(skip_serializing_if = "String::is_empty")]
    pub resource_name: String,

    /// The owner of the resource (optional).
    /// For example, "user:<owner email>" or "project:<Google developer project
    /// id>".
    #[serde(skip_serializing_if = "String::is_empty")]
    pub owner: String,

    /// Describes what error is encountered when accessing this resource.
    /// For example, updating a cloud project may require the `writer` permission
    /// on the developer console project.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
}

impl ResourceInfo {
    /// Sets the value of `resource_type`.
    pub fn set_resource_type<T: Into<String>>(mut self, v: T) -> Self {
        self.resource_type = v.into();
        self
    }

    /// Sets the value of `resource_name`.
    pub fn set_resource_name<T: Into<String>>(mut self, v: T) -> Self {
        self.resource_name = v.into();
        self
    }

    /// Sets the value of `owner`.
    pub fn set_owner<T: Into<String>>(mut self, v: T) -> Self {
        self.owner = v.into();
        self
    }

    /// Sets the value of `description`.
    pub fn set_description<T: Into<String>>(mut self, v: T) -> Self {
        self.description = v.into();
        self
    }
}

/// Provides links to documentation or for performing an out of band action.
/// For example, if a quota check failed with an error indicating the calling
/// project hasn't enabled the accessed service, this can contain a URL pointing
/// directly to the right place in the developer console to flip the bit.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Help {
    /// URL(s) pointing to additional information on handling the current error.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<crate::model::help::Link>,
}

impl Help {
    /// Sets the value of `links`.
    pub fn set_links<T: Into<Vec<crate::model::help::Link>>>(mut self, v: T) -> Self {
        self.links = v.into();
        self
    }
}

/// Defines additional types related to Help
pub mod help {

    /// Describes a URL link.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct Link {
        /// Describes what the link offers.
        #[serde(skip_serializing_if = "String::is_empty")]
        pub description: String,

        /// The URL of the link.
        #[serde(skip_serializing_if = "String::is_empty")]
        pub url: String,
    }

    impl Link {
        /// Sets the value of `description`.
        pub fn set_description<T: Into<String>>(mut self, v: T) -> Self {
            self.description = v.into();
            self
        }

        /// Sets the value of `url`.
        pub fn set_url<T: Into<String>>(mut self, v: T) -> Self {
            self.url = v.into();
            self
        }
    }
}

/// Provides a localized error message that is safe to return to the user
/// which can be attached to an RPC error.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct LocalizedMessage {
    /// The locale used following the specification defined at
    /// https://www.rfc-editor.org/rfc/bcp/bcp47.txt.
    /// Examples are: "en-US", "fr-CH", "es-MX"
    #[serde(skip_serializing_if = "String::is_empty")]
    pub locale: String,

    /// The localized error message in the above locale.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}

impl LocalizedMessage {
    /// Sets the value of `locale`.
    pub fn set_locale<T: Into<String>>(mut self, v: T) -> Self {
        self.locale = v.into();
        self
    }

    /// Sets the value of `message`.
    pub fn set_message<T: Into<String>>(mut self, v: T) -> Self {
        self.message = v.into();
        self
    }
}

/// Represents an HTTP request.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct HttpRequest {
    /// The HTTP request method.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub method: String,

    /// The HTTP request URI.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub uri: String,

    /// The HTTP request headers. The ordering of the headers is significant.
    /// Multiple headers with the same key may present for the request.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<crate::model::HttpHeader>,

    /// The HTTP request body. If the body is not expected, it should be empty.
    #[serde(skip_serializing_if = "bytes::Bytes::is_empty")]
    #[serde_as(as = "serde_with::base64::Base64")]
    pub body: bytes::Bytes,
}

impl HttpRequest {
    /// Sets the value of `method`.
    pub fn set_method<T: Into<String>>(mut self, v: T) -> Self {
        self.method = v.into();
        self
    }

    /// Sets the value of `uri`.
    pub fn set_uri<T: Into<String>>(mut self, v: T) -> Self {
        self.uri = v.into();
        self
    }

    /// Sets the value of `headers`.
    pub fn set_headers<T: Into<Vec<crate::model::HttpHeader>>>(mut self, v: T) -> Self {
        self.headers = v.into();
        self
    }

    /// Sets the value of `body`.
    pub fn set_body<T: Into<bytes::Bytes>>(mut self, v: T) -> Self {
        self.body = v.into();
        self
    }
}

/// Represents an HTTP response.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct HttpResponse {
    /// The HTTP status code, such as 200 or 404.
    pub status: i32,

    /// The HTTP reason phrase, such as "OK" or "Not Found".
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// The HTTP response headers. The ordering of the headers is significant.
    /// Multiple headers with the same key may present for the response.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<crate::model::HttpHeader>,

    /// The HTTP response body. If the body is not expected, it should be empty.
    #[serde(skip_serializing_if = "bytes::Bytes::is_empty")]
    #[serde_as(as = "serde_with::base64::Base64")]
    pub body: bytes::Bytes,
}

impl HttpResponse {
    /// Sets the value of `status`.
    pub fn set_status<T: Into<i32>>(mut self, v: T) -> Self {
        self.status = v.into();
        self
    }

    /// Sets the value of `reason`.
    pub fn set_reason<T: Into<String>>(mut self, v: T) -> Self {
        self.reason = v.into();
        self
    }

    /// Sets the value of `headers`.
    pub fn set_headers<T: Into<Vec<crate::model::HttpHeader>>>(mut self, v: T) -> Self {
        self.headers = v.into();
        self
    }

    /// Sets the value of `body`.
    pub fn set_body<T: Into<bytes::Bytes>>(mut self, v: T) -> Self {
        self.body = v.into();
        self
    }
}

/// Represents an HTTP header.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct HttpHeader {
    /// The HTTP header key. It is case insensitive.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub key: String,

    /// The HTTP header value.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value: String,
}

impl HttpHeader {
    /// Sets the value of `key`.
    pub fn set_key<T: Into<String>>(mut self, v: T) -> Self {
        self.key = v.into();
        self
    }

    /// Sets the value of `value`.
    pub fn set_value<T: Into<String>>(mut self, v: T) -> Self {
        self.value = v.into();
        self
    }
}

/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](https://github.com/grpc). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](https://cloud.google.com/apis/design/errors).
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Status {
    /// The status code, which should be an enum value of
    /// [google.rpc.Code][google.rpc.Code].
    pub code: i32,

    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// [google.rpc.Status.details][google.rpc.Status.details] field, or localized
    /// by the client.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,

    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<wkt::Any>,
}

impl Status {
    /// Sets the value of `code`.
    pub fn set_code<T: Into<i32>>(mut self, v: T) -> Self {
        self.code = v.into();
        self
    }

    /// Sets the value of `message`.
    pub fn set_message<T: Into<String>>(mut self, v: T) -> Self {
        self.message = v.into();
        self
    }

    /// Sets the value of `details`.
    pub fn set_details<T: Into<Vec<wkt::Any>>>(mut self, v: T) -> Self {
        self.details = v.into();
        self
    }
}

/// The canonical error codes for gRPC APIs.
/// Sometimes multiple error codes may apply.  Services should return
/// the most specific error code that applies.  For example, prefer
/// `OUT_OF_RANGE` over `FAILED_PRECONDITION` if both codes apply.
/// Similarly prefer `NOT_FOUND` or `ALREADY_EXISTS` over `FAILED_PRECONDITION`.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Code(String);

impl Code {
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

/// Useful constants to work with [Code](Code)
pub mod code {

    /// Not an error; returned on success.
    /// HTTP Mapping: 200 OK
    pub const OK: &str = "OK";

    /// The operation was cancelled, typically by the caller.
    /// HTTP Mapping: 499 Client Closed Request
    pub const CANCELLED: &str = "CANCELLED";

    /// Unknown error.  For example, this error may be returned when
    /// a `Status` value received from another address space belongs to
    /// an error space that is not known in this address space.  Also
    /// errors raised by APIs that do not return enough error information
    /// may be converted to this error.
    /// HTTP Mapping: 500 Internal Server Error
    pub const UNKNOWN: &str = "UNKNOWN";

    /// The client specified an invalid argument.  Note that this differs
    /// from `FAILED_PRECONDITION`.  `INVALID_ARGUMENT` indicates arguments
    /// that are problematic regardless of the state of the system
    /// (e.g., a malformed file name).
    /// HTTP Mapping: 400 Bad Request
    pub const INVALID_ARGUMENT: &str = "INVALID_ARGUMENT";

    /// The deadline expired before the operation could complete. For operations
    /// that change the state of the system, this error may be returned
    /// even if the operation has completed successfully.  For example, a
    /// successful response from a server could have been delayed long
    /// enough for the deadline to expire.
    /// HTTP Mapping: 504 Gateway Timeout
    pub const DEADLINE_EXCEEDED: &str = "DEADLINE_EXCEEDED";

    /// Some requested entity (e.g., file or directory) was not found.
    /// Note to server developers: if a request is denied for an entire class
    /// of users, such as gradual feature rollout or undocumented allowlist,
    /// `NOT_FOUND` may be used. If a request is denied for some users within
    /// a class of users, such as user-based access control, `PERMISSION_DENIED`
    /// must be used.
    /// HTTP Mapping: 404 Not Found
    pub const NOT_FOUND: &str = "NOT_FOUND";

    /// The entity that a client attempted to create (e.g., file or directory)
    /// already exists.
    /// HTTP Mapping: 409 Conflict
    pub const ALREADY_EXISTS: &str = "ALREADY_EXISTS";

    /// The caller does not have permission to execute the specified
    /// operation. `PERMISSION_DENIED` must not be used for rejections
    /// caused by exhausting some resource (use `RESOURCE_EXHAUSTED`
    /// instead for those errors). `PERMISSION_DENIED` must not be
    /// used if the caller can not be identified (use `UNAUTHENTICATED`
    /// instead for those errors). This error code does not imply the
    /// request is valid or the requested entity exists or satisfies
    /// other pre-conditions.
    /// HTTP Mapping: 403 Forbidden
    pub const PERMISSION_DENIED: &str = "PERMISSION_DENIED";

    /// The request does not have valid authentication credentials for the
    /// operation.
    /// HTTP Mapping: 401 Unauthorized
    pub const UNAUTHENTICATED: &str = "UNAUTHENTICATED";

    /// Some resource has been exhausted, perhaps a per-user quota, or
    /// perhaps the entire file system is out of space.
    /// HTTP Mapping: 429 Too Many Requests
    pub const RESOURCE_EXHAUSTED: &str = "RESOURCE_EXHAUSTED";

    /// The operation was rejected because the system is not in a state
    /// required for the operation's execution.  For example, the directory
    /// to be deleted is non-empty, an rmdir operation is applied to
    /// a non-directory, etc.
    /// Service implementors can use the following guidelines to decide
    /// between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`:
    /// (a) Use `UNAVAILABLE` if the client can retry just the failing call.
    /// (b) Use `ABORTED` if the client should retry at a higher level. For
    /// example, when a client-specified test-and-set fails, indicating the
    /// client should restart a read-modify-write sequence.
    /// (c) Use `FAILED_PRECONDITION` if the client should not retry until
    /// the system state has been explicitly fixed. For example, if an "rmdir"
    /// fails because the directory is non-empty, `FAILED_PRECONDITION`
    /// should be returned since the client should not retry unless
    /// the files are deleted from the directory.
    /// HTTP Mapping: 400 Bad Request
    pub const FAILED_PRECONDITION: &str = "FAILED_PRECONDITION";

    /// The operation was aborted, typically due to a concurrency issue such as
    /// a sequencer check failure or transaction abort.
    /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
    /// `ABORTED`, and `UNAVAILABLE`.
    /// HTTP Mapping: 409 Conflict
    pub const ABORTED: &str = "ABORTED";

    /// The operation was attempted past the valid range.  E.g., seeking or
    /// reading past end-of-file.
    /// Unlike `INVALID_ARGUMENT`, this error indicates a problem that may
    /// be fixed if the system state changes. For example, a 32-bit file
    /// system will generate `INVALID_ARGUMENT` if asked to read at an
    /// offset that is not in the range [0,2^32-1], but it will generate
    /// `OUT_OF_RANGE` if asked to read from an offset past the current
    /// file size.
    /// There is a fair bit of overlap between `FAILED_PRECONDITION` and
    /// `OUT_OF_RANGE`.  We recommend using `OUT_OF_RANGE` (the more specific
    /// error) when it applies so that callers who are iterating through
    /// a space can easily look for an `OUT_OF_RANGE` error to detect when
    /// they are done.
    /// HTTP Mapping: 400 Bad Request
    pub const OUT_OF_RANGE: &str = "OUT_OF_RANGE";

    /// The operation is not implemented or is not supported/enabled in this
    /// service.
    /// HTTP Mapping: 501 Not Implemented
    pub const UNIMPLEMENTED: &str = "UNIMPLEMENTED";

    /// Internal errors.  This means that some invariants expected by the
    /// underlying system have been broken.  This error code is reserved
    /// for serious errors.
    /// HTTP Mapping: 500 Internal Server Error
    pub const INTERNAL: &str = "INTERNAL";

    /// The service is currently unavailable.  This is most likely a
    /// transient condition, which can be corrected by retrying with
    /// a backoff. Note that it is not always safe to retry
    /// non-idempotent operations.
    /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
    /// `ABORTED`, and `UNAVAILABLE`.
    /// HTTP Mapping: 503 Service Unavailable
    pub const UNAVAILABLE: &str = "UNAVAILABLE";

    /// Unrecoverable data loss or corruption.
    /// HTTP Mapping: 500 Internal Server Error
    pub const DATA_LOSS: &str = "DATA_LOSS";
}
