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

use std::sync::Arc;

/// A dyn-compatible, crate-private version of [super::VpcAccessService].
#[async_trait::async_trait]
pub trait VpcAccessService: std::fmt::Debug + Send + Sync {
    async fn create_connector(
        &self,
        req: crate::model::CreateConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_connector(
        &self,
        req: crate::model::GetConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Connector>;

    async fn list_connectors(
        &self,
        req: crate::model::ListConnectorsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectorsResponse>;

    async fn delete_connector(
        &self,
        req: crate::model::DeleteConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [crate::stubs::VpcAccessService] also implement [VpcAccessService].
#[async_trait::async_trait]
impl<T: crate::stubs::VpcAccessService> VpcAccessService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_connector(
        &self,
        req: crate::model::CreateConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_connector(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_connector(
        &self,
        req: crate::model::GetConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Connector> {
        T::get_connector(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_connectors(
        &self,
        req: crate::model::ListConnectorsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectorsResponse> {
        T::list_connectors(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_connector(
        &self,
        req: crate::model::DeleteConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_connector(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        T::get_polling_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
