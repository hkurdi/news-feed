use crate::card_list::CardList;

mod props;
mod card;
mod card_list;

fn main() {
    yew::Renderer::<CardList>::new().render();
}
