use firecore_battle_gui::pokedex::engine::{
    self,
    graphics::{self, Color, ScalingMode},
    gui::{MessageBox, Panel},
    text::{Message, MessagePage},
    utils::{Completable, Entity},
    Context, ContextBuilder, State,
};

fn main() {
    engine::run(
        ContextBuilder::new(
            "MessageBox",
            2 * engine::utils::WIDTH as i32,
            (2.0 * engine::utils::HEIGHT) as _,
        ),
        async {},
        move |_, _| {},
        |_, _| Game::new(),
    )
}

struct Game {
    messagebox: MessageBox,
}

impl Game {
    pub fn new() -> Self {
        Self {
            messagebox: MessageBox::new(Default::default(), 0),
        }
    }
}

impl State for Game {
    fn start(&mut self, ctx: &mut Context) {
        graphics::set_scaling_mode(ctx, ScalingMode::Stretch);

        //-> Result {
        let page = MessagePage {
            lines: vec![
                "Test Page Test Page".to_owned(),
                "Page Test Page Test".to_owned(),
            ],
            wait: None,
        };
        let page2 = MessagePage {
            lines: page.lines.clone(),
            wait: Some(1.0),
        };
        self.messagebox.message = Message {
            pages: vec![page, page2],
            color: Message::BLACK,
        };
        self.messagebox.spawn();
        // Ok(())
    }

    fn update(&mut self, ctx: &mut Context, delta: f32) {
        //-> Result {
        if !self.messagebox.alive() {
            ctx.quit();
        } else {
            self.messagebox.update(ctx, delta);
            if self.messagebox.finished() {
                self.messagebox.despawn();
            }
        }
        // Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) {
        //-> Result<(), ()> {
        graphics::clear(ctx, Color::rgb(0.1, 0.2, 0.56));
        Panel::draw(
            ctx,
            10.0,
            10.0,
            engine::utils::WIDTH - 20.0,
            engine::utils::HEIGHT - 20.0,
        );
        self.messagebox.draw(ctx);
        // Ok(())
    }
}
