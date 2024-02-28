pub struct EnvelopeFollower {
    current_value: f32,
    time: f32,
}

impl EnvelopeFollower {
    pub fn new() -> Self {
        Self {
            current_value: 0.5,
            time: 50.0, // not measured in any specific unit
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let abs_input = input.abs();
        self.current_value += self.time * (abs_input - self.current_value);
        self.current_value
    }

    pub fn update_time(&mut self, new_time: f32) {
        self.time = new_time;
    }
}