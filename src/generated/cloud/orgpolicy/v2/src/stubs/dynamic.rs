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

/// A dyn-compatible, crate-private version of [super::OrgPolicy].
#[async_trait::async_trait]
pub trait OrgPolicy: std::fmt::Debug + Send + Sync {
    async fn list_constraints(
        &self,
        req: crate::model::ListConstraintsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConstraintsResponse>;

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

    async fn get_effective_policy(
        &self,
        req: crate::model::GetEffectivePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Policy>;

    async fn create_policy(
        &self,
        req: crate::model::CreatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Policy>;

    async fn update_policy(
        &self,
        req: crate::model::UpdatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Policy>;

    async fn delete_policy(
        &self,
        req: crate::model::DeletePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn create_custom_constraint(
        &self,
        req: crate::model::CreateCustomConstraintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CustomConstraint>;

    async fn update_custom_constraint(
        &self,
        req: crate::model::UpdateCustomConstraintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CustomConstraint>;

    async fn get_custom_constraint(
        &self,
        req: crate::model::GetCustomConstraintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CustomConstraint>;

    async fn list_custom_constraints(
        &self,
        req: crate::model::ListCustomConstraintsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListCustomConstraintsResponse>;

    async fn delete_custom_constraint(
        &self,
        req: crate::model::DeleteCustomConstraintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;
}

/// All implementations of [crate::stubs::OrgPolicy] also implement [OrgPolicy].
#[async_trait::async_trait]
impl<T: crate::stubs::OrgPolicy> OrgPolicy for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_constraints(
        &self,
        req: crate::model::ListConstraintsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConstraintsResponse> {
        T::list_constraints(self, req, options).await
    }

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
    async fn get_effective_policy(
        &self,
        req: crate::model::GetEffectivePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Policy> {
        T::get_effective_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_policy(
        &self,
        req: crate::model::CreatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Policy> {
        T::create_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_policy(
        &self,
        req: crate::model::UpdatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Policy> {
        T::update_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_policy(
        &self,
        req: crate::model::DeletePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_custom_constraint(
        &self,
        req: crate::model::CreateCustomConstraintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CustomConstraint> {
        T::create_custom_constraint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_custom_constraint(
        &self,
        req: crate::model::UpdateCustomConstraintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CustomConstraint> {
        T::update_custom_constraint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_custom_constraint(
        &self,
        req: crate::model::GetCustomConstraintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CustomConstraint> {
        T::get_custom_constraint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_custom_constraints(
        &self,
        req: crate::model::ListCustomConstraintsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListCustomConstraintsResponse> {
        T::list_custom_constraints(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_custom_constraint(
        &self,
        req: crate::model::DeleteCustomConstraintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_custom_constraint(self, req, options).await
    }
}
