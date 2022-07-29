
use yew::{html, Component, Html, Context};
use super::Playground;

#[derive(Debug)]
pub struct App {
    rows: usize,
    columns: usize,
    amount_bombs: usize,
}


pub enum Msg {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            amount_bombs: 9,
            columns: 8,
            rows: 10
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="app">
                <Playground rows={self.rows} columns={self.columns} amount_bombs={self.amount_bombs}/>
            </div>
        }
    }
}