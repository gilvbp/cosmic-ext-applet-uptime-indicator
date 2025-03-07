// SPDX-License-Identifier: GPL-3.0-only

use procfs::Uptime;
use cosmic::app::{Core, Task};
use cosmic::iced::platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::window::Id;
use cosmic::widget::{ Column, Text, MouseArea};
use cosmic::{Application, Element};

#[derive(Default)]
pub struct UptimeIndicator {
    core: Core,
    popup: Option<Id>,
    uptime: String,
    short_uptime:String
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
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
            popup: None,
            uptime: calculate_uptime(),
            short_uptime: short_uptime(),
        };

        (app, Task::none())
    }

    fn view(&self) -> Element<Self::Message> {
        let button = self
            .core
            .applet
            .icon_button((&self.short_uptime).as_ref()) // passe diretamente uma String
            .on_press(Message::TogglePopup);

        MouseArea::new(button).into()
    }





    fn view_window(&self, _id: Id) -> Element<Self::Message> {
        self.core
            .applet
            .popup_container(
                Column::new()
                    .push(Text::new("Uptime"))
                    .push(Text::new(&self.uptime))
                    .padding(10),
            )
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::TogglePopup => {
                self.uptime = calculate_uptime();
                self.short_uptime = short_uptime();

                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    get_popup(popup_settings)
                };
            }

        }

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