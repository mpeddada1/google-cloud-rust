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
use crate as wkt;

/// Api is a light-weight descriptor for an API Interface.
///
/// Interfaces are also described as "protocol buffer services" in some contexts,
/// such as by the "service" keyword in a .proto file, but they are different
/// from API Services, which represent a concrete implementation of an interface
/// as opposed to simply a description of methods and bindings. They are also
/// sometimes simply referred to as "APIs" in other contexts, such as the name of
/// this message itself. See <https://cloud.google.com/apis/design/glossary> for
/// detailed terminology.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Api {
    /// The fully qualified name of this interface, including package name
    /// followed by the interface's simple name.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The methods of this interface, in unspecified order.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub methods: std::vec::Vec<crate::Method>,

    /// Any metadata attached to the interface.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub options: std::vec::Vec<crate::Option>,

    /// A version string for this interface. If specified, must have the form
    /// `major-version.minor-version`, as in `1.10`. If the minor version is
    /// omitted, it defaults to zero. If the entire version field is empty, the
    /// major version is derived from the package name, as outlined below. If the
    /// field is not empty, the version in the package name will be verified to be
    /// consistent with what is provided here.
    ///
    /// The versioning schema uses [semantic
    /// versioning](http://semver.org) where the major version number
    /// indicates a breaking change and the minor version an additive,
    /// non-breaking change. Both version numbers are signals to users
    /// what to expect from different versions, and should be carefully
    /// chosen based on the product plan.
    ///
    /// The major version is also reflected in the package name of the
    /// interface, which must end in `v<major-version>`, as in
    /// `google.feature.v1`. For major versions 0 and 1, the suffix can
    /// be omitted. Zero major versions must only be used for
    /// experimental, non-GA interfaces.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub version: std::string::String,

    /// Source context for the protocol buffer service represented by this
    /// message.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub source_context: std::option::Option<crate::SourceContext>,

    /// Included interfaces. See [Mixin][].
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub mixins: std::vec::Vec<crate::Mixin>,

    /// The source syntax of the service.
    pub syntax: crate::Syntax,
}

impl Api {
    /// Sets the value of `name`.
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `methods`.
    pub fn set_methods<T: std::convert::Into<std::vec::Vec<crate::Method>>>(
        mut self,
        v: T,
    ) -> Self {
        self.methods = v.into();
        self
    }

    /// Sets the value of `options`.
    pub fn set_options<T: std::convert::Into<std::vec::Vec<crate::Option>>>(
        mut self,
        v: T,
    ) -> Self {
        self.options = v.into();
        self
    }

    /// Sets the value of `version`.
    pub fn set_version<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.version = v.into();
        self
    }

    /// Sets the value of `source_context`.
    pub fn set_source_context<T: std::convert::Into<std::option::Option<crate::SourceContext>>>(
        mut self,
        v: T,
    ) -> Self {
        self.source_context = v.into();
        self
    }

    /// Sets the value of `mixins`.
    pub fn set_mixins<T: std::convert::Into<std::vec::Vec<crate::Mixin>>>(mut self, v: T) -> Self {
        self.mixins = v.into();
        self
    }

    /// Sets the value of `syntax`.
    pub fn set_syntax<T: std::convert::Into<crate::Syntax>>(mut self, v: T) -> Self {
        self.syntax = v.into();
        self
    }
}

impl wkt::message::Message for Api {
    fn typename() -> &'static str {
        "type.googleapis.com/google.protobuf.Api"
    }
}

/// Method represents a method of an API interface.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Method {
    /// The simple name of this method.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// A URL of the input message type.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub request_type_url: std::string::String,

    /// If true, the request is streamed.
    pub request_streaming: bool,

    /// The URL of the output message type.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub response_type_url: std::string::String,

    /// If true, the response is streamed.
    pub response_streaming: bool,

    /// Any metadata attached to the method.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub options: std::vec::Vec<crate::Option>,

    /// The source syntax of this method.
    pub syntax: crate::Syntax,
}

impl Method {
    /// Sets the value of `name`.
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `request_type_url`.
    pub fn set_request_type_url<T: std::convert::Into<std::string::String>>(
        mut self,
        v: T,
    ) -> Self {
        self.request_type_url = v.into();
        self
    }

