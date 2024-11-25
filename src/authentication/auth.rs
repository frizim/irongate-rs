use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::identity::User;

pub enum RegistrationError {
    Closed, // Registration not enabled
    InvalidInvite, // Invite token (if required) is not valid
    IdentityTaken, // User name, E-Mail address, etc. is already taken
    RateLimited,
    ProviderFailed
}

pub enum AuthenticationError {
    Disabled, // Login not enabled
    RateLimited,
    InvalidCredentials,
    ProviderFailed
}

pub trait AuthenticationProvider<T> {
    fn register(user: &User) -> Result<User, RegistrationError>;

    fn authenticate(input: String, credentials: T) -> Result<User, AuthenticationError>;
}

pub struct LocalPasswordAuthenticationProvider {
    
}

impl AuthenticationProvider<PasswordHash<'_>> for LocalPasswordAuthenticationProvider {

    fn register(user: &User) -> Result<User, RegistrationError> {
        //TODO
        Result::Err(RegistrationError::Closed)
    }

    fn authenticate(input: String, credentials: PasswordHash) -> Result<User, AuthenticationError> {
        if Argon2::default().verify_password(input.as_bytes(), &credentials).is_err() {
            return Result::Err(AuthenticationError::InvalidCredentials);
        }

        //TODO
        Result::Err(AuthenticationError::Disabled)
    }

}