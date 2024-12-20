use eframe::{egui, App, CreationContext};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use rand::seq::SliceRandom;

struct MyApp {
    counter: Arc<Mutex<i32>>,
    progress: Arc<Mutex<Vec<f32>>>,
    thread_values: Arc<Mutex<Vec<i32>>>,
    quotes: Arc<Mutex<Vec<String>>>,
    threads_running: bool,                // 新增：线程运行标志
    completed_threads: Arc<AtomicUsize>,  // 新增：已完成线程计数
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            progress: Arc::new(Mutex::new(vec![0.0; 5])),
            thread_values: Arc::new(Mutex::new(vec![0; 5])),
            quotes: Arc::new(Mutex::new(vec!["".to_string(); 5])),
            threads_running: false,
            completed_threads: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("启动线程").clicked() {
                if !self.threads_running {
                    self.threads_running = true;
                    self.completed_threads.store(0, Ordering::SeqCst);

                    // 重置进度和数据
                    {
                        let mut prog = self.progress.lock().unwrap();
                        for p in prog.iter_mut() {
                            *p = 0.0;
                        }
                        let mut values = self.thread_values.lock().unwrap();
                        for v in values.iter_mut() {
                            *v = 0;
                        }
                        let mut qs = self.quotes.lock().unwrap();
                        for q in qs.iter_mut() {
                            *q = "".to_string();
                        }
                        let mut cnt = self.counter.lock().unwrap();
                        *cnt = 0;
                    }

                    let counter = Arc::clone(&self.counter);
                    let progress = Arc::clone(&self.progress);
                    let thread_values = Arc::clone(&self.thread_values);
                    let quotes = Arc::clone(&self.quotes);
                    let completed_threads = Arc::clone(&self.completed_threads);

                    let famous_quotes = vec![
                        "封侯非我愿，但愿海波平。——戚继光",
                        "砍头不要紧，只要主义真。杀了夏明翰，还有后来人。——夏明翰",
                        "威武不能挫其气，利禄不能动其心。——李大钊",
                        "人生只有一生一死，要生的有意义，死的有价值。——邓中夏",
                        "热的心会把冰雪溶消。——《烈士诗抄》",
                        "死里逃生唯斗争，铁窗难锁钢铁心。——王若飞",
                        "清贫、洁白、朴素的生活，正是我们革命者能够战胜许多困难的地方。——方志敏",
                        "忠诚印寸心，浩然充两间。——蔡和森",
                        "吾愿吾亲爱之青年，生于青春，死于青春。——李大钊",
                        "人生应该如蜡烛一样，从顶燃到底，一直都是光明的。——萧楚女",
                    ];

                    for i in 0..5 {
                        let counter = Arc::clone(&counter);
                        let progress = Arc::clone(&progress);
                        let thread_values = Arc::clone(&thread_values);
                        let quotes = Arc::clone(&quotes);
                        let completed_threads = Arc::clone(&completed_threads);
                        let quotes_list = famous_quotes.clone();
                        thread::spawn(move || {
                            // 生成随机名言
                            let mut rng = rand::thread_rng();
                            let quote = quotes_list.choose(&mut rng).unwrap().to_string();
                            // 存储名言
                            {
                                let mut qs = quotes.lock().unwrap();
                                qs[i] = quote;
                            }
                            for j in 0..=9 {
                                thread::sleep(Duration::from_millis(50));
                                {
                                    let mut num = counter.lock().unwrap();
                                    *num += 1;
                                }
                                {
                                    let mut prog = progress.lock().unwrap();
                                    prog[i] = j as f32 / 9.0;
                                }
                            }
                            {
                                let mut values = thread_values.lock().unwrap();
                                values[i] = 10;
                            }
                            // 增加已完成线程计数
                            completed_threads.fetch_add(1, Ordering::SeqCst);
                        });
                    }
                } else {
                    ui.label("线程已在运行，请稍候...");
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

            let quotes = self.quotes.lock().unwrap();
            for (i, quote) in quotes.iter().enumerate() {
                ui.label(format!("线程 {} 名人名言: {}", i + 1, quote));
            }

            // 检查是否所有线程已完成
            if self.completed_threads.load(Ordering::SeqCst) == 5 {
                self.threads_running = false;
            }

            ctx.request_repaint();
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
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "my_font".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("my_font".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(app))
        }),
    )
    .unwrap();
}