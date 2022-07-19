use super::Field;
use crate::mines::{Mine, MineState, Position};
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Debug)]
pub struct Minefield{
    mines: Vec<Mine>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub amount_bombs: usize,
    pub size: usize,
    pub on_bomb_click: Callback<()>,
}

pub enum Msg {
    OnStateChange(Position, MineState),
}

impl Component for Minefield {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let Props {
            size,
            amount_bombs,
            on_bomb_click: _,
        } = ctx.props().clone();
        Self {
            mines: Mine::generate_mines(size, amount_bombs),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Props {
            size: _,
            amount_bombs: _,
            on_bomb_click,
        } = ctx.props().clone();

        match msg {
            Msg::OnStateChange(pos, state) => {
                let mut minefield = self.mines.clone();
                if let Some(mine) = minefield
                    .iter_mut()
                    .find(|mine| mine.get_position().equals(&pos))
                {
                    mine.set_state(state);
                    if state == MineState::Revealed {
                        if mine.is_bomb() {
                            reveal_all(&mut minefield);
                            on_bomb_click.emit(());
                        } else if mine.get_number_of_neighboring_bombs() == 0 {
                            reveal_neighbors_outer(&pos, &mut minefield);
                        }
                    }
                }
                self.mines = minefield;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            size,
            amount_bombs: _,
            on_bomb_click: _,
        } = ctx.props().clone();

        let mines = self
            .mines
            .clone()
            .into_iter()
            .map(|mine| {
                let on_click = ctx.link().callback(move |state: MineState| {
                    Msg::OnStateChange(mine.get_position(), state)
                });
                return html! {
                    <>
                        <Field mine={mine} {on_click}/>
                    </>
                };
            })
            .collect::<Html>();

        html! {
            <>
                <div class="minefield" style={format!("grid-template-columns: repeat({size}, 50px); grid-template-rows: repeat({size}, 50px);")}>
                    { mines }
                </div>
            </>
        }
    }
}

fn reveal_neighbors_inner(pos: &Position, minefield: &mut Vec<Mine>) -> Vec<Position> {
    let neighbors = pos.get_neighbors();
    let mut tmp = Vec::<Position>::new();
    minefield
        .iter_mut()
        .filter(|mine| {
            neighbors
                .iter()
                .position(|p| p.equals(&mine.get_position()))
                .is_some()
        })
        .for_each(|mine| {
            if mine.get_state() != MineState::Revealed && mine.get_number_of_neighboring_bombs() == 0 {
                tmp.push(mine.get_position());
            }
            mine.reveal();

        });
    tmp
}

fn reveal_neighbors_outer(pos: &Position, minefield: &mut Vec<Mine>){
    let pos_vec = reveal_neighbors_inner(&pos, minefield);
    for inner_pos in pos_vec{
        let some_vec = reveal_neighbors_inner(&inner_pos, minefield);
        for outer_pos in some_vec{
            reveal_neighbors_outer(&outer_pos, minefield);
        }
    }
}

fn reveal_all(minefield: &mut Vec<Mine>){
    for mine in minefield{
        mine.reveal();
    }
}