# TODO.md

## 🚧 Active Feature Checklist (from Changelog)

### Text Drawing

- [ ] **Implement horizontal text drawing** - Add basic left-to-right text rendering functionality with proper glyph positioning and kerning
- [ ] **Implement multiline text drawing** - Support automatic line breaks and manual line breaks (\\n) with configurable line spacing
- [ ] **Support inverted multiline text drawing** - Allow text to be rendered upside-down or rotated while maintaining proper multiline behavior
- [ ] **Allow text color configuration** - Add RGB/RGBA color settings for text foreground with support for transparency
- [ ] **Add `min_font_size` parameter in `TextConfig`** - Set minimum font size limit to prevent text from becoming unreadably small during auto-sizing
- [ ] **Rename `font_size` to `max_font_size` in `TextConfig`** - Clarify that this parameter sets the upper bound for automatic font sizing algorithms
- [ ] **Optimize text fitting with binary search-like algorithm** - Replace linear font size reduction with efficient binary search for faster text fitting calculations

### Barcode Support

- [ ] **Add Code 128 barcode drawing** - Implement Code 128 barcode generation with automatic checksum calculation and error handling
  - [ ] **Ensure field-filling behavior** - Make barcodes automatically scale to fill their designated container area
  - [ ] **Use top-level config keys in `options`** - Move barcode configuration parameters to the root level of the options object for consistency
- [ ] **Add QR code drawing support** - Implement QR code generation with configurable size and encoding options
  - [ ] **Support `error_correction` parameter** - Allow selection of error correction levels (L, M, Q, H) for QR code robustness
  - [ ] **Support `mask_pattern` parameter** - Enable manual selection of QR code mask patterns for optimization or compliance
  - [ ] **Support `background_color` parameter** - Allow customization of QR code background color beyond default white
  - [ ] **Support `foreground_color` parameter** - Allow customization of QR code foreground (data) color beyond default black

### Configuration Models

- [ ] **Finalize `TextConfig` model with updated parameters** - Complete the text configuration structure with all new parameters and validation rules
- [ ] **Implement full `QRCodeConfig` model** - Create comprehensive configuration model for QR code generation with all supported options
- [ ] **Update `Code128Config` to use top-level keys** - Refactor Code 128 configuration to match the standardized config structure
- [ ] **Rename and implement `RelativeContainer` with `direction`** - Add directional layout support (horizontal/vertical) to relative positioning containers
- [ ] **Update `RelativeDataFieldFormat` with `direction` support** - Extend data field formatting to handle directional layout constraints

### Utilities

- [ ] **Create utilities for angled text rendering** - Develop helper functions for rotating text at arbitrary angles with proper bounding box calculations

### Validation & Safety

- [ ] **Validate coordinates in `XYXY` model** - Add bounds checking and validation for top-left/bottom-right coordinate pairs
- [ ] **Validate coordinates in `XYWH` model** - Add bounds checking and validation for position/width/height coordinate specifications
- [ ] **Ensure `Draw.text` and `Draw.barcode` ignore `None` or empty strings** - Add null/empty input handling to prevent rendering errors
- [ ] **Forbid extra fields in models from `draw.py` and `fields.py`** - Implement strict field validation to prevent configuration errors from typos or invalid fields

______________________________________________________________________

## 🏗️ Architecture Checklist

### Core Architecture

- [ ] **Central image rendering logic separated into a pure core crate** - Extract rendering engine into standalone library without external dependencies for maximum reusability
- [ ] **Shared data structures and rendering primitives used by all interfaces** - Create common types and functions shared between CLI, API, and library interfaces
- [ ] **Plug-in system for optional features like filters, barcode types, etc.** - Design extensible architecture allowing third-party plugins for custom functionality
- [ ] **Error handling with rich context (`thiserror`, `anyhow`)** - Implement comprehensive error types with detailed context and stack traces for debugging
- [ ] **Feature flags to reduce dependency load for CLI/API-specific builds** - Use Cargo features to create minimal builds excluding unused functionality

### CLI Tool Features

- [ ] **Uses `clap` or `argh` for robust argument parsing** - Implement professional command-line interface with help text, validation, and subcommands
- [ ] **Supports config file loading (`.toml`, `.json`, `.yaml`)** - Allow configuration via files in multiple formats with environment variable overrides
- [ ] **Subcommands:**
  - [ ] **`generate`: produce output from a source/template** - Main command for rendering images from template files and data sources
  - [ ] **`template list`: list all templates** - Discovery command showing available templates with metadata and descriptions
  - [ ] **`fonts install`: import fonts to user/system cache** - Font management command for installing and caching custom fonts
  - [ ] **`info`: display metadata about input/output images** - Inspection command showing image properties, embedded data, and rendering statistics
