use crate::model::Dish;
use maud::{Markup, html};

pub type ErrorList<'a> = [&'a str];

pub fn dish_table(dishes: &[Dish]) -> Markup {
    html! {
        table {
            thead {
                tr {
                    th { "Gericht" } th { "Fällig seit" } th { "Steuerung" }
                }
            }
            tbody {
                @for dish in dishes {
                    @let not_cooked_for = match dish.not_cooked_for() {
                        Some(t) => format_time(t),
                        None => "-".to_string(),
                    };
                    @let cooked_url = format!("/dishes/{}/cooked", dish.name());
                    @let delete_url = format!("/dishes/{}/delete", dish.name());
                    tr {
                        td { (dish.name()) }
                        td { (not_cooked_for) }
                        td {
                            form action=(cooked_url) method="post" {
                                button { "Woar des gut!" }
                            }
                        }
                        td {
                            form action=(delete_url) method="post" {
                                button { "Nie widder!" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn new_dish_form(errors: &ErrorList) -> Markup {
    let errors = errors_as_markup(errors);
    html! {
        form action="/dishes/new" method="post" {
            fieldset {
                legend { "A neus Gericht, auf gedds!" }
                p {
                    label for="name" { "Bezeichnung: " }
                    input name="name" id="name" type="text" placeholder="z.B. Woschd" { }
                    span class="error" { (errors) }
                }
                button { "Schuss!" }
            }
        }
        p { a href="/dishes" { "Lieber doch nier ..." } }
    }
}

fn errors_as_markup(errors: &ErrorList) -> Markup {
    let as_list = match errors.len() {
        0 => html! {},
        1 => html! { (errors.first().unwrap() ) },
        _ => html! {
            ul {
                @for err in errors {
                    li { (err) }
                }
            }
        },
    };
    html! {
        p style="margin-left: 2em;" { (as_list) }
    }
}

fn format_time(secs: u64) -> String {
    if secs < 60 {
        format!("{secs} s")
    } else if secs < 60 * 60 {
        format!("{} m", secs / 60)
    } else if secs < 24 * 60 * 60 {
        format!("{} h", secs / (60 * 60))
    } else {
        format!("{} d", secs / (24 * 60 * 60))
    }
}
