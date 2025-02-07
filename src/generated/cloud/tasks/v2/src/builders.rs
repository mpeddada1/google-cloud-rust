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

pub mod cloud_tasks {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::CloudTasks] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::stubs::dynamic::CloudTasks>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a CloudTasks::list_queues call.
    #[derive(Clone, Debug)]
    pub struct ListQueues(RequestBuilder<crate::model::ListQueuesRequest>);

    impl ListQueues {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListQueuesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListQueuesResponse> {
            (*self.0.stub)
                .list_queues(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListQueuesResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListQueuesRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListQueuesRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListQueuesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListQueuesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListQueues {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::get_queue call.
    #[derive(Clone, Debug)]
    pub struct GetQueue(RequestBuilder<crate::model::GetQueueRequest>);

    impl GetQueue {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetQueueRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Queue> {
            (*self.0.stub)
                .get_queue(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetQueueRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetQueue {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::create_queue call.
    #[derive(Clone, Debug)]
    pub struct CreateQueue(RequestBuilder<crate::model::CreateQueueRequest>);

    impl CreateQueue {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateQueueRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Queue> {
            (*self.0.stub)
                .create_queue(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateQueueRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [queue][crate::model::CreateQueueRequest::queue].
        pub fn set_queue<T: Into<std::option::Option<crate::model::Queue>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.queue = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateQueue {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::update_queue call.
    #[derive(Clone, Debug)]
    pub struct UpdateQueue(RequestBuilder<crate::model::UpdateQueueRequest>);

    impl UpdateQueue {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateQueueRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Queue> {
            (*self.0.stub)
                .update_queue(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [queue][crate::model::UpdateQueueRequest::queue].
        pub fn set_queue<T: Into<std::option::Option<crate::model::Queue>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.queue = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateQueueRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateQueue {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::delete_queue call.
    #[derive(Clone, Debug)]
    pub struct DeleteQueue(RequestBuilder<crate::model::DeleteQueueRequest>);

    impl DeleteQueue {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteQueueRequest>>(mut self, v: V) -> Self {
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
                .delete_queue(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteQueueRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteQueue {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::purge_queue call.
    #[derive(Clone, Debug)]
    pub struct PurgeQueue(RequestBuilder<crate::model::PurgeQueueRequest>);

    impl PurgeQueue {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::PurgeQueueRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Queue> {
            (*self.0.stub)
                .purge_queue(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::PurgeQueueRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for PurgeQueue {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::pause_queue call.
    #[derive(Clone, Debug)]
    pub struct PauseQueue(RequestBuilder<crate::model::PauseQueueRequest>);

    impl PauseQueue {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::PauseQueueRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Queue> {
            (*self.0.stub)
                .pause_queue(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::PauseQueueRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for PauseQueue {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::resume_queue call.
    #[derive(Clone, Debug)]
    pub struct ResumeQueue(RequestBuilder<crate::model::ResumeQueueRequest>);

    impl ResumeQueue {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ResumeQueueRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Queue> {
            (*self.0.stub)
                .resume_queue(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::ResumeQueueRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ResumeQueue {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::get_iam_policy call.
    #[derive(Clone, Debug)]
    pub struct GetIamPolicy(RequestBuilder<iam_v1::model::GetIamPolicyRequest>);

    impl GetIamPolicy {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
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

    /// The request builder for a CloudTasks::set_iam_policy call.
    #[derive(Clone, Debug)]
    pub struct SetIamPolicy(RequestBuilder<iam_v1::model::SetIamPolicyRequest>);

    impl SetIamPolicy {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
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

    /// The request builder for a CloudTasks::test_iam_permissions call.
    #[derive(Clone, Debug)]
    pub struct TestIamPermissions(RequestBuilder<iam_v1::model::TestIamPermissionsRequest>);

    impl TestIamPermissions {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
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

    /// The request builder for a CloudTasks::list_tasks call.
    #[derive(Clone, Debug)]
    pub struct ListTasks(RequestBuilder<crate::model::ListTasksRequest>);

    impl ListTasks {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListTasksRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListTasksResponse> {
            (*self.0.stub)
                .list_tasks(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListTasksResponse, gax::error::Error> {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListTasksRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [response_view][crate::model::ListTasksRequest::response_view].
        pub fn set_response_view<T: Into<crate::model::task::View>>(mut self, v: T) -> Self {
            self.0.request.response_view = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListTasksRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListTasksRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListTasks {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::get_task call.
    #[derive(Clone, Debug)]
    pub struct GetTask(RequestBuilder<crate::model::GetTaskRequest>);

    impl GetTask {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetTaskRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Task> {
            (*self.0.stub)
                .get_task(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetTaskRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [response_view][crate::model::GetTaskRequest::response_view].
        pub fn set_response_view<T: Into<crate::model::task::View>>(mut self, v: T) -> Self {
            self.0.request.response_view = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetTask {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::create_task call.
    #[derive(Clone, Debug)]
    pub struct CreateTask(RequestBuilder<crate::model::CreateTaskRequest>);

    impl CreateTask {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateTaskRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Task> {
            (*self.0.stub)
                .create_task(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateTaskRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [task][crate::model::CreateTaskRequest::task].
        pub fn set_task<T: Into<std::option::Option<crate::model::Task>>>(mut self, v: T) -> Self {
            self.0.request.task = v.into();
            self
        }

        /// Sets the value of [response_view][crate::model::CreateTaskRequest::response_view].
        pub fn set_response_view<T: Into<crate::model::task::View>>(mut self, v: T) -> Self {
            self.0.request.response_view = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateTask {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::delete_task call.
    #[derive(Clone, Debug)]
    pub struct DeleteTask(RequestBuilder<crate::model::DeleteTaskRequest>);

    impl DeleteTask {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteTaskRequest>>(mut self, v: V) -> Self {
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
                .delete_task(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteTaskRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteTask {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::run_task call.
    #[derive(Clone, Debug)]
    pub struct RunTask(RequestBuilder<crate::model::RunTaskRequest>);

    impl RunTask {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::RunTaskRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Task> {
            (*self.0.stub)
                .run_task(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::RunTaskRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [response_view][crate::model::RunTaskRequest::response_view].
        pub fn set_response_view<T: Into<crate::model::task::View>>(mut self, v: T) -> Self {
            self.0.request.response_view = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for RunTask {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::list_locations call.
    #[derive(Clone, Debug)]
    pub struct ListLocations(RequestBuilder<location::model::ListLocationsRequest>);

    impl ListLocations {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<location::model::ListLocationsRequest>>(
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
        pub async fn send(self) -> Result<location::model::ListLocationsResponse> {
            (*self.0.stub)
                .list_locations(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<location::model::ListLocationsResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [name][location::model::ListLocationsRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [filter][location::model::ListLocationsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][location::model::ListLocationsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][location::model::ListLocationsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListLocations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudTasks::get_location call.
    #[derive(Clone, Debug)]
    pub struct GetLocation(RequestBuilder<location::model::GetLocationRequest>);

    impl GetLocation {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::CloudTasks>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<location::model::GetLocationRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<location::model::Location> {
            (*self.0.stub)
                .get_location(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][location::model::GetLocationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetLocation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
