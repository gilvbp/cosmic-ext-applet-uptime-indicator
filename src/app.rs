use std::time::Duration;

// SPDX-License-Identifier: GPL-3.0-only
use procfs::Uptime;
use cosmic::app::{Core, Task};
use cosmic::{Application, Element};
use cosmic::widget::{Row, Text, MouseArea};
use cosmic::iced::{time, Subscription, Alignment};


#[derive(Default)]
pub struct UptimeIndicator {
    core: Core,
    uptime: String,
    short_uptime:String
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

impl Application for UptimeIndicator {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "io.github.uptime-indicator";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = UptimeIndicator {
            core,
            uptime: calculate_uptime(),
            short_uptime: short_uptime(),
        };

        (app, Task::none())
    }


    fn view(&self) -> Element<Self::Message> {
        // 1) Row numa linha só, centralizando o Text
        let row = Row::new()
            .align_y(Alignment::Center)       // ✅ alinha verticalmente ao centro
            .push(
                Text::new(&self.short_uptime)
                    .size(14),
            );

        // 4) Se quiser clique:
        MouseArea::new(row)
            .on_press(Message::Tick)
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Self::Message> {
        match message {
            Message::Tick => {
                // recalcula e força o redraw
                self.uptime = calculate_uptime();
                self.short_uptime = short_uptime();
                Task::none()
            }
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(60)).map(|_| Message::Tick)
    }

}


fn calculate_uptime() -> String {
    // Obtém o uptime do sistema
    if let Ok(uptime) = Uptime::new() {
        let seconds = uptime.uptime as u64; // `uptime` retorna um f64, convertendo para u64
        format!(
            "{}h {}m {}s",
            seconds / 3600,
            (seconds % 3600) / 60,
            seconds % 60
        )
    } else {
        "Failed to retrieve uptime".into()
    }
}

fn short_uptime() -> String {
    if let Ok(uptime) = Uptime::new() {
        let total_minutes = uptime.uptime as u64 / 60;
        let days = total_minutes / 1440;
        let hours = (total_minutes % 1440) / 60;
        format!("{}D{:02}h", days, hours)
    } else {
        "N/A".into()
    }
}