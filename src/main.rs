use eframe::{egui, App, Frame, NativeOptions};
use std::time::{Instant, Duration};
use rand::Rng;

#[derive(PartialEq)]
enum AppState {
    Loading(Instant),
    MainMenu,
    Snake,
    Pong,
    Breakout,
}

struct SnakeGame {
    body: Vec<(i32, i32)>,
    direction: (i32, i32),
    food: (i32, i32),
    last_move: Instant,
    move_interval: Duration,
    game_over: bool,
}

impl SnakeGame {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            body: vec![(10, 10), (10, 11), (10, 12)],
            direction: (0, -1),
            food: (rng.gen_range(5..30), rng.gen_range(5..30)),
            last_move: Instant::now(),
            move_interval: Duration::from_millis(200),
            game_over: false,
        }
    }

    fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        self.body = vec![(10, 10), (10, 11), (10, 12)];
        self.direction = (0, -1);
        self.food = (rng.gen_range(5..30), rng.gen_range(5..30));
        self.game_over = false;
    }

    fn update(&mut self, ctx: &egui::Context) {
    if self.game_over {
        return;
    }

    // Buffer the next direction from input
    let mut next_direction = self.direction;
    
    // Check all possible directions and buffer the last pressed valid one
    if ctx.input(|i| i.key_down(egui::Key::ArrowUp)) && self.direction != (0, 1) {
        next_direction = (0, -1);
    }
    if ctx.input(|i| i.key_down(egui::Key::ArrowDown)) && self.direction != (0, -1) {
        next_direction = (0, 1);
    }
    if ctx.input(|i| i.key_down(egui::Key::ArrowLeft)) && self.direction != (1, 0) {
        next_direction = (-1, 0);
    }
    if ctx.input(|i| i.key_down(egui::Key::ArrowRight)) && self.direction != (-1, 0) {
        next_direction = (1, 0);
    }

    if self.last_move.elapsed() >= self.move_interval {
        self.last_move = Instant::now();
        self.direction = next_direction; // Apply the buffered direction
        self.move_snake();
    }
}

    fn move_snake(&mut self) {
        let head = self.body[0];
        let new_head = (head.0 + self.direction.0, head.1 + self.direction.1);

        // Check for collisions
        if new_head.0 < 0 || new_head.0 >= 40 || new_head.1 < 0 || new_head.1 >= 30 {
            self.game_over = true;
            return;
        }

        if self.body[1..].contains(&new_head) {  // Skip checking the head
            self.game_over = true;
            return;
        }

        // Check for food
        if new_head == self.food {
            let mut rng = rand::thread_rng();
            self.food = (rng.gen_range(5..30), rng.gen_range(5..30));
            self.body.insert(0, new_head);
        } else {
            self.body.insert(0, new_head);
            self.body.pop();
        }
    }

    
    fn draw(&self, ui: &mut egui::Ui) {
        let painter = ui.painter();
        let rect = ui.available_rect_before_wrap();
        let progress = self.last_move.elapsed().as_secs_f32() / self.move_interval.as_secs_f32();
    let _progress = progress.min(1.0);

        painter.rect_stroke(
            rect,
            0.0,
            egui::Stroke::new(2.0, egui::Color32::GREEN),
        );

        let cell_size = (rect.width() / 40.0).min(rect.height() / 30.0);
        let offset_x = rect.center().x - (20.0 * cell_size);
        let offset_y = rect.center().y - (15.0 * cell_size);
        
        for (i, &segment) in self.body.iter().enumerate() {
            let color = if i == 0 { 
                egui::Color32::from_rgb(0, 200, 0)
            } else { 
                egui::Color32::from_rgb(0, 150, 0)
            };
            
            let x = offset_x + (segment.0 as f32 * cell_size);
            let y = offset_y + (segment.1 as f32 * cell_size);
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(x, y),
                    egui::vec2(cell_size, cell_size)
                ),
                0.0,
                color,
            );
        }
        
        let food_x = offset_x + (self.food.0 as f32 * cell_size);
        let food_y = offset_y + (self.food.1 as f32 * cell_size);
        painter.circle_filled(
            egui::pos2(food_x + cell_size/2.0, food_y + cell_size/2.0),
            cell_size/2.0,
            egui::Color32::RED,
        );
        
        painter.text(
            rect.min + egui::vec2(10.0, 10.0),
            egui::Align2::LEFT_TOP,
            format!("Score: {}", self.body.len() - 3),
            egui::FontId::monospace(20.0),
            egui::Color32::GREEN,
        );
        
        if self.game_over {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "GAME OVER\nPress ESC to return to menu",
                egui::FontId::monospace(30.0),
                egui::Color32::RED,
            );
        }
    }
}

