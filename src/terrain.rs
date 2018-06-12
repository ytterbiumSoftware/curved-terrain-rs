use sfml::graphics::{Color, Drawable, PrimitiveType, RenderStates, RenderTarget,
                     Vertex, VertexArray};
use attrib::Attrib;
use loader::WorldDef;

pub struct Terrain {
    vtx_arr: VertexArray,
}

impl Terrain {
    pub fn new(world: &WorldDef) -> Terrain {
        Terrain {
            vtx_arr: Self::compute(world),
        }
    }

    fn compute(world: &WorldDef) -> VertexArray {
        let mut vtx_arr = VertexArray::new(PrimitiveType::LineStrip, world.len());

        for i in world {
            let pos = match *i {
                Attrib::Node(pos) => pos,
            };

            vtx_arr.append(&Vertex::with_pos_color(pos, Color::BLACK))
        }

        vtx_arr
    }
}

impl Drawable for Terrain {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
            &'a self, target: &mut RenderTarget, states: RenderStates) {
        target.draw_vertex_array(&self.vtx_arr, states)
    }
}
