use sfml::graphics::{BlendMode, Color, PrimitiveType, RenderTarget, RenderTexture,
                     Vertex, VertexArray};
use sfml::system::Vector2u;
use attrib::Attrib;
use loader::WorldDef;

const COLOR: Color = Color::BLACK;

pub fn render(size: Vector2u, def: &WorldDef) -> RenderTexture {
    let mut vtx_arr = VertexArray::new(PrimitiveType::LineStrip, def.len());
    for i in def {
        let pos = match *i {
            Attrib::Node(node_pos) => node_pos,
        };

        vtx_arr.append(&Vertex::with_pos_color(pos, COLOR));
    }

    let mut tex = RenderTexture::new(size.x, size.y, false).unwrap();
    tex.set_smooth(true);
    tex.clear(&Color::TRANSPARENT);

    tex.draw(&vtx_arr);
    tex.display();

    tex
}
