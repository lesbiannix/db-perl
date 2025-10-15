//! A Rust implementation of the `db-tickets` library.
use crate::error::Error;
use reqwest::Client as ReqwestClient;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename = "rporderdetails")]
pub struct Ticket {
    #[serde(rename = "order")]
    pub order: Vec<Order>,
}

#[derive(Deserialize, Debug)]
pub struct Order {
    #[serde(rename = "$")]
    pub details: OrderDetails,
    #[serde(rename = "tcklist")]
    pub tck_list: Vec<TckList>,
    #[serde(rename = "schedulelist")]
    pub schedule_list: Vec<ScheduleList>,
}

#[derive(Deserialize, Debug)]
pub struct OrderDetails {
    pub on: String,
    pub cdt: String,
    pub vfrom: String,
    pub vto: String,
    pub sdt: String,
    pub cid: String,
}

#[derive(Deserialize, Debug)]
pub struct TckList {
    #[serde(rename = "tck")]
    pub tck: Vec<Tck>,
}

#[derive(Deserialize, Debug)]
pub struct Tck {
    #[serde(rename = "mtk")]
    pub mtk: Vec<Mtk>,
}

#[derive(Deserialize, Debug)]
pub struct Mtk {
    pub reisender_vorname: Vec<String>,
    pub reisender_nachname: Vec<String>,
    pub txt: Vec<String>,
    #[serde(rename = "nvplist")]
    pub nvp_list: Vec<NvpList>,
}

#[derive(Deserialize, Debug)]
pub struct NvpList {
    #[serde(rename = "nvp")]
    pub nvp: Vec<Nvp>,
}

#[derive(Deserialize, Debug)]
pub struct Nvp {
    #[serde(rename = "_")]
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct ScheduleList {
    #[serde(rename = "out")]
    pub out: Vec<Out>,
}

#[derive(Deserialize, Debug)]
pub struct Out {
    #[serde(rename = "trainlist")]
    pub train_list: Vec<TrainList>,
}

#[derive(Deserialize, Debug)]
pub struct TrainList {
    #[serde(rename = "train")]
    pub train: Vec<Leg>,
}

#[derive(Deserialize, Debug)]
pub struct Journey {
    pub legs: Vec<Leg>,
}

#[derive(Deserialize, Debug)]
pub struct Leg {
    pub origin: Stop,
    pub destination: Stop,
    pub line: Line,
    pub departure: String,
    pub arrival: String,
}

#[derive(Deserialize, Debug)]
pub struct Stop {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Line {
    pub name: String,
}

/// The client for interacting with the DB Tickets API.
pub struct Client {
    client: ReqwestClient,
}

impl Client {
    /// Creates a new `Client`.
    pub fn new() -> Self {
        Self {
            client: ReqwestClient::new(),
        }
    }

    /// Queries a ticket by ticket number and last name.
    pub async fn query_ticket(&self, ticket_number: &str, last_name: &str) -> Result<Ticket, Error> {
        let mut kwid = None;
        if ticket_number.len() > 5 {
            kwid = Some(self.find_ticket(ticket_number, last_name).await?);
        }

        let req_body = format!(
            r#"<?xml version="1.0"?> <rqorderdetails version="2.0"> <rqheader l="de" v="23080000" d="iPhone16.4.1" os="iOS_15.7.5" app="NAVIGATOR" /> <rqorder on="{}" {}/> <authname tln="{}"/> </rqorderdetails>"#,
            ticket_number,
            if let Some(kwid) = kwid {
                format!(r#"kwid="{}""#, kwid)
            } else {
                "".to_string()
            },
            last_name
        );

        let res = self
            .client
            .post("https://reiseauskunft.bahn.de/bin/mgate.exe")
            .header("Content-Type", "application/xml")
            .body(req_body)
            .send()
            .await?
            .text()
            .await?;

        let ticket: Ticket = quick_xml::de::from_str(&res)?;

        Ok(ticket)
    }

    async fn find_ticket(&self, ticket_number: &str, last_name: &str) -> Result<String, Error> {
        let req_body = format!(
            r#"<?xml version="1.0"?><rqfindorder version="1.0"><rqheader l="de" v="23080000" d="iPhone16.4.1" os="iOS_15.7.5" app="NAVIGATOR"/><rqorder on="{}"/><authname tln="{}"/></rqfindorder>"#,
            ticket_number, last_name
        );

        let res = self
            .client
            .post("https://reiseauskunft.bahn.de/bin/mgate.exe")
            .header("Content-Type", "application/xml")
            .body(req_body)
            .send()
            .await?
            .text()
            .await?;

        let mut reader = quick_xml::Reader::from_str(&res);
        reader.trim_text(true);

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(quick_xml::events::Event::Start(e)) => {
                    if e.name() == b"orderhead" {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key == b"kwid" {
                                return Ok(String::from_utf8(attr.value.to_vec())?);
                            }
                        }
                    }
                }
                Ok(quick_xml::events::Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => (),
            }
            buf.clear();
        }

        Err(Error::Unknown)
    }
}