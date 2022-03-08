#[derive(Debug, Serialize, Clone)]
pub struct RegisterElectOrWithdraw {
    #[serde(rename = "$value")]
    pub elect_or_withdraw: ElectOrWithdraw,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}RegisterType")]
    pub register_type: super::base_types::RegisterType,
}

#[derive(Debug, Serialize, Clone)]
pub enum ElectOrWithdraw {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ElectToHold")]
    Elect(bool),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}WithdrawElectionToHold")]
    Withdraw(bool)
}