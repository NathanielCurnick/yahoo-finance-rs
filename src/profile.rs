use crate::error::YahooError;
use crate::yahoo;
/// Symbols which represent a company can have an address associated with them.
/// This is usually the company headquarters.
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    pub street1: Option<String>,
    pub street2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub zip: Option<String>,
}
impl Address {
    fn new(data: &yahoo::CompanyProfile) -> Result<Address, YahooError> {
        Ok(Address {
            street1: data.address1.clone(),
            street2: data.address2.clone(),
            city: data.city.clone(),
            state: data.state.clone(),
            country: data.country.clone(),
            zip: data.zip.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Company {
    /// Optional address on file for the symbol - typically the HQ for publicly
    /// traded companies.
    pub address: Option<Address>,

    /// The industry, according to Yahoo.  ie. 'Gold'
    pub industry: Option<String>,

    /// The common name for the symbol.
    pub name: String,

    // The sector, according to Yahoo.  ie. 'Basic Materials'
    pub sector: Option<String>,

    /// A summary description for the symbol.
    pub summary: Option<String>,

    /// A website with more information - generally a corporate home page.
    pub website: Option<String>,
}
impl Company {
    fn new(data: yahoo::QuoteSummaryStore) -> Result<Company, YahooError> {
        let profile = data.company_profile.expect("asdf");
        let address = Some(Address::new(&profile)?);

        Ok(Company {
            name: data.quote_type.name,
            summary: profile.summary,
            address,
            industry: profile.industry,
            sector: profile.sector,
            website: profile.website,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fund {
    pub name: String,

    pub family: Option<String>,

    pub kind: String,
}
impl Fund {
    fn new(data: yahoo::QuoteSummaryStore) -> Result<Fund, YahooError> {
        let profile = data.fund_profile.expect("asdf");

        Ok(Fund {
            name: data.quote_type.name,
            kind: profile.kind,
            family: profile.family,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Profile {
    Company(Company),
    Fund(Fund),
}
impl Profile {
    pub async fn load(symbol: &str) -> Result<Profile, YahooError> {
        let data = match yahoo::scrape(symbol).await {
            Ok(x) => x.quote_summary_store,
            Err(y) => return Err(y),
        };

        let kind = &data.quote_type.kind;
        return if kind.as_str() == "EQUITY" {
            let company = match Company::new(data) {
                Ok(x) => x,
                Err(y) => return Err(y),
            };
            Ok(Profile::Company(company))
        } else if kind.as_str() == "ETF" {
            let fund = match Fund::new(data) {
                Ok(x) => x,
                Err(y) => return Err(y),
            };
            Ok(Profile::Fund(fund))
        } else {
            Err(YahooError::UnsupportedSecurity {
                kind: kind.to_string(),
            })
        };
    }
}
