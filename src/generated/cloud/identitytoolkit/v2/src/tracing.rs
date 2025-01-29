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

/// Implements a [AccountManagementService](crate::stubs::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AccountManagementService<T>
where
    T: crate::stubs::AccountManagementService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AccountManagementService<T>
where
    T: crate::stubs::AccountManagementService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::AccountManagementService for AccountManagementService<T>
where
    T: crate::stubs::AccountManagementService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn finalize_mfa_enrollment(
        &self,
        req: crate::model::FinalizeMfaEnrollmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FinalizeMfaEnrollmentResponse> {
        self.inner.finalize_mfa_enrollment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn start_mfa_enrollment(
        &self,
        req: crate::model::StartMfaEnrollmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StartMfaEnrollmentResponse> {
        self.inner.start_mfa_enrollment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn withdraw_mfa(
        &self,
        req: crate::model::WithdrawMfaRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::WithdrawMfaResponse> {
        self.inner.withdraw_mfa(req, options).await
    }
}

/// Implements a [AuthenticationService](crate::stubs::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AuthenticationService<T>
where
    T: crate::stubs::AuthenticationService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AuthenticationService<T>
where
    T: crate::stubs::AuthenticationService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::AuthenticationService for AuthenticationService<T>
where
    T: crate::stubs::AuthenticationService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn finalize_mfa_sign_in(
        &self,
        req: crate::model::FinalizeMfaSignInRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FinalizeMfaSignInResponse> {
        self.inner.finalize_mfa_sign_in(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn start_mfa_sign_in(
        &self,
        req: crate::model::StartMfaSignInRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StartMfaSignInResponse> {
        self.inner.start_mfa_sign_in(req, options).await
    }
}
