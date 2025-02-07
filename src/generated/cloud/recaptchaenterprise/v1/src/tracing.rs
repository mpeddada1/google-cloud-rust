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

/// Implements a [RecaptchaEnterpriseService](crate::stubs::RecaptchaEnterpriseService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct RecaptchaEnterpriseService<T>
where
    T: crate::stubs::RecaptchaEnterpriseService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> RecaptchaEnterpriseService<T>
where
    T: crate::stubs::RecaptchaEnterpriseService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::RecaptchaEnterpriseService for RecaptchaEnterpriseService<T>
where
    T: crate::stubs::RecaptchaEnterpriseService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_assessment(
        &self,
        req: crate::model::CreateAssessmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Assessment> {
        self.inner.create_assessment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn annotate_assessment(
        &self,
        req: crate::model::AnnotateAssessmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnnotateAssessmentResponse> {
        self.inner.annotate_assessment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_key(
        &self,
        req: crate::model::CreateKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Key> {
        self.inner.create_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_keys(
        &self,
        req: crate::model::ListKeysRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListKeysResponse> {
        self.inner.list_keys(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn retrieve_legacy_secret_key(
        &self,
        req: crate::model::RetrieveLegacySecretKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RetrieveLegacySecretKeyResponse> {
        self.inner.retrieve_legacy_secret_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_key(
        &self,
        req: crate::model::GetKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Key> {
        self.inner.get_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_key(
        &self,
        req: crate::model::UpdateKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Key> {
        self.inner.update_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_key(
        &self,
        req: crate::model::DeleteKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn migrate_key(
        &self,
        req: crate::model::MigrateKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Key> {
        self.inner.migrate_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn add_ip_override(
        &self,
        req: crate::model::AddIpOverrideRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AddIpOverrideResponse> {
        self.inner.add_ip_override(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn remove_ip_override(
        &self,
        req: crate::model::RemoveIpOverrideRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RemoveIpOverrideResponse> {
        self.inner.remove_ip_override(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_ip_overrides(
        &self,
        req: crate::model::ListIpOverridesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListIpOverridesResponse> {
        self.inner.list_ip_overrides(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_metrics(
        &self,
        req: crate::model::GetMetricsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Metrics> {
        self.inner.get_metrics(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_firewall_policy(
        &self,
        req: crate::model::CreateFirewallPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FirewallPolicy> {
        self.inner.create_firewall_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_firewall_policies(
        &self,
        req: crate::model::ListFirewallPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListFirewallPoliciesResponse> {
        self.inner.list_firewall_policies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_firewall_policy(
        &self,
        req: crate::model::GetFirewallPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FirewallPolicy> {
        self.inner.get_firewall_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_firewall_policy(
        &self,
        req: crate::model::UpdateFirewallPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FirewallPolicy> {
        self.inner.update_firewall_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_firewall_policy(
        &self,
        req: crate::model::DeleteFirewallPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_firewall_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn reorder_firewall_policies(
        &self,
        req: crate::model::ReorderFirewallPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ReorderFirewallPoliciesResponse> {
        self.inner.reorder_firewall_policies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_related_account_groups(
        &self,
        req: crate::model::ListRelatedAccountGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRelatedAccountGroupsResponse> {
        self.inner.list_related_account_groups(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_related_account_group_memberships(
        &self,
        req: crate::model::ListRelatedAccountGroupMembershipsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRelatedAccountGroupMembershipsResponse> {
        self.inner
            .list_related_account_group_memberships(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn search_related_account_group_memberships(
        &self,
        req: crate::model::SearchRelatedAccountGroupMembershipsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchRelatedAccountGroupMembershipsResponse> {
        self.inner
            .search_related_account_group_memberships(req, options)
            .await
    }
}
