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

/// Implements a [DashboardsService](crate::stubs::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct DashboardsService<T>
where
    T: crate::stubs::DashboardsService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> DashboardsService<T>
where
    T: crate::stubs::DashboardsService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::DashboardsService for DashboardsService<T>
where
    T: crate::stubs::DashboardsService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_dashboard(
        &self,
        req: crate::model::CreateDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dashboard> {
        self.inner.create_dashboard(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_dashboards(
        &self,
        req: crate::model::ListDashboardsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDashboardsResponse> {
        self.inner.list_dashboards(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_dashboard(
        &self,
        req: crate::model::GetDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dashboard> {
        self.inner.get_dashboard(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_dashboard(
        &self,
        req: crate::model::DeleteDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_dashboard(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_dashboard(
        &self,
        req: crate::model::UpdateDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dashboard> {
        self.inner.update_dashboard(req, options).await
    }
}
