use crate::markdown_cfg::MarkdownConfig;
use std::borrow::Cow;
use std::path::Path;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
/// Global app-wide config
pub struct GlobalConfig<'a> {
    /// Config for lower-level server options
    pub server: ServerConfig<'a>,
    /// HTML rendering settings
    pub html: HtmlConfig<'a>,
    /// Markdown rendering settings
    pub markdown: MarkdownConfig<'a>,
    /// General blog config
    pub general: GeneralConfig<'a>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
/// Config for lower-level server options
pub struct ServerConfig<'a> {
    /// Address the server should bind to
    pub bind_address: ServerBindType<'a>,

    /// Log filter
    ///
    /// This is passed directly to `tracing-subscriber`.
    /// The `RUST_LOG` environment variable overrides this.
    pub log_filter: Option<String>,
}

/// The type of socket the server should bind to.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ServerBindType<'a> {
    /// Bind to a TCP socket
    ///
    /// 0: address
    ///
    /// 1: port
    Tcp(Cow<'a, str>, u16),
    /// Bind to a Unix Domain Socket
    ///
    /// 0: path
    Unix(Cow<'a, str>),
}

impl Default for ServerBindType<'_> {
    fn default() -> Self {
        Self::Tcp("0.0.0.0".into(), 8080)
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
/// HTML rendering settings
pub struct HtmlConfig<'a> {
    /// Directory to HTML files to render if you'd like to overwrite the built in ones.
    ///
    /// This directory must contain three HTML files:
    /// * index.html
    /// * blog_post.html
    /// * 404.html
    ///
    /// If any are missing, you will get a panic at runtime.
    pub custom_render_dir: Option<Cow<'a, str>>,

    /// Custom HTML to inject on every page.
    pub custom_html: CustomHtml<'a>,

    /// Custom CSS to inject into a `<style>` tag in the `<head>` of every page
    pub custom_css: Cow<'a, str>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
/// Optional custom HTML to inject
pub struct CustomHtml<'a> {
    /// Custom HTML to inject right before the closing `</head>` tag
    pub head: Cow<'a, str>,
    /// Custom HTML to inject right before the closing `</body>` tag
    pub body: Cow<'a, str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
/// General blog config
pub struct GeneralConfig<'a> {
    /// Blog title
    pub title: Cow<'a, str>,
    /// Blog description
    pub description: Cow<'a, str>,

    /// Data directory location.
    ///
    /// Defaults to `./blog`
    pub data_dir: Cow<'a, Path>,

    /// Blog language
    ///
    /// Defaults to English (`en`)
    ///
    /// Should be the two or five letter code, like `fi` for Finnish or `fr-ca` for French (Canada)
    pub language: Cow<'a, str>,

    /// Should rendered pages be cached?
    ///
    /// Defaults to true. It's recommended to only set this to false if:
    /// * you have a very large number of posts
    /// * you have *very* little memory (even with ~50 5,000 character rendered files cache only takes up around 500kb RAM)
    /// * you are setting up custom CSS
    ///
    /// If this is false, pages will be re-rendered every time they are requested.
    pub cache_rendered_pages: bool,

    /// Should pages be lazily-rendered?
    ///
    /// If true, pages will only be rendered upon first request.
    ///
    /// If false, all pages will be rendered on startup and inserted into cache.
    ///
    /// If cache is not enabled, this setting does nothing.
    ///
    /// Defaults to false.
    pub render_on_request: bool,
}

impl Default for GeneralConfig<'_> {
    fn default() -> Self {
        Self {
            title: Cow::default(),
            description: Cow::default(),
            data_dir: Path::new("./blog").into(),
            language: "en".into(),
            cache_rendered_pages: true,
            render_on_request: false,
        }
    }
}
