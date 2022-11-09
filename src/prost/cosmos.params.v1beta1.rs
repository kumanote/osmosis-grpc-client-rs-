/// ParameterChangeProposal defines a proposal to change one or more parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterChangeProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub changes: ::prost::alloc::vec::Vec<ParamChange>,
}
/// ParamChange defines an individual parameter change, for use in
/// ParameterChangeProposal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamChange {
    #[prost(string, tag="1")]
    pub subspace: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub value: ::prost::alloc::string::String,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
    /// subspace defines the module to query the parameter for.
    #[prost(string, tag="1")]
    pub subspace: ::prost::alloc::string::String,
    /// key defines the key of the parameter in the subspace.
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// param defines the queried parameter.
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<ParamChange>,
}
/// QuerySubspacesRequest defines a request type for querying for all registered
/// subspaces and all keys for a subspace.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubspacesRequest {
}
/// QuerySubspacesResponse defines the response types for querying for all
/// registered subspaces and all keys for a subspace.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubspacesResponse {
    #[prost(message, repeated, tag="1")]
    pub subspaces: ::prost::alloc::vec::Vec<Subspace>,
}
/// Subspace defines a parameter subspace name and all the keys that exist for
/// the subspace.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subspace {
    #[prost(string, tag="1")]
    pub subspace: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Params queries a specific parameter of a module, given its subspace and
        /// key.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.params.v1beta1.Query/Params",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Subspaces queries for all registered subspaces and all keys for a subspace.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn subspaces(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySubspacesRequest>,
        ) -> Result<tonic::Response<super::QuerySubspacesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.params.v1beta1.Query/Subspaces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
