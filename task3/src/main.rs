use eframe::{egui, App, CreationContext};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct MyApp {
    counter: Arc<Mutex<i32>>,
    progress: Arc<Mutex<Vec<f32>>>,
    thread_values: Arc<Mutex<Vec<i32>>>, // 新增字段存储每个线程的最终值
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            progress: Arc::new(Mutex::new(vec![0.0; 5])), // 减少线程数量
            thread_values: Arc::new(Mutex::new(vec![0; 5])), // 初始化每个线程的值为 0
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("启动线程").clicked() {
                let counter = Arc::clone(&self.counter);
                let progress = Arc::clone(&self.progress);
                let thread_values = Arc::clone(&self.thread_values);
                for i in 0..5 { // 减少线程数量
                    let counter = Arc::clone(&counter);
                    let progress = Arc::clone(&progress);
                    let thread_values = Arc::clone(&thread_values);
                    thread::spawn(move || {
                        for j in 0..=9 {
                            thread::sleep(Duration::from_millis(50)); // 增加间隔时间
                            let mut num = counter.lock().unwrap();
                            *num += 1;
                            let mut prog = progress.lock().unwrap();
                            prog[i] = j as f32 / 9.0;
                        }
                        // 更新每个线程的最终值
                        let mut values = thread_values.lock().unwrap();
                        values[i] = 10; // 每个线程循环 10 次，最终值为 10
                    });
                }
            }

            let progress = self.progress.lock().unwrap();
            for (i, &prog) in progress.iter().enumerate() {
                ui.label(format!("线程 {} 进度:", i + 1));
                ui.add(egui::ProgressBar::new(prog));
            }

            let counter = *self.counter.lock().unwrap();
            ui.label(format!("最终计数值: {}", counter));

            let thread_values = self.thread_values.lock().unwrap();
            for (i, &value) in thread_values.iter().enumerate() {
                ui.label(format!("线程 {} 最终值: {}", i + 1, value));
            }

            ctx.request_repaint(); // 确保界面重新绘制
        });
    }
}

fn main() {
    let app = MyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "多线程计数器",
        native_options,
        Box::new(|cc: &CreationContext| {
            // 设置自定义字体
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "my_font".to_owned(),
                egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")),
            );
            fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "my_font".to_owned());
            fonts.families.entry(egui::FontFamily::Monospace).or_default().push("my_font".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(app))
        }),
    ).unwrap();
}