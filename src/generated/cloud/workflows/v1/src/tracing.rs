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
use crate::Result;

/// Implements a [Workflows](crate::stubs::Workflows) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Workflows<T>
where
    T: crate::stubs::Workflows + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Workflows<T>
where
    T: crate::stubs::Workflows + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::Workflows for Workflows<T>
where
    T: crate::stubs::Workflows + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_workflows(
        &self,
        req: crate::model::ListWorkflowsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListWorkflowsResponse> {
        self.inner.list_workflows(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_workflow(
        &self,
        req: crate::model::GetWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Workflow> {
        self.inner.get_workflow(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_workflow(
        &self,
        req: crate::model::CreateWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_workflow(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_workflow(
        &self,
        req: crate::model::DeleteWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_workflow(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_workflow(
        &self,
        req: crate::model::UpdateWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_workflow(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
