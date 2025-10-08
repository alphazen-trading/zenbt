# ZenBT Development Commands
# ===========================
#
# This justfile imports commands from domain-specific modules.
# For help, run: just help
# For all commands, run: just --list

set dotenv-load

# ============================================================================
# Development Commands
# ============================================================================
import 'justfiles/dev/install.just'
import 'justfiles/dev/cz.just'
import 'justfiles/dev/czr.just'
import 'justfiles/dev/cleanup.just'
import 'justfiles/dev/clean.just'
import 'justfiles/dev/local_docker.just'
import 'justfiles/dev/gdocker.just'
import 'justfiles/dev/dev.just'
import 'justfiles/dev/rs_dev.just'
import 'justfiles/dev/zellij.just'
import 'justfiles/dev/import_pickl.just'
import 'justfiles/dev/test_pickl.just'

# ============================================================================
# Tools (Dependency Management)
# ============================================================================
import 'justfiles/tools/cargo-update.just'
import 'justfiles/tools/cargo-outdated.just'
import 'justfiles/tools/install-cargo-outdated.just'
import 'justfiles/tools/rust-update.just'
import 'justfiles/tools/uv-upgrade.just'
import 'justfiles/tools/uv-sync-deps.just'

# ============================================================================
# QA (Code Quality)
# ============================================================================
import 'justfiles/qa/clippy.just'
import 'justfiles/qa/pre-commit-test.just'

# ============================================================================
# Build Commands
# ============================================================================
import 'justfiles/build/build-all-platforms.just'
import 'justfiles/build/build-all-linux.just'
import 'justfiles/build/build-linux-docker.just'
import 'justfiles/build/build-all-docker.just'
import 'justfiles/build/build-all.just'
import 'justfiles/build/build_pyi.just'

# ============================================================================
# Publish Commands
# ============================================================================
import 'justfiles/publish/pub.just'
import 'justfiles/publish/pub-docs.just'

# ============================================================================
# Documentation Commands
# ============================================================================
import 'justfiles/docs/docs.just'
import 'justfiles/docs/docs-build.just'

# Show comprehensive help (default action)
default:
    @just help

# Show comprehensive help about all commands
help:
    @echo "\033[1;36mZenBT Development Commands\033[0m"
    @echo "\033[1;36m==========================\033[0m"
    @echo ""
    @echo "\033[1;32m🚀 Most Common Commands:\033[0m"
    @echo "  just \033[0;33mdev\033[0m                      \033[0;32mStart development environment\033[0m"
    @echo "  just \033[0;33mtest\033[0m                     \033[0;32mRun all tests\033[0m"
    @echo "  just \033[0;33mbuild\033[0m                    \033[0;32mBuild all projects\033[0m"
    @echo "  just \033[0;33mclean\033[0m                    \033[0;32mClean all build artifacts\033[0m"
    @echo ""
    @echo "\033[1;34m🦀 Rust Commands:\033[0m"
    @echo "  just \033[0;33mrust-build\033[0m               \033[0;32mBuild Rust project\033[0m"
    @echo "  just \033[0;33mrust-test\033[0m                \033[0;32mRun Rust tests\033[0m"
    @echo "  just \033[0;33mrust-check\033[0m               \033[0;32mRun cargo check\033[0m"
    @echo "  just \033[0;33mrust-clean\033[0m               \033[0;32mClean Rust build artifacts\033[0m"
    @echo ""
    @echo "\033[1;35m🐍 Python Commands:\033[0m"
    @echo "  just \033[0;33mpy-install\033[0m               \033[0;32mInstall Python dependencies\033[0m"
    @echo "  just \033[0;33mpy-test\033[0m                  \033[0;32mRun Python tests\033[0m"
    @echo "  just \033[0;33mpy-format\033[0m                \033[0;32mFormat Python code\033[0m"
    @echo "  just \033[0;33mpy-lint\033[0m                  \033[0;32mLint Python code\033[0m"
    @echo ""
    @echo "\033[1;36m✅ Code Quality:\033[0m"
    @echo "  just \033[0;33mformat\033[0m                   \033[0;32mFormat all code\033[0m"
    @echo "  just \033[0;33mlint\033[0m                     \033[0;32mLint all code\033[0m"
    @echo "  just \033[0;33mcheck\033[0m                    \033[0;32mRun all quality checks\033[0m"
    @echo ""
    @echo "\033[1;33m📚 Documentation:\033[0m"
    @echo "  just \033[0;33mdocs-serve\033[0m               \033[0;32mServe documentation locally\033[0m"
    @echo "  just \033[0;33mdocs-build\033[0m               \033[0;32mBuild documentation\033[0m"
    @echo ""
    @echo "\033[1;37m📋 Reference:\033[0m"
    @echo "  just \033[0;33mhelp\033[0m                     \033[0;32mShow this help\033[0m"
    @echo "  just \033[0;33mlist\033[0m                     \033[0;32mShow all available commands\033[0m"
    @echo ""

# Show all available commands
list:
    @just --list --unsorted
