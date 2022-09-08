use yew::{Component, html, Context, Callback, Properties};
use crate::browser_util::{get_body, get_item_from_client_storage, save_item_to_client_storage};

use super::{ColorSchema, SelectableColorSchema};

#[derive(Clone, PartialEq, Properties)]
pub struct Props{
    pub back_to_main_menu: Callback<()>
}

pub struct SettingsMenu{
    color_schema: ColorSchema
}

pub enum Msg{
    ChangeSchema(ColorSchema),
    BackToMainMenu
}

impl Component for SettingsMenu{
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let color_schema = get_current_color_schema();
        Self{
            color_schema
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let back_to_main_menu  = ctx.props().clone().back_to_main_menu;
        match msg{
            Msg::ChangeSchema(schema) => {
                update_color_schema(&schema);
                self.color_schema = schema;
            },
            Msg::BackToMainMenu => {
                back_to_main_menu.emit(());
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let on_bluegray_click = ctx.link().callback(move |_| Msg::ChangeSchema(ColorSchema::BlueGray));
        let on_blue_click = ctx.link().callback(move |_| Msg::ChangeSchema(ColorSchema::Blue));
        let on_amber_click = ctx.link().callback(move |_| Msg::ChangeSchema(ColorSchema::Amber));
        let on_teal_click = ctx.link().callback(move |_| Msg::ChangeSchema(ColorSchema::Teal));

        let back_to_main_menu = ctx.link().callback(move |_| Msg::BackToMainMenu);

        html!{
            <div class="main_menu__settings">
                <SelectableColorSchema row={0} column={0} color_schema={ColorSchema::BlueGray} active={self.color_schema == ColorSchema::BlueGray} on_click={on_bluegray_click}/>
                <SelectableColorSchema row={0} column={1} color_schema={ColorSchema::Blue} active={self.color_schema == ColorSchema::Blue} on_click={on_blue_click}/>
                <SelectableColorSchema row={1} column={0} color_schema={ColorSchema::Amber} active={self.color_schema == ColorSchema::Amber} on_click={on_amber_click}/>
                <SelectableColorSchema row={1} column={1} color_schema={ColorSchema::Teal} active={self.color_schema == ColorSchema::Teal} on_click={on_teal_click}/>
                <div class="main_menu__settings__menu_item" style={format!("grid-row: 3/4; grid-column:1/3")} onclick={back_to_main_menu}>{"Back to main menu"}</div>
            </div>
        }
    }
}

static STORAGE_KEY: &str = "color_schema";

fn get_current_color_schema() -> ColorSchema{
    let schema = get_item_from_client_storage(STORAGE_KEY).unwrap_or_else(||{
        let body = get_body().unwrap();
        body.class_name()
    });
    ColorSchema::from(schema)
}

fn update_color_schema(schema: &ColorSchema){
    let body = get_body().unwrap();
    body.set_class_name(&schema.to_string());
    save_item_to_client_storage(STORAGE_KEY, &schema.to_string());
}