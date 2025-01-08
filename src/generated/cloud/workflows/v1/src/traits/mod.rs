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

/// Workflows is used to deploy and execute workflow programs.
/// Workflows makes sure the program executes reliably, despite hardware and
/// networking interruptions.
///
/// # Mocking
///
/// Application developers may use this trait to mock the workflows clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Workflows: std::fmt::Debug + Send + Sync {
    /// Lists workflows in a given project and location.
    /// The default order is not specified.
    fn list_workflows(
        &self,
        _req: crate::model::ListWorkflowsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListWorkflowsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListWorkflowsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets details of a single workflow.
    fn get_workflow(
        &self,
        _req: crate::model::GetWorkflowRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Workflow>> + Send {
        std::future::ready::<crate::Result<crate::model::Workflow>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Creates a new workflow. If a workflow with the specified name already
    /// exists in the specified project and location, the long running operation
    /// returns a [
    /// ALREADY_
    /// EXISTS][
    /// google.rpc.Code.ALREADY_
    /// EXISTS
    /// ] error.
    fn create_workflow(
        &self,
        _req: crate::model::CreateWorkflowRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a workflow with the specified name.
    /// This method also cancels and deletes all running executions of the
    /// workflow.
    fn delete_workflow(
        &self,
        _req: crate::model::DeleteWorkflowRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Updates an existing workflow.
    /// Running this method has no impact on already running executions of the
    /// workflow. A new revision of the workflow might be created as a result of a
    /// successful update operation. In that case, the new revision is used
    /// in new workflow executions.
    fn update_workflow(
        &self,
        _req: crate::model::UpdateWorkflowRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }
}

/// Manages location-related information with an API service.
///
/// # Mocking
///
/// Application developers may use this trait to mock the workflows clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Locations: std::fmt::Debug + Send + Sync {
    /// Lists information about the supported locations for this service.
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Gets information about a location.
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }
}

/// Manages long-running operations with an API service.
///
/// # Mocking
///
/// Application developers may use this trait to mock the workflows clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Operations: std::fmt::Debug + Send + Sync {
    /// ListOperations is an RPC method of Operations.
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

    /// GetOperation is an RPC method of Operations.
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

    /// DeleteOperation is an RPC method of Operations.
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }
}
