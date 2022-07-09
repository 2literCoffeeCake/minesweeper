
use yew::{html, Component, Context, Html, Properties, Callback};
use crate::mines::{Mine, MineState};

#[derive(Debug)]
pub struct Field{
}

pub enum Msg {
    OnRightClick,
    OnLeftClick
}

#[derive(Clone, PartialEq, Properties)]
pub struct FieldProps{
    pub mine: Mine,
    pub on_click: Callback<MineState>,
} 

impl Component for Field{
    type Message = Msg;

    type Properties = FieldProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let FieldProps { mine, on_click } = ctx.props().clone();
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
        let FieldProps { mine, on_click: _ } = ctx.props().clone();
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
            <div class="field" style={style} onclick={on_left_click} oncontextmenu={on_right_click}>
                {inner_html}
            </div>
        }
    }
}