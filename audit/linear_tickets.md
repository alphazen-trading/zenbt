# ZenBT Codebase Improvements - Linear Tickets

## 🔴 Critical Tickets

### SECURITY-001: Remove Hardcoded Credentials
**Labels**: security, credentials

**Description**: Remove all hardcoded credentials and implement secure credential management.

**Acceptance Criteria**:
- [ ] Remove hardcoded password "pass" from `src/zenbt/grafana/dashboard.py`
- [ ] Remove hardcoded password from `src/zenbt/grafana/token_manager.py`
- [ ] Remove hardcoded password from `docker/docker-compose.yml`
- [ ] Implement environment variable-based credential management
- [ ] Add credential validation and error handling
- [ ] Update documentation on credential setup

**Files to Modify**:
- `src/zenbt/grafana/dashboard.py`
- `src/zenbt/grafana/token_manager.py`
- `docker/docker-compose.yml`

---

### SECURITY-002: Fix Rust Array Bounds Vulnerabilities
**Labels**: security, rust, backend

**Description**: Fix potential array bounds violations in Rust indicators that could cause panics.

**Acceptance Criteria**:
- [ ] Fix array indexing in `rs/src/indicators/cross_above.rs`
- [ ] Fix array indexing in `rs/src/indicators/cross_below.rs`
- [ ] Add bounds checking before array access
- [ ] Replace unsafe `.unwrap()` calls with proper error handling
- [ ] Add unit tests for edge cases (empty arrays, single element)

**Files to Modify**:
- `rs/src/indicators/cross_above.rs`
- `rs/src/indicators/cross_below.rs`

---

### SECURITY-003: Implement Input Validation
**Labels**: security, validation, backend

**Description**: Add comprehensive input validation for all external API calls and user inputs.

**Acceptance Criteria**:
- [ ] Add validation for price values (non-negative, reasonable ranges)
- [ ] Add validation for array indices before access
- [ ] Add validation for DataFrame operations
- [ ] Add validation for HTTP request parameters
- [ ] Implement proper error responses for invalid inputs
- [ ] Add input sanitization for string inputs

**Files to Modify**:
- `src/zenbt/grafana/dashboard.py`
- `src/zenbt/grafana/token_manager.py`
- `rs/src/strategy/base.rs`
- `rs/src/backtest/methods.rs`

---

### TESTING-001: Implement Comprehensive Test Suite
**Labels**: testing, qa, coverage

**Description**: Replace trivial test with comprehensive test coverage for core functionality.

**Acceptance Criteria**:
- [ ] Remove trivial test in `tests/test_main.py`
- [ ] Add unit tests for all Python modules (>80% coverage)
- [ ] Add unit tests for Rust modules (>80% coverage)
- [ ] Add integration tests for backtesting workflow
- [ ] Add performance tests for critical paths
- [ ] Set up test fixtures and test data
- [ ] Configure test automation in CI/CD

**Files to Create/Modify**:
- `tests/test_zenbt/` (new directory structure)
- `tests/test_rust/` (new directory structure)
- `tests/fixtures/` (test data)
- `.github/workflows/test.yml` (new CI workflow)

---

## 🟡 High Tickets

### ARCH-001: Fix Package Import Structure
**Labels**: architecture, imports, backend

**Description**: Fix broken package imports by properly implementing `__init__.py` files.

**Acceptance Criteria**:
- [ ] Implement proper `__init__.py` in `src/zenbt/`
- [ ] Implement proper `__init__.py` in `src/zenbt/sdk/`
- [ ] Implement proper `__init__.py` in `src/zenbt/strategies/`
- [ ] Implement proper `__init__.py` in `src/zenbt/grafana/`
- [ ] Test package imports work correctly
- [ ] Update documentation with import examples

**Files to Modify**:
- `src/zenbt/__init__.py`
- `src/zenbt/sdk/__init__.py`
- `src/zenbt/strategies/__init__.py`
- `src/zenbt/grafana/__init__.py`