- [ ] **Input/output via files or stdin/stdout** - Support Unix pipeline workflows with streaming input/output capabilities
- [ ] **Hot-reload mode: watch templates/data sources for live re-rendering** - Development mode that automatically regenerates output when source files change
- [ ] **Image previews optionally shown in terminal (via ASCII/iterm preview)** - Quick preview functionality for immediate visual feedback in terminal environments

### Library API Features

- [ ] **Modular crate exposing both high- and low-level APIs** - Provide simple high-level interface alongside detailed low-level control for advanced users
- [ ] **Builder-style usage (`ImageBuilder::new().add_text(...).render()`)** - Implement fluent API pattern for intuitive programmatic image construction
- [ ] **Accepts both in-memory and file-based inputs** - Support direct byte arrays, file paths, and streaming readers for maximum flexibility
- [ ] **Extensible image/filter/text layout primitives** - Provide building blocks for custom rendering pipelines and effects
- [ ] **Exports to `Vec<u8>`, `DynamicImage`, or streaming writers** - Multiple output formats to integrate with different application architectures
- [ ] **Optional hooks for:**
  - [ ] **Font loading** - Customizable font discovery and loading mechanisms for different deployment scenarios
  - [ ] **Custom render passes** - Extension points for implementing custom rendering effects and transformations
  - [ ] **Post-processing** - Hooks for applying filters, optimizations, or format conversions after rendering
- [ ] **Optional compile-time features for minimal builds:**
  - [ ] **`text`** - Text rendering functionality (fonts, layout, typography)
  - [ ] **`barcode`** - Barcode and QR code generation capabilities
  - [ ] **`filters`** - Image filters and effects processing
  - [ ] **`api`** - Web API server components and dependencies
  - [ ] **`cli`** - Command-line interface tools and argument parsing

### Web API Server Features

- [ ] **Built on `axum`, `actix-web`, or `warp`** - Modern async web framework with high performance and safety guarantees
- [ ] **Exposed routes:**
  - [ ] **`POST /render`: accepts JSON + base image (multipart)** - Main rendering endpoint accepting template configuration and optional base image
  - [ ] **`GET /preview`: renders and returns a quick preview PNG** - Fast preview generation for UI feedback and validation
  - [ ] **`GET /healthz`: health check** - Service health monitoring endpoint for load balancers and orchestration
  - [ ] **`GET /version`: API versioning** - Version information endpoint for client compatibility checking
- [ ] **Static file serving for template storage, preview HTML** - Serve template files and web UI components directly from the API server
- [ ] **Token-based auth and optional IP whitelisting** - Security mechanisms to control API access and prevent abuse
- [ ] **CORS support** - Cross-origin resource sharing configuration for web browser integration
- [ ] **Optional WebSocket endpoint for continuous preview** - Real-time preview updates for interactive design tools
- [ ] **Rate limiting middleware** - Request throttling to prevent API abuse and ensure fair resource allocation

### Image Feature Set

- [ ] **Composable layers with Z-index and blend modes** - Layer-based rendering system with depth control and Photoshop-style blending
- [ ] **Text rendering with alignment, justification, dynamic wrapping** - Advanced typography with professional text layout capabilities
- [ ] **Anchored layout (e.g., top-left, center-center, bottom-right)** - Flexible positioning system using anchor points for responsive layouts
- [ ] **Automatic font sizing and multi-line text blocks** - Intelligent text fitting that adjusts size and breaks lines to fit containers
- [ ] **Barcode generation (Code128, QR, more via `barcoders`)** - Comprehensive barcode support for various industrial and commercial standards
- [ ] **Shape drawing: lines, circles, polygons** - Vector graphics primitives for creating custom designs and layouts
- [ ] **Effects and filters: blur, brightness, contrast, etc.** - Image processing filters for enhancing and stylizing rendered output
- [ ] **Gradient fills and overlays** - Support for linear and radial gradients as fills and overlay effects
- [ ] **External font injection and caching** - Dynamic font loading with performance optimization through caching
- [ ] **Templating system for reusability and batch processing** - Template engine for generating multiple variations from single designs
- [ ] **SVG/GIF/WebP support** - Extended format support beyond basic PNG/JPEG for modern web applications
- [ ] **JSON/CSV data ingestion for templated bulk output** - Data-driven rendering for generating large batches of personalized images
- [ ] **Export to PNG, JPG, SVG, and optionally PDF** - Multiple output formats for different use cases and distribution channels
- [ ] **Image masking and clipping paths** - Advanced compositing with alpha masks and vector clipping paths
- [ ] **Content-aware scaling/cropping** - Intelligent image resizing that preserves important visual elements
- [ ] **Text effects (shadows, outlines, 3D, glow)** - Advanced typography effects for enhanced visual appeal
- [ ] **Image optimization pipeline for web delivery** - Automatic optimization for file size and loading performance
- [ ] **Responsive image layouts (similar to CSS media queries)** - Adaptive layouts that change based on output dimensions or device characteristics
- [ ] **Color palettes and theme extraction from base images** - Automatic color scheme generation from source images for cohesive designs
- [ ] **Variable data printing with dynamic field positioning** - Professional print industry features for personalized marketing materials
- [ ] **Raster-to-vector conversion for text/shape elements** - Convert rendered elements to scalable vector formats when possible

