use inevents::{
    anyhow::Context,
    events::event::{Event, Query, QueryEndpoint},
    modules::{
        http_server::HttpServer, redis_to_postgres::RedisToPostgres,
        websocket_server::WebsocketServer, EventModule,
    },
};
use intear_events::events::price::price_token::{OhlcEndpoint, PriceTokenEvent};
use std::{collections::HashMap, fs, sync::Arc};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    let module_names = std::env::args().skip(1).collect::<Vec<String>>();
    if module_names.is_empty() {
        log::error!("No modules specified. Available modules: 'http-server', 'websocket-server', 'redis-to-postgres', 'all'. Usage: `./inevents [module]...`");
        return;
    }
    let module_names = if module_names.contains(&"all".to_string()) {
        vec![
            "http-server".to_string(),
            "websocket-server".to_string(),
            "redis-to-postgres".to_string(),
        ]
    } else {
        module_names
    };
    log::info!("Running {}", module_names.join(", "));

    let events = match load_events_from_json("events") {
        Ok(events) => events,
        Err(e) => {
            log::error!("Failed to load events: {e:?}");
            return;
        }
    };

    let mut futures = Vec::new();
    for module_name in module_names {
        match module_name.as_str() {
            "http-server" => {
                futures.push(
                    HttpServer::new(
                        Some("https://docs.intear.tech/docs/events-api/".to_string()),
                        HashMap::from_iter([(
                            PriceTokenEvent::ID.to_string(),
                            vec![QueryEndpoint {
                                route: "ohlc".to_string(),
                                openapi: None,
                                query: Query::Custom(Arc::new(OhlcEndpoint)),
                            }],
                        )]),
                    )
                    .start(events.clone()),
                );
            }
            "websocket-server" => {
                futures.push(
                    WebsocketServer::new(Some(
                        "https://docs.intear.tech/docs/events-api/".to_string(),
                    ))
                    .start(events.clone()),
                );
            }
            "redis-to-postgres" => {
                futures.push(RedisToPostgres.start(events.clone()));
            }
            _ => {
                log::error!("Unknown module: {module_name}");
                return;
            }
        }
    }
    let results = futures::future::join_all(futures).await;
    for result in results {
        if let Err(e) = result {
            log::error!("Error in module: {e:?}");
        }
    }
}

fn load_events_from_json(dir: &str) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let mut events = Vec::new();
    for entry in fs::read_dir(dir).context(format!("Failed to read event directory {:?}", dir))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)
                .context(format!("Failed to read event file {:?}", path.as_os_str()))?;
            let event: Event = serde_json::from_str(&content)
                .context(format!("Failed to parse event file {:?}", path.as_os_str()))?;
            events.push(event);
        }
    }
    Ok(events)
}
