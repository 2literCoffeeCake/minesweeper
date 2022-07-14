use super::Field;
use crate::{mines::{Mine, MineState, Position}};
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Debug)]
pub struct Playground {
    minefield: Vec<Mine>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PlaygroundProps {
    pub amount_bombs: usize,
    pub size: usize,
    pub on_bomb_click: Callback<()>,
}

pub enum PlaygroundMsg {
    OnStateChange(Position, MineState),
}

impl Component for Playground {
    type Message = PlaygroundMsg;
    type Properties = PlaygroundProps;

    fn create(ctx: &Context<Self>) -> Self {
        let PlaygroundProps {
            size,
            amount_bombs,
            on_bomb_click: _,
        } = ctx.props().clone();
        Self {
            minefield: Mine::generate_mines(size, amount_bombs),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let PlaygroundProps {
            size: _,
            amount_bombs: _,
            on_bomb_click,
        } = ctx.props().clone();

        match msg {
            PlaygroundMsg::OnStateChange(pos, state) => {
                let mut minefield = self.minefield.clone();
                if let Some(mine) = minefield
                    .iter_mut()
                    .find(|mine| mine.get_position().equals(&pos))
                {
                    mine.set_state(state);
                    if state == MineState::Revealed{
                        if mine.is_bomb() {
                            on_bomb_click.emit(());
                        } else if mine.get_amount_neighbors() == 0 {
                            reveal_neighbors(pos, &mut minefield);
                        }
                    }
                }
                self.minefield = minefield;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let PlaygroundProps {
            size,
            amount_bombs: _,
            on_bomb_click: _,
        } = ctx.props().clone();

        let mines = self
            .minefield
            .clone()
            .into_iter()
            .map(|mine| {
                let on_click = ctx.link().callback(move |state: MineState| {
                    PlaygroundMsg::OnStateChange(mine.get_position(), state)
                });
                return html! {
                    <>
                        <Field mine={mine} {on_click}/>
                    </>
                };
            })
            .collect::<Html>();


        html! {
            <div class="minefield" style={format!("grid-template-columns: repeat({size}, 50px); grid-template-rows: repeat({size}, 50px);")}>
                { mines }
            </div>
        }
    }
}

fn reveal_neighbors(pos: Position, minefield: &mut Vec<Mine>){
    let n = pos.get_neighbors();
    let vec: Vec<&mut Mine> =  minefield.iter_mut().filter(|mine| n.iter().position(|p| p.equals(&mine.get_position())).is_some()).collect();
    for mine in vec{
        mine.set_state(MineState::Revealed);
    }
}