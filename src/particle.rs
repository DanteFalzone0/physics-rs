use rand::Rng;

#[derive (Copy, Clone)]
pub struct Particle {
    pub x_pos:      f32,
    pub y_pos:      f32,
    pub x_momentum: f32,
    pub y_momentum: f32,
}


pub fn new() -> Particle {
    Particle {    
        x_pos: 0.0,
        y_pos: 0.0,
        x_momentum: 0.0,
        y_momentum: 0.0 
    }
}


impl Particle {
    pub fn update(
        &mut self,
        grav_x: i32,
        grav_y: i32,
        space_drag: f32,
        wall_bounciness: f32,
        window_width: u32,
        window_height: u32,
        gravity: f32
    ) {    
        self.x_pos += self.x_momentum;
        self.y_pos += self.y_momentum;
        self.x_momentum *= 1.0 - space_drag;
        self.y_momentum *= 1.0 - space_drag;

        if self.x_pos < 0.0 {
            self.x_pos = 0.0;
            self.x_momentum *= -1.0 * wall_bounciness;
        }

        if self.x_pos > window_width as f32 {
            self.x_pos = window_width as f32;
            self.x_momentum *= -1.0 * wall_bounciness;
        }

        if self.y_pos < 0.0 {
            self.y_pos = 0.0;
            self.y_momentum *= -1.0 * wall_bounciness;
        }

        if self.y_pos > window_height as f32 {
            self.y_pos = window_width as f32;
            self.y_momentum *= -1.0 * wall_bounciness;
        }

        let y_diff = self.y_pos - (grav_y as f32);
        let x_diff = self.x_pos - (grav_x as f32);
        let hypot = x_diff.hypot(y_diff); /* very convenient! */
        self.x_momentum -= gravity * (x_diff / hypot);
        self.y_momentum -= gravity * (y_diff / hypot);
    }

    pub fn collide_with(
        &mut self,
        other: Particle
    ) {
        self.x_momentum = other.y_momentum;
        self.y_momentum = other.x_momentum;

        let mut rng = rand::thread_rng();
        self.x_pos += rng.gen_range(-1, 2) as f32;
        self.y_pos += rng.gen_range(-1, 2) as f32;
    }
}
