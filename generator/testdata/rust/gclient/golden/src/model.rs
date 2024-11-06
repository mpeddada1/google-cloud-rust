#![allow(dead_code)]


/// A [Secret][google.cloud.secretmanager.v1.Secret] is a logical secret whose
/// value and versions can be accessed.
/// 
/// A [Secret][google.cloud.secretmanager.v1.Secret] is made up of zero or more
/// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] that represent
/// the secret data.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Secret {

    /// Output only. The resource name of the
    /// [Secret][google.cloud.secretmanager.v1.Secret] in the format
    /// `projects/*/secrets/*`.
    pub name: String,

    /// Optional. Immutable. The replication policy of the secret data attached to
    /// the [Secret][google.cloud.secretmanager.v1.Secret].
    /// 
    /// The replication policy cannot be changed after the Secret has been created.
    pub replication: Option<crate::Replication>,

    /// Output only. The time at which the
    /// [Secret][google.cloud.secretmanager.v1.Secret] was created.
    pub create_time: Option<gax_placeholder::Timestamp>,

    /// The labels assigned to this Secret.
    /// 
    /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
    /// of maximum 128 bytes, and must conform to the following PCRE regular
    /// expression: `[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}`
    /// 
    /// Label values must be between 0 and 63 characters long, have a UTF-8
    /// encoding of maximum 128 bytes, and must conform to the following PCRE
    /// regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}`
    /// 
    /// No more than 64 labels can be assigned to a given resource.
    pub labels: std::collections::HashMap<String,String>,

    /// Optional. A list of up to 10 Pub/Sub topics to which messages are published
    /// when control plane operations are called on the secret or its versions.
    pub topics: Vec<crate::Topic>,

    /// Optional. Etag of the currently stored
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    pub etag: String,

    /// Optional. Rotation policy attached to the
    /// [Secret][google.cloud.secretmanager.v1.Secret]. May be excluded if there is
    /// no rotation policy.
    pub rotation: Option<crate::Rotation>,

    /// Optional. Mapping from version alias to version name.
    /// 
    /// A version alias is a string with a maximum length of 63 characters and can
    /// contain uppercase and lowercase letters, numerals, and the hyphen (`-`)
    /// and underscore ('_') characters. An alias string must start with a
    /// letter and cannot be the string 'latest' or 'NEW'.
    /// No more than 50 aliases can be assigned to a given secret.
    /// 
    /// Version-Alias pairs will be viewable via GetSecret and modifiable via
    /// UpdateSecret. Access by alias is only be supported on
    /// GetSecretVersion and AccessSecretVersion.
    pub version_aliases: std::collections::HashMap<String,i64>,

    /// Optional. Custom metadata about the secret.
    /// 
    /// Annotations are distinct from various forms of labels.
    /// Annotations exist to allow client tools to store their own state
    /// information without requiring a database.
    /// 
    /// Annotation keys must be between 1 and 63 characters long, have a UTF-8
    /// encoding of maximum 128 bytes, begin and end with an alphanumeric character
    /// ([a-z0-9A-Z]), and may have dashes (-), underscores (_), dots (.), and
    /// alphanumerics in between these symbols.
    /// 
    /// The total size of annotation keys and values must be less than 16KiB.
    pub annotations: std::collections::HashMap<String,String>,

    /// Optional. Secret Version TTL after destruction request
    /// 
    /// This is a part of the Delayed secret version destroy feature.
    /// For secret with TTL>0, version destruction doesn't happen immediately
    /// on calling destroy instead the version goes to a disabled state and
    /// destruction happens after the TTL expires.
    pub version_destroy_ttl: Option<gax_placeholder::Duration>,

    /// Optional. The customer-managed encryption configuration of the Regionalised
    /// Secrets. If no configuration is provided, Google-managed default encryption
    /// is used.
    /// 
    /// Updates to the [Secret][google.cloud.secretmanager.v1.Secret] encryption
    /// configuration only apply to
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] added
    /// afterwards. They do not apply retroactively to existing
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
    pub customer_managed_encryption: Option<crate::CustomerManagedEncryption>,

    /// Expiration policy attached to the
    /// [Secret][google.cloud.secretmanager.v1.Secret]. If specified the
    /// [Secret][google.cloud.secretmanager.v1.Secret] and all
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] will be
    /// automatically deleted at expiration. Expired secrets are irreversibly
    /// deleted.
    /// 
    /// Expiration is *not* the recommended way to set time-based permissions. [IAM
    /// Conditions](https://cloud.google.com/secret-manager/docs/access-control#conditions)
    /// is recommended for granting time-based permissions because the operation
    /// can be reversed.
    pub expiration: Option<crate::secret::Expiration>,
}

