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
#[allow(unused_imports)]
use gax::error::Error;

/// Implements [DashboardsService](crate::stubs::DashboardsService) using a [gax::http_client::ReqwestClient].
#[derive(Clone)]
pub struct DashboardsService {
    inner: gax::http_client::ReqwestClient,
}

impl std::fmt::Debug for DashboardsService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("DashboardsService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl DashboardsService {
    pub async fn new(config: gax::http_client::ClientConfig) -> Result<Self> {
        let inner = gax::http_client::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl crate::stubs::DashboardsService for DashboardsService {
    async fn create_dashboard(
        &self,
        req: crate::model::CreateDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dashboard> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/dashboards", req.parent),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("validateOnly", &req.validate_only)]);
        self.inner
            .execute(builder, Some(req.dashboard), options)
            .await
    }

    async fn list_dashboards(
        &self,
        req: crate::model::ListDashboardsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDashboardsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/dashboards", req.parent),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn get_dashboard(
        &self,
        req: crate::model::GetDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dashboard> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn delete_dashboard(
        &self,
        req: crate::model::DeleteDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, format!("/v1/{}", req.name))
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn update_dashboard(
        &self,
        req: crate::model::UpdateDashboardRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dashboard> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.dashboard
                        .as_ref()
                        .ok_or_else(|| gax::path_parameter::missing("dashboard"))?
                        .name
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("validateOnly", &req.validate_only)]);
        self.inner
            .execute(builder, Some(req.dashboard), options)
            .await
    }
}
