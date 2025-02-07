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

/// Defines the trait used to implement [crate::client::AccessApproval].
///
/// Application developers may need to implement this trait to mock
/// `client::AccessApproval`.  In other use-cases, application developers only
/// use `client::AccessApproval` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait AccessApproval: std::fmt::Debug + Send + Sync {
    /// Implements [crate::client::AccessApproval::list_approval_requests].
    fn list_approval_requests(
        &self,
        _req: crate::model::ListApprovalRequestsMessage,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListApprovalRequestsResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListApprovalRequestsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::AccessApproval::get_approval_request].
    fn get_approval_request(
        &self,
        _req: crate::model::GetApprovalRequestMessage,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ApprovalRequest>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ApprovalRequest>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::AccessApproval::approve_approval_request].
    fn approve_approval_request(
        &self,
        _req: crate::model::ApproveApprovalRequestMessage,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ApprovalRequest>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ApprovalRequest>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::AccessApproval::dismiss_approval_request].
    fn dismiss_approval_request(
        &self,
        _req: crate::model::DismissApprovalRequestMessage,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ApprovalRequest>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ApprovalRequest>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::AccessApproval::invalidate_approval_request].
    fn invalidate_approval_request(
        &self,
        _req: crate::model::InvalidateApprovalRequestMessage,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ApprovalRequest>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ApprovalRequest>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::AccessApproval::get_access_approval_settings].
    fn get_access_approval_settings(
        &self,
        _req: crate::model::GetAccessApprovalSettingsMessage,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AccessApprovalSettings>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AccessApprovalSettings>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::AccessApproval::update_access_approval_settings].
    fn update_access_approval_settings(
        &self,
        _req: crate::model::UpdateAccessApprovalSettingsMessage,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AccessApprovalSettings>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AccessApprovalSettings>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::AccessApproval::delete_access_approval_settings].
    fn delete_access_approval_settings(
        &self,
        _req: crate::model::DeleteAccessApprovalSettingsMessage,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::AccessApproval::get_access_approval_service_account].
    fn get_access_approval_service_account(
        &self,
        _req: crate::model::GetAccessApprovalServiceAccountMessage,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AccessApprovalServiceAccount>>
           + Send {
        std::future::ready::<crate::Result<crate::model::AccessApprovalServiceAccount>>(Err(
            Error::other("unimplemented"),
        ))
    }
}
