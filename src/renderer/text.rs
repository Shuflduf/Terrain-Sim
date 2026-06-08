use glyphon::*;
use wgpu::{Device, Queue, RenderPass, SurfaceConfiguration};

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
        let mut font_system = FontSystem::new();
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

    pub fn prepare(
        &mut self,
        device: &Device,
        queue: &Queue,
        width: u32,
        height: u32,
        fps: f32,
        chunk_count: usize,
        rendered_chunks: usize,
    ) {
        self.viewport.update(queue, Resolution { width, height });

        self.buffer.set_size(
            &mut self.font_system,
            Some(width as f32),
            Some(height as f32),
        );

        self.buffer.set_text(
            &mut self.font_system,
            &format!("FPS: {fps:.0}\nChunks: {chunk_count}\nRendered: {rendered_chunks}"),
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
