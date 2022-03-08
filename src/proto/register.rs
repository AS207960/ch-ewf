#[derive(Debug, Deserialize)]
pub enum MemberName {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CorporateName")]
    CorporateName(String),
    #[serde(rename = "$value")]
    Person(Person)
}

#[derive(Debug, Deserialize)]
pub struct Person {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Forename")]
    pub forename: String,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Surname")]
    pub surname: String,
}

#[derive(Debug, Deserialize)]
pub enum StocksOrSharesHeld {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}SharesHeld")]
    SharesHeld(SharesHeld),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}StocksHeld")]
    StocksHeld(StocksHeld)
}

#[derive(Debug, Deserialize)]
pub struct SharesHeld {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}ShareClass")]
    pub share_class: String,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}NumShares")]
    pub num_shares: f64,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}ShareReference", default)]
    pub share_reference: Option<String>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}AmountPaidUp")]
    pub paid_up: f64,
}

#[derive(Debug, Deserialize)]
pub struct StocksHeld {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}StockClass")]
    pub stock_class: String,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}AmountHeld")]
    pub amount_held: f64,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Currency")]
    pub currency: String,
}