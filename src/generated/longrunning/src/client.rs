// Copyright 2024 Google LLC
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
use std::sync::Arc;

/// An implementation of [crate::traits::Operations] to make requests with.
///
/// `Operations` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `Operations` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Operations` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
///
/// Manages long-running operations with an API service.
/// When an API method normally takes long time to complete, it can be designed
/// to return [Operation][google.longrunning.Operation] to the client, and the
/// client can use this interface to receive the real response asynchronously by
/// polling the operation resource, or pass the operation resource to another API
/// (such as Pub/Sub API) to receive the response.  Any API service that returns
/// long-running operations should implement the `Operations` interface so
/// developers can have a consistent client experience.
#[derive(Clone, Debug)]
pub struct Operations {
    inner: Arc<dyn crate::traits::dyntraits::Operations>,
}

impl Operations {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: crate::traits::Operations + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::traits::dyntraits::Operations>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::traits::Operations> {
        crate::transport::Operations::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::traits::Operations> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Operations::new)
    }

    /// Lists operations that match the specified filter in the request. If the
    /// server doesn't support this method, it returns `UNIMPLEMENTED`.
    pub fn list_operations(&self, name: impl Into<String>) -> crate::builders::ListOperations {
        crate::builders::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets the latest state of a long-running operation.  Clients can use this
    /// method to poll the operation result at intervals as recommended by the API
    /// service.
    pub fn get_operation(&self, name: impl Into<String>) -> crate::builders::GetOperation {
        crate::builders::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Deletes a long-running operation. This method indicates that the client is
    /// no longer interested in the operation result. It does not cancel the
    /// operation. If the server doesn't support this method, it returns
    /// `google.rpc.Code.UNIMPLEMENTED`.
    pub fn delete_operation(&self, name: impl Into<String>) -> crate::builders::DeleteOperation {
        crate::builders::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Starts asynchronous cancellation on a long-running operation.  The server
    /// makes a best effort to cancel the operation, but success is not
    /// guaranteed.  If the server doesn't support this method, it returns
    /// `google.rpc.Code.UNIMPLEMENTED`.  Clients can use
    /// [Operations.GetOperation][google.longrunning.Operations.GetOperation] or
    /// other methods to check whether the cancellation succeeded or whether the
    /// operation completed despite cancellation. On successful cancellation,
    /// the operation is not deleted; instead, it becomes an operation with
    /// an [Operation.error][google.longrunning.Operation.error] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of `1`, corresponding to
    /// `Code.CANCELLED`.
    pub fn cancel_operation(&self, name: impl Into<String>) -> crate::builders::CancelOperation {
        crate::builders::CancelOperation::new(self.inner.clone()).set_name(name.into())
    }
}
