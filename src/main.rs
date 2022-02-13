use nannou::image;
use nannou::prelude::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

struct Tile {
    texture: wgpu::Texture,
}

struct Model {
    tiles: Vec<Tile>,
    board: Vec<usize>,
}

impl Model {
    const ROWS: usize = 4;
    const COLS: usize = 4;

    fn new(mut image: image::DynamicImage, app: &App) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        for r in 0..Self::ROWS {
            for c in 0..Self::COLS {
                let x = c as u32 * 100;
                let y = r as u32 * 100;
                let img = image.crop(x, y, 100, 100);
                let texture = wgpu::Texture::from_image(app, &img);
                tiles.push(Tile { texture })
            }
        }
        let mut board: Vec<usize> = Vec::new();
        for i in 0..16 {
            board.push(i);
        }
        Self { tiles, board }
    }
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.board.shuffle(&mut rng);
    }
    fn tap(&mut self, r: usize, c: usize) {
        let i = r * 4 + c;
        let j = self.board.iter().position(|x| x == &0).unwrap();
        let r2 = j / 4;
        let c2 = j % 4;
        if (r == r2 && c2 > 0 && c2 - 1 == c)
            || (r == r2 && c2 < 3 && c2 + 1 == c)
            || (c == c2 && r2 > 0 && r2 - 1 == r)
            || (c == c2 && r2 < 3 && r2 + 1 == r)
        {
            let v = self.board[i];
            self.board[i] = 0;
            self.board[j] = v;
        }
    }
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(400, 400)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    let assets = app.assets_path().unwrap();
    let img_path = assets.join("choochoobot.png");
    let image = image::open(img_path).unwrap();
    let mut m = Model::new(image, app);
    m.shuffle();
    m
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    let c = ((app.mouse.x + 200.0) / 100.0) as usize;
    let r = ((200.0 - app.mouse.y) / 100.0) as usize;
    model.tap(r, c);
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let win = app.window_rect();
    let draw = app.draw();
    for (j, &i) in model.board.iter().enumerate() {
        let r = j / 4;
        let c = j % 4;
        let x = 100.0 * c as f32 - 150.0;
        let y = 300.0 - 100.0 * r as f32 - 150.0;
        if i != 0 {
            let texture = &model.tiles[i].texture;
            let r = Rect::from_w_h(100.0, 100.0).top_left_of(win);

            draw.texture(texture).x_y(x, y).wh(r.wh());
        }
        draw.polyline().weight(3.0).points([
            pt2(x - 50.0, y - 50.0),
            pt2(x + 150.0, y - 50.0),
            pt2(x + 150.0, y + 150.0),
            pt2(x - 50.0, y + 150.0),
            pt2(x - 50.0, y - 50.0),
        ]);
        draw.to_frame(app, &frame).unwrap();
    }
}
