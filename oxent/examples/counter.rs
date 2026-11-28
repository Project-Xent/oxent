#![windows_subsystem = "windows"]

use oxent::{App, ButtonStyle, El, FlexAlign, FontWeight, StackDirection, Ui, Window};

#[derive(Clone)]
enum Msg {
    Inc,
    Dec,
    Reset,
}

#[derive(oxent::Model)]
struct Counter {
    n: i32,
}

impl App for Counter {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Inc => self.n += 1,
            Msg::Dec => self.n -= 1,
            Msg::Reset => self.n = 0,
        }
    }

    fn view<'ui>(&'ui self, ui: &'ui Ui<'ui, Self>) -> El<'ui, Self> {
        let buttons = ui
            .container()
            .dir(StackDirection::Row)
            .gap(8.0)
            .items(FlexAlign::Center)
            .child(ui.button().text("−").w(48.0).on_invoke(Msg::Dec))
            .child(ui.button().text("+").w(48.0).style(ButtonStyle::Accent).on_invoke(Msg::Inc))
            .child(ui.button().text("Reset").style(ButtonStyle::Subtle).on_invoke(Msg::Reset));
        ui.container()
            .dir(StackDirection::Column)
            .grow(1.0)
            .gap(16.0)
            .pl(32.0)
            .pt(32.0)
            .pr(32.0)
            .pb(32.0)
            .items(FlexAlign::Start)
            .child(ui.text().text("Counter").fontsize(28.0).fontweight(FontWeight::SemiBold))
            .child(ui.text().text(&format!("Value: {}", self.n)).fontsize(20.0))
            .child(buttons)
            .into()
    }
}

oxent::app!(Counter, Counter { n: 0 }, Window { title: "oxent counter", ..Default::default() });
