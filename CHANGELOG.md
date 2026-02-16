# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive rustdoc documentation for all public types and functions
- Examples for all public APIs
- `homepage` and `documentation` fields in Cargo.toml
- `Default` trait implementation for `TaskStatus`
- `PartialEq` trait for `Balance` and `TaskResult` types
- `Hash` trait for `ProxyConfig` type
- `Copy` trait for `TaskStatus` enum
- Error and panic documentation for all trait methods

### Changed
- Enhanced documentation with hyperlinks and detailed examples
- Improved error documentation in `CaptchaSolver` trait methods

## [0.1.0] - Initial Release

### Added
- Support for multiple captcha solving providers:
  - Anticaptcha
  - CapSolver
  - CapMonster
  - 2Captcha
- Unified async/await API via `CaptchaSolver` trait
- Support for various captcha types:
  - Image to Text
  - ReCaptcha v2 (Proxyless & with Proxy)
  - ReCaptcha v3 (Proxyless & with Proxy)
  - ReCaptcha v3 Enterprise (Proxyless & with Proxy)
  - hCaptcha (Proxyless & with Proxy)
  - FunCaptcha (Proxyless & with Proxy)
  - Custom task types
- Proxy support for providers that support it
- Balance checking functionality
- Task submission and polling
- Comprehensive error handling via `thiserror`
- Uses rustls-tls for better cross-platform compatibility
