use self::red_hat_boy_states::*;
use crate::browser;
use crate::engine;
use crate::engine::{Game, KeyState, Point, Rect, Renderer};
use anyhow::Result;
use async_trait::async_trait;
use gloo_utils::format::JsValueSerdeExt;
use serde::Deserialize;
use std::collections::HashMap;
use web_sys::HtmlImageElement;

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

#[derive(Deserialize)]
struct SheetRect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}

pub struct WalkTheDog {
    image: Option<HtmlImageElement>,
    sheet: Option<Sheet>,
    frame: u8,
    position: Point,
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog {
            image: None,
            sheet: None,
            frame: 0,
            position: Point { x: 0, y: 0 },
        }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let sheet: Sheet = browser::fetch_json("rhb.json")
            .await
            .expect("Could not fetch rhb.json")
            .into_serde()
            .expect("Could not convert rhb.json into a Sheet structure");

        let image = engine::load_image("rhb.png")
            .await
            .expect("Could not load rhb.png");

        Ok(Box::new(WalkTheDog {
            image: Some(image),
            sheet: Some(sheet),
            frame: self.frame,
            position: self.position,
        }))
    }

    fn update(&mut self, keystate: &KeyState) {
        let mut velocity = Point { x: 0, y: 0 };
        if keystate.is_pressed("ArrowDown") {
            velocity.y += 3;
        }
        if keystate.is_pressed("ArrowUp") {
            velocity.y -= 3;
        }
        if keystate.is_pressed("ArrowRight") {
            velocity.x += 3;
        }
        if keystate.is_pressed("ArrowLeft") {
            velocity.x -= 3;
        }

        if self.position.x + velocity.x + 130 > 600 {
            self.position.x = 600 - 130;
        } else if self.position.x + velocity.x < 0 {
            self.position.x = 0;
        } else {
            self.position.x += velocity.x;
        }
        if self.position.y + velocity.y + 120 > 600 {
            self.position.y = 600 - 120;
        } else if self.position.y + velocity.y < 0 {
            self.position.y = 0;
        } else {
            self.position.y += velocity.y;
        }

        if self.frame < 23 {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
    }

    fn draw(&self, renderer: &Renderer) {
        let frame_name = format!("Run ({}).png", (self.frame / 3) + 1);
        let sprite = self
            .sheet
            .as_ref()
            .and_then(|sheet| sheet.frames.get(&frame_name))
            .expect("Cell not found");

        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });

        self.image.as_ref().map(|image| {
            renderer.draw_image(
                &image,
                &Rect {
                    x: sprite.frame.x.into(),
                    y: sprite.frame.y.into(),
                    width: sprite.frame.w.into(),
                    height: sprite.frame.h.into(),
                },
                &Rect {
                    x: self.position.x.into(),
                    y: self.position.y.into(),
                    width: sprite.frame.w.into(),
                    height: sprite.frame.h.into(),
                },
            );
        });
    }
}

struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: Sheet,
    image: HtmlImageElement,
}

#[derive(Copy, Clone)]
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
}

pub enum Event {
    Run,
}

impl RedHatBoyStateMachine {
    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (RedHatBoyStateMachine::Idle(state), Event::Run) => {
                RedHatBoyStateMachine::Running(state.run())
            }
            _ => self,
        }
    }
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
    }
}

mod red_hat_boy_states {
    use crate::engine::Point;

    #[derive(Copy, Clone)]
    pub struct Idle;
    #[derive(Copy, Clone)]
    pub struct Running;

    #[derive(Copy, Clone)]
    pub struct RedHatBoyState<S> {
        context: RedHatBoyContext,
        _state: S,
    }

    #[derive(Copy, Clone)]
    pub struct RedHatBoyContext {
        frame: u8,
        position: Point,
        velocity: Point,
    }

    impl RedHatBoyState<Idle> {
        pub fn run(self) -> RedHatBoyState<Running> {
            RedHatBoyState {
                context: self.context,
                _state: Running {},
            }
        }
    }
}