/// Defines additional types related to Secret
pub mod secret {

    /// Expiration policy attached to the
    /// [Secret][google.cloud.secretmanager.v1.Secret]. If specified the
    /// [Secret][google.cloud.secretmanager.v1.Secret] and all
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] will be
    /// automatically deleted at expiration. Expired secrets are irreversibly
    /// deleted.
    /// 
    /// Expiration is *not* the recommended way to set time-based permissions. [IAM
    /// Conditions](https://cloud.google.com/secret-manager/docs/access-control#conditions)
    /// is recommended for granting time-based permissions because the operation
    /// can be reversed.
    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase", untagged)]
    #[non_exhaustive]
    pub enum Expiration {
        /// Optional. Timestamp in UTC when the
        /// [Secret][google.cloud.secretmanager.v1.Secret] is scheduled to expire.
        /// This is always provided on output, regardless of what was sent on input.
        ExpireTime(gax_placeholder::Timestamp),
        /// Input only. The TTL for the
        /// [Secret][google.cloud.secretmanager.v1.Secret].
        Ttl(gax_placeholder::Duration),
    }
}

/// A secret version resource in the Secret Manager API.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SecretVersion {

    /// Output only. The resource name of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*/versions/*`.
    /// 
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] IDs in a
    /// [Secret][google.cloud.secretmanager.v1.Secret] start at 1 and are
    /// incremented for each subsequent version of the secret.
    pub name: String,

    /// Output only. The time at which the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] was created.
    pub create_time: Option<gax_placeholder::Timestamp>,

    /// Output only. The time this
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] was destroyed.
    /// Only present if [state][google.cloud.secretmanager.v1.SecretVersion.state]
    /// is
    /// [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED].
    pub destroy_time: Option<gax_placeholder::Timestamp>,

    /// Output only. The current state of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub state: crate::secret_version::State,

    /// The replication status of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub replication_status: Option<crate::ReplicationStatus>,

    /// Output only. Etag of the currently stored
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub etag: String,

    /// Output only. True if payload checksum specified in
    /// [SecretPayload][google.cloud.secretmanager.v1.SecretPayload] object has
    /// been received by
    /// [SecretManagerService][google.cloud.secretmanager.v1.SecretManagerService]
    /// on
    /// [SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion].
    pub client_specified_payload_checksum: bool,

    /// Optional. Output only. Scheduled destroy time for secret version.
    /// This is a part of the Delayed secret version destroy feature. For a
    /// Secret with a valid version destroy TTL, when a secert version is
    /// destroyed, the version is moved to disabled state and it is scheduled for
    /// destruction. The version is destroyed only after the
    /// `scheduled_destroy_time`.
    pub scheduled_destroy_time: Option<gax_placeholder::Timestamp>,

    /// Output only. The customer-managed encryption status of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. Only
    /// populated if customer-managed encryption is used and
    /// [Secret][google.cloud.secretmanager.v1.Secret] is a Regionalised Secret.
    pub customer_managed_encryption: Option<crate::CustomerManagedEncryptionStatus>,
}

/// Defines additional types related to SecretVersion
pub mod secret_version {

    /// The state of a
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion], indicating if
    /// it can be accessed.
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct State(String);

    /// Useful constants to work with [State](State)
    pub mod state {

        /// Not specified. This value is unused and invalid.
        pub const STATE_UNSPECIFIED: &str = "STATE_UNSPECIFIED";

        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] may be
        /// accessed.
        pub const ENABLED: &str = "ENABLED";

        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] may not
        /// be accessed, but the secret data is still available and can be placed
        /// back into the
        /// [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED]
        /// state.
        pub const DISABLED: &str = "DISABLED";

        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] is
        /// destroyed and the secret data is no longer stored. A version may not
        /// leave this state once entered.
        pub const DESTROYED: &str = "DESTROYED";
    }
}

/// A policy that defines the replication and encryption configuration of data.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Replication {

    /// The replication policy for this secret.
    pub replication: Option<crate::replication::Replication>,
}

/// Defines additional types related to Replication
pub mod replication {

