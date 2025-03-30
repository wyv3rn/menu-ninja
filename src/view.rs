use crate::model::Dish;
use maud::{Markup, html};

pub fn dish_table(dishes: &[Dish]) -> Markup {
    html! {
        table {
            thead {
                tr {
                    th {"Gericht"} th {"Fällig seit"}
                }
            }
            tbody {
                @for dish in dishes {
                    @let not_cooked_for = match dish.not_cooked_for() {
                        Some(t) => format!("{t}s"),
                        None => "-".to_string(),
                    };
                    tr {
                        td { (dish.name()) } td { (not_cooked_for) }
                    }
                }
            }
        }
    }
}
