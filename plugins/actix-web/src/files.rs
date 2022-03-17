#[cfg(any(feature = "actix2", feature = "actix3"))]
extern crate actix_service1 as actix_service;

#[cfg(feature = "actix4")]
extern crate actix_service2 as actix_service;

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, io,
    path::{Path, PathBuf},
};

use actix_service::{IntoServiceFactory, ServiceFactory};
use actix_web::{
    dev::{
        AppService, HttpServiceFactory, RequestHead, ServiceRequest,
        ServiceResponse,
    },
    error::Error,
    guard::Guard,
    http::header::DispositionType,
    HttpRequest,
};
use futures_core::future::LocalBoxFuture;

use actix_files::{
    // directory_listing, named,
    FilesService,
    Directory, //DirectoryRenderer, HttpNewService, MimeOverride, PathFilter,
};

use paperclip_core::v2::models::{
    DefaultOperationRaw, DefaultSchemaRaw, HttpMethod, SecurityScheme, DefaultResponseRaw
};

use super::Mountable;

/// Wrapper for [`actix-files::Files`](https://docs.rs/actix-files/latest/actix_files/struct.Files.html)
#[derive(Clone)]
pub struct Files {
    pub mount_path: String,
    inner: actix_files::Files,
}

impl fmt::Debug for Files {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Files {
    /// Wrapper for [actix_files::Files::new](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.new)
    pub fn new<T: Into<PathBuf>>(mount_path: &str, serve_from: T) -> Self {
        let files = actix_files::Files::new(mount_path, serve_from);
        Self {
            mount_path: mount_path.trim_end_matches('/').to_owned(),
            inner: files,
        }
    }

    /// Wrapper for [actix_files::Files::show_files_listing](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.show_files_listing)
    pub fn show_files_listing(mut self) -> Self {
        self.inner = self.inner.show_files_listing();
        self
    }

    /// Wrapper for [actix_files::Files::redirect_to_slash_directory](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.redirect_to_slash_directory)
    pub fn redirect_to_slash_directory(mut self) -> Self {
        self.inner = self.inner.redirect_to_slash_directory();
        self
    }

    /// Wrapper for [actix_files::Files::files_listing_renderer](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.files_listing_renderer)
    pub fn files_listing_renderer<F>(mut self, f: F) -> Self
    where
        for<'r, 's> F:
            Fn(&'r Directory, &'s HttpRequest) -> Result<ServiceResponse, io::Error> + 'static,
    {
        self.inner = self.inner.files_listing_renderer(f);
        self
    }

    /// Wrapper for [actix_files::Files::mime_override](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.mime_override)
    pub fn mime_override<F>(mut self, f: F) -> Self
    where
        F: Fn(&mime::Name<'_>) -> DispositionType + 'static,
    {
        self.inner = self.inner.mime_override(f);
        self
    }

    /// Wrapper for [actix_files::Files::path_filter](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.path_filter)
    pub fn path_filter<F>(mut self, f: F) -> Self
    where
        F: Fn(&Path, &RequestHead) -> bool + 'static,
    {
        self.inner = self.inner.path_filter(f);
        self
    }

    /// Wrapper for [actix_files::Files::index_file](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.index_file)
    pub fn index_file<T: Into<String>>(mut self, index: T) -> Self {
        self.inner = self.inner.index_file(index);
        self
    }

    /// Wrapper for [actix_files::Files::use_etag](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.use_etag)
    pub fn use_etag(mut self, value: bool) -> Self {
        self.inner = self.inner.use_etag(value);
        self
    }

    /// Wrapper for [actix_files::Files::use_last_modified](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.use_last_modified)
    pub fn use_last_modified(mut self, value: bool) -> Self {
        self.inner = self.inner.use_last_modified(value);
        self
    }

    /// Wrapper for [actix_files::Files::prefer_utf8](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.prefer_utf8)
    pub fn prefer_utf8(mut self, value: bool) -> Self {
        self.inner = self.inner.prefer_utf8(value);
        self
    }

    /// Wrapper for [actix_files::Files::guard](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.guard)
    pub fn guard<G: Guard + 'static>(mut self, guard: G) -> Self {
        self.inner = self.inner.guard(guard);
        self
    }

    /// Wrapper for [actix_files::Files::method_guard](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.method_guard)
    pub fn method_guard<G: Guard + 'static>(mut self, guard: G) -> Self {
        self.inner = self.inner.method_guard(guard);
        self
    }

    #[doc(hidden)]
    #[deprecated(since = "0.6.0", note = "Renamed to `method_guard`.")]
    pub fn use_guards<G: Guard + 'static>(self, guard: G) -> Self {
        self.method_guard(guard)
    }

    /// Wrapper for [actix_files::Files::disable_content_disposition](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.disable_content_disposition)
    pub fn disable_content_disposition(mut self) -> Self {
        self.inner = self.inner.disable_content_disposition();
        self
    }

    /// Wrapper for [actix_files::Files::default_handler](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.default_handler)
    pub fn default_handler<F, U>(mut self, f: F) -> Self
    where
        F: IntoServiceFactory<U, ServiceRequest>,
        U: ServiceFactory<
                ServiceRequest,
                Config = (),
                Response = ServiceResponse,
                Error = Error,
            > + 'static,
    {
        self.inner = self.inner.default_handler(f);
        self
    }

    /// Wrapper for [actix_files::Files::use_hidden_files](https://docs.rs/actix-files/latest/actix_files/struct.Files.html#method.use_hidden_files)
    pub fn use_hidden_files(mut self) -> Self {
        self.inner = self.inner.use_hidden_files();
        self
    }
}

impl HttpServiceFactory for Files {
    fn register(self, config: &mut AppService) {
        self.inner.register(config)
    }
}

impl ServiceFactory<ServiceRequest> for Files {
    type Response = ServiceResponse;
    type Error = Error;
    type Config = ();
    type Service = FilesService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, cfg: ()) -> Self::Future {
        self.inner.new_service(cfg)
    }
}

#[cfg(feature = "actix-files")]
impl Mountable for Files {
    fn path(&self) -> &str { &self.mount_path }

    /// Map of HTTP methods and the associated API operations.
    fn operations(&mut self) -> BTreeMap<HttpMethod, DefaultOperationRaw> {
        BTreeMap::from([
            (HttpMethod::Get, DefaultOperationRaw {
                operation_id: None,
                summary: None,
                description: None,
                consumes: None,
                produces: None,
                security: vec![],
                schemes: BTreeSet::new(),
                responses: BTreeMap::from([("200".to_string(), paperclip_core::v2::models::Either::Right(DefaultResponseRaw::default()))]),
                parameters: vec![],
                deprecated: false,
                tags: vec![],
            })
        ])
    }

    /// The definitions recorded by this object.
    fn definitions(&mut self) -> BTreeMap<String, DefaultSchemaRaw> { BTreeMap::default() }

    /// The security definitions recorded by this object.
    fn security_definitions(&mut self) -> BTreeMap<String, SecurityScheme> { BTreeMap::default() }
}
