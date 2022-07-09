
use yew::{html, Component, Context, Html, Properties, Callback};
use crate::{mines::{Mine, MineState, Positition}, browser_util::console_log};

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
                let info = format!("Pos: {:?}; State: {:?}", pos, state);
                let mut minefield = self.minefield.clone();

                if let Some(mine) = minefield.iter_mut().find(|mine| mine.get_position().equals(&pos)) {
                    mine.set_state(state);
                } 
                self.minefield = minefield;

                console_log(&info);
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
                    <UIMine mine={mine} {on_click}/>
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

#[derive(Debug)]
struct UIMine{
}

pub enum Msg {
    OnRightClick,
    OnLeftClick
}

#[derive(Clone, PartialEq, Properties)]
struct UIMineProps{
    mine: Mine,
    on_click: Callback<MineState>,
} 

impl Component for UIMine{
    type Message = Msg;

    type Properties = UIMineProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let UIMineProps { mine, on_click } = ctx.props().clone();
        match(msg, mine.get_state()){
            (Msg::OnRightClick, MineState::Unknown) => on_click.emit(MineState::Marked),
            (Msg::OnRightClick, MineState::Marked) => on_click.emit(MineState::Unknown),
            (Msg::OnRightClick, MineState::Revealed) => todo!(),
            (Msg::OnLeftClick, MineState::Unknown) => on_click.emit(MineState::Revealed),
            (Msg::OnLeftClick, MineState::Marked) => on_click.emit(MineState::Revealed),
            (Msg::OnLeftClick, MineState::Revealed) => todo!(),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let UIMineProps { mine, on_click: _ } = ctx.props().clone();
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

        let on_left_click = ctx.link().callback(|_| Msg::OnLeftClick);

        let on_right_click = ctx.link().callback(|_| Msg::OnRightClick);

        html! {
            <div class="mine" style={style} onclick={on_left_click} oncontextmenu={on_right_click}>
                {inner_html}
            </div>
        }
    }
}

