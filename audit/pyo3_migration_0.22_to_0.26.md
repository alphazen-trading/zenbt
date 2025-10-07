# PyO3 Migration: Version 0.22 → 0.26

## Executive Summary

This document outlines the differences between PyO3 versions 0.22 and 0.26, and provides a detailed migration plan for updating the codebase. The upgrade involves several breaking changes, primarily around API renames and trait replacements, but also introduces valuable new features and improvements.

---

## Version-by-Version Changes

### PyO3 0.23 (First Major Update)

#### Breaking Changes
- **GIL Refs API Removal**: Old API removed in favor of `Bound<T>` smart pointer
- **Free-threaded Python Support**: `#[pyclass]` types now require `Sync` implementation
- **Python Version Support**: Dropped PyPy 3.7 and 3.8 support

#### New Features
- GraalPy support (24.0 and up)
- `PyMemoryView` type added
- Async fn support with `experimental-async` feature
- `#[pyclass]` now works on enums with non-unit variants
- `chrono` feature now compatible with `abi3` feature
- Free-threaded Python 3.13t support
- Introduction of `Bound<T>` and `Borrowed<T>` smart pointers

#### Deprecations
- `IntoPy<PyObject>` trait deprecated
- `ToPyObject` trait deprecated
- Various GIL-related APIs deprecated

---

### PyO3 0.24 (Incremental Improvements)

#### Changes
- Fixes for free-threaded Python 3.13
- Improvements to conversions between Python and chrono datetimes
- Build configuration fixes for Windows
- Edge case fixes in PyO3's macros
- Added support for jiff datetime conversions
- Added UUID conversion support

**Note**: No significant migration-breaking changes from 0.23 to 0.24

---

### PyO3 0.25 (API Cleanup)

#### Breaking Changes
- **Removed all functionality deprecated in 0.23**:
  - `IntoPy<PyObject>` trait removed
  - `ToPyObject` trait removed
- **MSRV Update**: Minimum Rust version bumped to 1.74

#### New Features
- **Python 3.14 support** (beta)
- **`IntoPyObject` trait**: New fallible conversion trait (replaces `IntoPy` and `ToPyObject`)
- **`#[pyclass(str="<format>")]`**: Generate `__str__` based on Display implementation or format string
- `PyRange` wrapper added
- Enhanced optional dependencies for type conversions

---

### PyO3 0.26 (Latest - Major API Changes)

#### Breaking Changes
- **Method Renames**:
  - `Python::with_gil` → `Python::attach`
  - `Python::allow_threads` → `Python::detach`
  - `Python::with_gil_unchecked` → `Python::attach_unchecked`
  - `pyo3::prepare_freethreaded_python` → `Python::initialize`

- **FFI Removals**:
  - Private types from `pyo3-ffi` removed (types starting with `_Py`)
  - `PyCode_GetNumFree` definition removed
  - Private static variables removed
  - Some FFI definitions removed on Python 3.14+

- **Dependency Updates**:
  - `hashbrown` dependency bumped to 0.15

#### New Features
- **`#[pyclass(generic)]`**: Support for runtime generic typing
- **`PyClassGuard(Mut)`**: New pyclass holders
- **`PyOnceLock`**: Thread-safe single initialization primitive
- **Type stub generation**: Initial support for generating `.pyi` files
- **`#[pyclass(immutable_type)]`**: Mark classes as immutable at type level
- **Better error handling**: `PyMemoryError` now maps to `io::ErrorKind::OutOfMemory`

#### Deprecations
- `PyObject` type alias deprecated
- `GILOnceCell` deprecated (use `PyOnceLock` instead)
- `GILProtected` deprecated

---

## Key Improvements Across All Versions

### Type Safety
- Better error handling with fallible conversions (`IntoPyObject`)
- Stricter type checking with `Bound<T>` smart pointers
- Thread-safety requirements for free-threaded Python

### Developer Experience
- More intuitive method names (`attach`/`detach` vs `with_gil`/`allow_threads`)
- Derive macros for common conversions (`#[derive(IntoPyObject)]`)
- Automatic `__str__` generation with `#[pyclass(str)]`
- Type stub generation for better IDE support

### Performance
- Free-threaded Python support for better parallelism
- More efficient smart pointer API with `Bound<T>`

### Python Version Support
- Support for Python 3.13 (including free-threaded variant)
- Support for Python 3.14 beta
- GraalPy support

---

## Appendix A: Required Codebase Changes

### Summary of Impact

**Current Status**: PyO3 0.22.5
**Target Version**: PyO3 0.26
**Files Affected**: 7 source files
**Total Changes Required**: 15+ locations

---

### 1. Update Cargo.toml

**File**: `Cargo.toml:29-32`

**Current**:
```toml
[dependencies.pyo3]
version = "0.22.5"
features = ["rust_decimal", "abi3", "abi3-py311", "extension-module", "macros", "auto-initialize", "serde", "chrono"]
```

