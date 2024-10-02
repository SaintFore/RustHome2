use eframe::egui;
mod logger; // 导入 logger 模块
use std::str::FromStr;
use chrono::Local; // 引入 chrono 库来获取当前时间
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))))
        .expect("Failed to run eframe native");
}

#[derive(Default)]
struct MyEguiApp {
    logger: logger::Logger,
    log_output: logger::LogOutput,
    input1: String,
    input2: String,
    result: String,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            logger: logger::Logger::default(),
            log_output: logger::LogOutput::File("log.txt".to_string()), // 初始化日志输出方式
            input1: String::new(),
            input2: String::new(),
            result: String::new(),
        }
    }

    fn calculate(&mut self, operator: &str) {
        let num1 = f64::from_str(&self.input1).unwrap_or(0.0);
        let num2 = f64::from_str(&self.input2).unwrap_or(0.0);
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => if num2 != 0.0 { num1 / num2 } else { 0.0 }, // 避免除以零
            _ => 0.0,
        };
        self.result = result.to_string();

        // 获取当前时间，并格式化为字符串
        let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_message = format!("{} - 计算: {} {} {} = {}", current_time, num1, operator, num2, self.result);

        // 记录日志
        self.logger.log(&log_message);
    }

    fn clear_data(&mut self) {
        // 清空输入和结果
        self.input1.clear();
        self.input2.clear();
        self.result.clear();

        // 记录清空操作的时间
        let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_message = format!("{} - 数据已清空", current_time);
        self.logger.log(&log_message);
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 创建字体定义
        let mut fonts = egui::FontDefinitions::default();
        
        // 使用系统字体
        fonts.font_data.insert("default".to_owned(), egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc"))); // 请根据你的系统路径更改
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "default".to_owned());

        // 应用字体定义
        ctx.set_fonts(fonts);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("简单计算器");

            // 选择日志输出方式
            ui.label("选择日志输出方式:");
            ui.horizontal(|ui| {
                // 控制台输出按钮
                let console_button_color = if matches!(self.log_output, logger::LogOutput::Console) {
                    egui::Color32::LIGHT_BLUE
                } else {
                    egui::Color32::LIGHT_GRAY
                };
                let file_button_color = if matches!(self.log_output, logger::LogOutput::File(_)) {
                    egui::Color32::LIGHT_BLUE
                } else {
                    egui::Color32::LIGHT_GRAY
                };

                if ui.add(egui::Button::new("控制台").fill(console_button_color)).clicked() {
                    self.log_output = logger::LogOutput::Console; // 更新输出方式
                    self.logger.set_output(logger::LogOutput::Console); // 更新 Logger 的输出方式
                }
                if ui.add(egui::Button::new("文件").fill(file_button_color)).clicked() {
                    self.log_output = logger::LogOutput::File("log.txt".to_string()); // 更新输出方式
                    self.logger.set_output(logger::LogOutput::File("log.txt".to_string())); // 更新 Logger 的输出方式
                }
            });

            // 输入和结果部分
            ui.horizontal(|ui| {
                ui.label("输入第一个数字:");
                ui.text_edit_singleline(&mut self.input1);
            });

            ui.horizontal(|ui| {
                ui.label("输入第二个数字:");
                ui.text_edit_singleline(&mut self.input2);
            });

            ui.horizontal(|ui| {
                if ui.button("+").clicked() {
                    self.calculate("+");
                }
                if ui.button("-").clicked() {
                    self.calculate("-"); 
                }
                if ui.button("*").clicked() {
                    self.calculate("*");
                }
                if ui.button("/").clicked() {
                    self.calculate("/");
                }
            });

            ui.horizontal(|ui| {
                ui.label("结果:");
                ui.label(&self.result);
            });

            // 添加清空按钮
            if ui.button("清空数据").clicked() {
                self.clear_data();
            }

            // 显示当前日志文件路径
            ui.label("日志保存路径: log.txt");
        });
    }
}
