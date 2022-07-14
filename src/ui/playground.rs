use super::Field;
use crate::mines::{Mine, MineState, Position};
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
                    if state == MineState::Revealed {
                        if mine.is_bomb() {
                            on_bomb_click.emit(());
                        } else if mine.get_number_of_neighboring_bombs() == 0 {
                            some_outer_function(&pos, &mut minefield);
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

fn reveal_neighbors(pos: &Position, minefield: &mut Vec<Mine>) -> Vec<Position> {
    let neighbors = get_neighbors(pos);
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

fn some_outer_function(pos: &Position, minefield: &mut Vec<Mine>){
    let pos_vec = reveal_neighbors(&pos, minefield);
    for inner_pos in pos_vec{
        let some_vec = reveal_neighbors(&inner_pos, minefield);
        for outer_pos in some_vec{
            some_outer_function(&outer_pos, minefield);
        }
    }
}

fn get_neighbors(pos: &Position) -> Vec<Position>{
    let mut result = Vec::new();
    let row = pos.get_row() as i8;
    let column = pos.get_column() as i8;
    for delta_row in -1..=1 {
        for delta_column in -1..=1 {
            if !compare_i8(delta_column, delta_row) {
                let tmp_row = row + delta_row;
                let tmp_column = column + delta_column;
                if tmp_column >= 0 && tmp_row >= 0 {
                    result.push(Position::new().column(tmp_column as usize).row(tmp_row as usize).build());
                }
            }
        }   
    }
    result
}

fn compare_i8(a: i8, b: i8) -> bool{
    (a*a) == (b*b)
}