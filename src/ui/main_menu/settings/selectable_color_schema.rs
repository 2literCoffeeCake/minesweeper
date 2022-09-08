use yew::{Callback, Properties, Component, Context, Html, html};

use crate::mines::Mine;
use crate::ui::playground::Field;

use super::ColorSchema;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub row: usize,
    pub column: usize,
    pub color_schema: ColorSchema,
    pub active: bool,
    pub on_click: Callback<ColorSchema>,
}

pub enum Msg{
    OnSchemeChange
}
pub struct SelectableColorSchema;

impl Component for SelectableColorSchema{
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Props { on_click, active: _, color_schema, column: _, row: _ } = ctx.props().clone();
        match msg{
            Msg::OnSchemeChange => {
                on_click.emit(color_schema)
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let Props {
            row,
            column,
            color_schema,
            active,
            on_click: _
        } = ctx.props().clone();

        let mines = Mine::generate_preview()
        .into_iter()
        .map(|mine| {
            let on_click = ctx.link().callback(move |_| Msg::OnSchemeChange);
            return html! {
                <>
                    <Field mine={mine} {on_click}/>
                </>
            };
        })
        .collect::<Html>();

        let on_click = ctx.link().callback(|_| Msg::OnSchemeChange);

        html!{
            <div class={get_classes(color_schema, active)} style={format!("grid-row: {}/{}; grid-column: {}/{}", row + 1, row + 2, column + 1, column + 2)} onclick={on_click}>
                <div>
                    {mines}
                </div>
            </div>
        }
    }
}

fn get_classes(color_schema: ColorSchema, is_active: bool) -> String{
    let mut classes = color_schema.to_string();
    classes.push(' ');
    classes.push_str("main_menu__settings__color_selection");
    classes.push(' ');
    classes.push_str("main_menu__settings__color_selection");
    if is_active{
        classes.push_str("--active");
    } else {
        classes.push_str("--inactive");
    }

    classes
}