
use yew::{html, Component, Html, Context};
use super::Playground;

#[derive(Debug)]
pub struct App {
    size: usize,
    amount_bombs: usize,
    game_state: GameState,
}

#[derive(Debug)]
enum GameState{
    Playing,
    GameOver,
    _Pausing
}

pub enum Msg {
    OnBombClick
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            amount_bombs: 13,
            size: 10,
            game_state: GameState::Playing
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::OnBombClick => self.game_state = GameState::GameOver,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_bomb_click = ctx.link().callback(move |_| {
            Msg::OnBombClick
        });


        html! {
            <>
                <Playground size={self.size} amount_bombs={self.amount_bombs} {on_bomb_click}/>
            </>
        }
    }
}