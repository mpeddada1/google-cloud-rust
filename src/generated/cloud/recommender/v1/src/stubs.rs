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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;

pub(crate) mod dynamic;

/// Defines the trait used to implement [crate::client::Recommender].
///
/// Application developers may need to implement this trait to mock
/// `client::Recommender`.  In other use-cases, application developers only
/// use `client::Recommender` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Recommender: std::fmt::Debug + Send + Sync {
    /// Implements [crate::client::Recommender::list_insights].
    fn list_insights(
        &self,
        _req: crate::model::ListInsightsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListInsightsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListInsightsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Recommender::get_insight].
    fn get_insight(
        &self,
        _req: crate::model::GetInsightRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Insight>> + Send {
        std::future::ready::<crate::Result<crate::model::Insight>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Recommender::mark_insight_accepted].
    fn mark_insight_accepted(
        &self,
        _req: crate::model::MarkInsightAcceptedRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Insight>> + Send {
        std::future::ready::<crate::Result<crate::model::Insight>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Recommender::list_recommendations].
    fn list_recommendations(
        &self,
        _req: crate::model::ListRecommendationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListRecommendationsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListRecommendationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::Recommender::get_recommendation].
    fn get_recommendation(
        &self,
        _req: crate::model::GetRecommendationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Recommendation>> + Send {
        std::future::ready::<crate::Result<crate::model::Recommendation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Recommender::mark_recommendation_dismissed].
    fn mark_recommendation_dismissed(
        &self,
        _req: crate::model::MarkRecommendationDismissedRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Recommendation>> + Send {
        std::future::ready::<crate::Result<crate::model::Recommendation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Recommender::mark_recommendation_claimed].
    fn mark_recommendation_claimed(
        &self,
        _req: crate::model::MarkRecommendationClaimedRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Recommendation>> + Send {
        std::future::ready::<crate::Result<crate::model::Recommendation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Recommender::mark_recommendation_succeeded].
    fn mark_recommendation_succeeded(
        &self,
        _req: crate::model::MarkRecommendationSucceededRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Recommendation>> + Send {
        std::future::ready::<crate::Result<crate::model::Recommendation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Recommender::mark_recommendation_failed].
    fn mark_recommendation_failed(
        &self,
        _req: crate::model::MarkRecommendationFailedRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Recommendation>> + Send {
        std::future::ready::<crate::Result<crate::model::Recommendation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Recommender::get_recommender_config].
    fn get_recommender_config(
        &self,
        _req: crate::model::GetRecommenderConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::RecommenderConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::RecommenderConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Recommender::update_recommender_config].
    fn update_recommender_config(
        &self,
        _req: crate::model::UpdateRecommenderConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::RecommenderConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::RecommenderConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Recommender::get_insight_type_config].
    fn get_insight_type_config(
        &self,
        _req: crate::model::GetInsightTypeConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::InsightTypeConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::InsightTypeConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::Recommender::update_insight_type_config].
    fn update_insight_type_config(
        &self,
        _req: crate::model::UpdateInsightTypeConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::InsightTypeConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::InsightTypeConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }
}
