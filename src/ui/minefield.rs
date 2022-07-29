use super::Field;
use crate::mines::{Mine, MineState, MineVec, Position};
use uuid::Uuid;
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Debug)]
pub struct Minefield {
    mines: Vec<Mine>,
    game_id: Uuid,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub game_id: Uuid,
    
    pub rows: usize,
    pub columns: usize,
    pub amount_bombs: usize,

    pub on_bomb_click: Callback<()>,
    pub on_game_win: Callback<()>,    
}

pub enum Msg {
    OnStateChange(Position, MineState),
}

impl Component for Minefield {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let Props {
            rows,
            columns,
            amount_bombs,
            on_bomb_click: _,
            game_id,
            on_game_win: _,
        } = ctx.props().clone();
        Self {
            mines: Mine::generate_mines(rows, columns, amount_bombs),
            game_id,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let on_bomb_click = ctx.props().clone().on_bomb_click;
        let on_game_win = ctx.props().clone().on_game_win;

        match msg {
            Msg::OnStateChange(pos, state) => {
                let mut minefield = self.mines.clone();
                if let Some(mine) = minefield.get_by_pos(&pos) {
                    mine.set_state(state);
                    if state == MineState::Revealed {
                        if mine.is_bomb() {
                            minefield.reveal_all();
                            on_bomb_click.emit(());
                        } else if mine.has_no_neighbors() {
                            minefield.reveal_neighbors(&pos);
                        }
                    }
                }
                self.mines = minefield;
            }
        }
        if self.check_if_user_wins(ctx){
            on_game_win.emit(());
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let Props {
            rows,
            columns,
            amount_bombs,
            on_bomb_click: _,
            game_id,
            on_game_win: _,
        } = ctx.props().clone();

        if self.game_id != game_id {
            self.mines = Mine::generate_mines(rows, columns, amount_bombs);
            self.game_id = game_id;
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            rows,
            columns,
            amount_bombs: _,
            on_bomb_click: _,
            game_id: _,
            on_game_win: _,
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
                <div class="minefield" style={format!("grid-template-columns: repeat({columns}, 50px); grid-template-rows: repeat({rows}, 50px);")}>
                    { mines }
                </div>
            </>
        }
    }
}

impl Minefield {
    fn check_if_user_wins(&self, ctx: &Context<Self>) -> bool {
        let Props {
            rows,
            columns,
            amount_bombs,
            on_bomb_click: _,
            on_game_win: _,
            game_id: _,
        } = ctx.props().clone();

        let amount_revealed_mines = self
            .mines
            .clone()
            .into_iter()
            .filter(|mine| mine.get_state() == MineState::Revealed)
            .count();
        ((rows * columns) - amount_bombs) == amount_revealed_mines
    }
}
