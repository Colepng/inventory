use cfg_if::cfg_if;
use leptos::ServerFnError;
use leptos::*;
use serde::{Deserialize, Serialize};

use leptos::server;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::Pool;
        use sqlx::Sqlite;
        use sqlx::SqlitePool;

        pub async fn db() -> Result<Pool<Sqlite>, ServerFnError> {
            Ok(SqlitePool::connect("sqlite:/home/cole/temp/test/inventory.db")
                .await.map_err(|e| ServerFnError::ServerError(e.to_string()))?)
        }

        pub fn register_server_functions() {
            _ = GetAll::register();
            _ = AddItem::register();
            _ = DeleteItem::register();
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
        pub struct Item {
            pub name: String,
            pub sku: String,
            pub std: String,
            pub quantity: u32,
            pub unit_price: f64,
            pub link: String,
            pub id: u32,
        }

        impl IntoView for Item {
            fn into_view(self, cx: Scope) -> leptos::View {
                leptos::View::Text(leptos_dom::Text::new(self.name.into()))
            }
        }
    } else {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct Item {
            pub name: String,
            pub sku: String,
            pub std: String,
            pub quantity: u32,
            pub unit_price: f64,
            pub link: String,
            pub id: u32,
        }

        impl IntoView for Item {
            fn into_view(self, _cx: Scope) -> leptos::View {
                leptos::View::Text(leptos_dom::Text::new(self.name.into()))
            }
        }
    }
}

#[server(GetAll, "/api")]
pub async fn get_all(cx: Scope) -> Result<Vec<Item>, ServerFnError> {
    let req = use_context::<actix_web::HttpRequest>(cx);

    if let Some(req) = req {
        println!("req.path = {:#?}", req.path());
    }
    // use futures::TryStreamExt;
    let conn = db().await?;
    let rows = sqlx::query_as::<_, Item>("SELECT * FROM inventory;")
        .fetch_all(&conn)
        .await
        .expect("maybe");
    Ok(rows)
}

#[server(AddItem, "/api")]
pub async fn add_item(
    name: String,
    sku: String,
    std: String,
    quantity: u32,
    unit_price: f64,
    link: String,
) -> Result<(), ServerFnError> {
    let conn = db().await?;

    sqlx::query!(
        "INSERT INTO inventory (name, sku, std, quantity, unit_price, link) 
        VALUES ($1, $2, $3, $4, $5, $6)",
        name,
        sku,
        std,
        quantity,
        unit_price,
        link,
    )
    .fetch_all(&conn)
    .await
    .expect("idk man");
    Ok(())
}

#[server(DeleteItem, "/api")]
pub async fn delete_item(id: u32) -> Result<(), ServerFnError> {
    let conn = db().await?;

    sqlx::query!("DELETE FROM inventory WHERE id = $1", id)
        .fetch_all(&conn)
        .await
        .expect("let me finsh this faster pls");
    Ok(())
}
