#[allow(clippy::unused_async)]
pub async fn info() -> String {
    format!(
        "\
    powered by nbb version {}\n\
    git hash {:?} branch {:?}\n\
    built at {} on {} for {}\n\
    {}\n\
    {} build, opt level {}
    ",
        nbb_consts::PKG_VERSION,
        nbb_consts::GIT_VERSION,
        nbb_consts::GIT_HEAD_REF,
        nbb_consts::BUILT_TIME_UTC,
        nbb_consts::HOST,
        nbb_consts::TARGET,
        nbb_consts::RUSTC_VERSION,
        nbb_consts::PROFILE,
        nbb_consts::OPT_LEVEL
    )
}
