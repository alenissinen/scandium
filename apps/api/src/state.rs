use std::sync::Arc;

use application::{
    auth::{
        forgot_password::ForgotPasswordUseCase, login::LoginUseCase, register::RegisterUseCase,
    },
    user::get_user::GetUserUseCase,
};
use infrastructure::jwt::JwtService;

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<AuthContainer>,
    pub users: Arc<UserContainer>,
}

#[derive(Clone)]
pub struct AuthContainer {
    pub register: RegisterUseCase,
    pub login: LoginUseCase,
    pub jwt: JwtService,
    pub forgot_password: ForgotPasswordUseCase,
}

pub struct UserContainer {
    pub get: GetUserUseCase,
}
