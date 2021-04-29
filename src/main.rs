use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        // TODO: fill this stub later
        ctx.cls();
        ctx.print(1, 1, "STATE: Menu");
        self.mode = GameMode::Playing;
    }
    fn playing(&mut self, ctx: &mut BTerm) {
        // TODO: fill this stub later
        ctx.print(1, 2, "STATE: Playing");
        self.mode = GameMode::End;

    }
    fn end_game(&mut self, ctx: &mut BTerm) {
        // TODO: fill this stub later
        ctx.print(1, 3, "STATE: End");

    }
}

// GameState trait cames from Bracket
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.playing(ctx),
            GameMode::End => self.end_game(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
