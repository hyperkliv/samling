use std::{
    env::current_dir,
    fs::File,
    io::{stdout, Read, Write},
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    process::Command,
};

use clap::Parser;
use http::{
    header::{HeaderName, CONTENT_TYPE},
    HeaderMap, HeaderValue,
};
use tokio::signal::unix::SignalKind;
use tracing::{debug, error, info, metadata::LevelFilter, Level};
use tracing_subscriber::{filter::FilterFn, prelude::*};
use url::Url;

use crate::{
    app::serve,
    auth::{
        hashing::Hasher, rbac::Role, repo::AuthRepo, signing::JwtSigner, CreateUser, UpdateUser,
        User,
    },
    cloudflare::CloudflareApi,
    db_migrations::Migrator,
    entity_ref::Id,
    errors::{CliResult, Error},
    helpers::create_deadpool_manager,
    jsonschema::generate_schema_file,
    organizations::{repo::OrgRepo, CreateOrganization, Organization},
    CliError, Environment, UpdateOrganization,
};

static DEFAULT_DB_HOST: &str = "127.0.0.1";
static DEFAULT_DB_NAME: &str = "samling";
static DEFAULT_DB_CONNECTIONS: u32 = 10;

/// Samling CLI
///
/// If you want to override which .env file is read you can set the environment
/// variable SAMLING_CONFIG to the path were it resides.
#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, env, global = true)]
    log_level: Option<Level>,
    /// Ignore the given log prefix (e.g. "hyper::proto")
    #[arg(long, env, global = true, value_delimiter = ',')]
    log_ignore: Vec<String>,
    #[arg(long, env, global = true)]
    sentry_dsn: Option<String>,
    #[arg(long, env, default_value_t = Environment::Development)]
    environment: Environment,
    #[arg(long, env, default_value = DEFAULT_DB_NAME, global = true)]
    db_name: String,
    #[arg(long, env, default_value = DEFAULT_DB_HOST, global = true)]
    db_host: String,
    #[arg(long, env, default_value_t = 5432, global = true)]
    db_port: u16,
    #[arg(long, env, default_value = "postgres", global = true)]
    db_user: String,
    #[arg(long, env, global = true)]
    db_password: Option<String>,
    #[arg(long, env, global = true, default_value = "images.samling.io")]
    cloudflare_custom_images_domain: String,
    /// Used for setting `created_by` when using the CLI
    #[arg(long, env, global = true, default_value = "1")]
    superuser_id: Id<User>,
    #[arg(long, env, default_value_t = DEFAULT_DB_CONNECTIONS, global = true)]
    max_db_connections: u32,
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand, Debug)]
enum Subcommand {
    Version,
    Serve {
        #[arg(long, env = "APP_HOST", default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
        host: IpAddr,
        #[arg(long, env = "APP_PORT", default_value_t = 8080)]
        port: u16,
        #[arg(long, env)]
        secret: String,
        #[arg(long, env)]
        cloudflare_account_id: String,
        #[arg(long, env)]
        cloudflare_token: String,
        /// Set the members of the CORS Access-Control-Allowed-Origin header. Defaults to no allowed origins at all.
        #[arg(
            short = 'C',
            long = "cors-allowed-origin",
            env = "CORS_ALLOWED_ORIGINS",
            value_delimiter = ','
        )]
        cors_allowed_origins: Vec<HeaderValue>,
        /// Migrate to latest database schema version on startup. Only use when running 1 process of the app!
        #[arg(long, env)]
        db_auto_migrate: bool,
    },
    Migrate,
    Users {
        #[command(subcommand)]
        subcommand: UsersSubcommand,
    },
    Organizations {
        #[arg(long, env)]
        cloudflare_account_id: String,
        #[arg(long, env)]
        cloudflare_token: String,
        #[command(subcommand)]
        subcommand: OrganizationsSubcommand,
    },
    /// Conveniently make API calls to the samling service
    Api {
        #[arg(short, long, env = "SAMLING_TOKEN")]
        token: String,
        #[arg(long, env = "SAMLING_HOST", default_value = "127.0.0.1:8080")]
        host: String,
        #[arg(short = 's', long, env = "SAMLING_SCHEME", default_value = "http")]
        scheme: String,
        method: http::Method,
        api_path: String,
        #[arg(short = 'd', long)]
        data: Option<PathBuf>,
        /// Pretty print JSON responses
        #[arg(short = 'p', long, env = "SAMLING_PRETTY_RESPONSES")]
        pretty: bool,
        #[arg(short = 'H', long = "header", env = "SAMLING_HEADERS", value_parser = parse_key_val::<HeaderName, HeaderValue>)]
        headers: Vec<(HeaderName, HeaderValue)>,
    },
    GenerateUserToken {
        user_id: Id<User>,
        #[arg(long, env)]
        secret: String,
        #[arg(long, default_value_t=7 * 24 * 3600)]
        token_ttl: u32,
    },
    /// Generate JSON Schema and Typescript files for structs and enums in the project
    GenerateTypes,
}