### Scaling, Concurrency, and Job Queues

- [ ] **Use of thread-safe queues for background rendering (`crossbeam`, `tokio::sync::mpsc`)** - Implement async job processing with safe concurrent access patterns
- [ ] **Asynchronous API server endpoints using `tokio`** - Non-blocking request handling for high-throughput server applications
- [ ] **Job queue integration:**
  - [ ] **In-memory queues for dev/test** - Simple queue implementation for development and testing environments
  - [ ] **Redis-based queue (e.g. via `bollard`, `lapin`, or `deadpool`)** - Production-ready distributed queue system for scalable deployments
- [ ] **Multithreaded image processing pipeline using `rayon`** - Parallel processing of image operations to utilize multi-core systems efficiently
- [ ] **Pooled font and image resource caches to reduce IO overhead** - Optimize performance by caching frequently accessed resources in memory
- [ ] **Batching system for processing N templates or renders per job** - Group multiple rendering tasks for improved throughput and resource efficiency
- [ ] **Fine-grained task parallelism: separate threads for text rendering, decoding, encoding** - Specialized worker threads for different aspects of image processing
- [ ] **Optional GPU acceleration (via `wgpu`, future)** - Hardware acceleration for computationally intensive image processing operations
- [ ] **Metrics and instrumentation for latency tracking (`tracing`, Prometheus)** - Comprehensive monitoring and observability for production deployments
- [ ] **Graceful shutdown with draining queues** - Clean shutdown process that completes in-flight work before terminating
- [ ] **Worker thread pool sizing tunable via config/env** - Configurable concurrency limits based on deployment environment and hardware
- [ ] **Optional headless job consumer mode (e.g., `--worker` CLI mode)** - Dedicated worker process mode for distributed processing architectures
- [ ] **Distributed rendering farm for high-volume workloads** - Horizontal scaling across multiple machines for enterprise-level throughput
- [ ] **Intelligent caching of render components** - Smart caching system that reuses computed elements across similar rendering jobs
- [ ] **Progressive/partial rendering for large batch jobs** - Incremental rendering approach for handling massive batch processing jobs
- [ ] **Preview generation with lower resolution/quality for speed** - Fast preview mode with reduced quality for immediate feedback
- [ ] **Dynamic resource allocation based on job complexity** - Adaptive resource management that allocates more resources to complex rendering tasks
- [ ] **Blue/green deployment support for API server** - Zero-downtime deployment strategy for production service updates
- [ ] **Observability stack with tracing spans for complex operations** - Detailed performance profiling and debugging capabilities

### Developer Experience & Documentation

- [ ] **Interactive playground web UI for testing templates/designs** - Web-based tool for experimenting with templates and configurations without coding
- [ ] **Comprehensive examples repository with common use cases** - Curated collection of example templates and configurations for learning and reference
- [ ] **API documentation with runnable code samples** - Interactive documentation that allows developers to test API calls directly
- [ ] **Visual debugging mode showing layer composition/render steps** - Debug visualization that shows how images are built layer by layer
- [ ] **Benchmarking suite for performance regression testing** - Automated performance testing to ensure optimizations don't introduce regressions

### Integration & Ecosystem

- [ ] **Headless browser automation for capturing web content as image source** - Integration with browser automation tools to render web pages as base images
- [ ] **Plug-ins for design tools (Figma, Sketch) to export templates** - Bridge tools to import designs from professional design applications
- [ ] **Direct integration with popular CMS systems** - Pre-built connectors for WordPress, Drupal, and other content management platforms
- [ ] **CDN-optimized output with automatic format negotiation** - Smart image delivery that serves optimal formats based on client capabilities
- [ ] **Template marketplace/repository system** - Community platform for sharing and discovering reusable templates
- [ ] **Integration with vector graphics libraries** - Support for importing and manipulating SVG and other vector formats
- [ ] **AI-assisted layout suggestions and automated design** - Machine learning features for suggesting optimal layouts and design improvements
- [ ] **Container images for easy deployment** - Docker containers and Kubernetes manifests for simplified deployment

### Security & Compliance

- [ ] **Image content validation/sanitization** - Security measures to prevent malicious content injection through image inputs
- [ ] **Input validation and sanitization system** - Comprehensive input validation to prevent injection attacks and data corruption
- [ ] **Resource usage quotas and limits** - Configurable limits on memory, CPU time, and output size to prevent resource exhaustion
- [ ] **Content policies for generated images** - Configurable content filtering and policy enforcement for appropriate use cases
- [ ] **Privacy-focused data handling with optional PII redaction** - Tools for handling sensitive data responsibly and compliance with privacy regulations
