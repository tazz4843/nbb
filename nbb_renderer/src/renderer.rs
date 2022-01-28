use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::io::Read;
use std::sync::Arc;
use tera::{Context, Tera};

static RENDERER: OnceCell<Arc<RwLock<Tera>>> = OnceCell::new();

#[must_use]
fn load_rendering_engine() -> Arc<RwLock<Tera>> {
    let cfg = nbb_config::get_config();

    let renderer = match cfg.html.custom_render_dir {
        Some(ref dir) => match Tera::new(dir) {
            Ok(r) => r,
            Err(e) => panic!("failed to load rendering engine: {}", e),
        },
        None => {
            let mut renderer = Tera::default();
            renderer
                .add_raw_templates(vec![
                    ("index", include_str!("../../templates/index.html")),
                    ("blog_post", include_str!("../../templates/blog_post.html")),
                    ("404", include_str!("../../templates/404.html")),
                ])
                .expect("internal error: compiled-in templates are invalid");
            renderer
        }
    };
    let renderer_arc = Arc::new(RwLock::new(renderer));

    let r2 = renderer_arc.clone();
    std::thread::spawn(move || loop {
        let (mut rx, tx) =
            std::os::unix::net::UnixStream::pair().expect("failed to make unix pipe pair");

        signal_hook::low_level::pipe::register(signal_hook::consts::SIGHUP, tx)
            .expect("failed to register signal handler");

        let mut buf = [0];
        let _ = rx.read_exact(&mut buf);

        let _ = r2.write().full_reload();
    });
    renderer_arc
}

pub fn render(template_name: &str, mut context: Context) -> tera::Result<String> {
    let cfg = nbb_config::get_config();
    context.insert("lang", &cfg.general.language);
    context.insert("title", &cfg.general.title);
    context.insert("description", &cfg.general.description);
    context.insert("custom_css", &cfg.html.custom_css);
    context.insert("custom_html", &cfg.html.custom_html);
    context.insert("nbb_version", nbb_consts::PKG_VERSION);

    let renderer = RENDERER.get_or_init(load_rendering_engine);
    renderer.read().render(template_name, &context)
}
