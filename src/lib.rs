//! # Dexter: IO Proxy
//! 
//! Dexter: IO Proxy is an HTTP proxy wrapper that provides organization-based storage with buckets, allowing for secure and organized file management through a RESTful API.
//! 
//! ## Alternative to UploadThing, Supabase Storage etc.
//! 
//! ## Features
//! - **Organization Management**
//!   - Create organizations
//!   - Manage organization settings and permissions
//!   - Delete organizations
//! 
//! - **Bucket Management** 
//!   - Create buckets within organizations
//!   - Configure bucket policies and access controls
//!   - Delete buckets
//!
//! - **File Operations**
//!   - Upload files to buckets
//!   - Download files from buckets 
//!   - Update existing files
//!   - Delete files
//!   - List files in buckets
//!
//! - **Advanced Features**
//!   - File deduplication with RMLint
//!   - File hashing and checksums
//!   - File compression/decompression
//!   - File encryption/decryption
//!   - CDN caching integration
//!
//! - **Caching & CDN Integration**
//!   - Multi-level caching (memory, disk, CDN)
//!   - AWS CloudFront CDN integration
//!   - Cloudflare CDN support
//!   - DigitalOcean Spaces CDN
//!   - Cache invalidation and purging
//!   - Cache statistics and monitoring
//!   - Configurable cache TTL
//!   - Cache warming strategies
//!   - Regional edge caching
//! 
//! ## API
//! The API provides endpoints for managing organizations, buckets, files and caching.
//! 
//! ### Organization Endpoints
//! - **POST /organizations** - Create organization
//! - **GET /organizations** - List organizations
//! - **GET /organizations/{org_id}** - Get organization details
//! - **DELETE /organizations/{org_id}** - Delete organization
//!
//! ### Bucket Endpoints
//! - **POST /organizations/{org_id}/buckets** - Create bucket
//! - **GET /organizations/{org_id}/buckets** - List buckets
//! - **GET /organizations/{org_id}/buckets/{bucket_id}** - Get bucket details
//! - **DELETE /organizations/{org_id}/buckets/{bucket_id}** - Delete bucket
//!
//! ### File Endpoints
//! - **POST /organizations/{org_id}/buckets/{bucket_id}/files** - Upload file
//! - **GET /organizations/{org_id}/buckets/{bucket_id}/files/{file_id}** - Download file
//! - **PUT /organizations/{org_id}/buckets/{bucket_id}/files/{file_id}** - Update file
//! - **DELETE /organizations/{org_id}/buckets/{bucket_id}/files/{file_id}** - Delete file
//! - **GET /organizations/{org_id}/buckets/{bucket_id}/files** - List files
//!
//! ### Cache Management Endpoints
//! - **POST /cache/flush** - Flush entire cache
//! - **POST /cache/flush/{region}** - Flush regional cache
//! - **POST /cache/invalidate** - Invalidate specific cache entries
//! - **GET /cache/stats** - Get cache statistics
//! - **POST /cache/warm** - Trigger cache warming
//! - **GET /cache/config** - Get cache configuration
//! - **PUT /cache/config** - Update cache configuration


#![allow(dead_code)]
#![allow(unused_imports)]


pub mod api;
pub mod utils;
pub mod drivers;