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

pub mod iam_policy {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::IAMPolicy] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::traits::dyntraits::IAMPolicy>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::IAMPolicy>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a IAMPolicy::set_iam_policy call.
    #[derive(Clone, Debug)]
    pub struct SetIamPolicy(RequestBuilder<crate::model::SetIamPolicyRequest>);

    impl SetIamPolicy {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::IAMPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::SetIamPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Policy> {
            (*self.0.stub)
                .set_iam_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][crate::model::SetIamPolicyRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [policy][crate::model::SetIamPolicyRequest::policy].
        pub fn set_policy<T: Into<std::option::Option<crate::model::Policy>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.policy = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::SetIamPolicyRequest::update_mask].
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

    /// The request builder for a IAMPolicy::get_iam_policy call.
    #[derive(Clone, Debug)]
    pub struct GetIamPolicy(RequestBuilder<crate::model::GetIamPolicyRequest>);

    impl GetIamPolicy {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::IAMPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetIamPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Policy> {
            (*self.0.stub)
                .get_iam_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][crate::model::GetIamPolicyRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [options][crate::model::GetIamPolicyRequest::options].
        pub fn set_options<T: Into<std::option::Option<crate::model::GetPolicyOptions>>>(
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

    /// The request builder for a IAMPolicy::test_iam_permissions call.
    #[derive(Clone, Debug)]
    pub struct TestIamPermissions(RequestBuilder<crate::model::TestIamPermissionsRequest>);

    impl TestIamPermissions {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::IAMPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::TestIamPermissionsRequest>>(
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
        pub async fn send(self) -> Result<crate::model::TestIamPermissionsResponse> {
            (*self.0.stub)
                .test_iam_permissions(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][crate::model::TestIamPermissionsRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [permissions][crate::model::TestIamPermissionsRequest::permissions].
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