**Updated**:
```toml
[dependencies.pyo3]
version = "0.26"
features = ["rust_decimal", "abi3", "abi3-py311", "extension-module", "macros", "auto-initialize", "serde", "chrono"]
```

---

### 2. Replace `Python::with_gil` with `Python::attach`

**Critical**: This is a direct rename with identical functionality.

#### Affected Files (9 occurrences):

**File**: `src/strategy/base.rs:37`
```rust
// Before
Python::with_gil(|py| {
    // ...
})

// After
Python::attach(|py| {
    // ...
})
```

**File**: `src/backtest/backtester.rs:48, 96, 172`
- Line 48: Replace `Python::with_gil` with `Python::attach`
- Line 96: Replace `Python::with_gil` with `Python::attach`
- Line 172: Replace `Python::with_gil` with `Python::attach`
- Line 164: Commented out - update if uncommented

**File**: `src/backtest/helpers.rs:27, 37, 47, 57`
- Line 27: Replace `Python::with_gil` with `Python::attach`
- Line 37: Replace `Python::with_gil` with `Python::attach`
- Line 47: Replace `Python::with_gil` with `Python::attach`
- Line 57: Replace `Python::with_gil` with `Python::attach`

---

### 3. Replace `ToPyObject` with `IntoPyObject`

**Important**: `IntoPyObject` is fallible (returns `Result`), while `ToPyObject` was infallible. Review each implementation to ensure error handling is appropriate.

#### Affected Files (6 implementations):

**File**: `src/sdk/position.rs:61`
```rust
// Before
impl ToPyObject for Position {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        // ...
    }
}

// After - Option 1: Manual implementation
impl IntoPyObject<'_> for Position {
    type Target = PyAny;
    type Output = Bound<'_, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'_>) -> Result<Self::Output, Self::Error> {
        // Convert implementation
        // Note: Now returns Result
    }
}

// After - Option 2: Derive macro (if applicable)
#[derive(IntoPyObject)]
pub struct Position {
    // fields
}
```

**File**: `src/sdk/order.rs:40`
- Replace `impl ToPyObject for Order` with `impl IntoPyObject`

**File**: `src/sdk/enums.rs:16, 33, 55, 78`
- Line 16: Replace `impl ToPyObject for Side`
- Line 33: Replace `impl ToPyObject for OrderStatus`
- Line 55: Replace `impl ToPyObject for OrderType`
- Line 78: Replace `impl ToPyObject for CloseReason`

**File**: `src/backtest/stats.rs:57`
- Replace `impl ToPyObject for Stats` with `impl IntoPyObject`

---

### 4. Verify `Sync` Implementation for `#[pyclass]` Types

**Requirement**: All `#[pyclass]` types must implement `Sync` for free-threaded Python compatibility.

#### Action Items:
1. Review all types decorated with `#[pyclass]`
2. Ensure internal fields are `Sync` or wrapped appropriately
3. Add `unsafe impl Sync` if manual implementation is required

**File**: `src/pyo3.rs:3-6`
```rust
#[pyclass()]
#[derive(Debug, Clone)]
pub struct Signal {
    pub index: i32,
}
```
**Status**: ✅ Already `Sync` (all fields are `Sync`)

Review other `#[pyclass]` types in:
- `src/backtest/` modules
- `src/sdk/` modules
- `src/strategy/` modules

---

### 5. Update to `Bound<T>` API Pattern (Optional but Recommended)

The new `Bound<T>` API provides better ergonomics and type safety. Consider migrating existing code that manually manages Python object lifetimes.

**Benefits**:
- Clearer ownership semantics
- Better compile-time safety
- More idiomatic Rust patterns

**Example**:
```rust
// Old pattern
fn process_data(py: Python<'_>, data: &PyAny) -> PyResult<PyObject> {
    // ...
}

// New pattern with Bound<T>
fn process_data(py: Python<'_>, data: Bound<'_, PyAny>) -> PyResult<Bound<'_, PyAny>> {
    // ...
}
```

---

### 6. Consider New Features for Implementation

#### Type Stub Generation
Enable type stub generation for better IDE support:
```rust
#[pymodule]
fn my_module(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Your module code
    Ok(())
}
```

#### Auto-generate `__str__` Methods
```rust
#[pyclass(str = "{index}")]
#[derive(Debug, Clone)]
pub struct Signal {
    pub index: i32,
}
```

#### Thread-safe Initialization with `PyOnceLock`
Replace any `GILOnceCell` usage:
```rust
use pyo3::sync::PyOnceLock;

static CACHE: PyOnceLock<MyData> = PyOnceLock::new();
```

---

## Appendix B: Migration Checklist

