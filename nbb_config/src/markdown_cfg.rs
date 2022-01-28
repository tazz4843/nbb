use std::borrow::Cow;

/// Markdown rendering settings
#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::struct_excessive_bools)]
#[serde(default)]
pub struct MarkdownConfig<'a> {
    /// Allow custom HTML in Markdown of posts?
    ///
    /// This may be a security issue depending on who can publish,
    /// however, for most blogs, this is fine.
    ///
    /// Defaults to true.
    pub allow_custom_html: bool,

    /// If custom HTML is denied, remove the HTML or escape it?
    ///
    /// Ignored if `allow_custom_html` is true.
    ///
    /// Example input:
    /// ```md
    /// <i>italic text</i>
    /// ```
    ///
    /// If true:
    /// ```html
    /// <p>&lt;i&gt;italic text&lt;/i&gt;</p>
    /// ```
    ///
    /// If false:
    /// ```html
    /// <p><!-- raw HTML omitted -->italic text<!-- raw HTML omitted --></p>
    /// ```
    ///
    /// Defaults to true.
    pub escape_custom_html: bool,

    /// Allow rendering of raw HTML and potentially dangerous links?
    ///
    /// This can easily be a security issue, as it allows links such as
    /// ```md
    /// [Dangerous](javascript:alert(document.cookie))
    /// ```
    ///
    /// Defaults to false.
    pub r#unsafe: bool,

    /// The maximum length of a single line.
    /// Any lines longer than this are wrapped at word breaks to this maximum.
    ///
    /// Defaults to 4,294,967,295.
    pub max_line_length: u32,

    /// Should soft line breaks in the Markdown be converted to hard line breaks in the HTML?
    ///
    /// For example, if this were true, this
    /// ```md
    /// Hello.
    /// World.
    /// ```
    /// would be turned into
    /// ```html
    /// <p>Hello.<br />
    /// World.</p>
    /// ```
    /// If this were false, the same Markdown would turn into
    /// ```html
    /// <p>Hello.
    /// World.</p>
    /// ```
    ///
    /// Defaults to false.
    pub soft_breaks_to_hard_breaks: bool,

    /// How should code be defined?
    ///
    /// Useful for custom CSS.
    ///
    /// Example input (sans backslashes, that is just to prevent rustdoc from taking the blocks):
    /// ```md
    /// \```rust
    /// fn hello();
    /// \```
    /// ```
    ///
    /// If this is true:
    /// ```html
    /// <pre lang="rust"><code>fn hello();
    /// </code></pre>
    /// ```
    ///
    /// If this is false:
    /// ```html
    /// <pre><code class="language-rust">fn hello();
    /// </code></pre>
    /// ```
    ///
    /// Defaults to true.
    pub github_pre_lang: bool,

    /// Convert normal punctuation to "smart" punctuation?
    ///
    /// Example input:
    /// ```md
    /// 'Hello,' \"world\" ...
    /// ```
    ///
    /// If true:
    /// ```html
    /// <p>‘Hello,’ “world” …</p>
    /// ```
    ///
    /// If false:
    /// ```html
    /// <p>'Hello,' &quot;world&quot; ...</p>
    /// ```
    ///
    /// Defaults to false.
    pub smart_punctuation: bool,

    /// The default language for code blocks.
    ///
    /// Defaults to None.
    pub default_code_language: Option<Cow<'a, str>>,

    /// Toggle more extensions to the Markdown spec.
    pub extensions: MarkdownExtensionConfig,
}

impl Default for MarkdownConfig<'_> {
    fn default() -> Self {
        Self {
            allow_custom_html: true,
            escape_custom_html: true,
            r#unsafe: false,
            max_line_length: 4_294_967_295,
            soft_breaks_to_hard_breaks: false,
            github_pre_lang: true,
            smart_punctuation: false,
            default_code_language: None,
            extensions: MarkdownExtensionConfig::default(),
        }
    }
}

/// Markdown rendering extension config options
#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::struct_excessive_bools)]
#[serde(default)]
pub struct MarkdownExtensionConfig {
    /// Enables the [autolink extension](https://github.github.com/gfm/#autolinks-extension-)
    /// from the GFM spec.
    ///
    /// Defaults to true.
    pub autolink: bool,

    /// Enables the description lists extension.
    ///
    /// Each term must be defined in one paragraph,
    /// followed by a blank line, and then by the details.
    /// Details begins with a colon.
    ///
    ///
    /// ```md
    /// First term
    ///
    /// : Details for the **first term**
    ///
    /// Second term
    ///
    /// : Details for the **second term**
    ///
    ///     More details in second paragraph.
    /// ```
    ///
    /// Defaults to false.
    pub description_lists: bool,

    /// Enables the
    /// [tagfilter extension](https://github.github.com/gfm/#disallowed-raw-html-extension-)
    /// from the GFM spec.
    ///
    /// This disallows the following HTML tags:
    /// * \<title>
    /// * \<textarea>
    /// * \<style>
    /// * \<xmp>
    /// * \<iframe>
    /// * \<noembed>
    /// * \<noframes>
    /// * \<script>
    /// * \<plaintext>
    ///
    /// Defaults to true.
    pub disallow_some_html: bool,

    /// Enables the footnotes extension per `cmark-gfm`.
    ///
    /// The extension is modelled after
    /// [Kramdown](https://kramdown.gettalong.org/syntax.html#footnotes).
    ///
    /// Input:
    /// ```md
    /// Hi[^x].
    ///
    /// [^x]: A greeting.
    /// ```
    ///
    /// Output:
    /// ```html
    /// <p>Hi<sup class="footnote-ref"><a href="#fn1" id="fnref1">1</a></sup>.</p>
    /// <section class="footnotes">
    /// <ol>
    /// <li id="fn1">
    /// <p>A greeting. <a href="#fnref1" class="footnote-backref">↩</a></p>
    /// </li>
    /// </ol>
    /// </section>
    /// ```
    ///
    /// Defaults to true.
    pub footnotes: bool,

    /// Enables the
    /// [strikethrough extension](https://github.github.com/gfm/#strikethrough-extension-)
    /// from the GFM spec.
    ///
    /// Defaults to true.
    pub strikethrough: bool,

    /// Enables the superscript Comrak extension.
    ///
    /// Input:
    /// ```md
    /// e = mc^2^.
    /// ```
    ///
    /// Output:
    /// ```html
    /// <p>e = mc<sup>2</sup>.</p>
    /// ```
    ///
    /// Defaults to false.
    pub superscript: bool,

    /// Enables the [table extension](https://github.github.com/gfm/#tables-extension-)
    /// from the GFM spec.
    ///
    /// Defaults to true.
    pub tables: bool,

    /// Enables the
    /// [task list items extension](https://github.github.com/gfm/#task-list-items-extension-)
    /// from the GFM spec.
    ///
    /// Note that the spec does not define the precise output, so only the bare essentials are
    /// rendered.
    ///
    /// Defaults to true.
    pub task_lists: bool,
}

impl Default for MarkdownExtensionConfig {
    fn default() -> Self {
        Self {
            autolink: true,
            description_lists: false,
            disallow_some_html: true,
            footnotes: true,
            strikethrough: true,
            superscript: false,
            tables: true,
            task_lists: true,
        }
    }
}
