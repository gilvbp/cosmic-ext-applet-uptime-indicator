// SPDX-License-Identifier: GPL-3.0-only

mod app;

use app::UptimeIndicator;

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<UptimeIndicator>(())
}
