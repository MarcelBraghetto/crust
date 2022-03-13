pub struct TextureData<'a> {
    pub width: u32,
    pub height: u32,
    surface: sdl2::surface::Surface<'a>,
}

impl<'a> TextureData<'a> {
    pub fn new(surface: sdl2::surface::Surface) -> TextureData {
        let width = surface.width();
        let height = surface.height();

        TextureData {
            surface: surface,
            width: width,
            height: height,
        }
    }

    pub fn surface(&mut self) -> &sdl2::surface::Surface {
        &self.surface
    }
}
