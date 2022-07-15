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
                    <div class="playground__button" style="grid-column: 2/3; grid-row: 2/3">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                            <path d="M272 431.952v-73.798h128c26.51 0 48-21.49 48-48V201.846c0-26.51-21.49-48-48-48H272V80.057c0-42.638-51.731-64.15-81.941-33.941l-176 175.943c-18.745 18.745-18.746 49.137 0 67.882l176 175.952C220.208 496.042 272 474.675 272 431.952zM48 256L224 80v121.846h176v108.308H224V432L48 256z"/>
                        </svg>
                    </div>

                    <Minefield size={size} amount_bombs={amount_bombs} {on_bomb_click}/>

                    <div class="playground__button" style="grid-column: 2/3; grid-row: 4/5">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                            <path d="M16 132h416c8.837 0 16-7.163 16-16V76c0-8.837-7.163-16-16-16H16C7.163 60 0 67.163 0 76v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16z"/>
                        </svg>
                    </div>
                </div>
            </div>
        }
    }
}