    /// Sets the value of `request_streaming`.
    pub fn set_request_streaming<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.request_streaming = v.into();
        self
    }

    /// Sets the value of `response_type_url`.
    pub fn set_response_type_url<T: std::convert::Into<std::string::String>>(
        mut self,
        v: T,
    ) -> Self {
        self.response_type_url = v.into();
        self
    }

    /// Sets the value of `response_streaming`.
    pub fn set_response_streaming<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.response_streaming = v.into();
        self
    }

    /// Sets the value of `options`.
    pub fn set_options<T: std::convert::Into<std::vec::Vec<crate::Option>>>(
        mut self,
        v: T,
    ) -> Self {
        self.options = v.into();
        self
    }

    /// Sets the value of `syntax`.
    pub fn set_syntax<T: std::convert::Into<crate::Syntax>>(mut self, v: T) -> Self {
        self.syntax = v.into();
        self
    }
}

impl wkt::message::Message for Method {
    fn typename() -> &'static str {
        "type.googleapis.com/google.protobuf.Method"
    }
}

/// Declares an API Interface to be included in this interface. The including
/// interface must redeclare all the methods from the included interface, but
/// documentation and options are inherited as follows:
///
/// - If after comment and whitespace stripping, the documentation
///   string of the redeclared method is empty, it will be inherited
///   from the original method.
///
/// - Each annotation belonging to the service config (http,
///   visibility) which is not set in the redeclared method will be
///   inherited.
///
/// - If an http annotation is inherited, the path pattern will be
///   modified as follows. Any version prefix will be replaced by the
///   version of the including interface plus the [root][Mixin.root] path if
///   specified.
///
///
/// Example of a simple mixin:
///
/// ```norust
/// package google.acl.v1;
/// service AccessControl {
///   // Get the underlying ACL object.
///   rpc GetAcl(GetAclRequest) returns (Acl) {
///     option (google.api.http).get = "/v1/{resource=**}:getAcl";
///   }
/// }
///
/// package google.storage.v2;
/// service Storage {
///   rpc GetAcl(GetAclRequest) returns (Acl);
///
///   // Get a data record.
///   rpc GetData(GetDataRequest) returns (Data) {
///     option (google.api.http).get = "/v2/{resource=**}";
///   }
/// }
/// ```
///
/// Example of a mixin configuration:
///
/// ```norust
/// apis:
/// - name: google.storage.v2.Storage
///   mixins:
///   - name: google.acl.v1.AccessControl
/// ```
///
/// The mixin construct implies that all methods in `AccessControl` are
/// also declared with same name and request/response types in
/// `Storage`. A documentation generator or annotation processor will
/// see the effective `Storage.GetAcl` method after inheriting
/// documentation and annotations as follows:
///
/// ```norust
/// service Storage {
///   // Get the underlying ACL object.
///   rpc GetAcl(GetAclRequest) returns (Acl) {
///     option (google.api.http).get = "/v2/{resource=**}:getAcl";
///   }
///   ...
/// }
/// ```
///
/// Note how the version in the path pattern changed from `v1` to `v2`.
///
/// If the `root` field in the mixin is specified, it should be a
/// relative path under which inherited HTTP paths are placed. Example:
///
/// ```norust
/// apis:
/// - name: google.storage.v2.Storage
///   mixins:
///   - name: google.acl.v1.AccessControl
///     root: acls
/// ```
///
/// This implies the following inherited HTTP annotation:
///
/// ```norust
/// service Storage {
///   // Get the underlying ACL object.
///   rpc GetAcl(GetAclRequest) returns (Acl) {
///     option (google.api.http).get = "/v2/acls/{resource=**}:getAcl";
///   }
///   ...
/// }
/// ```
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Mixin {
    /// The fully qualified name of the interface which is included.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// If non-empty specifies a path under which inherited HTTP paths
    /// are rooted.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub root: std::string::String,
}

impl Mixin {
    /// Sets the value of `name`.
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `root`.
    pub fn set_root<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.root = v.into();
        self
    }
}

impl wkt::message::Message for Mixin {
    fn typename() -> &'static str {
        "type.googleapis.com/google.protobuf.Mixin"
    }
}

