use yew::{html, Callback, Component, Context, Html, Properties};

pub enum MenuAction{
    Continue,
    BackToMainMenu,
    NewGame
}
pub struct Menu;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub on_item_click: Callback<MenuAction>,
    pub active: bool
}



impl Component for Menu{
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            title,
            active,
            on_item_click
        } = ctx.props().clone();
        
        let on_click = ctx.link().callback(move|_| on_item_click.emit(MenuAction::Continue));


        let classes = get_classes(active);
        html!{
            <div class={classes}>
                <div style={"grid-row: 2/3;font-size: 90px;"} class="menu_menu_item">{title}</div>
                <div style={"grid-row: 4/5;font-size: 50px;"} class="menu_menu_item" onclick={on_click}>{"Continue"}</div>
                <div style={"grid-row: 6/7;font-size: 50px;"} class="menu_menu_item">{"New game"}</div>
                <div style={"grid-row: 8/9;font-size: 50px;"} class="menu_menu_item">{"Back to main menu"}</div>
            </div>
        }
    }
}

fn get_classes(active: bool) -> String{
    let mut classes = String::from("menu ");
    if active{
        classes.push_str("menu--active");
    } else {
        classes.push_str("menu--inactive");
    }
    classes
}