struct PongGame;
impl PongGame {
    fn new() -> Self { Self }
    fn reset(&mut self) {}
    fn update(&mut self, _ctx: &egui::Context) {}
    fn draw(&self, ui: &mut egui::Ui) {
        ui.label("Pong game coming soon!");
    }
}

struct BreakoutGame;
impl BreakoutGame {
    fn new() -> Self { Self }
    fn reset(&mut self) {}
    fn update(&mut self, _ctx: &egui::Context) {}
    fn draw(&self, ui: &mut egui::Ui) {
        ui.label("Breakout game coming soon!");
    }
}

pub struct RetroApp {
    state: AppState,
    is_fullscreen: bool,
    snake: SnakeGame,
    pong: PongGame,
    breakout: BreakoutGame,
}

impl Default for RetroApp {
    fn default() -> Self {
        Self {
            state: AppState::Loading(Instant::now()),
            is_fullscreen: true,
            snake: SnakeGame::new(),
            pong: PongGame::new(),
            breakout: BreakoutGame::new(),
        }
    }
}

impl App for RetroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(0, 255, 0);
        ctx.set_visuals(visuals);
        
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            match self.state {
                AppState::MainMenu => {
                    self.is_fullscreen = !self.is_fullscreen;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.is_fullscreen));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(!self.is_fullscreen));
                }
                _ => self.state = AppState::MainMenu,
            }
        }

        match &mut self.state {
            AppState::Loading(start_time) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.spinner();
                        ui.label("\nLoading Retro VM...");
                        let progress = (start_time.elapsed().as_secs_f32() / 2.0).min(1.0);
                        ui.add(egui::ProgressBar::new(progress).text(format!(
                            "{}%",
                            (progress * 100.0) as u32
                        )));
                    });
                });

                if start_time.elapsed().as_secs_f32() > 2.0 {
                    self.state = AppState::MainMenu;
                }
            }
            AppState::MainMenu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("💿 Retro VM Game Collection");
                        ui.label("Pastel vibes ✨ meet hacker aesthetics 💻");
                        ui.separator();
                        
                        if ui.button("🐍 Snake").clicked() {
                            self.snake.reset();
                            self.state = AppState::Snake;
                        }
                        if ui.button("🎾 Pong").clicked() {
                            self.pong.reset();
                            self.state = AppState::Pong;
                        }
                        if ui.button("🧱 Breakout").clicked() {
                            self.breakout.reset();
                            self.state = AppState::Breakout;
                        }
                        
                        ui.separator();
                        ui.label(format!(
                            "Fullscreen: {} (ESC to toggle)",
                            if self.is_fullscreen { "ON" } else { "OFF" }
                        ));
                    });
                });
            }
            AppState::Snake => {
                self.snake.update(ctx);
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.snake.draw(ui);
                });
            }
            AppState::Pong => {
                self.pong.update(ctx);
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.pong.draw(ui);
                });
            }
            AppState::Breakout => {
                self.breakout.update(ctx);
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.breakout.draw(ui);
                });
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_decorations(false)
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Retro VM",
        native_options,
        Box::new(|_cc| Ok(Box::new(RetroApp::default()))),
    )
}