/// `SourceContext` represents information about the source of a
/// protobuf element, like the file in which it is defined.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct SourceContext {
    /// The path-qualified name of the .proto file that contained the associated
    /// protobuf element.  For example: `"google/protobuf/source_context.proto"`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub file_name: std::string::String,
}

impl SourceContext {
    /// Sets the value of `file_name`.
    pub fn set_file_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.file_name = v.into();
        self
    }
}

impl wkt::message::Message for SourceContext {
    fn typename() -> &'static str {
        "type.googleapis.com/google.protobuf.SourceContext"
    }
}

/// A protocol buffer message type.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Type {
    /// The fully qualified message name.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The list of fields.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub fields: std::vec::Vec<crate::Field>,

    /// The list of types appearing in `oneof` definitions in this type.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub oneofs: std::vec::Vec<std::string::String>,

    /// The protocol buffer options.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub options: std::vec::Vec<crate::Option>,

    /// The source context.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub source_context: std::option::Option<crate::SourceContext>,

    /// The source syntax.
    pub syntax: crate::Syntax,

    /// The source edition string, only valid when syntax is SYNTAX_EDITIONS.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub edition: std::string::String,
}

impl Type {
    /// Sets the value of `name`.
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `fields`.
    pub fn set_fields<T: std::convert::Into<std::vec::Vec<crate::Field>>>(mut self, v: T) -> Self {
        self.fields = v.into();
        self
    }

    /// Sets the value of `oneofs`.
    pub fn set_oneofs<T: std::convert::Into<std::vec::Vec<std::string::String>>>(
        mut self,
        v: T,
    ) -> Self {
        self.oneofs = v.into();
        self
    }

    /// Sets the value of `options`.
    pub fn set_options<T: std::convert::Into<std::vec::Vec<crate::Option>>>(
        mut self,
        v: T,
    ) -> Self {
        self.options = v.into();
        self
    }

    /// Sets the value of `source_context`.
    pub fn set_source_context<T: std::convert::Into<std::option::Option<crate::SourceContext>>>(
        mut self,
        v: T,
    ) -> Self {
        self.source_context = v.into();
        self
    }

    /// Sets the value of `syntax`.
    pub fn set_syntax<T: std::convert::Into<crate::Syntax>>(mut self, v: T) -> Self {
        self.syntax = v.into();
        self
    }

    /// Sets the value of `edition`.
    pub fn set_edition<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.edition = v.into();
        self
    }
}

impl wkt::message::Message for Type {
    fn typename() -> &'static str {
        "type.googleapis.com/google.protobuf.Type"
    }
}

/// A single field of a message type.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Field {
    /// The field type.
    pub kind: crate::field::Kind,

    /// The field cardinality.
    pub cardinality: crate::field::Cardinality,

    /// The field number.
    pub number: i32,

    /// The field name.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The field type URL, without the scheme, for message or enumeration
    /// types. Example: `"type.googleapis.com/google.protobuf.Timestamp"`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub type_url: std::string::String,

    /// The index of the field type in `Type.oneofs`, for message or enumeration
    /// types. The first type has index 1; zero means the type is not in the list.
    pub oneof_index: i32,

    /// Whether to use alternative packed wire representation.
    pub packed: bool,

    /// The protocol buffer options.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub options: std::vec::Vec<crate::Option>,

    /// The field JSON name.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub json_name: std::string::String,

    /// The string value of the default value of this field. Proto2 syntax only.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub default_value: std::string::String,
}

impl Field {
    /// Sets the value of `kind`.
    pub fn set_kind<T: std::convert::Into<crate::field::Kind>>(mut self, v: T) -> Self {
        self.kind = v.into();
        self
    }

    /// Sets the value of `cardinality`.
    pub fn set_cardinality<T: std::convert::Into<crate::field::Cardinality>>(
        mut self,
        v: T,
    ) -> Self {
        self.cardinality = v.into();
        self
    }

