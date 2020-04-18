use ggez::*;

#[derive(Clone, Copy)]
pub struct Tile {
    pub src_rect: graphics::Rect,
}

pub fn generate(img: &graphics::Image) -> [[Tile; 30]; 30] {
    let mut ret = [[Tile {
        src_rect: graphics::Rect::new(0.0, 0.0, 0.0, 0.0),
    }; 30]; 30];
    for y in 0..30 {
        for x in 0..30 {
            let chance = rand::random::<f32>();
            if chance < 0.5 {
                ret[y][x] = Tile {
                    src_rect: graphics::Rect::new(
                        16.0 / img.width() as f32,
                        64.0 / img.height() as f32,
                        16.0 / img.width() as f32,
                        16.0 / img.height() as f32,
                    ),
                };
            } else {
                ret[y][x] = Tile {
                    src_rect: graphics::Rect::new(
                        32.0 / img.width() as f32,
                        64.0 / img.height() as f32,
                        16.0 / img.width() as f32,
                        16.0 / img.height() as f32,
                    ),
                };
            }
        }
    }
    ret
}
