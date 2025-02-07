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

pub mod connection_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::ConnectionService] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::stubs::dynamic::ConnectionService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ConnectionService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a ConnectionService::create_connection call.
    #[derive(Clone, Debug)]
    pub struct CreateConnection(RequestBuilder<crate::model::CreateConnectionRequest>);

    impl CreateConnection {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ConnectionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateConnectionRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Connection> {
            (*self.0.stub)
                .create_connection(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateConnectionRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [connection_id][crate::model::CreateConnectionRequest::connection_id].
        pub fn set_connection_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.connection_id = v.into();
            self
        }

        /// Sets the value of [connection][crate::model::CreateConnectionRequest::connection].
        pub fn set_connection<T: Into<std::option::Option<crate::model::Connection>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.connection = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateConnection {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConnectionService::get_connection call.
    #[derive(Clone, Debug)]
    pub struct GetConnection(RequestBuilder<crate::model::GetConnectionRequest>);

    impl GetConnection {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ConnectionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetConnectionRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Connection> {
            (*self.0.stub)
                .get_connection(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetConnectionRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetConnection {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConnectionService::list_connections call.
    #[derive(Clone, Debug)]
    pub struct ListConnections(RequestBuilder<crate::model::ListConnectionsRequest>);

    impl ListConnections {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ConnectionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListConnectionsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListConnectionsResponse> {
            (*self.0.stub)
                .list_connections(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListConnectionsResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListConnectionsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListConnectionsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListConnectionsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListConnections {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConnectionService::update_connection call.
    #[derive(Clone, Debug)]
    pub struct UpdateConnection(RequestBuilder<crate::model::UpdateConnectionRequest>);

    impl UpdateConnection {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ConnectionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateConnectionRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Connection> {
            (*self.0.stub)
                .update_connection(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::UpdateConnectionRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [connection][crate::model::UpdateConnectionRequest::connection].
        pub fn set_connection<T: Into<std::option::Option<crate::model::Connection>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.connection = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateConnectionRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateConnection {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConnectionService::delete_connection call.
    #[derive(Clone, Debug)]
    pub struct DeleteConnection(RequestBuilder<crate::model::DeleteConnectionRequest>);

    impl DeleteConnection {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ConnectionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteConnectionRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<wkt::Empty> {
            (*self.0.stub)
                .delete_connection(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteConnectionRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteConnection {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConnectionService::get_iam_policy call.
    #[derive(Clone, Debug)]
    pub struct GetIamPolicy(RequestBuilder<iam_v1::model::GetIamPolicyRequest>);

    impl GetIamPolicy {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ConnectionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::GetIamPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<iam_v1::model::Policy> {
            (*self.0.stub)
                .get_iam_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][iam_v1::model::GetIamPolicyRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [options][iam_v1::model::GetIamPolicyRequest::options].
        pub fn set_options<T: Into<std::option::Option<iam_v1::model::GetPolicyOptions>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.options = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetIamPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConnectionService::set_iam_policy call.
    #[derive(Clone, Debug)]
    pub struct SetIamPolicy(RequestBuilder<iam_v1::model::SetIamPolicyRequest>);

    impl SetIamPolicy {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ConnectionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::SetIamPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<iam_v1::model::Policy> {
            (*self.0.stub)
                .set_iam_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][iam_v1::model::SetIamPolicyRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [policy][iam_v1::model::SetIamPolicyRequest::policy].
        pub fn set_policy<T: Into<std::option::Option<iam_v1::model::Policy>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.policy = v.into();
            self
        }

        /// Sets the value of [update_mask][iam_v1::model::SetIamPolicyRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for SetIamPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConnectionService::test_iam_permissions call.
    #[derive(Clone, Debug)]
    pub struct TestIamPermissions(RequestBuilder<iam_v1::model::TestIamPermissionsRequest>);

    impl TestIamPermissions {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ConnectionService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::TestIamPermissionsRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<iam_v1::model::TestIamPermissionsResponse> {
            (*self.0.stub)
                .test_iam_permissions(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][iam_v1::model::TestIamPermissionsRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [permissions][iam_v1::model::TestIamPermissionsRequest::permissions].
        pub fn set_permissions<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.permissions = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for TestIamPermissions {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
