# IronGate [WIP]
This crate aims to be a powerful, yet simple, user management framework, providing everything
needed for an application that handles multiple users with different permissions.

It is currently a work in progress and should not be used in production until further notice.

## Design Goals
- Authentication: Applications can offer one or more authentication providers, which may rely on external services
  - 2FA can be required, enabled or disabled on a per-role basis
- Authorization: Any action performed within the application may be checked against IronGate's authorization API, making sure that:
  - The performer's **Session** is valid
  - One of the session's associated **Roles** has the needed **Permission** assigned
  - The action and session are within the **Scope** of the **Permission Assignment**, if a scope is defined
- Identity Management: User accounts, API access tokens, and other Authentication/Authorization subjects can:
  - Be registered (if enabled), which may be restricted to invited users
  - Edit their associated data, credentials and login settings
  - Be created, edited and removed administratively

## Documentation
(TODO)