use std::sync::Arc;

use application::{
    auth::{
        forgot_password::ForgotPasswordUseCase, login::LoginUseCase, register::RegisterUseCase,
        reset_password::ResetPasswordUseCase, verify_reset_token::VerifyResetTokenUseCase,
    },
    listing::{create_listing::CreateListingUseCase, search_listing::SearchListingsUseCase},
    user::get_user::GetUserUseCase,
};
use infrastructure::jwt::JwtService;

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<AuthContainer>,
    pub users: Arc<UserContainer>,
    pub listings: Arc<ListingContainer>,
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

#[derive(Clone)]
pub struct UserContainer {
    pub get: GetUserUseCase,
}

#[derive(Clone)]
pub struct ListingContainer {
    pub create: CreateListingUseCase,
    pub search: SearchListingsUseCase,
}
