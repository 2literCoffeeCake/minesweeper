
use yew::{html, Component, Context, Html, Properties};
use crate::mines::{Mine, MineState, Positition};
use super::Field;

#[derive(Debug)]
pub struct Playground {
    minefield: Vec<Mine>
}

#[derive(Clone, PartialEq, Properties)]
pub struct PlaygroundProps{
    pub amount_bombs: usize,
    pub size: usize,
}

pub enum PlaygroundMsg{
    OnStateChange(Positition, MineState)
}

impl Component for Playground {
    type Message = PlaygroundMsg;
    type Properties = PlaygroundProps;

    fn create(ctx: &Context<Self>) -> Self {
        let PlaygroundProps { size, amount_bombs } = ctx.props().clone();
        Self{
            minefield: Mine::generate_mines(size, amount_bombs)
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            PlaygroundMsg::OnStateChange(pos, state) => {
                let mut minefield = self.minefield.clone();
                if let Some(mine) = minefield.iter_mut().find(|mine| mine.get_position().equals(&pos)) {
                    mine.set_state(state);
                } 
                self.minefield = minefield;
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let PlaygroundProps { size, amount_bombs: _ } = ctx.props().clone();
        let mines = self.minefield.clone().into_iter().map(|mine| {
            let on_click = ctx.link().callback(move |state: MineState| {
                PlaygroundMsg::OnStateChange(mine.get_position(), state)
            });
            return html!{
                <>
                    <Field mine={mine} {on_click}/>
                </>
            };
        }).collect::<Html>();


        html! {
            <div class="minefield" style={format!("grid-template-columns: repeat({size}, 50px); grid-template-rows: repeat({size}, 50px);")}>
                { mines }
            </div>
        }
    }
}