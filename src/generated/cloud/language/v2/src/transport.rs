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

/// Implements [LanguageService](crate::stubs::LanguageService) using a [gax::http_client::ReqwestClient].
#[derive(Clone)]
pub struct LanguageService {
    inner: gax::http_client::ReqwestClient,
}

impl std::fmt::Debug for LanguageService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("LanguageService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl LanguageService {
    pub async fn new(config: gax::http_client::ClientConfig) -> Result<Self> {
        let inner = gax::http_client::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl crate::stubs::LanguageService for LanguageService {
    async fn analyze_sentiment(
        &self,
        req: crate::model::AnalyzeSentimentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeSentimentResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                "/v2/documents:analyzeSentiment".to_string(),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn analyze_entities(
        &self,
        req: crate::model::AnalyzeEntitiesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeEntitiesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                "/v2/documents:analyzeEntities".to_string(),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn classify_text(
        &self,
        req: crate::model::ClassifyTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ClassifyTextResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                "/v2/documents:classifyText".to_string(),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn moderate_text(
        &self,
        req: crate::model::ModerateTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ModerateTextResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                "/v2/documents:moderateText".to_string(),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn annotate_text(
        &self,
        req: crate::model::AnnotateTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnnotateTextResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                "/v2/documents:annotateText".to_string(),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }
}
