use super::{MainMenu, Playground};
use yew::{html, Component, Context, Html};

#[derive(Debug)]
pub struct App {
    state: AppState,
}

#[derive(Debug)]
pub enum AppState {
    Playing(usize, usize, usize),
    Menu,
}

pub enum Msg {
    SetState(AppState),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: AppState::Menu,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetState(state) => self.state = state,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let back_to_menu = ctx.link().callback(|_| Msg::SetState(AppState::Menu));
        let start_game = ctx.link().callback(|(rows, columns, amount_bombs)| {
            Msg::SetState(AppState::Playing(rows, columns, amount_bombs))
        });

        let inner = match self.state {
            AppState::Playing(rows, columns, amount_bombs) => html! {
                <>
                    <Playground {rows} {columns} {amount_bombs} {back_to_menu}/>
                </>
            },
            AppState::Menu => html! {
                <>
                    <MainMenu start_game={start_game}/>
                </>
            },
        };

        html! {
            <div class="app">
                {inner}
            </div>
        }
    }
}
