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

/// A dyn-compatible, crate-private version of [super::Policies].
#[async_trait::async_trait]
pub trait Policies: std::fmt::Debug + Send + Sync {
    async fn list_policies(
        &self,
        req: crate::model::ListPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPoliciesResponse>;

    async fn get_policy(
        &self,
        req: crate::model::GetPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Policy>;

    async fn create_policy(
        &self,
        req: crate::model::CreatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_policy(
        &self,
        req: crate::model::UpdatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_policy(
        &self,
        req: crate::model::DeletePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

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

/// All implementations of [crate::stubs::Policies] also implement [Policies].
#[async_trait::async_trait]
impl<T: crate::stubs::Policies> Policies for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_policies(
        &self,
        req: crate::model::ListPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPoliciesResponse> {
        T::list_policies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_policy(
        &self,
        req: crate::model::GetPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Policy> {
        T::get_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_policy(
        &self,
        req: crate::model::CreatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_policy(
        &self,
        req: crate::model::UpdatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_policy(
        &self,
        req: crate::model::DeletePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_policy(self, req, options).await
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
