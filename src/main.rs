extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::InitFlag;

static NAME: &str = "Tiles 2";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
static FPS: u32 = 60;
const HORIZ_TILE_NUM: usize = 16;
const VERT_TILE_NUM: usize = 12;
const GAME_AREA_WIDTH: usize = 800;
const GAME_AREA_HEIGHT: usize = HEIGHT as usize - (2 * (HEIGHT as f64 / VERT_TILE_NUM as f64) as usize);


enum EventCode {
    Quit,
}

#[derive(PartialEq)]
enum ClickMode {
    Off,
    Residential,
}

enum TileState {
    Tile = 0,
    Residential = 1,
    Legend = 2,
}

struct Tile {
    rect: sdl2::rect::Rect,
    tile_state: u64,
}

struct GameState<'a> {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    fps_manager: sdl2::gfx::framerate::FPSManager,
    tiles: Vec<Vec<Tile>>,
    legend: Vec<Vec<Tile>>,
    textures: Vec<sdl2::render::Texture<'a>>,
    click_mode: ClickMode,
}

impl GameState<'_> {
    fn handle_events(&mut self) -> Option<EventCode> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Some(EventCode::Quit)
                },
                Event::KeyDown { keycode: Some(Keycode::R), ..} => {
                    if self.click_mode == ClickMode::Residential {
                        self.click_mode = ClickMode::Off;
                    } else {
                        self.click_mode = ClickMode::Residential;
                    }
                    return None
                },
                Event::MouseButtonDown {x, y, ..} => {
                    for i in 0..self.tiles.len() {
                        for j in 0..self.tiles[i].len() {
                            if self.tiles[i][j].rect.contains_point(sdl2::rect::Point::new(x, y)) {
                                if self.click_mode == ClickMode::Residential {
                                    self.tiles[i][j].tile_state = TileState::Residential as u64;
                                    return None
                                }
                            }
                        }
                    }
                    return None
                },
                _ => {}
            }
        }
        None
    }

    fn render(&mut self) {
        self.canvas.clear();
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].len() {
                self.canvas.copy(&self.textures[self.tiles[i][j].tile_state as usize], None, self.tiles[i][j].rect).unwrap();
            }
        }
        for i in 0..self.legend.len() {
            for j in 0..self.legend[i].len() {
                self.canvas.copy(&self.textures[self.legend[i][j].tile_state as usize], None, self.legend[i][j].rect).unwrap();
            }
        }

        self.canvas.present();
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(NAME, WIDTH, HEIGHT).position_centered().build().unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut stage: GameState = GameState {
        canvas: canvas,
        event_pump: sdl_context.event_pump().unwrap(),
        fps_manager: sdl2::gfx::framerate::FPSManager::new(),
        tiles: (0..HORIZ_TILE_NUM).map(|_| Vec::with_capacity(VERT_TILE_NUM)).collect::<Vec<Vec<_>>>(),
        legend:(0..HORIZ_TILE_NUM).map(|_| Vec::with_capacity(HORIZ_TILE_NUM - VERT_TILE_NUM)).collect::<Vec<Vec<_>>>(), 
        textures: Vec::new(),
        click_mode: ClickMode::Off,
    };

    println!("Loading assets...");
    let mut sur: sdl2::surface::Surface = sdl2::image::LoadSurface::from_file(std::path::Path::new("./src/assets/tile.png")).expect("Failed to open tile.png");
    stage.textures.push(texture_creator.create_texture_from_surface(sur.as_mut()).unwrap());
    sur = sdl2::image::LoadSurface::from_file(std::path::Path::new("./src/assets/house.png")).expect("Failed to open house.png");
    stage.textures.push(texture_creator.create_texture_from_surface(sur.as_mut()).unwrap());
    sur = sdl2::image::LoadSurface::from_file(std::path::Path::new("./src/assets/legendTile.png")).expect("Failed to open legendTile.png");
    stage.textures.push(texture_creator.create_texture_from_surface(sur.as_mut()).unwrap());


    for i in 0..stage.tiles.len() {
        for j in 0..VERT_TILE_NUM {
            stage.tiles[i].push(Tile {
                rect: sdl2::rect::Rect::new((i as f64 * (GAME_AREA_WIDTH as f64 / HORIZ_TILE_NUM as f64)).round() as i32, (j as f64 * (GAME_AREA_HEIGHT as f64 / VERT_TILE_NUM as f64)).round() as i32, (GAME_AREA_WIDTH as f64 / HORIZ_TILE_NUM as f64).round() as u32, (GAME_AREA_HEIGHT as f64 / VERT_TILE_NUM as f64).round() as u32),
                tile_state: TileState::Tile as u64,
            });
        }
    }
    for i in 0..stage.legend.len() {
        for j in 0..(HORIZ_TILE_NUM - VERT_TILE_NUM) {
            stage.legend[i].push(Tile {
                rect: sdl2::rect::Rect::new((i as f64 * (GAME_AREA_WIDTH as f64 / HORIZ_TILE_NUM as f64)).round() as i32, ((j as f64 * (GAME_AREA_HEIGHT as f64 / VERT_TILE_NUM as f64)).round() as i32) + GAME_AREA_HEIGHT as i32, (GAME_AREA_WIDTH as f64 / HORIZ_TILE_NUM as f64).round() as u32, (GAME_AREA_HEIGHT as f64 / VERT_TILE_NUM as f64).round() as u32),
                tile_state: TileState::Legend as u64,
            });
        }
    }

    stage.fps_manager.set_framerate(FPS).unwrap();
    println!("Starting game...");

    loop {
        let code = stage.handle_events();
        match code {
            Some(EventCode::Quit) => return,
            None => {},
        }

        stage.render();
        stage.fps_manager.delay();
    }
}
