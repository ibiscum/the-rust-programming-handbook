# Chapter 14 Regression Tests

This directory contains integration tests for Chapter 14 examples covering async web frameworks with Axum and Tokio.

## Running Tests

To run all chapter-14 tests:
```bash
cargo test --package chapter-14-tests
```

To run a specific test:
```bash
cargo test --package chapter-14-tests test_01_hello_web_compiles
```

## Test Coverage

### Basic Web Servers (11 tests)
These tests verify that each web server example compiles successfully. Since web servers run indefinitely, tests check compilation only:

- `test_01_hello_web_compiles` - Basic "Hello, Web!" server
- `test_02_routing_basics_compiles` - Route handling fundamentals
- `test_03_shared_state_compiles` - Shared application state
- `test_04_path_extractor_compiles` - Path parameter extraction
- `test_05_query_params_compiles` - Query parameter handling
- `test_06_json_extractor_compiles` - JSON request/response bodies
- `test_07_restful_api_full_compiles` - Full REST API implementation
- `test_08_database_pool_compiles` - Database connection pooling
- `test_10_shared_pool_compiles` - Shared database pool patterns
- `test_14_main_compiles` - Comprehensive example application

### Database Projects (2 tests)
These projects use `sqlx` compile-time macros that require database connection for compilation:

- `test_12_final_db_app_compiles` - Complete database application
- `test_13_final_backend_with_cors_compiles` - Backend with CORS support

To test these with a real database:
```bash
export DATABASE_URL="postgres://user:password@localhost:5432/mydb"
cargo test --package chapter-14-tests
```

## Dependencies

Chapter-14 projects use:
- **axum** (0.8) - Web framework
- **tokio** (1) - Async runtime
- **serde/serde_json** (1.0/1) - Serialization
- **sqlx** (0.8) - Database access (selected projects)
- **dotenvy** (0.15) - Environment configuration
- **tower-http** (0.5) - HTTP middleware including CORS

## Notes

- All projects run on `127.0.0.1:8080` (or similar localhost ports)
- Web server examples need to be tested with actual HTTP clients for behavior validation
- Database examples require PostgreSQL for full compilation
- Tests validate that examples compile and initialize correctly