---

### ARCH-002: Standardize Data Processing Libraries
**Labels**: architecture, data-processing, backend

**Description**: use Polars for data processing

**Acceptance Criteria**:
- [ ] Analyze current usage of pandas vs polars
- [ ] Choose primary data processing library (recommend polars for performance)
- [ ] Refactor all data processing code to use chosen library
- [ ] Update dependencies to remove unused library
- [ ] Add migration guide for breaking changes
- [ ] Update benchmarks to reflect changes

**Files to Modify**:
- `src/zenbt/data/data.py`
- `src/zenbt/sdk/stats.py`
- `src/zenbt/multi_backtest.py`
- `pyproject.toml` (dependencies)

---

### QUALITY-001: Reduce Rust Panic Points
**Labels**: quality, rust, error-handling

**Description**: Replace `.unwrap()` calls with proper error handling in Rust code.

**Acceptance Criteria**:
- [ ] Replace `.unwrap()` with `?` operator where appropriate
- [ ] Add custom error types for different failure scenarios
- [ ] Implement proper error propagation to Python
- [ ] Add error handling tests
- [ ] Update documentation on error handling

**Files to Modify**:
- `rs/src/strategy/base.rs`
- `rs/src/backtest/backtester.rs`
- `rs/src/backtest/methods.rs`
- `rs/src/lib.rs`

---

### QUALITY-002: Fix Logic Errors
**Labels**: quality, bugfix, backend

**Description**: Fix identified logic errors in trading strategy implementation.

**Acceptance Criteria**:
- [ ] Fix `cross_below` creating "Long" orders instead of "Short" orders
- [ ] Remove duplicate `create_market_order` implementations
- [ ] Fix random number generation for order IDs
- [ ] Add tests to verify correct order creation logic

**Files to Modify**:
- `rs/src/strategy/base.rs`
- `rs/src/indicators/cross_below.rs`

---

### QUALITY-003: Clean Up Development Code
**Labels**: quality, cleanup, backend

**Description**: Remove development and debug code from production modules.

**Acceptance Criteria**:
- [ ] Remove or relocate code in `src/_dev/` directory
- [ ] Remove hardcoded debug paths like `/tmp/slpeters`
- [ ] Remove commented-out code blocks
- [ ] Clean up unused imports
- [ ] Remove dead code flagged by compiler warnings

**Files to Modify**:
- `src/_dev/` (entire directory)
- `rs/src/backtest/methods.rs`
- `rs/src/strategy/base.rs`

---

## 🟢 Medium Tickets

### PERF-001: Optimize Data Processing Performance
**Labels**: performance, optimization, backend

**Description**: Improve performance of data processing operations.

**Acceptance Criteria**:
- [ ] Reduce unnecessary type conversions
- [ ] Implement caching for expensive operations
- [ ] Optimize memory usage in multiprocessing
- [ ] Add performance benchmarks
- [ ] Document performance improvements

**Files to Modify**:
- `src/zenbt/data/data.py`
- `src/zenbt/multi_backtest.py`
- `src/_bench/` (benchmarking code)

---

### PERF-002: Optimize Rust Memory Usage
**Labels**: performance, memory, rust

**Description**: Reduce unnecessary memory allocations and cloning in Rust code.

**Acceptance Criteria**:
- [ ] Reduce unnecessary `clone()` operations
- [ ] Optimize `HashMap` and `Vec` usage in loops
- [ ] Implement lazy evaluation where possible
- [ ] Add memory usage benchmarks

**Files to Modify**:
- `rs/src/strategy/base.rs`
- `rs/src/backtest/backtester.rs`

---

### ARCH-003: Implement Dependency Injection
**Labels**: architecture, refactoring, backend

**Description**: Add dependency injection for better testability and maintainability.

