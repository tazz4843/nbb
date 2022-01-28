use std::borrow::Cow;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
/// Config for a specific post.
pub struct PostConfig<'a> {
    /// An alternate title for this post.
    alt_title: Option<Cow<'a, str>>,

    /// Should this post be hidden? If so, this post does not appear in any list,
    /// and only when visited directly.
    hidden: bool,
}