#[derive(clap::Subcommand, Debug)]
enum OrganizationsSubcommand {
    Create {
        name: String,
        logo_url: Option<url::Url>,
    },
    Update {
        id: Id<Organization>,
        name: Option<String>,
        logo_url: Option<url::Url>,
    },
}

#[derive(clap::Subcommand, Debug)]
enum UsersSubcommand {
    /// Get user's data by e-mail address
    FindByEmail { email: String },
    /// Create a new user
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        email: String,
        #[arg(long)]
        password: Option<String>,
        #[arg(short, long)]
        profile_image: Option<Url>,
    },
    /// Update user
    Update {
        id: Id<User>,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        email: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(short, long)]
        profile_image: Option<Url>,
    },
    /// Delete user
    Delete { id: Id<User> },
    /// Associate roles to the given user, which will only apply to the given organization
    AssociateRoles {
        user_id: Id<User>,
        organization_id: Id<Organization>,
        roles: Vec<Role>,
        #[arg(short, long)]
        all: bool,
    },
}

pub async fn run() -> CliResult<()> {
    load_dotenv_files()?;

    let cli = Cli::parse();

    let log_ignore = cli.log_ignore.clone();

    let log_ignore_filter = FilterFn::new(move |metadata| {
        for ignore in &log_ignore {
            if metadata.target().starts_with(ignore) {
                return false;
            }
        }
        true
    });

    if let Some(level) = cli.log_level {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(std::io::stderr)
                    .with_filter(LevelFilter::from_level(level))
                    .with_filter(log_ignore_filter),
            )
            .init();
    }

    debug!("{cli:#?}");

    let _guard = cli.sentry_dsn.map(|dsn| {
        debug!("Initializing Sentry error reporting");
        sentry::init((
            dsn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                environment: Some(cli.environment.to_string().to_ascii_lowercase().into()),
                ..Default::default()
            },
        ))
    });

    let pool = create_deadpool_manager(
        cli.db_host,
        cli.db_port,
        cli.db_name,
        cli.db_user,
        cli.db_password,
        cli.max_db_connections,
    )?;

    match cli.subcommand {
        Subcommand::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Subcommand::Serve {
            host,
            port,
            secret,
            cors_allowed_origins,
            db_auto_migrate,
            cloudflare_account_id,
            cloudflare_token,
        } => {
            if db_auto_migrate {
                let migrator = Migrator::new(pool.clone());
                migrator.migrate_to_latest().await?;
            }

            let cloudflare_api = CloudflareApi::new(
                cloudflare_account_id,
                cloudflare_token,
                Some(cli.cloudflare_custom_images_domain),
            )?;

            let address = (host, port).into();
            let app_future = serve(
                address,
                pool,
                secret,
                cors_allowed_origins,
                cloudflare_api,
                cli.environment,
            );
            let app_join_handle = tokio::spawn(app_future);

            let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())?;
            let mut sigint = tokio::signal::unix::signal(SignalKind::interrupt())?;

            let signal_name = tokio::select! {
                _ = sigterm.recv() => "SIGTERM",
                _ = sigint.recv() => "SIGINT",
            };

            tracing::info!("{} received, stopping app...", signal_name);
            app_join_handle.abort();
            println!("Done");
            Ok(())
        }
        Subcommand::Migrate => {
            let migrator = Migrator::new(pool);
            migrator.migrate_to_latest().await?;
            Ok(())
        }
        Subcommand::Organizations {
            cloudflare_account_id,
            cloudflare_token,
            subcommand,
        } => {
            let client = pool.get().await?;

            let cloudflare_api = CloudflareApi::new(
                cloudflare_account_id,
                cloudflare_token,
                Some(cli.cloudflare_custom_images_domain),
            )?;
            match subcommand {
                OrganizationsSubcommand::Create { name, logo_url } => {
                    let org = CreateOrganization {
                        name,
                        logo: logo_url.map(|url| url.into()),
                    };
                    let org = OrgRepo
                        .insert(&client, cloudflare_api, org, cli.superuser_id)
                        .await?;
                    println!("Organization {org} created!");
                    Ok(())
                }
                OrganizationsSubcommand::Update { id, name, logo_url } => {
                    let org = UpdateOrganization {
                        name,
                        logo: logo_url.map(|url| url.into()),
                    };
                    let org = OrgRepo.update(&client, cloudflare_api, id, org).await?;
                    println!("Organization {org} updated!");
                    Ok(())
                }
            }
        }
        Subcommand::Users { subcommand } => {
            let client = pool.get().await?;
            let hasher = Hasher::default();
            match subcommand {
                UsersSubcommand::FindByEmail { email } => {
                    if let Some(user) = AuthRepo.find_user_by_email(&client, &email).await? {
                        println!("{}", serde_json::to_string(&user).unwrap());
                        Ok(())
                    } else {
                        Err(Error::UserEmailNotFound(email).into())
                    }
                }
                UsersSubcommand::Create {
                    name,
                    email,
                    password,
                    profile_image,
                } => {
                    let password_hash = match &password {
                        Some(pwd) => Some(hasher.argon2_hash(pwd)?),
                        None => None,
                    };
                    let user = CreateUser {
                        email,
                        password,
                        name,
                        profile_image,
                        roles: None,
                        groups: None,
                    };
                    let user = AuthRepo.insert_user(&client, user, password_hash).await?;
                    println!("User with id={} created!", user.id);
                    Ok(())
                }
                UsersSubcommand::Delete { id } => {
                    AuthRepo.delete_user(&client, id).await?;
                    println!("Deleted user with id={}!", id);
                    Ok(())
                }
                UsersSubcommand::Update {
                    id,
                    name,
                    email,
                    password,
                    profile_image,
                } => {
                    let user = UpdateUser {
                        email,
                        password,
                        name,
                        profile_image,
                        roles: None,
                        groups: None,
                    };
                    let password_hash = match &user.password {
                        Some(pwd) => Some(hasher.argon2_hash(pwd)?),
                        None => None,
                    };
                    AuthRepo
                        .update_user(&client, id, user, password_hash)
                        .await?;
                    println!("Successfully updated user with id={id}!");
                    Ok(())
                }
                UsersSubcommand::AssociateRoles {
                    user_id,
                    organization_id,
                    roles,
                    all,
                } => {
                    let roles = if all { Role::all() } else { roles };
                    let user = AuthRepo.get_user(&client, user_id).await?;
                    let org = OrgRepo.get(&client, organization_id).await?;
                    AuthRepo
                        .associate_organization_and_roles(&client, user.id, org.id, Some(&roles))
                        .await?;
                    println!(
                        "Successfully associated roles {} on org {org} to user {user}",
                        roles
                            .iter()
                            .map(|r| r.to_string())
                            .collect::<Vec<_>>()
                            .join(" + ")
                    );
                    Ok(())
                }
            }
        }
        Subcommand::GenerateUserToken {
            user_id,
            secret,
            token_ttl,
        } => {
            let client = pool.get().await?;
            let jwt_signer = JwtSigner::new(&secret).with_ttl(token_ttl);
            let user = AuthRepo.get_user(&client, user_id).await?;
            let claims = jwt_signer.claims(&user)?;
            let token = jwt_signer.encode(claims)?;
            println!("{}", token);
            Ok(())
        }
        Subcommand::GenerateTypes => {
            let dir = current_dir()?;
            // Ensure that we're in the correct folder
            if !dir.join("Cargo.toml").exists() {
                return Err(CliError::NotInProjectRoot);
            }
            let schema_target_path = dir.join("schema.json");
            let types_target_path = dir.join("ui/src/types/api.ts");
            // First ensure that schema.json is up-to-date
            tracing::info!("Creating JSON schema...");
            generate_schema_file(&schema_target_path).await?;
            // Now we convert it to typescript
            // using [quicktype](https://github.com/quicktype/quicktype).
            tracing::info!("Converting schema.json to typescript...");
            let output = Command::new("quicktype")
                .args([
                    "--converters=all-objects",
                    "--raw-type=any",
                    "--acronym-style=original",
                    "--no-date-times",
                    "--src-lang=schema",
                    "--telemetry=disable",
                    &schema_target_path.to_string_lossy(),
                    "--out",
                    &types_target_path.to_string_lossy(),
                ])
                .output()?;
            tracing::info!("Done!");
            println!("{}", String::from_utf8_lossy(&output.stdout));
            Ok(())
        }
        Subcommand::Api {
            token,
            scheme,
            host,
            method,
            api_path,
            data,
            pretty,
            headers,
        } => {
            let mut header_map = HeaderMap::from_iter(headers);
            let content_type = header_map
                .entry(CONTENT_TYPE)
                .or_insert("application/json".try_into().unwrap());
            let is_json = content_type
                .to_str()
                .unwrap_or_default()
                .contains("application/json");

            let client = reqwest::Client::new();

            let base_url = format!("{scheme}://{host}/api");
            debug!("API base url: {base_url}");

            let api_path = api_path.trim_start_matches('/');
            let url = format!("{base_url}/{api_path}");
            let builder = client
                .request(method, &url)
                .bearer_auth(token)
                .headers(header_map);

            let request = if let Some(filepath) = data {
                let filepath = if filepath.to_string_lossy() == "-" {
                    PathBuf::from("/dev/stdin")
                } else {
                    filepath
                };
                let mut file = File::open(filepath)?;
                let mut body = Vec::new();
                file.read_to_end(&mut body)?;
                builder.body(body).build()?
            } else {
                builder.build()?
            };

            info!("Making request to {}...", &url);
            let resp = client.execute(request).await?;
            let err_result = resp.error_for_status_ref();

            if let Err(error) = err_result {
                error!("Response code HTTP{} (error: {})", &resp.status(), error);
            } else {
                info!("Response code HTTP{}", &resp.status());
            }

            let body = resp.bytes().await?;
            if is_json && pretty && !body.is_empty() {
                match serde_json::from_slice::<serde_json::Value>(&body) {
                    Ok(json) => {
                        serde_json::to_writer_pretty(stdout(), &json)?;
                    }
                    Err(_) => {
                        stdout().write_all(&body)?;
                    }
                }
            } else {
                stdout().write_all(&body)?;
            }
            Ok(())
        }
    }
}

/// Load default.env and user.env from current directory and its parents
fn load_dotenv_files() -> CliResult<()> {
    if let Ok(path) = std::env::var("SAMLING_CONFIG") {
        info!(
            "Loading config from SAMLING_CONFIG env var path: {:?}",
            &path
        );
        dotenv::from_path(&path)?;
    } else {
        match dotenv::dotenv() {
            Ok(_) => {}
            Err(err) => tracing::debug!("Failed to read .env file with error: {err}"),
        };
    }
    Ok(())
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
