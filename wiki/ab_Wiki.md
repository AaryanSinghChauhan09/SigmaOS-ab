# SigmaOS-ab Component Wiki

## Overview
SigmaOS-ab is a SigmaOS component providing a basic 'ab' (already been) utility functionality. It demonstrates component initialization patterns, fail-fast error handling, and AAA (Arrange-Act-ASSERT) test methodology.

## Principles
- Single Responsibility
- Fail-Fast System
- Defensive Programming
- Deterministic Testing
- Clear Abstraction
- Encapsulation

## Architecture
SigmaOS-ab follows a component lifecycle pattern:
- **Component Struct**: Holds initialization state and name
- **Lifecycle Methods**: `init()`, `run()`, `reset()` with state validation
- **Error Handling**: `ComponentError` enum with descriptive messages
- **Result Type**: `ComponentResult<T>` for consistent error handling

## API
The component provides the following public APIs:
- `Component::new()` - Create uninitialised component
- `Component::init()` - Initialise the component (returns `ComponentResult<()>`)
- `Component::is_ready()` - Check if component is initialised
- `Component::run()` - Execute primary operation (fails if not initialised)
- `Component::reset()` - Reset to uninitialised state
- `Component::name()` - Get component name

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test fixtures (create Component instance)
2. **Act**: Execute the code under test (call method)
3. **Assert**: Verify the expected outcome (check state/result)

Example:
```rust
#[test]
fn test_init_makes_ready_aaa() {
    // Arrange
    let mut c = Component::new();
    // Act
    c.init().unwrap();
    // Assert
    assert!(c.is_ready());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- All public functions documented with doc comments
- Consistent error handling with thiserror
- Unit tests provided for all public functionality
- Fail-fast on invalid state (component not initialised)

## TODO
- Add more comprehensive integration tests
- Expand component functionality beyond basic lifecycle
- Add CLI interface for component control