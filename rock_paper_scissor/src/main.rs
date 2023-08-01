use raylib::prelude::*;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

struct Button {
    rect: Rectangle,
    choice: Choice,
    player: u8,
}

impl Button {
    fn new(x: i32, y: i32, width: i32, height: i32, choice: Choice,player_num:u8) -> Self {
        Self {
            rect: Rectangle::new(x as f32, y as f32, width as f32, height as f32),
            choice,
            player: player_num,
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(self.rect, Color::WHITE);
        let text = match &self.choice {
            Choice::Rock => "Rock",
            Choice::Paper => "Paper",
            Choice::Scissors => "Scissors",
        };
        let text_size = 20;
        let text_pos = Vector2::new(self.rect.x + 10.0, self.rect.y + 10.0);
        d.draw_text(text, text_pos.x as i32, text_pos.y as i32, text_size, Color::BLACK);
    }

    fn is_clicked(&self, d: &mut RaylibDrawHandle) -> bool {
        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse_pos = d.get_mouse_position();
            if self.rect.check_collision_point_rec(&mouse_pos) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Rock Paper Scissors")
        .build();

    let mut buttons = [
        Button::new(10, 10, 100, 50, Choice::Rock,0),
        Button::new(10, 70, 100, 50, Choice::Paper,0),
        Button::new(10, 130, 100, 50, Choice::Scissors,0),
        Button::new(690, 10, 100, 50, Choice::Rock,1),
        Button::new(690, 70, 100, 50, Choice::Paper,1),
        Button::new(690, 130, 100, 50, Choice::Scissors,1),
    ];

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(128, 128, 128, 255)); // Grey background

        for button in &mut buttons {
            button.draw(&mut d);
            if button.is_clicked(&mut d) {
                to_cairo(&button.choice);
            }
        }
    }
}

fn to_cairo(choice: &Choice) {
    // Insert your code here.
    match choice {
        Choice::Rock => println!("Rock button clicked"),
        Choice::Paper => println!("Paper button clicked"),
        Choice::Scissors => println!("Scissors button clicked"),
    }
}