/// In Rust, Protobuf messages are typically compiled into structs using Prost and Tonic.
/// Below is the conceptual equivalent of the generated Rust types.

use prost::Message;

/// An enum is a fixed set of named values. The first MUST be 0 (the default).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Role {
    /// 0 is the implicit default, reserve it
    Unspecified = 0,
    Member = 1,
    Admin = 2,
}

/// A message is a typed record. Each FIELD has a type, a name, and a NUMBER.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub full_name: ::prost::alloc::string::String,
    /// a nested enum
    #[prost(enumeration = "Role", tag = "4")]
    pub role: i32,
    /// "repeated" = a list/array of strings
    #[prost(string, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// unix seconds; proto3 has no native date
    #[prost(int64, tag = "6")]
    pub created_at: i64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserResponse {
    /// messages nest inside messages
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserRequest {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub full_name: ::prost::alloc::string::String,
    /// "optional" tracks presence: set vs unset vs ""
    #[prost(string, optional, tag = "3")]
    pub phone: ::core::option::Option<::prost::alloc::string::String>,
}

// Tonic would generate the UserService trait for the server and client struct.
#[tonic::async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn get_user(
        &self,
        request: tonic::Request<GetUserRequest>,
    ) -> Result<tonic::Response<GetUserResponse>, tonic::Status>;

    async fn create_user(
        &self,
        request: tonic::Request<CreateUserRequest>,
    ) -> Result<tonic::Response<User>, tonic::Status>;
}
