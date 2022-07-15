use super::Minefield;

use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Debug)]
pub struct Playground {}

#[derive(Clone, PartialEq, Properties)]
pub struct PlaygroundProps {
    pub amount_bombs: usize,
    pub size: usize,
    pub on_bomb_click: Callback<()>,
}

impl Component for Playground {
    type Message = ();
    type Properties = PlaygroundProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let PlaygroundProps {
            size,
            amount_bombs,
            on_bomb_click,
        } = ctx.props().clone();

        html! {
            <div class="game">
                <div class="playground">
                    <Minefield size={size} amount_bombs={amount_bombs} {on_bomb_click}/>
                </div>
            </div>
        }
    }
}
