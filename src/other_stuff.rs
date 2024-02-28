pub struct SimpleEnvelopeFollower { // TODO: make the inputs vector a global circular queue
    current_value: f32,
    inputs: Vec<f32>,
    sum: f32,
    size: i32,
}

impl SimpleEnvelopeFollower {
    pub fn new(size: i32) -> Self {
        Self {
            current_value: 0.0,
            inputs: vec![0.0; size as usize],
            sum: 0.0,
            size,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        if self.inputs.len() == self.size as usize {
            let old_value = self.inputs.remove(0);
            self.sum -= old_value;
        }
        self.inputs.push(input);
        self.sum += input;
        self.current_value = self.sum / self.inputs.len() as f32;
        self.current_value
    }
}