    /// A replication policy that replicates the
    /// [Secret][google.cloud.secretmanager.v1.Secret] payload without any
    /// restrictions.
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct Automatic {

        /// Optional. The customer-managed encryption configuration of the
        /// [Secret][google.cloud.secretmanager.v1.Secret]. If no configuration is
        /// provided, Google-managed default encryption is used.
        /// 
        /// Updates to the [Secret][google.cloud.secretmanager.v1.Secret] encryption
        /// configuration only apply to
        /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] added
        /// afterwards. They do not apply retroactively to existing
        /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
        pub customer_managed_encryption: Option<crate::CustomerManagedEncryption>,
    }

    /// A replication policy that replicates the
    /// [Secret][google.cloud.secretmanager.v1.Secret] payload into the locations
    /// specified in [Secret.replication.user_managed.replicas][]
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct UserManaged {

        /// Required. The list of Replicas for this
        /// [Secret][google.cloud.secretmanager.v1.Secret].
        /// 
        /// Cannot be empty.
        pub replicas: Vec<crate::replication::user_managed::Replica>,
    }

    /// Defines additional types related to UserManaged
    pub mod user_managed {

        /// Represents a Replica for this
        /// [Secret][google.cloud.secretmanager.v1.Secret].
        #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        #[non_exhaustive]
        pub struct Replica {

            /// The canonical IDs of the location to replicate data.
            /// For example: `"us-east1"`.
            pub location: String,

            /// Optional. The customer-managed encryption configuration of the
            /// [User-Managed Replica][Replication.UserManaged.Replica]. If no
            /// configuration is provided, Google-managed default encryption is used.
            /// 
            /// Updates to the [Secret][google.cloud.secretmanager.v1.Secret]
            /// encryption configuration only apply to
            /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] added
            /// afterwards. They do not apply retroactively to existing
            /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
            pub customer_managed_encryption: Option<crate::CustomerManagedEncryption>,
        }
    }

    /// The replication policy for this secret.
    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase", untagged)]
    #[non_exhaustive]
    pub enum Replication {
        /// The [Secret][google.cloud.secretmanager.v1.Secret] will automatically be
        /// replicated without any restrictions.
        Automatic(crate::replication::Automatic),
        /// The [Secret][google.cloud.secretmanager.v1.Secret] will only be
        /// replicated into the locations specified.
        UserManaged(crate::replication::UserManaged),
    }
}

/// Configuration for encrypting secret payloads using customer-managed
/// encryption keys (CMEK).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CustomerManagedEncryption {

    /// Required. The resource name of the Cloud KMS CryptoKey used to encrypt
    /// secret payloads.
    /// 
    /// For secrets using the
    /// [UserManaged][google.cloud.secretmanager.v1.Replication.UserManaged]
    /// replication policy type, Cloud KMS CryptoKeys must reside in the same
    /// location as the [replica location][Secret.UserManaged.Replica.location].
    /// 
    /// For secrets using the
    /// [Automatic][google.cloud.secretmanager.v1.Replication.Automatic]
    /// replication policy type, Cloud KMS CryptoKeys must reside in `global`.
    /// 
    /// The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    pub kms_key_name: String,
}

/// The replication status of a
/// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ReplicationStatus {

    /// The replication status of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub replication_status: Option<crate::replication_status::ReplicationStatus>,
}

/// Defines additional types related to ReplicationStatus
pub mod replication_status {

    /// The replication status of a
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] using
    /// automatic replication.
    /// 
    /// Only populated if the parent [Secret][google.cloud.secretmanager.v1.Secret]
    /// has an automatic replication policy.
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct AutomaticStatus {

        /// Output only. The customer-managed encryption status of the
        /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. Only
        /// populated if customer-managed encryption is used.
        pub customer_managed_encryption: Option<crate::CustomerManagedEncryptionStatus>,
    }

    /// The replication status of a
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] using
    /// user-managed replication.
    /// 
    /// Only populated if the parent [Secret][google.cloud.secretmanager.v1.Secret]
    /// has a user-managed replication policy.
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct UserManagedStatus {

        /// Output only. The list of replica statuses for the
        /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        pub replicas: Vec<crate::replication_status::user_managed_status::ReplicaStatus>,
    }

    /// Defines additional types related to UserManagedStatus
    pub mod user_managed_status {

        /// Describes the status of a user-managed replica for the
        /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        #[non_exhaustive]
        pub struct ReplicaStatus {

            /// Output only. The canonical ID of the replica location.
            /// For example: `"us-east1"`.
            pub location: String,

            /// Output only. The customer-managed encryption status of the
            /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. Only
            /// populated if customer-managed encryption is used.
            pub customer_managed_encryption: Option<crate::CustomerManagedEncryptionStatus>,
        }
    }

    /// The replication status of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase", untagged)]
    #[non_exhaustive]
    pub enum ReplicationStatus {
        /// Describes the replication status of a
        /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] with
        /// automatic replication.
        /// 
        /// Only populated if the parent
        /// [Secret][google.cloud.secretmanager.v1.Secret] has an automatic
        /// replication policy.
        Automatic(crate::replication_status::AutomaticStatus),
        /// Describes the replication status of a
        /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] with
        /// user-managed replication.
        /// 
        /// Only populated if the parent
        /// [Secret][google.cloud.secretmanager.v1.Secret] has a user-managed
        /// replication policy.
        UserManaged(crate::replication_status::UserManagedStatus),
    }
}

