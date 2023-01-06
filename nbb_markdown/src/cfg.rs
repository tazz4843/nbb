use comrak::ComrakOptions;
use once_cell::sync::OnceCell;

static RENDERER_CONFIG: OnceCell<ComrakOptions> = OnceCell::new();

fn load_config() -> ComrakOptions {
    let cfg = nbb_config::get_config();
    let md_cfg = &cfg.markdown;
    let md_ext_cfg = &md_cfg.extensions;

    let mut options = ComrakOptions::default();

    let mut parse_opts = &mut options.parse;
    parse_opts.default_info_string = md_cfg.default_code_language.clone().map(|x| x.to_string());
    parse_opts.smart = md_cfg.smart_punctuation;

    let mut render_opts = &mut options.render;
    render_opts.escape = md_cfg.escape_custom_html;
    render_opts.github_pre_lang = md_cfg.github_pre_lang;
    render_opts.hardbreaks = md_cfg.soft_breaks_to_hard_breaks;
    render_opts.unsafe_ = md_cfg.r#unsafe;
    render_opts.width = md_cfg.max_line_length as usize;

    let mut extension_opts = &mut options.extension;
    extension_opts.autolink = md_ext_cfg.autolink;
    extension_opts.description_lists = md_ext_cfg.description_lists;
    extension_opts.footnotes = md_ext_cfg.footnotes;
    extension_opts.superscript = md_ext_cfg.superscript;
    extension_opts.strikethrough = md_ext_cfg.strikethrough;
    extension_opts.table = md_ext_cfg.tables;
    extension_opts.tagfilter = md_ext_cfg.disallow_some_html;
    extension_opts.tasklist = md_ext_cfg.task_lists;

    options
}

pub fn get_config<'a>() -> &'a ComrakOptions {
    RENDERER_CONFIG.get_or_init(load_config)
}
