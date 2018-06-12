extern crate sfml;
extern crate curved_terrain_rs;

use sfml::graphics::*;
use sfml::window::*;
use curved_terrain_rs::loader::*;
//use curved_terrain_rs::renderer;
use curved_terrain_rs::terrain::Terrain;

fn main() {
    let context_settings = ContextSettings {
        antialiasing_level: 4,
        ..Default::default()
    };

    let mut win = RenderWindow::new((800, 600), "curved-terrain-rs", Default::default(),
                                    &context_settings);
    win.set_vertical_sync_enabled(true);

    println!("{:?}", win.settings());

    let def = WorldDef::load("test.txt").unwrap();
    for i in &def {
        println!("{:?}", *i);
    }

    //let tex = renderer::render(win.size(), &def);
    //let sprite = Sprite::with_texture(tex.texture());

    let terrain = Terrain::new(&def);

    'game: loop {
        win.clear(&Color::WHITE);
        //win.draw(&sprite);
        win.draw(&terrain);
        win.display();

        while let Some(ev) = win.poll_event() {
            match ev {
                Event::KeyPressed { code: Key::Escape, .. } => break 'game,
                Event::Closed => break 'game,
                _ => {},
            }
        }
    }
}
