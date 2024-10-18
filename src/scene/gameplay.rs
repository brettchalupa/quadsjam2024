use super::pause::Pause;
use super::Scene;
use crate::audio::play_sfx;
use crate::context::Context;
use crate::input::action_down;
use crate::input::action_pressed;
use crate::input::Action;
use crate::text::draw_text;
use crate::vec2::Vec2;
use macroquad::color::WHITE;
use macroquad::texture::draw_texture;

pub struct Gameplay {
    pause_subscene: Pause,
    pos: Vec2,
}

impl Scene for Gameplay {
    fn update(&mut self, ctx: &mut Context) {
        if self.pause_subscene.active {
            self.pause_subscene.update(ctx);
        } else if action_pressed(Action::Pause, &ctx.gamepads) {
            self.pause_subscene.active = true;
            play_sfx(ctx, &ctx.audio.sfx.menu_select);
        }

        if action_down(Action::Up, &ctx.gamepads) {
            self.pos.y -= 5;
        }
        if action_down(Action::Down, &ctx.gamepads) {
            self.pos.y += 5;
        }
        if action_down(Action::Right, &ctx.gamepads) {
            self.pos.x += 5;
        }
        if action_down(Action::Left, &ctx.gamepads) {
            self.pos.x -= 5;
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        if self.pause_subscene.active {
            self.pause_subscene.draw(ctx);
        } else {
            draw_texture(
                &ctx.textures.example,
                self.pos.x as f32,
                self.pos.y as f32,
                WHITE,
            );
            draw_text(
                ctx,
                "Gameplay!",
                100.,
                100.,
                crate::text::Size::Medium,
                WHITE,
            );
        }
    }
}

impl Gameplay {
    pub async fn new(ctx: &mut Context) -> Self {
        let pause_subscene = Pause::new(ctx);
        Self {
            pause_subscene,
            pos: Vec2 { x: 100, y: 100 },
        }
    }
}
