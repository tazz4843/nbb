use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPost<'a> {
    title: Cow<'a, str>,
    description: Option<Cow<'a, str>>,
    date: u64,
    body: Cow<'a, str>,
    target: Cow<'a, str>,
}

impl<'a> BlogPost<'a> {
    #[inline]
    #[must_use]
    pub fn post_page(
        title: Cow<'a, str>,
        description: Option<Cow<'a, str>>,
        date: u64,
        body: Cow<'a, str>,
    ) -> Self {
        Self {
            title,
            description,
            date,
            body,
            target: Cow::default(),
        }
    }

    #[inline]
    #[must_use]
    pub fn post_index(
        title: Cow<'a, str>,
        description: Option<Cow<'a, str>>,
        date: u64,
        target: Cow<'a, str>,
    ) -> Self {
        Self {
            title,
            description,
            date,
            body: Cow::default(),
            target,
        }
    }
}
