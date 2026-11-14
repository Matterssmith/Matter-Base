# Matter Base

Reusable core infrastructure for the Matterssmith ecosystem.

Matter Base is the public foundation used by applications and
services belonging to the Matterssmith ecosystem.

It provides reusable infrastructure without containing the
business logic of individual applications.

## Responsibilities

- Runtime foundation
- Application context
- Configuration management
- Logging
- Lifecycle foundation
- Service registry
- Event system
- Internationalization foundation
- Module contracts

## Architecture

Matter Base is designed as an independent library.

Applications consume Matter Base as a dependency.

Matter Base does not depend on Matter Bot, Matter Web,
Matter AI, Matter API, or any other ecosystem application.

## Repository

https://github.com/matterssmith-net/Matter-Base

## Usage

Add Matter Base to a Rust project:

```toml
[dependencies]
matter-base = {
    git = "https://github.com/matterssmith-net/Matter-Base",
    branch = "matter"
}