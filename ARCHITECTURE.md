# Rust WASM Calculator Architecture

This document describes the architecture of the Rust WASM Calculator application, including the module organization, responsibilities, and the rationale for the design decisions.

## Overview

The Rust WASM Calculator is built using a hybrid architecture:

1. **Rust Backend**: Core calculator logic compiled to WebAssembly
2. **JavaScript Frontend**: UI components and interaction logic

The application follows SOLID principles and maintains a clear separation of concerns between different components.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        Web Browser                          │
└───────────────────────────┬─────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────┐
│                      HTML/CSS UI                            │
└───────────────────────────┬─────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────┐
│                   JavaScript Modules                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ calculator.js│  │ controller.js│  │ events.js          │  │
│  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
│         │                │                    │             │
│  ┌──────▼──────┐  ┌──────▼──────┐  ┌──────────▼──────────┐  │
│  │ display.js  │  │ memory.js   │  │ DOM Event Handlers  │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└───────────────────────────┬─────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────┐
│                    WebAssembly Bridge                        │
└───────────────────────────┬─────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────┐
│                       Rust Modules                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ calculator  │  │ state       │  │ errors              │  │
│  │ ┌─────────┐ │  │ ┌─────────┐ │  └─────────────────────┘  │
│  │ │operations│ │  │ │types    │ │                          │
│  │ └─────────┘ │  │ └─────────┘ │                          │
│  │ ┌─────────┐ │  │ ┌─────────┐ │                          │
│  │ │memory   │ │  │ │operations│ │                          │
│  │ └─────────┘ │  │ └─────────┘ │                          │
│  └─────────────┘  └─────────────┘                          │
└─────────────────────────────────────────────────────────────┘
```

## Module Organization

### Rust Backend

#### 1. Calculator Module (`src/calculator/`)

**Responsibility**: Core calculator functionality

- **operations.rs**: Implements basic arithmetic operations (add, subtract, multiply, divide)
- **memory.rs**: Manages calculator memory operations (store, recall, clear, add, subtract)
- **mod.rs**: Exports the module's public interface

#### 2. State Module (`src/state/`)

**Responsibility**: Manages calculator state

- **types.rs**: Defines the `Operation` enum and `CalculatorState` struct
- **operations.rs**: Implements methods for the `CalculatorState` struct
- **mod.rs**: Exports the module's public interface

#### 3. Errors Module (`src/errors.rs`)

**Responsibility**: Error handling

- Defines error types and conversion functions
- Provides JavaScript-friendly error handling

#### 4. Library Entry Point (`src/lib.rs`)

**Responsibility**: Main entry point for the WebAssembly module

- Exports functions to JavaScript
- Configures WebAssembly bindings

### JavaScript Frontend

#### 1. Main Module (`js/calculator.js`)

**Responsibility**: Entry point for the JavaScript frontend

- Initializes the calculator
- Connects WASM functions to the UI

#### 2. Controller Module (`js/calculator/controller.js`)

**Responsibility**: Manages interaction between UI and WASM

- Handles user input
- Updates the display
- Maintains calculator state

#### 3. Events Module (`js/calculator/events.js`)

**Responsibility**: Handles DOM events

- Binds click events to calculator buttons
- Manages keyboard input
- Dynamically adds UI elements

#### 4. Display Module (`js/calculator/display.js`)

**Responsibility**: Manages the calculator display

- Formats values for display
- Updates the display element
- Handles error display

#### 5. Memory Module (`js/calculator/memory.js`)

**Responsibility**: Interfaces with WASM memory functions

- Provides a clean API for memory operations
- Abstracts WASM memory functions

## Test Organization

### 1. Calculator Tests (`tests/calculator/`)

- **operations_tests.rs**: Tests for arithmetic operations
- **memory_tests.rs**: Tests for memory functions

### 2. State Tests (`tests/state/`)

- **types_tests.rs**: Tests for state type definitions
- **operations_tests.rs**: Tests for state operations
- **mod.rs**: Integration tests for state management

### 3. WASM Integration Tests (`tests/web.rs`)

- Tests for WASM-specific functionality
- Browser environment tests

## Refactoring Rationale

The codebase was refactored to achieve the following goals:

### 1. Separation of Concerns

- **Before**: Monolithic code with mixed responsibilities
- **After**: Clear separation between calculator operations, memory functions, state management, and error handling

### 2. Improved Maintainability

- **Before**: Changes to one part of the code could affect other parts
- **After**: Modular structure allows for isolated changes

### 3. Better Testability

- **Before**: Difficult to test individual components
- **After**: Each module can be tested independently

### 4. Enhanced Readability

- **Before**: Complex, intertwined code
- **After**: Clear module boundaries and responsibilities

### 5. Easier Extensibility

- **Before**: Adding new features required modifying existing code
- **After**: New features can be added by extending existing modules or adding new ones

## Design Patterns

The application uses several design patterns:

1. **Module Pattern**: Organizes code into cohesive modules with clear responsibilities
2. **Facade Pattern**: Provides a simplified interface to the complex subsystem (WASM functions)
3. **Observer Pattern**: UI components react to state changes
4. **Dependency Injection**: WASM functions are injected into the JavaScript modules

## Future Considerations

1. **State Management**: Consider using a more robust state management solution for complex state
2. **Web Workers**: Move heavy calculations to web workers for better performance
3. **Progressive Web App**: Add PWA capabilities for offline use
4. **Accessibility**: Enhance keyboard navigation and screen reader support
5. **Internationalization**: Add support for multiple languages 