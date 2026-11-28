#![windows_subsystem = "windows"]

use oxent::{App, El, FlexAlign, FontWeight, ListSelMode, StackDirection, Ui, Window};

#[derive(Clone)]
enum Msg {
    Select(i32),
}

#[derive(oxent::Model)]
struct Rows {
    selected: i32,
}

fn row<'a>(ui: &'a Ui<'a, Rows>, i: usize) -> El<'a, Rows> {
    ui.container()
        .dir(StackDirection::Row)
        .gap(12.0)
        .items(FlexAlign::Center)
        .child(ui.text().text(&format!("{i}")).w(72.0).fontsize(12.0))
        .child(ui.text().text(&format!("Item number {i}")).fontsize(14.0))
        .into()
}

impl App for Rows {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Select(i) => self.selected = i,
        }
    }

    fn view<'ui>(&'ui self, ui: &'ui Ui<'ui, Self>) -> El<'ui, Self> {
        let status = match self.selected {
            i if i >= 0 => format!("Selected row: {i}"),
            _ => "Nothing selected yet.".to_owned(),
        };
        ui.container()
            .dir(StackDirection::Column)
            .grow(1.0)
            .gap(12.0)
            .pl(32.0)
            .pt(32.0)
            .pr(32.0)
            .pb(32.0)
            .child(ui.text().text("100,000 rows").fontsize(28.0).fontweight(FontWeight::SemiBold))
            .child(ui.text().text(&status))
            .child(
                ui.list()
                    .grow(1.0)
                    .itemh(40.0)
                    .mode(ListSelMode::Single)
                    .sel(self.selected)
                    .on_select(Msg::Select)
                    .items(100_000, row),
            )
            .into()
    }
}

oxent::app!(Rows, Rows { selected: -1 }, Window { title: "oxent virtual list", ..Default::default() });
