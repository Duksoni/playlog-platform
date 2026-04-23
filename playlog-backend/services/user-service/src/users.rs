pub mod dto;
pub mod error;
pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

pub use dto::{
    BlockUserRequest, FindUsersQuery, FindUsersResponse, UpdatePasswordRequest,
    UpdateProfileRequest, UpdateUserRoleRequest, UserRoleChangeResponse,
};
pub use error::{Result, UserError};
pub use handler::router;
pub use model::{SimpleUser, UserDetails};
pub use repository::{PostgresUserRepository, UserRepository};
pub use service::UserService;
