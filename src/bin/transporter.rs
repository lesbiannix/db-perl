use clap::Parser;
use transport_api_wrapper::client::Client;

/// A simple command-line interface for the transport.rest API.
#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "transport-api-wrapper",
    about = "A Rust wrapper for the transport API"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    /// Find locations by name.
    Locations(Locations),
    /// Get departures for a specific stop.
    Departures(Departures),
    /// Decode a "via" string.
    ViaDecode(ViaDecode),
    /// Find journeys with the Vendo client.
    VendoJourneys(VendoJourneys),
    /// Query a ticket.
    Tickets(Tickets),
}

/// A subcommand for finding locations.
#[derive(Parser)]
struct Locations {
    /// The name of the location to search for.
    query: String,
}

/// A subcommand for getting departures.
#[derive(Parser)]
struct Departures {
    /// The ID of the stop to get departures for.
    stop_id: String,
}

/// A subcommand for decoding a "via" string.
#[derive(Parser)]
struct ViaDecode {
    /// The "via" string to decode.
    via: String,
}

/// A subcommand for finding journeys with the Vendo client.
#[derive(Parser)]
struct VendoJourneys {
    /// The origin station ID.
    from: String,
    /// The destination station ID.
    to: String,
}

/// A subcommand for querying a ticket.
#[derive(Parser)]
struct Tickets {
    /// The ticket number.
    ticket_number: String,
    /// The last name of the passenger.
    last_name: String,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    let client = Client::new();

    let result = match opts.subcmd {
        SubCommand::Locations(l) => {
            client.locations(&l.query).await.map(|locs| format!("{:#?}", locs))
        }
        SubCommand::Departures(d) => {
            client.stop_departures(&d.stop_id).await.map(|deps| format!("{:#?}", deps))
        }
        SubCommand::ViaDecode(v) => {
            transport_api_wrapper::via_decoder::parse_wegetext(&v.via)
                .map(|decoded| format!("{:#?}", decoded))
                .map_err(|e| e.into())
        }
        SubCommand::VendoJourneys(v) => {
            let vendo_client = transport_api_wrapper::vendo_client::Client::new(
                transport_api_wrapper::vendo_client::Profile::Db,
            );
            vendo_client
                .journeys(&v.from, &v.to)
                .await
                .map(|journeys| format!("{:#?}", journeys))
        }
        SubCommand::Tickets(t) => {
            let tickets_client = transport_api_wrapper::tickets_client::Client::new();
            tickets_client
                .query_ticket(&t.ticket_number, &t.last_name)
                .await
                .map(|ticket| format!("{:#?}", ticket))
        }
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Error: {}", e),
    }
}