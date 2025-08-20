use chrono::prelude::*;
use chrono_tz::Tz;
use eframe::egui::{self, Color32, Pos2, Shape, Stroke, Vec2};

struct MarketClockApp;

impl eframe::App for MarketClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("🌐 Global Market Clocks");
            });

            ui.horizontal_centered(|ui| {
                draw_market_clock(ui, "Tokyo", chrono_tz::Asia::Tokyo, "Opens at 9:00 AM JST");
                draw_market_clock(ui, "London", chrono_tz::Europe::London, "Opens at 8:00 AM GMT");
                draw_market_clock(ui, "New York", chrono_tz::America::New_York, "Opens at 9:30 AM EST");
            });

            ctx.request_repaint(); // Keep updating
        });
    }
}

fn draw_market_clock(ui: &mut egui::Ui, label: &str, tz: Tz, open_time: &str) {
    ui.vertical(|ui| {
        ui.label(label);
        let (rect, _response) = ui.allocate_exact_size(Vec2::splat(140.0), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        let center = rect.center();
        let radius = rect.width().min(rect.height()) / 2.0 - 10.0;

        // Draw clock circle
        painter.circle_stroke(center, radius, Stroke::new(2.0, Color32::WHITE));

        // Draw hour digits
        for hour in 1..=12 {
            let angle = std::f32::consts::TAU * (hour as f32 / 12.0) - std::f32::consts::FRAC_PI_2;
            let pos = center + Vec2::angled(angle) * (radius - 15.0);
            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                hour.to_string(),
                egui::TextStyle::Body.resolve(ui.style()),
                Color32::WHITE,
            );
        }

        // Get current time
        let now = Utc::now().with_timezone(&tz);
        let hour = now.hour() % 12;
        let minute = now.minute();
        let second = now.second();

        let hour_angle = std::f32::consts::TAU * ((hour as f32 + minute as f32 / 60.0) / 12.0) - std::f32::consts::FRAC_PI_2;
        let minute_angle = std::f32::consts::TAU * (minute as f32 / 60.0) - std::f32::consts::FRAC_PI_2;
        let second_angle = std::f32::consts::TAU * (second as f32 / 60.0) - std::f32::consts::FRAC_PI_2;

        draw_hand(&painter, center, radius * 0.5, hour_angle, Stroke::new(4.0, Color32::WHITE));
        draw_hand(&painter, center, radius * 0.7, minute_angle, Stroke::new(2.0, Color32::LIGHT_BLUE));
        draw_hand(&painter, center, radius * 0.9, second_angle, Stroke::new(1.0, Color32::RED));

        ui.label(open_time);
    });
}

fn draw_hand(painter: &egui::Painter, center: Pos2, length: f32, angle: f32, stroke: Stroke) {
    let dir = Vec2::angled(angle);
    let tip = center + dir * length;
    painter.line_segment([center, tip], stroke);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Market Analog Clocks", options, Box::new(|_cc| Box::new(MarketClockApp)))
}