### Pre-Migration
- [ ] Review current codebase for deprecated API usage
- [ ] Backup current working branch
- [ ] Ensure all tests pass on PyO3 0.22.5
- [ ] Review team's availability for testing

### Phase 1: Dependency Update
- [ ] Update `Cargo.toml` to PyO3 0.26
- [ ] Run `cargo update` to update lock file
- [ ] Verify MSRV is at least Rust 1.74

### Phase 2: API Renames
- [ ] Replace all `Python::with_gil` → `Python::attach` (9 occurrences)
- [ ] Search for any `Python::allow_threads` → `Python::detach`
- [ ] Update any `pyo3::prepare_freethreaded_python` → `Python::initialize`

### Phase 3: Trait Migration
- [ ] Replace `ToPyObject` in `src/sdk/position.rs`
- [ ] Replace `ToPyObject` in `src/sdk/order.rs`
- [ ] Replace `ToPyObject` in `src/sdk/enums.rs` (4 implementations)
- [ ] Replace `ToPyObject` in `src/backtest/stats.rs`
- [ ] Update method signatures to handle `Result` returns

### Phase 4: Type Safety
- [ ] Verify all `#[pyclass]` types implement `Sync`
- [ ] Add explicit `Sync` implementations where needed
- [ ] Review thread-safety of internal state

### Phase 5: Testing
- [ ] Run `cargo build` and fix compilation errors
- [ ] Run full test suite
- [ ] Test Python integration
- [ ] Verify performance characteristics
- [ ] Test with different Python versions (3.11, 3.12, 3.13)

### Phase 6: Optimization (Optional)
- [ ] Consider migrating to `Bound<T>` API patterns
- [ ] Add `#[pyclass(str)]` attributes for better repr
- [ ] Evaluate `IntoPyObject` derive macro opportunities
- [ ] Consider enabling type stub generation

### Phase 7: Documentation
- [ ] Update internal documentation
- [ ] Document any API changes for downstream users
- [ ] Update README if necessary

---

## Appendix C: Testing Strategy

### Unit Tests
- Verify all Rust unit tests pass
- Add tests for new `IntoPyObject` error handling

### Integration Tests
- Test Python → Rust conversions
- Test Rust → Python conversions
- Verify error propagation works correctly

### Python Compatibility Tests
- Test with Python 3.11 (abi3 minimum)
- Test with Python 3.12
- Test with Python 3.13 (if available)

### Performance Tests
- Benchmark GIL acquisition (`attach` vs old `with_gil`)
- Benchmark conversion performance
- Compare memory usage

---

## Appendix D: Rollback Plan

If critical issues arise during migration:

1. **Immediate Rollback**:
   ```toml
   [dependencies.pyo3]
   version = "0.22.5"
   ```

2. **Revert Code Changes**:
   ```bash
   git revert <migration-commit-hash>
   ```

3. **Clean Build**:
   ```bash
   cargo clean
   cargo build
   ```

4. **Document Issues**:
   - Record specific failures
   - Note Python version incompatibilities
   - Document performance regressions

---

## Appendix E: Resources

### Official Documentation
- PyO3 Changelog: https://pyo3.rs/main/changelog.html
- Migration Guide: https://pyo3.rs/main/migration.html
- PyO3 Documentation: https://pyo3.rs/

### GitHub Resources
- PyO3 Repository: https://github.com/PyO3/pyo3
- Release Notes: https://github.com/PyO3/pyo3/releases
- Issue Tracker: https://github.com/PyO3/pyo3/issues

### Community Support
- Discord: PyO3 community server
- GitHub Discussions: https://github.com/PyO3/pyo3/discussions

---

## Appendix F: Estimated Effort

| Task | Estimated Time | Risk Level |
|------|---------------|------------|
| Dependency update | 5 minutes | Low |
| API renames (9 files) | 30 minutes | Low |
| Trait migration (6 implementations) | 2-4 hours | Medium |
| Sync verification | 1-2 hours | Medium |
| Testing | 2-4 hours | Low |
| Documentation | 1 hour | Low |
| **Total** | **6-12 hours** | **Medium** |

**Recommended Approach**: Allocate 2 full days for migration, testing, and contingency.

---

## Conclusion

The migration from PyO3 0.22 to 0.26 is manageable and worthwhile. The breaking changes are well-documented and mostly involve straightforward renames and trait replacements. The new features, particularly around thread safety, error handling, and type stubs, provide significant value.

**Primary Concerns**:
1. `ToPyObject` → `IntoPyObject` requires careful review due to fallibility
2. Ensuring all `#[pyclass]` types are `Sync` for free-threaded support

**Benefits**:
1. Better thread safety and free-threaded Python support
2. Improved error handling with fallible conversions
3. More intuitive API names
4. Future-proofing for Python 3.14+
5. Type stub generation support

**Recommendation**: Proceed with migration. The effort is reasonable and the API improvements are significant.
