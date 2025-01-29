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

/// Implements a [Folders](crate::stubs::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Folders<T>
where
    T: crate::stubs::Folders + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Folders<T>
where
    T: crate::stubs::Folders + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::Folders for Folders<T>
where
    T: crate::stubs::Folders + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_folder(
        &self,
        req: crate::model::GetFolderRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Folder> {
        self.inner.get_folder(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_folders(
        &self,
        req: crate::model::ListFoldersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListFoldersResponse> {
        self.inner.list_folders(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_folders(
        &self,
        req: crate::model::SearchFoldersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchFoldersResponse> {
        self.inner.search_folders(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_folder(
        &self,
        req: crate::model::CreateFolderRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_folder(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_folder(
        &self,
        req: crate::model::UpdateFolderRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_folder(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn move_folder(
        &self,
        req: crate::model::MoveFolderRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.move_folder(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_folder(
        &self,
        req: crate::model::DeleteFolderRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_folder(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn undelete_folder(
        &self,
        req: crate::model::UndeleteFolderRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.undelete_folder(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
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

/// Implements a [Organizations](crate::stubs::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Organizations<T>
where
    T: crate::stubs::Organizations + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Organizations<T>
where
    T: crate::stubs::Organizations + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::Organizations for Organizations<T>
where
    T: crate::stubs::Organizations + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_organization(
        &self,
        req: crate::model::GetOrganizationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Organization> {
        self.inner.get_organization(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_organizations(
        &self,
        req: crate::model::SearchOrganizationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchOrganizationsResponse> {
        self.inner.search_organizations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }
}

/// Implements a [Projects](crate::stubs::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Projects<T>
where
    T: crate::stubs::Projects + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Projects<T>
where
    T: crate::stubs::Projects + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::Projects for Projects<T>
where
    T: crate::stubs::Projects + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_project(
        &self,
        req: crate::model::GetProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Project> {
        self.inner.get_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_projects(
        &self,
        req: crate::model::ListProjectsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListProjectsResponse> {
        self.inner.list_projects(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_projects(
        &self,
        req: crate::model::SearchProjectsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchProjectsResponse> {
        self.inner.search_projects(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_project(
        &self,
        req: crate::model::CreateProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_project(
        &self,
        req: crate::model::UpdateProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn move_project(
        &self,
        req: crate::model::MoveProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.move_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_project(
        &self,
        req: crate::model::DeleteProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn undelete_project(
        &self,
        req: crate::model::UndeleteProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.undelete_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
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

/// Implements a [TagBindings](crate::stubs::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct TagBindings<T>
where
    T: crate::stubs::TagBindings + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> TagBindings<T>
where
    T: crate::stubs::TagBindings + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::TagBindings for TagBindings<T>
where
    T: crate::stubs::TagBindings + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_tag_bindings(
        &self,
        req: crate::model::ListTagBindingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTagBindingsResponse> {
        self.inner.list_tag_bindings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_tag_binding(
        &self,
        req: crate::model::CreateTagBindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_tag_binding(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_tag_binding(
        &self,
        req: crate::model::DeleteTagBindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_tag_binding(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_effective_tags(
        &self,
        req: crate::model::ListEffectiveTagsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListEffectiveTagsResponse> {
        self.inner.list_effective_tags(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
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

/// Implements a [TagHolds](crate::stubs::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct TagHolds<T>
where
    T: crate::stubs::TagHolds + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> TagHolds<T>
where
    T: crate::stubs::TagHolds + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::TagHolds for TagHolds<T>
where
    T: crate::stubs::TagHolds + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_tag_hold(
        &self,
        req: crate::model::CreateTagHoldRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_tag_hold(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_tag_hold(
        &self,
        req: crate::model::DeleteTagHoldRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_tag_hold(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_tag_holds(
        &self,
        req: crate::model::ListTagHoldsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTagHoldsResponse> {
        self.inner.list_tag_holds(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
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

/// Implements a [TagKeys](crate::stubs::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct TagKeys<T>
where
    T: crate::stubs::TagKeys + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> TagKeys<T>
where
    T: crate::stubs::TagKeys + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::TagKeys for TagKeys<T>
where
    T: crate::stubs::TagKeys + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_tag_keys(
        &self,
        req: crate::model::ListTagKeysRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTagKeysResponse> {
        self.inner.list_tag_keys(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_tag_key(
        &self,
        req: crate::model::GetTagKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TagKey> {
        self.inner.get_tag_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_namespaced_tag_key(
        &self,
        req: crate::model::GetNamespacedTagKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TagKey> {
        self.inner.get_namespaced_tag_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_tag_key(
        &self,
        req: crate::model::CreateTagKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_tag_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_tag_key(
        &self,
        req: crate::model::UpdateTagKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_tag_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_tag_key(
        &self,
        req: crate::model::DeleteTagKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_tag_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
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

/// Implements a [TagValues](crate::stubs::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct TagValues<T>
where
    T: crate::stubs::TagValues + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> TagValues<T>
where
    T: crate::stubs::TagValues + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::TagValues for TagValues<T>
where
    T: crate::stubs::TagValues + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_tag_values(
        &self,
        req: crate::model::ListTagValuesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTagValuesResponse> {
        self.inner.list_tag_values(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_tag_value(
        &self,
        req: crate::model::GetTagValueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TagValue> {
        self.inner.get_tag_value(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_namespaced_tag_value(
        &self,
        req: crate::model::GetNamespacedTagValueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TagValue> {
        self.inner.get_namespaced_tag_value(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_tag_value(
        &self,
        req: crate::model::CreateTagValueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_tag_value(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_tag_value(
        &self,
        req: crate::model::UpdateTagValueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_tag_value(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_tag_value(
        &self,
        req: crate::model::DeleteTagValueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_tag_value(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
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