    /// Sets the value of `number`.
    pub fn set_number<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.number = v.into();
        self
    }

    /// Sets the value of `name`.
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `type_url`.
    pub fn set_type_url<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.type_url = v.into();
        self
    }

    /// Sets the value of `oneof_index`.
    pub fn set_oneof_index<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.oneof_index = v.into();
        self
    }

    /// Sets the value of `packed`.
    pub fn set_packed<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.packed = v.into();
        self
    }

    /// Sets the value of `options`.
    pub fn set_options<T: std::convert::Into<std::vec::Vec<crate::Option>>>(
        mut self,
        v: T,
    ) -> Self {
        self.options = v.into();
        self
    }

    /// Sets the value of `json_name`.
    pub fn set_json_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.json_name = v.into();
        self
    }

    /// Sets the value of `default_value`.
    pub fn set_default_value<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.default_value = v.into();
        self
    }
}

impl wkt::message::Message for Field {
    fn typename() -> &'static str {
        "type.googleapis.com/google.protobuf.Field"
    }
}

/// Defines additional types related to Field
pub mod field {
    #[allow(unused_imports)]
    use super::*;

    /// Basic field types.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Kind(std::string::String);

    impl Kind {
        /// Sets the enum value.
        pub fn set_value<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
            self.0 = v.into();
            self
        }

        /// Gets the enum value.
        pub fn value(&self) -> &str {
            &self.0
        }
    }

    /// Useful constants to work with [Kind](Kind)
    pub mod kind {

        /// Field type unknown.
        pub const TYPE_UNKNOWN: &str = "TYPE_UNKNOWN";

        /// Field type double.
        pub const TYPE_DOUBLE: &str = "TYPE_DOUBLE";

        /// Field type float.
        pub const TYPE_FLOAT: &str = "TYPE_FLOAT";

        /// Field type int64.
        pub const TYPE_INT64: &str = "TYPE_INT64";

        /// Field type uint64.
        pub const TYPE_UINT64: &str = "TYPE_UINT64";

        /// Field type int32.
        pub const TYPE_INT32: &str = "TYPE_INT32";

        /// Field type fixed64.
        pub const TYPE_FIXED64: &str = "TYPE_FIXED64";

        /// Field type fixed32.
        pub const TYPE_FIXED32: &str = "TYPE_FIXED32";

        /// Field type bool.
        pub const TYPE_BOOL: &str = "TYPE_BOOL";

        /// Field type string.
        pub const TYPE_STRING: &str = "TYPE_STRING";

        /// Field type group. Proto2 syntax only, and deprecated.
        pub const TYPE_GROUP: &str = "TYPE_GROUP";

        /// Field type message.
        pub const TYPE_MESSAGE: &str = "TYPE_MESSAGE";

        /// Field type bytes.
        pub const TYPE_BYTES: &str = "TYPE_BYTES";

        /// Field type uint32.
        pub const TYPE_UINT32: &str = "TYPE_UINT32";

        /// Field type enum.
        pub const TYPE_ENUM: &str = "TYPE_ENUM";

        /// Field type sfixed32.
        pub const TYPE_SFIXED32: &str = "TYPE_SFIXED32";

        /// Field type sfixed64.
        pub const TYPE_SFIXED64: &str = "TYPE_SFIXED64";

        /// Field type sint32.
        pub const TYPE_SINT32: &str = "TYPE_SINT32";

        /// Field type sint64.
        pub const TYPE_SINT64: &str = "TYPE_SINT64";
    }

    /// Whether a field is optional, required, or repeated.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Cardinality(std::string::String);

    impl Cardinality {
        /// Sets the enum value.
        pub fn set_value<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
            self.0 = v.into();
            self
        }

        /// Gets the enum value.
        pub fn value(&self) -> &str {
            &self.0
        }
    }

    /// Useful constants to work with [Cardinality](Cardinality)
    pub mod cardinality {

        /// For fields with unknown cardinality.
        pub const CARDINALITY_UNKNOWN: &str = "CARDINALITY_UNKNOWN";

        /// For optional fields.
        pub const CARDINALITY_OPTIONAL: &str = "CARDINALITY_OPTIONAL";

        /// For required fields. Proto2 syntax only.
        pub const CARDINALITY_REQUIRED: &str = "CARDINALITY_REQUIRED";

        /// For repeated fields.
        pub const CARDINALITY_REPEATED: &str = "CARDINALITY_REPEATED";
    }
}

