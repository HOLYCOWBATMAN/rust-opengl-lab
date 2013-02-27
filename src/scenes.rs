// extern mod sdl;
use sdl;
use sdl::video::Surface;

struct GridScene
{
    cube_amount_width: u16,
    cube_width: u16,
    cube_height: u16
}

impl GridScene
{
    static fn new(surface: &Surface, cube_amount_width: u16) -> GridScene
    {
        GridScene
        {
            cube_amount_width: cube_amount_width,
            cube_width: surface.get_width() / cube_amount_width,
            cube_height: surface.get_height() / cube_amount_width
        }
    }

    fn render(&self, surface: &Surface)
    {
        let rng = rand::Rng();

        for u16::range(0, self.cube_amount_width) |i|
        {
            for u16::range(0, self.cube_amount_width) |j|
            {
                surface.fill_rect(
                    Some(sdl::Rect
                    {
                        x: i * self.cube_width as i16,
                        y: j * self.cube_height as i16,
                        w: self.cube_width as u16,
                        h: self.cube_height as u16
                    }),
                    rng.gen::<sdl::video::Color>()
                );
            }
        }
    }
}