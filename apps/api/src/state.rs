use std::sync::Arc;

use application::{
    auth::{
        forgot_password::ForgotPasswordUseCase, login::LoginUseCase, register::RegisterUseCase,
        reset_password::ResetPasswordUseCase, verify_reset_token::VerifyResetTokenUseCase,
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
    pub verify_reset_token: VerifyResetTokenUseCase,
    pub reset_password: ResetPasswordUseCase,
}

pub struct UserContainer {
    pub get: GetUserUseCase,
}
