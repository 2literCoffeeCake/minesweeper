
use yew::{html, Component, Context, Html, Properties};
use crate::mines::Mine;

#[derive(Debug)]
pub struct Playground {
    minefield: Vec<Mine>
}

#[derive(Clone, PartialEq, Properties)]
pub struct PlaygroundProps{
    pub amount_bombs: usize,
    pub size: usize,
}

pub enum Msg {
}

impl Component for Playground {
    type Message = Msg;
    type Properties = PlaygroundProps;

    fn create(ctx: &Context<Self>) -> Self {
        let PlaygroundProps { size, amount_bombs } = ctx.props().clone();
        Self{
            minefield: Mine::generate_mines(size, amount_bombs)
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let PlaygroundProps { size, amount_bombs: _ } = ctx.props().clone();


        html! {
            <div class="minefield" style={format!("grid-template-columns: repeat({size}, 50px); grid-template-rows: repeat({size}, 50px);")}>
            {
                self.minefield.clone().into_iter().map(|mine| {
                    html!{<><UIMine mine={mine}/></>}
                }).collect::<Html>()
            }
            </div>
        }
    }
}

#[derive(Debug)]
struct UIMine{
}


#[derive(Clone, PartialEq, Properties)]
struct UIMineProps{
    mine: Mine
} 

impl Component for UIMine{
    type Message = ();

    type Properties = UIMineProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let UIMineProps { mine } = ctx.props().clone();
        let pos = mine.get_position();
        let column = pos.get_column();
        let row = pos.get_row();
        let style = format!("grid-column: {}/{}; grid-row: {}/{};", column + 1, column + 2, row + 1, row + 2);
        let mut inner_html = html!{
            <></>
        };
        if mine.is_bomb() {
            inner_html = html!{
                <>{"B"}</>
            };
        }


        html! {
            <div class="mine" style={style}>
                {inner_html}
            </div>
        }
    }
}

