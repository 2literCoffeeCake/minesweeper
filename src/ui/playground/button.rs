use yew::{html, Callback, Component, Context, Html, Properties, Children};

#[derive(Debug)]
pub struct Button;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    pub row: usize,
    pub on_click: Callback<()>,
}

#[derive(PartialEq, Eq)]
pub enum Msg {
    Click
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Props {
            on_click,
            row: _,
            children: _
        } = ctx.props().clone();
        if msg == Msg::Click {
            on_click.emit(());
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            row,
            children,
            on_click: _
        } = ctx.props().clone();

        let on_button_click = ctx.link().callback(|_| Msg::Click);
        let style = format!("grid-column: 2/3; grid-row: {}/{}", row, row + 1);

        html! {
            <div class="playground__button" style={style} onclick={on_button_click}>
                { for children.iter() }
            </div>
        }
    }
}