/// Describes the status of customer-managed encryption.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CustomerManagedEncryptionStatus {

    /// Required. The resource name of the Cloud KMS CryptoKeyVersion used to
    /// encrypt the secret payload, in the following format:
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*/versions/*`.
    pub kms_key_version_name: String,
}

/// A Pub/Sub topic which Secret Manager will publish to when control plane
/// events occur on this secret.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Topic {

    /// Required. The resource name of the Pub/Sub topic that will be published to,
    /// in the following format: `projects/*/topics/*`. For publication to succeed,
    /// the Secret Manager service agent must have the `pubsub.topic.publish`
    /// permission on the topic. The Pub/Sub Publisher role
    /// (`roles/pubsub.publisher`) includes this permission.
    pub name: String,
}

/// The rotation time and period for a
/// [Secret][google.cloud.secretmanager.v1.Secret]. At next_rotation_time, Secret
/// Manager will send a Pub/Sub notification to the topics configured on the
/// Secret. [Secret.topics][google.cloud.secretmanager.v1.Secret.topics] must be
/// set to configure rotation.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Rotation {

    /// Optional. Timestamp in UTC at which the
    /// [Secret][google.cloud.secretmanager.v1.Secret] is scheduled to rotate.
    /// Cannot be set to less than 300s (5 min) in the future and at most
    /// 3153600000s (100 years).
    /// 
    /// [next_rotation_time][google.cloud.secretmanager.v1.Rotation.next_rotation_time]
    /// MUST  be set if
    /// [rotation_period][google.cloud.secretmanager.v1.Rotation.rotation_period]
    /// is set.
    pub next_rotation_time: Option<gax_placeholder::Timestamp>,

    /// Input only. The Duration between rotation notifications. Must be in seconds
    /// and at least 3600s (1h) and at most 3153600000s (100 years).
    /// 
    /// If
    /// [rotation_period][google.cloud.secretmanager.v1.Rotation.rotation_period]
    /// is set,
    /// [next_rotation_time][google.cloud.secretmanager.v1.Rotation.next_rotation_time]
    /// must be set.
    /// [next_rotation_time][google.cloud.secretmanager.v1.Rotation.next_rotation_time]
    /// will be advanced by this period when the service automatically sends
    /// rotation notifications.
    pub rotation_period: Option<gax_placeholder::Duration>,
}

/// A secret payload resource in the Secret Manager API. This contains the
/// sensitive secret payload that is associated with a
/// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SecretPayload {

    /// The secret data. Must be no larger than 64KiB.
    pub data: bytes::Bytes,

    /// Optional. If specified,
    /// [SecretManagerService][google.cloud.secretmanager.v1.SecretManagerService]
    /// will verify the integrity of the received
    /// [data][google.cloud.secretmanager.v1.SecretPayload.data] on
    /// [SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion]
    /// calls using the crc32c checksum and store it to include in future
    /// [SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion]
    /// responses. If a checksum is not provided in the
    /// [SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion]
    /// request, the
    /// [SecretManagerService][google.cloud.secretmanager.v1.SecretManagerService]
    /// will generate and store one for you.
    /// 
    /// The CRC32C value is encoded as a Int64 for compatibility, and can be
    /// safely downconverted to uint32 in languages that support this type.
    /// https://cloud.google.com/apis/design/design_patterns#integer_types
    pub data_crc32c: Option<i64>,
}

/// Request message for
/// [SecretManagerService.CreateSecret][google.cloud.secretmanager.v1.SecretManagerService.CreateSecret].
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateSecretRequest {

    /// Required. The resource name of the project to associate with the
    /// [Secret][google.cloud.secretmanager.v1.Secret], in the format `projects/*`
    /// or `projects/*/locations/*`.
    pub parent: String,

    /// Required. This must be unique within the project.
    /// 
    /// A secret ID is a string with a maximum length of 255 characters and can
    /// contain uppercase and lowercase letters, numerals, and the hyphen (`-`) and
    /// underscore (`_`) characters.
    pub secret_id: String,

    /// Required. A [Secret][google.cloud.secretmanager.v1.Secret] with initial
    /// field values.
    pub secret: Vec<crate::Secret>,
}

/// Request message for
/// [SecretManagerService.GetSecret][google.cloud.secretmanager.v1.SecretManagerService.GetSecret].
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetSecretRequest {

    /// Required. The resource name of the
    /// [Secret][google.cloud.secretmanager.v1.Secret], in the format
    /// `projects/*/secrets/*` or `projects/*/locations/*/secrets/*`.
    pub name: String,
}
