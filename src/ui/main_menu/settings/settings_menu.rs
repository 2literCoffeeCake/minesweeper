use yew::{Component, html, Context};
use super::{ColorSchema, SelectableColorSchema};

pub struct SettingsMenu{
    color_schema: ColorSchema
}

pub enum Msg{
    ChangeSchema(ColorSchema)
}

impl Component for SettingsMenu{
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            color_schema: ColorSchema::BlueGray
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::ChangeSchema(schema) => self.color_schema = schema,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let on_bluegray_click = ctx.link().callback(move |_| Msg::ChangeSchema(ColorSchema::BlueGray));
        let on_blue_click = ctx.link().callback(move |_| Msg::ChangeSchema(ColorSchema::Blue));
        let on_amber_click = ctx.link().callback(move |_| Msg::ChangeSchema(ColorSchema::Amber));
        let on_teal_click = ctx.link().callback(move |_| Msg::ChangeSchema(ColorSchema::Teal));

        html!{
            <div class="main_menu__settings">
                <SelectableColorSchema row={0} column={0} color_schema={ColorSchema::BlueGray} active={self.color_schema == ColorSchema::BlueGray} on_click={on_bluegray_click}/>
                <SelectableColorSchema row={0} column={1} color_schema={ColorSchema::Blue} active={self.color_schema == ColorSchema::Blue} on_click={on_blue_click}/>
                <SelectableColorSchema row={1} column={0} color_schema={ColorSchema::Amber} active={self.color_schema == ColorSchema::Amber} on_click={on_amber_click}/>
                <SelectableColorSchema row={1} column={1} color_schema={ColorSchema::Teal} active={self.color_schema == ColorSchema::Teal} on_click={on_teal_click}/>
            </div>
        }
    }
}
