use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

pub struct StringManagerOwned {
    strings: Vec<String>,
}
impl StringManagerOwned {
    fn new() -> Self {
        StringManagerOwned { strings: Vec::new() }
    }
    fn add(&mut self, string: String) {
        self.strings.push(string);
    }
    fn get_longest(&self) -> Option<&String> {
        self.strings.iter().max_by_key(|s| s.len())
    }
    fn clear(&mut self)
    {
        self.strings.clear();
    }
}
struct MyEguiApp {
    String1:String,
    manager:StringManagerOwned
}
impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            String1: "".to_owned(),
            manager:StringManagerOwned::new(),
        }
    }
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
impl eframe::App for MyEguiApp {

   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello!");
           ui.horizontal(|ui|{
            ui.label("The String you want to add:");
            ui.text_edit_singleline(&mut self.String1);
        });
        ui.horizontal(|ui|{
            if ui.button("add string").clicked(){
                self.manager.add(self.String1.clone());
             }
            if ui.button("clear").clicked(){
                self.manager.clear();
            }
        });
        if let Some(longest) = self.manager.get_longest() {
            ui.label(format!("The longest string is: {}", longest));
        } else {
        }
       });
       

   }
}