/// Enum type definition.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Enum {
    /// Enum type name.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Enum value definitions.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub enumvalue: std::vec::Vec<crate::EnumValue>,

    /// Protocol buffer options.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub options: std::vec::Vec<crate::Option>,

    /// The source context.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub source_context: std::option::Option<crate::SourceContext>,

    /// The source syntax.
    pub syntax: crate::Syntax,

    /// The source edition string, only valid when syntax is SYNTAX_EDITIONS.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub edition: std::string::String,
}

impl Enum {
    /// Sets the value of `name`.
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `enumvalue`.
    pub fn set_enumvalue<T: std::convert::Into<std::vec::Vec<crate::EnumValue>>>(
        mut self,
        v: T,
    ) -> Self {
        self.enumvalue = v.into();
        self
    }

    /// Sets the value of `options`.
    pub fn set_options<T: std::convert::Into<std::vec::Vec<crate::Option>>>(
        mut self,
        v: T,
    ) -> Self {
        self.options = v.into();
        self
    }

    /// Sets the value of `source_context`.
    pub fn set_source_context<T: std::convert::Into<std::option::Option<crate::SourceContext>>>(
        mut self,
        v: T,
    ) -> Self {
        self.source_context = v.into();
        self
    }

    /// Sets the value of `syntax`.
    pub fn set_syntax<T: std::convert::Into<crate::Syntax>>(mut self, v: T) -> Self {
        self.syntax = v.into();
        self
    }

    /// Sets the value of `edition`.
    pub fn set_edition<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.edition = v.into();
        self
    }
}

impl wkt::message::Message for Enum {
    fn typename() -> &'static str {
        "type.googleapis.com/google.protobuf.Enum"
    }
}

/// Enum value definition.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct EnumValue {
    /// Enum value name.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Enum value number.
    pub number: i32,

    /// Protocol buffer options.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub options: std::vec::Vec<crate::Option>,
}

impl EnumValue {
    /// Sets the value of `name`.
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `number`.
    pub fn set_number<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.number = v.into();
        self
    }

    /// Sets the value of `options`.
    pub fn set_options<T: std::convert::Into<std::vec::Vec<crate::Option>>>(
        mut self,
        v: T,
    ) -> Self {
        self.options = v.into();
        self
    }
}

impl wkt::message::Message for EnumValue {
    fn typename() -> &'static str {
        "type.googleapis.com/google.protobuf.EnumValue"
    }
}

/// A protocol buffer option, which can be attached to a message, field,
/// enumeration, etc.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Option {
    /// The option's name. For protobuf built-in options (options defined in
    /// descriptor.proto), this is the short name. For example, `"map_entry"`.
    /// For custom options, it should be the fully-qualified name. For example,
    /// `"google.api.http"`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The option's value packed in an Any message. If the value is a primitive,
    /// the corresponding wrapper type defined in google/protobuf/wrappers.proto
    /// should be used. If the value is an enum, it should be stored as an int32
    /// value using the google.protobuf.Int32Value type.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub value: std::option::Option<crate::Any>,
}

impl Option {
    /// Sets the value of `name`.
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `value`.
    pub fn set_value<T: std::convert::Into<std::option::Option<crate::Any>>>(
        mut self,
        v: T,
    ) -> Self {
        self.value = v.into();
        self
    }
}

impl wkt::message::Message for Option {
    fn typename() -> &'static str {
        "type.googleapis.com/google.protobuf.Option"
    }
}

/// The syntax in which a protocol buffer element is defined.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Syntax(std::string::String);

impl Syntax {
    /// Sets the enum value.
    pub fn set_value<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.0 = v.into();
        self
    }

    /// Gets the enum value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Useful constants to work with [Syntax](Syntax)
pub mod syntax {

    /// Syntax `proto2`.
    pub const SYNTAX_PROTO2: &str = "SYNTAX_PROTO2";

    /// Syntax `proto3`.
    pub const SYNTAX_PROTO3: &str = "SYNTAX_PROTO3";

    /// Syntax `editions`.
    pub const SYNTAX_EDITIONS: &str = "SYNTAX_EDITIONS";
}
