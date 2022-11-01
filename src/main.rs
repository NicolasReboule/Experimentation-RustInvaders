// mod player;
// mod sprite;
// mod game_state;
use emerald::*;

pub struct MyGame {
    world: World,
    screen_center: Translation,
    background: ColorRect,
    rect: ColorRect,
    transform: Transform,
}

const MOVE_LEFT : &str = "left";
const MOVE_RIGHT : &str = "right";

impl Game for MyGame {
    fn initialize(&mut self, mut emd: Emerald) {
        emd.set_asset_folder_root(String::from("./assets/"));
        let mut spaceship = emd.loader()
        .sprite("spaceship.png")
        .unwrap();

        let mut invader = emd.loader().sprite("invaders.png").unwrap();

        spaceship.scale.x = 0.1;
        spaceship.scale.y = 0.1;
        emd.input().add_action_binding_key(&MOVE_LEFT.to_string(), KeyCode::A);
        emd.input().add_action_binding_key(&MOVE_RIGHT.to_string(), KeyCode::D);
        
        
        invader.offset.x = 50.0;
        invader.offset.y = 50.0;

        self.world.spawn((spaceship, Transform::from_translation((100.0, 100.0))));
        self.world.spawn((invader, Transform::from_translation((0.0, 0.0))));
    }
    
    fn update(&mut self, mut emd: Emerald) {
        let speed = 100.0 * emd.delta();
        let mouse = emd.input().mouse();
        let mut flash = Color::new(128, 128, 128, 128);
        let (width, height) = emd.screen_size();
        self.transform.translation.x = mouse.translation.x;
        self.transform.translation.y = mouse.translation.y;
        let mut color = Color::new(0, 0, 0, 255);

        for (entity_id, (transform, sprite)) in self.world.query::<(&mut Transform, &Sprite)>().iter() {
            if emd.input().is_action_pressed(&MOVE_LEFT.to_string()) {
                transform.translation.x -= speed;
            }
            if emd.input().is_action_pressed(&MOVE_RIGHT.to_string()) {
                transform.translation.x += speed;
            }
            if mouse.left.is_pressed {
                color.r = 255;
            }
            if mouse.left.is_just_pressed() {
                flash.r = 192;
            }
        }
        self.rect = ColorRect::new(color, 40, 40);

        self.screen_center = Translation::new(width / 2.0, height / 2.0);
        self.background = ColorRect::new(flash, width as u32, height as u32);

        // for (_, (transform, _)) in self.world.query::<(&mut Transform, &mut Sprite)>().iter() {
        //     // It's important to convert coordinates to the physical world space.
        //     *transform = self.transform - Transform::from_translation(self.screen_center);
        // }
    }

    fn draw(&mut self, mut emd: Emerald) {
        emd.graphics().begin().unwrap();

        emd.graphics()
        .draw_color_rect(
            &self.background,
            &Transform::from_translation(self.screen_center),
        )
        .ok();

        emd.graphics().draw_world(&mut self.world).unwrap();

        emd.graphics().render().unwrap();
    }
}


fn main() {
    let mut settings = GameSettings::default();
    let render_settings = RenderSettings {
        resolution: (1920, 1080),
        ..Default::default()
    };
    settings.render_settings = render_settings;
    emerald::start(
        Box::new(MyGame {
            rect: ColorRect::new(BLACK, 0, 0),
            transform: Transform::default(),
            background: ColorRect::new(BLACK, 0, 0),
            screen_center: Translation::default(),
            world: World::new(),
        }),
        settings,
    )
}