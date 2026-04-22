use std::sync::Arc;

use application::user::{create_user::CreateUserUseCase, get_user::GetUserUseCase};

#[derive(Clone)]
pub struct AppState {
    pub create_user: Arc<CreateUserUseCase>,
    pub get_user: Arc<GetUserUseCase>,
}
