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
use std::sync::Arc;

pub(crate) mod dynamic;

/// Defines the trait used to implement [crate::client::DatabaseAdmin].
///
/// Application developers may need to implement this trait to mock
/// `client::DatabaseAdmin`.  In other use-cases, application developers only
/// use `client::DatabaseAdmin` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait DatabaseAdmin: std::fmt::Debug + Send + Sync {
    /// Implements [crate::client::DatabaseAdmin::list_databases].
    fn list_databases(
        &self,
        _req: crate::model::ListDatabasesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListDatabasesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListDatabasesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::create_database].
    fn create_database(
        &self,
        _req: crate::model::CreateDatabaseRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::get_database].
    fn get_database(
        &self,
        _req: crate::model::GetDatabaseRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Database>> + Send {
        std::future::ready::<crate::Result<crate::model::Database>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::update_database].
    fn update_database(
        &self,
        _req: crate::model::UpdateDatabaseRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::update_database_ddl].
    fn update_database_ddl(
        &self,
        _req: crate::model::UpdateDatabaseDdlRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::drop_database].
    fn drop_database(
        &self,
        _req: crate::model::DropDatabaseRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::DatabaseAdmin::get_database_ddl].
    fn get_database_ddl(
        &self,
        _req: crate::model::GetDatabaseDdlRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GetDatabaseDdlResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::GetDatabaseDdlResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::DatabaseAdmin::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::DatabaseAdmin::create_backup].
    fn create_backup(
        &self,
        _req: crate::model::CreateBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::copy_backup].
    fn copy_backup(
        &self,
        _req: crate::model::CopyBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::get_backup].
    fn get_backup(
        &self,
        _req: crate::model::GetBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Backup>> + Send {
        std::future::ready::<crate::Result<crate::model::Backup>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::update_backup].
    fn update_backup(
        &self,
        _req: crate::model::UpdateBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Backup>> + Send {
        std::future::ready::<crate::Result<crate::model::Backup>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::delete_backup].
    fn delete_backup(
        &self,
        _req: crate::model::DeleteBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::DatabaseAdmin::list_backups].
    fn list_backups(
        &self,
        _req: crate::model::ListBackupsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListBackupsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListBackupsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::restore_database].
    fn restore_database(
        &self,
        _req: crate::model::RestoreDatabaseRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::list_database_operations].
    fn list_database_operations(
        &self,
        _req: crate::model::ListDatabaseOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListDatabaseOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListDatabaseOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::DatabaseAdmin::list_backup_operations].
    fn list_backup_operations(
        &self,
        _req: crate::model::ListBackupOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListBackupOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListBackupOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::DatabaseAdmin::list_database_roles].
    fn list_database_roles(
        &self,
        _req: crate::model::ListDatabaseRolesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListDatabaseRolesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListDatabaseRolesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::DatabaseAdmin::add_split_points].
    fn add_split_points(
        &self,
        _req: crate::model::AddSplitPointsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AddSplitPointsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AddSplitPointsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::DatabaseAdmin::create_backup_schedule].
    fn create_backup_schedule(
        &self,
        _req: crate::model::CreateBackupScheduleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::BackupSchedule>> + Send {
        std::future::ready::<crate::Result<crate::model::BackupSchedule>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::get_backup_schedule].
    fn get_backup_schedule(
        &self,
        _req: crate::model::GetBackupScheduleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::BackupSchedule>> + Send {
        std::future::ready::<crate::Result<crate::model::BackupSchedule>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::update_backup_schedule].
    fn update_backup_schedule(
        &self,
        _req: crate::model::UpdateBackupScheduleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::BackupSchedule>> + Send {
        std::future::ready::<crate::Result<crate::model::BackupSchedule>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::delete_backup_schedule].
    fn delete_backup_schedule(
        &self,
        _req: crate::model::DeleteBackupScheduleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::DatabaseAdmin::list_backup_schedules].
    fn list_backup_schedules(
        &self,
        _req: crate::model::ListBackupSchedulesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListBackupSchedulesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListBackupSchedulesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::DatabaseAdmin::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::DatabaseAdmin::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::DatabaseAdmin::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::DatabaseAdmin::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
