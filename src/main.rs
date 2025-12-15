#[cfg(feature = "ssr")]
use actix_web::web::Data;
use leptos_ssr_first::api;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::handle_server_fns_with_context;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_ssr_first::app::*;
    use leptos_ssr_first::server_utils::authorization::Authorisation;
    use leptos_ssr_first::server_utils::configuration;
    use leptos_ssr_first::server_utils::logging::Logger;
    use sqlx::{Pool, Postgres};

    //LEPTOS CODE
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    //LSF CODE
    let configuration =
        configuration::get_configuration().expect("Couldn't read configuration file.");
    Logger::init(configuration.log).expect("Couldn't initialize logger");
    let jwt_keys = api::jwt::get_jwt_keys(configuration.session_secret);
    let db_url = configuration.database.connection_string();
    let db_pool = Pool::<Postgres>::connect(db_url.as_str())
        .await
        .expect("Couldn't connect to database.");
    //LSF CODE END

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();
        //LSF CODE
        let db_pool_clone = db_pool.clone();
        let jwt_keys_clone = jwt_keys.clone();
        //LSF CODE END

        println!("listening on http://{}", &addr);

        App::new()
            //LSF CODE
            .service(
                web::scope("/api")
                    .app_data(Data::new(db_pool_clone.clone()))
                    .app_data(Data::new(jwt_keys_clone.clone()))
                    .wrap(Authorisation)
                    .route(
                        "/{func_name:.*}",
                        handle_server_fns_with_context(move || {
                            provide_context(Data::new(db_pool_clone.clone()));
                            provide_context(Data::new(jwt_keys_clone.clone()));
                        }),
                    ),
            )
            //LSF CODE END
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", &site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
            .app_data(Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos_ssr_first::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