**Acceptance Criteria**:
- [ ] Design dependency injection container
- [ ] Refactor large classes to use dependency injection
- [ ] Add interfaces/traits for major components
- [ ] Update tests to use dependency injection
- [ ] Add documentation on architecture

**Files to Modify**:
- `src/zenbt/sdk/base.py`
- `src/zenbt/strategies/`
- `rs/src/strategy/base.rs`

---

### QUALITY-004: Refactor Large Functions
**Labels**: quality, refactoring, backend

**Description**: Break down large functions into smaller, more manageable components.

**Acceptance Criteria**:
- [ ] Refactor `Strategy::new()` (102 lines) into smaller functions
- [ ] Extract common logic into helper functions
- [ ] Improve code readability and maintainability
- [ ] Add tests for refactored functions

**Files to Modify**:
- `rs/src/strategy/base.rs`
- `src/zenbt/multi_backtest.py`

---

### DOCS-001: Improve Documentation
**Labels**: documentation, readme, api-docs

**Description**: Add comprehensive documentation for the codebase.

**Acceptance Criteria**:
- [ ] Write proper README with setup instructions
- [ ] Add API documentation for all public functions
- [ ] Add code examples and tutorials
- [ ] Document architecture and design decisions
- [ ] Add troubleshooting guide

**Files to Create/Modify**:
- `README.md`
- `docs/` (new directory)
- `docs/api/` (API documentation)
- `docs/tutorials/` (tutorials)

---

## 🔵 Low Tickets

### INFRA-001: Improve CI/CD Pipeline
**Labels**: infrastructure, ci-cd, devops

**Description**: Add quality gates and validation to build pipeline.

**Acceptance Criteria**:
- [ ] Add linting checks to CI/CD
- [ ] Add type checking to CI/CD
- [ ] Add security scanning
- [ ] Add test coverage reporting
- [ ] Add automatic version bumping

**Files to Modify**:
- `.github/workflows/build.yml`
- `.github/workflows/test.yml` (new)
- `.github/workflows/security.yml` (new)

---

### INFRA-002: Fix Cargo.toml Typo
**Labels**: infrastructure, typo, rust

**Description**: Fix typo in Cargo.toml dependency configuration.

**Acceptance Criteria**:
- [ ] Fix `default-feautres` to `default-features` in polars-plan dependency
- [ ] Verify build still works after fix

**Files to Modify**:
- `rs/Cargo.toml`

---

### QUALITY-005: Improve Git Workflow
**Labels**: quality, git, process

**Description**: Improve git commit messages and branch strategy.

**Acceptance Criteria**:
- [ ] Implement conventional commits
- [ ] Add commit message templates
- [ ] Document branching strategy
- [ ] Add pre-commit hooks for commit message validation

**Files to Create**:
- `.gitmessage` (commit template)
- `docs/git_workflow.md`

---

## 📋 Implementation Timeline

### Sprint 1 (Critical Security & Testing)
- SECURITY-001: Remove Hardcoded Credentials
- SECURITY-002: Fix Rust Array Bounds Vulnerabilities  
- SECURITY-003: Implement Input Validation
- TESTING-001: Implement Comprehensive Test Suite

### Sprint 2 (Architecture & Quality)
- ARCH-001: Fix Package Import Structure
- ARCH-002: Standardize Data Processing Libraries
- QUALITY-001: Reduce Rust Panic Points
- QUALITY-002: Fix Logic Errors

### Sprint 3 (Performance & Documentation)
- QUALITY-003: Clean Up Development Code
- PERF-001: Optimize Data Processing Performance
- DOCS-001: Improve Documentation

### Sprint 4 (Infrastructure & Polish)
- PERF-002: Optimize Rust Memory Usage
- ARCH-003: Implement Dependency Injection
- INFRA-001: Improve CI/CD Pipeline

---

*These tickets should be imported into Linear with appropriate labels, assignees, and project associations.*