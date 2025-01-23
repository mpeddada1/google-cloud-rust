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

use gax::error::Error;

pub(crate) mod dyntraits;

/// Cloud Run Build Control Plane API
///
/// # Mocking
///
/// Application developers may use this trait to mock the run clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Builds: std::fmt::Debug + Send + Sync {
    /// Submits a build in a given project.
    fn submit_build(
        &self,
        _req: crate::model::SubmitBuildRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SubmitBuildResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::SubmitBuildResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn wait_operation(
        &self,
        _req: longrunning::model::WaitOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }
}

/// Cloud Run Execution Control Plane API.
///
/// # Mocking
///
/// Application developers may use this trait to mock the run clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Executions: std::fmt::Debug + Send + Sync {
    /// Gets information about an Execution.
    fn get_execution(
        &self,
        _req: crate::model::GetExecutionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Execution>> + Send {
        std::future::ready::<crate::Result<crate::model::Execution>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists Executions from a Job. Results are sorted by creation time,
    /// descending.
    fn list_executions(
        &self,
        _req: crate::model::ListExecutionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListExecutionsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListExecutionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Deletes an Execution.
    fn delete_execution(
        &self,
        _req: crate::model::DeleteExecutionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Cancels an Execution.
    fn cancel_execution(
        &self,
        _req: crate::model::CancelExecutionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn wait_operation(
        &self,
        _req: longrunning::model::WaitOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns the polling policy.
    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy>;

    /// Returns the polling backoff policy.
    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// Cloud Run Job Control Plane API.
///
/// # Mocking
///
/// Application developers may use this trait to mock the run clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Jobs: std::fmt::Debug + Send + Sync {
    /// Creates a Job.
    fn create_job(
        &self,
        _req: crate::model::CreateJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets information about a Job.
    fn get_job(
        &self,
        _req: crate::model::GetJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Job>> + Send {
        std::future::ready::<crate::Result<crate::model::Job>>(Err(Error::other("unimplemented")))
    }

    /// Lists Jobs. Results are sorted by creation time, descending.
    fn list_jobs(
        &self,
        _req: crate::model::ListJobsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListJobsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListJobsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Updates a Job.
    fn update_job(
        &self,
        _req: crate::model::UpdateJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a Job.
    fn delete_job(
        &self,
        _req: crate::model::DeleteJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Triggers creation of a new Execution of this Job.
    fn run_job(
        &self,
        _req: crate::model::RunJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets the IAM Access Control policy currently in effect for the given Job.
    /// This result does not include any inherited policies.
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the IAM Access control policy for the specified Job. Overwrites
    /// any existing policy.
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns permissions that a caller has on the specified Project.
    ///
    /// There are no permissions required for making this API call.
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn wait_operation(
        &self,
        _req: longrunning::model::WaitOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns the polling policy.
    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy>;

    /// Returns the polling backoff policy.
    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// Cloud Run Revision Control Plane API.
///
/// # Mocking
///
/// Application developers may use this trait to mock the run clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Revisions: std::fmt::Debug + Send + Sync {
    /// Gets information about a Revision.
    fn get_revision(
        &self,
        _req: crate::model::GetRevisionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Revision>> + Send {
        std::future::ready::<crate::Result<crate::model::Revision>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists Revisions from a given Service, or from a given location.  Results
    /// are sorted by creation time, descending.
    fn list_revisions(
        &self,
        _req: crate::model::ListRevisionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListRevisionsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListRevisionsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a Revision.
    fn delete_revision(
        &self,
        _req: crate::model::DeleteRevisionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn wait_operation(
        &self,
        _req: longrunning::model::WaitOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns the polling policy.
    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy>;

    /// Returns the polling backoff policy.
    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// Cloud Run Service Control Plane API
///
/// # Mocking
///
/// Application developers may use this trait to mock the run clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Services: std::fmt::Debug + Send + Sync {
    /// Creates a new Service in a given project and location.
    fn create_service(
        &self,
        _req: crate::model::CreateServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets information about a Service.
    fn get_service(
        &self,
        _req: crate::model::GetServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Service>> + Send {
        std::future::ready::<crate::Result<crate::model::Service>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists Services. Results are sorted by creation time, descending.
    fn list_services(
        &self,
        _req: crate::model::ListServicesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListServicesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListServicesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Updates a Service.
    fn update_service(
        &self,
        _req: crate::model::UpdateServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a Service.
    /// This will cause the Service to stop serving traffic and will delete all
    /// revisions.
    fn delete_service(
        &self,
        _req: crate::model::DeleteServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets the IAM Access Control policy currently in effect for the given
    /// Cloud Run Service. This result does not include any inherited policies.
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the IAM Access control policy for the specified Service. Overwrites
    /// any existing policy.
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns permissions that a caller has on the specified Project.
    ///
    /// There are no permissions required for making this API call.
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn wait_operation(
        &self,
        _req: longrunning::model::WaitOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns the polling policy.
    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy>;

    /// Returns the polling backoff policy.
    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// Cloud Run Task Control Plane API.
///
/// # Mocking
///
/// Application developers may use this trait to mock the run clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Tasks: std::fmt::Debug + Send + Sync {
    /// Gets information about a Task.
    fn get_task(
        &self,
        _req: crate::model::GetTaskRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Task>> + Send {
        std::future::ready::<crate::Result<crate::model::Task>>(Err(Error::other("unimplemented")))
    }

    /// Lists Tasks from an Execution of a Job.
    fn list_tasks(
        &self,
        _req: crate::model::ListTasksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListTasksResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListTasksResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn wait_operation(
        &self,
        _req: longrunning::model::WaitOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }
}
