// Editor

Struct SideboxEditor {
    params: Arc<SideboxParams>,
}

impl SideboxEditor {
    fn new(params: Arc<SideboxParams>) -> Self {
        Self { params }
    }
}

impl egui::Widget for SideboxEditor {
    fn ui(&mut self, ui: &mut egui::Ui) {
        // Create sliders for each parameter
        ui.label("Input gain");
        ui.add(egui::Slider::f32(&mut self.params.input_gain.value, self.params.input_gain.range.clone()).text(""));

        ui.label("Sidechain input gain");
        ui.add(egui::Slider::f32(&mut self.params.sidechain_input_gain.value, self.params.sidechain_input_gain.range.clone()).text(""));

        ui.label("Output gain");
        ui.add(egui::Slider::f32(&mut self.params.output_gain.value, self.params.output_gain.range.clone()).text(""));

        // Add more UI elements for other parameters as needed
    }
}