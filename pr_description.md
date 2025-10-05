## Files Changed
- .github/workflows/ci.yml
- .github/workflows/test.yml
- backend/.env.example
- backend/.sqlx/query-177358fec702a5f78c1ff0dbd5eed42fa868487c84ebef42dfcf695e9ce42725.json
- backend/.sqlx/query-5c8b9a2a1431a71613c6fd9d06e7f1dc430db4e2256fad1d38a33e31ef536810.json
- backend/.sqlx/query-7acd8c9bc567ef80f66a38130bb708068882a4559856e38e6231405e9acc5a74.json
- backend/.sqlx/query-b521c6c7f362753693d7059c6815de444a5c6aadc1a9950d9d71f49f52dee768.json
- backend/.sqlx/query-fd6f338fcae9c81fbf1d7590574fa950a74fa68daabb48c80a0a7754e4066987.json
- backend/Cargo.lock
- backend/Cargo.toml
- backend/migrations/002_add_github_login.sql
- backend/src/application/commands/create_profile.rs
- backend/src/application/commands/get_all_profiles.rs
- backend/src/application/commands/get_profile.rs
- backend/src/application/commands/update_profile.rs
- backend/src/application/dtos/profile_dtos.rs
- backend/src/domain/entities/profile.rs
- backend/src/domain/repositories/profile_repository.rs
- backend/src/infrastructure/repositories/postgres_profile_repository.rs
- backend/src/lib.rs
- backend/src/presentation/api.rs
- backend/src/presentation/handlers.rs
- backend/src/presentation/middlewares.rs
- backend/tests/integration_github_handle.rs
- backend/tests/profile_tests.rs
- backend/README.md

## Summary
- add end-to-end GitHub handle support across application, domain, and persistence layers
- expose REST endpoints for creating and updating profiles with validated, unique GitHub handles
- provide migration, dto, and test coverage for the new field
- align profile routes and HTTP status expectations with the audit guidance (no trailing slash, explicit 200 OK responses)
- document GitHub handle behavior and API usage in the backend README

## Notes
- integration tests run under `TEST_MODE=1`, using the test-only auth layer to bypass signature verification while still exercising the API
- profile integration tests now assert the refined status codes and updated `/profiles` paths

## Reviewer Instructions
- cargo test
- cargo test --test integration_github_handle -- --test-threads=1
