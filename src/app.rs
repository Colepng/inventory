use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::database::get_all;
use crate::database::AddItem;
use crate::database::DeleteItem;
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Items/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Items(cx: Scope) -> impl IntoView {
    let add_item = create_server_multi_action::<AddItem>(cx);
    let delete_item = create_server_action::<DeleteItem>(cx);
    let submissions = add_item.submissions();

    let items = create_resource(
        cx,
        move || (add_item.version().get(), delete_item.version().get()),
        move |_| get_all(cx),
    );

    let on_submit_multi = move |_ev: SubmitEvent| {
        // stop the page from reloading!
        // items.read(cx);
        // let _ = submissions
        //     .get()
        //     .into_iter()
        //     .filter(|x| x.pending().get())
        //     .for_each(|x| {
        //         if x.input.get().map(|data| data.name) == Some("testing".to_owned()) {
        //             log!("valid");
        //             x.cancel();
        //         }
        //     });
        // // ev.prevent_default();
        // leptos::log!("test");
    };

    let _pending_items = move || {
        submissions
            .get()
            .into_iter()
            .filter(|submission| submission.pending().get())
            .map(|submission| {
                view! {
                                    cx,
                <li class="pending">{move || submission.input.get().map(|_data| "loading") }</li>
                            }
            })
            .collect::<Vec<_>>()
    };

    let existing_items = move || {
        items
            .read(cx)
            .map(move |item| match item {
                Err(e) => {
                    vec![
                        view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}
                            .into_any(),
                    ]
                }
                Ok(item) => {
                    if !item.is_empty() {
                        item.into_iter()
                            .map(move |item| {
                                view! { cx,
                                <tr>
                                    <td>{&item.name}</td>
                                    <td>{item.sku}</td>
                                    <td>{item.std}</td>
                                    <td>{item.quantity}</td>
                                    <td>{item.unit_price}</td>
                                    <td><a href=item.link target="_blank">{item.name}</a></td>
                                    <td>
                                    <ActionForm action=delete_item>
                                        <input type="hidden" name="id" value={item.id}/>
                                        <input type="submit" value="X"/>
                                    </ActionForm>
                                    </td>
                                </tr>
                                }
                                .into_any()
                            })
                            .collect::<Vec<_>>()
                    } else {
                        vec![view! { cx, <p>"No items were found."</p> }.into_any()]
                    }
                }
            })
            .unwrap_or_default()
    };

    view! {
            cx,
            <div>
            <h1>"Welcome to 865 testing!"</h1>
    <MultiActionForm on:submit=on_submit_multi action=add_item>
        <input type="text" name="name" required="required"/>
        <input type="text" name="sku" required="required"/>
        <input type="text" name="std" required="required"/>
        <input type="number" name="quantity" min="0" required="required"/>
        <input type="number" name="unit_price" step="0.01" min="0.00" required="required"/>
        <input type="text" name="link" required="required"/>
        // <input type="hidden" name="link" required="required"/>
        <input type="submit" value="Add Item"/>
    </MultiActionForm>
        <Transition fallback=move || view! {cx, <p>"failed to update"</p> }>
        <table>
          <tr>
            <th>"Name"</th>
            <th>"SKU"</th>
            <th>"Short Text Description"</th>
            <th>"Quantity"</th>
            <th>"Unit Price"</th>
            <th>"Link"</th>
          </tr>
        {move || {


                view! {
                    cx,
                    {existing_items}
                    // {pending_items}
                }
            }
        }
        </table>
        </Transition>
        </div>
    }
}
//
// #[component]
// fn Item(
//     cx: Scope,
//     name: String,
//     sku: String,
//     std: String,
//     quantity: String,
//     price: String,
//     link: String,
// ) -> impl IntoView {
//     leptos::log!("{name}");
//     leptos::log!("{sku}");
//     leptos::log!("{std}");
//     leptos::log!("{quantity}");
//     leptos::log!("{price}");
//     leptos::log!("{link}");
//     view! {cx,
//     <tr>
//         <td>{name.clone()}</td>
//         <td>{sku}</td>
//         <td>{std}</td>
//         <td>{quantity}</td>
//         <td>{price}</td>
//         <td>
//             <a href=link target="_blank">{name}</a>
//             </td>
//     </tr>}
// }
