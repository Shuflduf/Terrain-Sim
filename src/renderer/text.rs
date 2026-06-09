use glyphon::*;
use wgpu::{Device, PresentMode, Queue, RenderPass, SurfaceConfiguration};

pub struct TextOverlay {
    font_system: FontSystem,
    swash_cache: SwashCache,
    cache: Cache,
    viewport: Viewport,
    atlas: TextAtlas,
    renderer: TextRenderer,
    buffer: cosmic_text::Buffer,
}

impl TextOverlay {
    pub fn new(device: &Device, queue: &Queue, config: &SurfaceConfiguration) -> Self {
        let mut db = fontdb::Database::new();
        db.load_font_data(
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/fonts/DejaVuSans.ttf"
            ))
            .to_vec(),
        );
        let mut font_system = FontSystem::new_with_locale_and_db("en-US".to_string(), db);
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, config.format);
        let renderer = TextRenderer::new(&mut atlas, device, Default::default(), None);
        let buffer =
            cosmic_text::Buffer::new(&mut font_system, cosmic_text::Metrics::new(16.0, 20.0));

        Self {
            font_system,
            swash_cache: SwashCache::new(),
            cache,
            viewport,
            atlas,
            renderer,
            buffer,
        }
    }

    pub fn resize(&mut self, device: &Device, queue: &Queue, config: &SurfaceConfiguration) {
        self.atlas = TextAtlas::new(device, queue, &self.cache, config.format);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        &mut self,
        device: &Device,
        queue: &Queue,
        width: u32,
        height: u32,
        fps: f32,
        chunk_count: usize,
        rendered_chunks: usize,
        vsync_mode: PresentMode,
    ) {
        self.viewport.update(queue, Resolution { width, height });

        self.buffer.set_size(
            &mut self.font_system,
            Some(width as f32),
            Some(height as f32),
        );

        let vsync_text = match vsync_mode {
            PresentMode::AutoVsync => "On",
            PresentMode::AutoNoVsync => "Off",
            _ => "Other",
        };
        let chunk_count = chunk_count.min(727);
        let lines = [
            format!("FPS: {fps:.0}"),
            format!("Chunks: {chunk_count}"),
            format!("Rendered: {rendered_chunks}"),
            format!("Vsync (V): {vsync_text}"),
            "Horizontal: WASD".into(),
            "Vertical: Q & E".into(),
            "Camera: LMB".into(),
        ];

        self.buffer.set_text(
            &mut self.font_system,
            &lines.join("\n"),
            &cosmic_text::Attrs::new(),
            cosmic_text::Shaping::Advanced,
            None,
        );

        let text_area = TextArea {
            buffer: &self.buffer,
            left: 10.0,
            top: 10.0,
            scale: 2.0,
            bounds: Default::default(),
            default_color: cosmic_text::Color::rgba(255, 255, 255, 255),
            custom_glyphs: &[],
        };

        self.renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                [text_area],
                &mut self.swash_cache,
            )
            .unwrap();
    }

    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        self.renderer
            .render(&self.atlas, &self.viewport, pass)
            .unwrap();
    }
}
