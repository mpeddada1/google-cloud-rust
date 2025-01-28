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

/// A dyn-compatible, crate-private version of `DashboardsService`.
#[async_trait::async_trait]
pub trait DashboardsService: std::fmt::Debug + Send + Sync {
    async fn create_dashboard(
        &self,
        req: crate::model::CreateDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Dashboard>;

    async fn list_dashboards(
        &self,
        req: crate::model::ListDashboardsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDashboardsResponse>;

    async fn get_dashboard(
        &self,
        req: crate::model::GetDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Dashboard>;

    async fn delete_dashboard(
        &self,
        req: crate::model::DeleteDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn update_dashboard(
        &self,
        req: crate::model::UpdateDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Dashboard>;
}

/// All implementations of [crate::traits::DashboardsService] also implement [DashboardsService].
#[async_trait::async_trait]
impl<T: crate::traits::DashboardsService> DashboardsService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_dashboard(
        &self,
        req: crate::model::CreateDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Dashboard> {
        T::create_dashboard(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_dashboards(
        &self,
        req: crate::model::ListDashboardsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDashboardsResponse> {
        T::list_dashboards(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_dashboard(
        &self,
        req: crate::model::GetDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Dashboard> {
        T::get_dashboard(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_dashboard(
        &self,
        req: crate::model::DeleteDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_dashboard(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_dashboard(
        &self,
        req: crate::model::UpdateDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Dashboard> {
        T::update_dashboard(self, req, options).await
    }
}
