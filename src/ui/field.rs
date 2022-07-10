
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
            (Msg::OnLeftClick, MineState::Unknown) => on_click.emit(MineState::Revealed),
            (Msg::OnLeftClick, MineState::Marked) => on_click.emit(MineState::Revealed),
            (_, _) => {},
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
        let state = mine.get_state();
        if mine.is_bomb() && state == MineState::Revealed{
            inner_html = html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M420.7 68.7l-56 56-38.1-38.1c-12.5-12.5-32.8-12.5-45.3 0l-17.1 17.1c-17.9-5-36.8-7.7-56.3-7.7C96 96 3.3 185.9.1 297.8-3.3 415.5 91.1 512 208 512c115.1 0 208-94.2 208-208 0-19.5-2.7-38.4-7.7-56.3l17.1-17.1c12.5-12.5 12.5-32.8 0-45.3l-38.1-38.1 56-56-22.6-22.5zm-48.3 169.6c5.8 20.5 11.6 37.8 11.6 65.7 0 96.2-78.5 176-176 176-24.2 0-47.7-4.8-69.7-14.3-65.2-28.2-108.4-93.4-106.2-167C34.8 204.1 113.2 128 208 128c27.9 0 45.1 5.8 65.7 11.6l30.3-30.3 98.7 98.7s-1.8 1.9-30.3 30.3zM512 72c0 6.6-5.4 12-12 12h-24c-6.6 0-12-5.4-12-12s5.4-12 12-12h24c6.6 0 12 5.4 12 12zm-60-60v24c0 6.6-5.4 12-12 12s-12-5.4-12-12V12c0-6.6 5.4-12 12-12s12 5.4 12 12zm5 43c-4.7-4.7-4.7-12.3 0-17l17-17c4.7-4.7 12.3-4.7 17 0 4.7 4.7 4.7 12.3 0 17l-17 17c-4.7 4.7-12.3 4.7-17 0zm-67.9-16.9c-4.7-4.7-4.7-12.3 0-17 4.7-4.7 12.3-4.7 17 0l17 17c4.7 4.7 4.7 12.3 0 17-4.7 4.7-12.3 4.7-17 0l-17-17zm101.8 67.8c4.7 4.7 4.7 12.3 0 17-4.7 4.7-12.3 4.7-17 0l-17-17c-4.7-4.7-4.7-12.3 0-17 4.7-4.7 12.3-4.7 17 0l17 17zM192 192c0 8.8-7.2 16-16 16-35.3 0-64 28.7-64 64 0 8.8-7.2 16-16 16s-16-7.2-16-16c0-52.9 43.1-96 96-96 8.8 0 16 7.2 16 16z"/></svg>
            };
        } else if state == MineState::Marked {
            inner_html = html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M32 0C14.3 0 0 14.3 0 32v464c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V32C64 14.3 49.7 0 32 0zm430.6 4.2C291.3 91.5 305.4-62.2 96 32.4V384c185.7-92.2 221.7 53.3 397.5-23.1 11.4-5 18.5-16.5 18.5-28.8V30.8c0-25.1-26.8-38.1-49.4-26.6z"/></svg>
            };
        }
        let on_left_click = ctx.link().callback(|_| Msg::OnLeftClick);
        let on_right_click = ctx.link().callback(|_| Msg::OnRightClick);

        let mut classes = String::from("mine");
        if mine.get_state() == MineState::Revealed{
            classes.push_str(" mine--revealed")
        } else {
            classes.push_str(" mine--unrevealed")
        }


        html! {
            <div class={classes} style={style} onclick={on_left_click} oncontextmenu={on_right_click}>
                {inner_html}
            </div>
        }
    }
}