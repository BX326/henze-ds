#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "AlternativeName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"AlternativeName\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/definitions/AlternativeNameType\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AlternativeName {
    #[serde(rename = "type")]
    pub type_: AlternativeNameType,
    pub value: String,
}
impl From<&AlternativeName> for AlternativeName {
    fn from(value: &AlternativeName) -> Self {
        value.clone()
    }
}
impl AlternativeName {
    pub fn builder() -> builder::AlternativeName {
        Default::default()
    }
}
#[doc = "AlternativeNameType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"AlternativeNameType\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"FULL\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AlternativeNameType {
    #[serde(rename = "FULL")]
    Full,
}
impl From<&AlternativeNameType> for AlternativeNameType {
    fn from(value: &AlternativeNameType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AlternativeNameType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Full => write!(f, "FULL"),
        }
    }
}
impl std::str::FromStr for AlternativeNameType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "FULL" => Ok(Self::Full),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for AlternativeNameType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AlternativeNameType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AlternativeNameType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Category"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Category\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"displayOrder\","]
#[doc = "    \"id\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"$ref\": \"#/definitions/Code\""]
#[doc = "    },"]
#[doc = "    \"displayOrder\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Category {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    #[serde(rename = "displayOrder")]
    pub display_order: String,
    pub id: String,
    pub name: String,
}
impl From<&Category> for Category {
    fn from(value: &Category) -> Self {
        value.clone()
    }
}
impl Category {
    pub fn builder() -> builder::Category {
        Default::default()
    }
}
#[doc = "Channel"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Channel\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"R\","]
#[doc = "    \"S\","]
#[doc = "    \"I\","]
#[doc = "    \"M\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Channel {
    R,
    S,
    I,
    M,
}
impl From<&Channel> for Channel {
    fn from(value: &Channel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Channel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::R => write!(f, "R"),
            Self::S => write!(f, "S"),
            Self::I => write!(f, "I"),
            Self::M => write!(f, "M"),
        }
    }
}
impl std::str::FromStr for Channel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "R" => Ok(Self::R),
            "S" => Ok(Self::S),
            "I" => Ok(Self::I),
            "M" => Ok(Self::M),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Channel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Channel {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Channel {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Clock"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Clock\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"lastUpdate\","]
#[doc = "    \"offset\","]
#[doc = "    \"state\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"lastUpdate\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"offset\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"state\": {"]
#[doc = "      \"$ref\": \"#/definitions/State\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Clock {
    pub id: String,
    #[serde(rename = "lastUpdate")]
    pub last_update: chrono::DateTime<chrono::offset::Utc>,
    pub offset: i64,
    pub state: State,
}
impl From<&Clock> for Clock {
    fn from(value: &Clock) -> Self {
        value.clone()
    }
}
impl Clock {
    pub fn builder() -> builder::Clock {
        Default::default()
    }
}
#[doc = "Code"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Code\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"FOOTBALL\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Code {
    #[serde(rename = "FOOTBALL")]
    Football,
}
impl From<&Code> for Code {
    fn from(value: &Code) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Code {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Football => write!(f, "FOOTBALL"),
        }
    }
}
impl std::str::FromStr for Code {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "FOOTBALL" => Ok(Self::Football),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Code {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Code {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Code {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Commentary"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Commentary\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"clock\","]
#[doc = "    \"facts\","]
#[doc = "    \"participants\","]
#[doc = "    \"periods\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"clock\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Clock\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"facts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Fact\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"participants\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Participant\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"periods\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Period\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Commentary {
    pub clock: Option<Clock>,
    pub facts: Vec<Fact>,
    pub participants: Vec<Participant>,
    pub periods: Vec<Period>,
}
impl From<&Commentary> for Commentary {
    fn from(value: &Commentary) -> Self {
        value.clone()
    }
}
impl Commentary {
    pub fn builder() -> builder::Commentary {
        Default::default()
    }
}
#[doc = "CompetitionSummary"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CompetitionSummary\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"competitionDrilldownTagId\","]
#[doc = "    \"eventCount\","]
#[doc = "    \"marketCount\","]
#[doc = "    \"typeIds\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"competitionDrilldownTagId\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"eventCount\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"marketCount\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"typeIds\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"format\": \"integer\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CompetitionSummary {
    #[serde(rename = "competitionDrilldownTagId")]
    pub competition_drilldown_tag_id: String,
    #[serde(rename = "eventCount")]
    pub event_count: i64,
    #[serde(rename = "marketCount")]
    pub market_count: Option<i64>,
    #[serde(rename = "typeIds")]
    pub type_ids: Vec<String>,
}
impl From<&CompetitionSummary> for CompetitionSummary {
    fn from(value: &CompetitionSummary) -> Self {
        value.clone()
    }
}
impl CompetitionSummary {
    pub fn builder() -> builder::CompetitionSummary {
        Default::default()
    }
}
#[doc = "Data"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Data\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"timeBandEvents\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"timeBandEvents\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/TimeBandEvent\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Data {
    #[serde(rename = "timeBandEvents")]
    pub time_band_events: Vec<TimeBandEvent>,
}
impl From<&Data> for Data {
    fn from(value: &Data) -> Self {
        value.clone()
    }
}
impl Data {
    pub fn builder() -> builder::Data {
        Default::default()
    }
}
#[doc = "Event"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Event\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"active\","]
#[doc = "    \"blurb\","]
#[doc = "    \"cashoutAvailable\","]
#[doc = "    \"category\","]
#[doc = "    \"channels\","]
#[doc = "    \"class\","]
#[doc = "    \"commentary\","]
#[doc = "    \"competitionDrilldownTagId\","]
#[doc = "    \"displayOrder\","]
#[doc = "    \"displayed\","]
#[doc = "    \"extKey\","]
#[doc = "    \"externalIds\","]
#[doc = "    \"fixedOddsAvailable\","]
#[doc = "    \"id\","]
#[doc = "    \"isVoid\","]
#[doc = "    \"liveBettingAvailable\","]
#[doc = "    \"liveNow\","]
#[doc = "    \"marketCount\","]
#[doc = "    \"marketCounts\","]
#[doc = "    \"markets\","]
#[doc = "    \"mediaProviders\","]
#[doc = "    \"meeting\","]
#[doc = "    \"name\","]
#[doc = "    \"neutralVenue\","]
#[doc = "    \"popularityOrder\","]
#[doc = "    \"raceNumber\","]
#[doc = "    \"rcpAvailable\","]
#[doc = "    \"resulted\","]
#[doc = "    \"retailCode\","]
#[doc = "    \"settled\","]
#[doc = "    \"sortCode\","]
#[doc = "    \"sportId\","]
#[doc = "    \"startTime\","]
#[doc = "    \"started\","]
#[doc = "    \"statisticsAvailable\","]
#[doc = "    \"status\","]
#[doc = "    \"teams\","]
#[doc = "    \"type\","]
#[doc = "    \"venue\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"active\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"blurb\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"cashoutAvailable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"category\": {"]
#[doc = "      \"$ref\": \"#/definitions/Category\""]
#[doc = "    },"]
#[doc = "    \"channels\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Channel\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"class\": {"]
#[doc = "      \"$ref\": \"#/definitions/Category\""]
#[doc = "    },"]
#[doc = "    \"commentary\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Commentary\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"competitionDrilldownTagId\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"displayOrder\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"displayed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"extKey\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"externalIds\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ExternalID\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fixedOddsAvailable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"isVoid\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"liveBettingAvailable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"liveNow\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"marketCount\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"marketCounts\": {"]
#[doc = "      \"$ref\": \"#/definitions/MarketCounts\""]
#[doc = "    },"]
#[doc = "    \"markets\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Market\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"mediaProviders\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/MediaProvider\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"meeting\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"neutralVenue\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"popularityOrder\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"raceNumber\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"rcpAvailable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"resulted\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"retailCode\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"settled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"sortCode\": {"]
#[doc = "      \"$ref\": \"#/definitions/SortCode\""]
#[doc = "    },"]
#[doc = "    \"sportId\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"startTime\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"started\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"statisticsAvailable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/Status\""]
#[doc = "    },"]
#[doc = "    \"teams\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Team\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeClass\""]
#[doc = "    },"]
#[doc = "    \"venue\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub active: bool,
    pub blurb: (),
    #[serde(rename = "cashoutAvailable")]
    pub cashout_available: bool,
    pub category: Category,
    pub channels: Vec<Channel>,
    pub class: Category,
    pub commentary: Option<Commentary>,
    #[serde(rename = "competitionDrilldownTagId")]
    pub competition_drilldown_tag_id: String,
    #[serde(rename = "displayOrder")]
    pub display_order: i64,
    pub displayed: bool,
    #[serde(rename = "extKey")]
    pub ext_key: (),
    #[serde(rename = "externalIds")]
    pub external_ids: Vec<ExternalId>,
    #[serde(rename = "fixedOddsAvailable")]
    pub fixed_odds_available: bool,
    pub id: String,
    #[serde(rename = "isVoid")]
    pub is_void: bool,
    #[serde(rename = "liveBettingAvailable")]
    pub live_betting_available: bool,
    #[serde(rename = "liveNow")]
    pub live_now: bool,
    #[serde(rename = "marketCount")]
    pub market_count: i64,
    #[serde(rename = "marketCounts")]
    pub market_counts: MarketCounts,
    pub markets: Vec<Market>,
    #[serde(rename = "mediaProviders")]
    pub media_providers: Vec<MediaProvider>,
    pub meeting: (),
    pub name: String,
    #[serde(rename = "neutralVenue")]
    pub neutral_venue: bool,
    #[serde(rename = "popularityOrder")]
    pub popularity_order: (),
    #[serde(rename = "raceNumber")]
    pub race_number: i64,
    #[serde(rename = "rcpAvailable")]
    pub rcp_available: bool,
    pub resulted: bool,
    #[serde(rename = "retailCode")]
    pub retail_code: (),
    pub settled: bool,
    #[serde(rename = "sortCode")]
    pub sort_code: SortCode,
    #[serde(rename = "sportId")]
    pub sport_id: String,
    #[serde(rename = "startTime")]
    pub start_time: chrono::DateTime<chrono::offset::Utc>,
    pub started: bool,
    #[serde(rename = "statisticsAvailable")]
    pub statistics_available: bool,
    pub status: Status,
    pub teams: Vec<Team>,
    #[serde(rename = "type")]
    pub type_: TypeClass,
    pub venue: (),
}
impl From<&Event> for Event {
    fn from(value: &Event) -> Self {
        value.clone()
    }
}
impl Event {
    pub fn builder() -> builder::Event {
        Default::default()
    }
}
#[doc = "ExternalId"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExternalID\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"provider\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"provider\": {"]
#[doc = "      \"$ref\": \"#/definitions/Provider\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExternalId {
    pub id: String,
    pub provider: Provider,
}
impl From<&ExternalId> for ExternalId {
    fn from(value: &ExternalId) -> Self {
        value.clone()
    }
}
impl ExternalId {
    pub fn builder() -> builder::ExternalId {
        Default::default()
    }
}
#[doc = "ExternalMediaId"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExternalMediaID\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/definitions/ExternalMediaIDType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExternalMediaId {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ExternalMediaIdType,
}
impl From<&ExternalMediaId> for ExternalMediaId {
    fn from(value: &ExternalMediaId) -> Self {
        value.clone()
    }
}
impl ExternalMediaId {
    pub fn builder() -> builder::ExternalMediaId {
        Default::default()
    }
}
#[doc = "ExternalMediaIdType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExternalMediaIDType\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"DATA\","]
#[doc = "    \"VIDEO\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExternalMediaIdType {
    #[serde(rename = "DATA")]
    Data,
    #[serde(rename = "VIDEO")]
    Video,
}
impl From<&ExternalMediaIdType> for ExternalMediaIdType {
    fn from(value: &ExternalMediaIdType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ExternalMediaIdType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Data => write!(f, "DATA"),
            Self::Video => write!(f, "VIDEO"),
        }
    }
}
impl std::str::FromStr for ExternalMediaIdType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "DATA" => Ok(Self::Data),
            "VIDEO" => Ok(Self::Video),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ExternalMediaIdType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ExternalMediaIdType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ExternalMediaIdType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Fact"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Fact\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"participantId\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"participantId\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/definitions/FactType\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    pub id: String,
    #[serde(rename = "participantId")]
    pub participant_id: String,
    #[serde(rename = "type")]
    pub type_: FactType,
    pub value: String,
}
impl From<&Fact> for Fact {
    fn from(value: &Fact) -> Self {
        value.clone()
    }
}
impl Fact {
    pub fn builder() -> builder::Fact {
        Default::default()
    }
}
#[doc = "FactType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FactType\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"SCORE\","]
#[doc = "    \"RED_CARDS\","]
#[doc = "    \"CORNERS\","]
#[doc = "    \"PENALTIES\","]
#[doc = "    \"YELLOW_RED_CARDS\","]
#[doc = "    \"YELLOW_CARDS\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FactType {
    #[serde(rename = "SCORE")]
    Score,
    #[serde(rename = "RED_CARDS")]
    RedCards,
    #[serde(rename = "CORNERS")]
    Corners,
    #[serde(rename = "PENALTIES")]
    Penalties,
    #[serde(rename = "YELLOW_RED_CARDS")]
    YellowRedCards,
    #[serde(rename = "YELLOW_CARDS")]
    YellowCards,
}
impl From<&FactType> for FactType {
    fn from(value: &FactType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FactType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Score => write!(f, "SCORE"),
            Self::RedCards => write!(f, "RED_CARDS"),
            Self::Corners => write!(f, "CORNERS"),
            Self::Penalties => write!(f, "PENALTIES"),
            Self::YellowRedCards => write!(f, "YELLOW_RED_CARDS"),
            Self::YellowCards => write!(f, "YELLOW_CARDS"),
        }
    }
}
impl std::str::FromStr for FactType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "SCORE" => Ok(Self::Score),
            "RED_CARDS" => Ok(Self::RedCards),
            "CORNERS" => Ok(Self::Corners),
            "PENALTIES" => Ok(Self::Penalties),
            "YELLOW_RED_CARDS" => Ok(Self::YellowRedCards),
            "YELLOW_CARDS" => Ok(Self::YellowCards),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FactType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FactType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FactType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Flag"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Flag\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"PRI\","]
#[doc = "    \"MAIN\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Flag {
    #[serde(rename = "PRI")]
    Pri,
    #[serde(rename = "MAIN")]
    Main,
}
impl From<&Flag> for Flag {
    fn from(value: &Flag) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Flag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pri => write!(f, "PRI"),
            Self::Main => write!(f, "MAIN"),
        }
    }
}
impl std::str::FromStr for Flag {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "PRI" => Ok(Self::Pri),
            "MAIN" => Ok(Self::Main),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Flag {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Flag {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Flag {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "GroupCode"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"GroupCode\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"MATCH_RESULT\","]
#[doc = "    \"TOTAL_GOALS_OVER/UNDER\","]
#[doc = "    \"HANDICAP\","]
#[doc = "    \"BOTH_TEAMS_TO_SCORE\","]
#[doc = "    \"MATCH_RESULT_1ST_HALF\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GroupCode {
    #[serde(rename = "MATCH_RESULT")]
    MatchResult,
    #[serde(rename = "TIE_BREAK_DECIDER")]
    TieBreakDecider,
    #[serde(rename = "WINNER_OF_TIE")]
    WinnerOfTie,
    #[serde(rename = "TOTAL_GOALS_OVER/UNDER")]
    TotalGoalsOverUnder,
    #[serde(rename = "HANDICAP")]
    Handicap,
    #[serde(rename = "BOTH_TEAMS_TO_SCORE")]
    BothTeamsToScore,
    #[serde(rename = "MATCH_RESULT_1ST_HALF")]
    MatchResult1stHalf,
}
impl From<&GroupCode> for GroupCode {
    fn from(value: &GroupCode) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for GroupCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MatchResult => write!(f, "MATCH_RESULT"),
            Self::TieBreakDecider => write!(f, "TIE_BREAK_DECIDER"),
            Self::WinnerOfTie => write!(f, "WINNER_OF_TIE"),
            Self::TotalGoalsOverUnder => write!(f, "TOTAL_GOALS_OVER/UNDER"),
            Self::Handicap => write!(f, "HANDICAP"),
            Self::BothTeamsToScore => write!(f, "BOTH_TEAMS_TO_SCORE"),
            Self::MatchResult1stHalf => write!(f, "MATCH_RESULT_1ST_HALF"),
        }
    }
}
impl std::str::FromStr for GroupCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "MATCH_RESULT" => Ok(Self::MatchResult),
            "TIE_BREAK_DECIDER" => Ok(Self::TieBreakDecider),
            "WINNER_OF_TIE" => Ok(Self::WinnerOfTie),
            "TOTAL_GOALS_OVER/UNDER" => Ok(Self::TotalGoalsOverUnder),
            "HANDICAP" => Ok(Self::Handicap),
            "BOTH_TEAMS_TO_SCORE" => Ok(Self::BothTeamsToScore),
            "MATCH_RESULT_1ST_HALF" => Ok(Self::MatchResult1stHalf),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for GroupCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for GroupCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for GroupCode {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Id"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ID\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"VST1\","]
#[doc = "    \"VST5\","]
#[doc = "    \"VST3\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Id {
    #[serde(rename = "VST1")]
    Vst1,
    #[serde(rename = "VST2")]
    Vst2,
    #[serde(rename = "VST3")]
    Vst3,
    #[serde(rename = "VST4")]
    Vst4,
    #[serde(rename = "VST5")]
    Vst5,
}
impl From<&Id> for Id {
    fn from(value: &Id) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Id {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Vst1 => write!(f, "VST1"),
            Self::Vst2 => write!(f, "VST2"),
            Self::Vst3 => write!(f, "VST3"),
            Self::Vst4 => write!(f, "VST4"),
            Self::Vst5 => write!(f, "VST5"),
        }
    }
}
impl std::str::FromStr for Id {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "VST1" => Ok(Self::Vst1),
            "VST2" => Ok(Self::Vst2),
            "VST3" => Ok(Self::Vst3),
            "VST4" => Ok(Self::Vst4),
            "VST5" => Ok(Self::Vst5),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Id {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Id {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Id {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Market"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Market\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"active\","]
#[doc = "    \"betInRun\","]
#[doc = "    \"blurb\","]
#[doc = "    \"cashoutAvailable\","]
#[doc = "    \"channels\","]
#[doc = "    \"collectionIds\","]
#[doc = "    \"displayOrder\","]
#[doc = "    \"displaySortId\","]
#[doc = "    \"displayed\","]
#[doc = "    \"eventId\","]
#[doc = "    \"fixedOddsAvailable\","]
#[doc = "    \"flags\","]
#[doc = "    \"groupCode\","]
#[doc = "    \"handicapValue\","]
#[doc = "    \"id\","]
#[doc = "    \"livePriceAvailable\","]
#[doc = "    \"maximumAccumulator\","]
#[doc = "    \"minimumAccumulator\","]
#[doc = "    \"name\","]
#[doc = "    \"outcomes\","]
#[doc = "    \"rcpAvailable\","]
#[doc = "    \"resulted\","]
#[doc = "    \"retailCode\","]
#[doc = "    \"status\","]
#[doc = "    \"subType\","]
#[doc = "    \"templateMarketId\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"active\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"betInRun\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"blurb\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"cashoutAvailable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"channels\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Channel\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"collectionIds\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"format\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"displayOrder\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"displaySortId\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"displayed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"eventId\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"fixedOddsAvailable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"flags\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Flag\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"groupCode\": {"]
#[doc = "      \"$ref\": \"#/definitions/GroupCode\""]
#[doc = "    },"]
#[doc = "    \"handicapValue\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"livePriceAvailable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"maximumAccumulator\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"minimumAccumulator\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/definitions/MarketName\""]
#[doc = "    },"]
#[doc = "    \"outcomes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Outcome\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"rcpAvailable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"resulted\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"retailCode\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/Status\""]
#[doc = "    },"]
#[doc = "    \"subType\": {"]
#[doc = "      \"$ref\": \"#/definitions/SubTypeEnum\""]
#[doc = "    },"]
#[doc = "    \"templateMarketId\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/definitions/PurpleType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Market {
    pub active: bool,
    #[serde(rename = "betInRun")]
    pub bet_in_run: bool,
    pub blurb: (),
    #[serde(rename = "cashoutAvailable")]
    pub cashout_available: bool,
    pub channels: Vec<Channel>,
    #[serde(rename = "collectionIds")]
    pub collection_ids: Vec<String>,
    #[serde(rename = "displayOrder")]
    pub display_order: i64,
    #[serde(rename = "displaySortId")]
    pub display_sort_id: (),
    pub displayed: bool,
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "fixedOddsAvailable")]
    pub fixed_odds_available: bool,
    pub flags: Vec<Flag>,
    #[serde(rename = "groupCode")]
    pub group_code: GroupCode,
    #[serde(rename = "handicapValue")]
    pub handicap_value: Option<f64>,
    pub id: String,
    #[serde(rename = "livePriceAvailable")]
    pub live_price_available: bool,
    #[serde(rename = "maximumAccumulator")]
    pub maximum_accumulator: i64,
    #[serde(rename = "minimumAccumulator")]
    pub minimum_accumulator: i64,
    pub name: MarketName,
    pub outcomes: Vec<Outcome>,
    #[serde(rename = "rcpAvailable")]
    pub rcp_available: bool,
    pub resulted: (),
    #[serde(rename = "retailCode")]
    pub retail_code: (),
    pub status: Status,
    #[serde(rename = "subType")]
    pub sub_type: SubTypeEnum,
    #[serde(rename = "templateMarketId")]
    pub template_market_id: String,
    #[serde(rename = "type")]
    pub type_: PurpleType,
}
impl From<&Market> for Market {
    fn from(value: &Market) -> Self {
        value.clone()
    }
}
impl Market {
    pub fn builder() -> builder::Market {
        Default::default()
    }
}
#[doc = "MarketCounts"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MarketCounts\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fixedOdds\","]
#[doc = "    \"total\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fixedOdds\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"total\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MarketCounts {
    #[serde(rename = "fixedOdds")]
    pub fixed_odds: i64,
    pub total: i64,
}
impl From<&MarketCounts> for MarketCounts {
    fn from(value: &MarketCounts) -> Self {
        value.clone()
    }
}
impl MarketCounts {
    pub fn builder() -> builder::MarketCounts {
        Default::default()
    }
}
#[doc = "MarketName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MarketName\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Match Result\","]
#[doc = "    \"Total Goals Over/Under 3.5\","]
#[doc = "    \"Total Goals Over/Under 4.5\","]
#[doc = "    \"Total Goals Over/Under 5.5\","]
#[doc = "    \"Total Goals Over/Under 6.5\","]
#[doc = "    \"Handicap 1\","]
#[doc = "    \"Handicap 2.0\","]
#[doc = "    \"Handicap -1.0\","]
#[doc = "    \"Total Goals Over/Under 7.5\","]
#[doc = "    \"Total Goals Over/Under 8.5\","]
#[doc = "    \"Handicap -2.0\","]
#[doc = "    \"Handicap -3.0\","]
#[doc = "    \"Handicap -4.0\","]
#[doc = "    \"Both Teams to Score\","]
#[doc = "    \"Total Goals Over/Under 2.5\","]
#[doc = "    \"Handicap 3.0\","]
#[doc = "    \"Handicap -5\","]
#[doc = "    \"Handicap -6.0\","]
#[doc = "    \"Total Goals Over/Under 1.5\","]
#[doc = "    \"Total Goals Over/Under 0.5\","]
#[doc = "    \"Half Time Result\","]
#[doc = "    \"Handicap 4.0\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MarketName {
    #[serde(rename = "Match Result")]
    MatchResult,
    #[serde(rename = "Hvordan bliver kampen afgjort?")]
    HvordanBliverKampenAfgjort,
    #[serde(rename = "Winner of Tie")]
    WinnerOfTie,
    #[serde(rename = "Total Goals Over/Under 3.5")]
    TotalGoalsOverUnder35,
    #[serde(rename = "Total Goals Over/Under 4.5")]
    TotalGoalsOverUnder45,
    #[serde(rename = "Total Goals Over/Under 5.5")]
    TotalGoalsOverUnder55,
    #[serde(rename = "Total Goals Over/Under 6.5")]
    TotalGoalsOverUnder65,
    #[serde(rename = "Handicap 1")]
    Handicap1,
    #[serde(rename = "Handicap 2.0")]
    Handicap20,
    #[serde(rename = "Handicap -1.0")]
    HandicapMinus10,
    #[serde(rename = "Total Goals Over/Under 7.5")]
    TotalGoalsOverUnder75,
    #[serde(rename = "Total Goals Over/Under 8.5")]
    TotalGoalsOverUnder85,
    #[serde(rename = "Handicap -2.0")]
    HandicapMinus20,
    #[serde(rename = "Handicap -3.0")]
    HandicapMinus30,
    #[serde(rename = "Handicap -4.0")]
    HandicapMinus40,
    #[serde(rename = "Both Teams to Score")]
    BothTeamsToScore,
    #[serde(rename = "Total Goals Over/Under 2.5")]
    TotalGoalsOverUnder25,
    #[serde(rename = "Handicap 3.0")]
    Handicap30,
    #[serde(rename = "Handicap -5")]
    HandicapMinus5,
    #[serde(rename = "Handicap -6.0")]
    HandicapMinus60,
    #[serde(rename = "Total Goals Over/Under 1.5")]
    TotalGoalsOverUnder15,
    #[serde(rename = "Total Goals Over/Under 0.5")]
    TotalGoalsOverUnder05,
    #[serde(rename = "Half Time Result")]
    HalfTimeResult,
    #[serde(rename = "Handicap 4.0")]
    Handicap40,
}
impl From<&MarketName> for MarketName {
    fn from(value: &MarketName) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MarketName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MatchResult => write!(f, "Match Result"),
            Self::HvordanBliverKampenAfgjort => write!(f, "Hvordan bliver kampen afgjort?"),
            Self::WinnerOfTie => write!(f, "Winner of Tie"),
            Self::TotalGoalsOverUnder35 => write!(f, "Total Goals Over/Under 3.5"),
            Self::TotalGoalsOverUnder45 => write!(f, "Total Goals Over/Under 4.5"),
            Self::TotalGoalsOverUnder55 => write!(f, "Total Goals Over/Under 5.5"),
            Self::TotalGoalsOverUnder65 => write!(f, "Total Goals Over/Under 6.5"),
            Self::Handicap1 => write!(f, "Handicap 1"),
            Self::Handicap20 => write!(f, "Handicap 2.0"),
            Self::HandicapMinus10 => write!(f, "Handicap -1.0"),
            Self::TotalGoalsOverUnder75 => write!(f, "Total Goals Over/Under 7.5"),
            Self::TotalGoalsOverUnder85 => write!(f, "Total Goals Over/Under 8.5"),
            Self::HandicapMinus20 => write!(f, "Handicap -2.0"),
            Self::HandicapMinus30 => write!(f, "Handicap -3.0"),
            Self::HandicapMinus40 => write!(f, "Handicap -4.0"),
            Self::BothTeamsToScore => write!(f, "Both Teams to Score"),
            Self::TotalGoalsOverUnder25 => write!(f, "Total Goals Over/Under 2.5"),
            Self::Handicap30 => write!(f, "Handicap 3.0"),
            Self::HandicapMinus5 => write!(f, "Handicap -5"),
            Self::HandicapMinus60 => write!(f, "Handicap -6.0"),
            Self::TotalGoalsOverUnder15 => write!(f, "Total Goals Over/Under 1.5"),
            Self::TotalGoalsOverUnder05 => write!(f, "Total Goals Over/Under 0.5"),
            Self::HalfTimeResult => write!(f, "Half Time Result"),
            Self::Handicap40 => write!(f, "Handicap 4.0"),
        }
    }
}
impl std::str::FromStr for MarketName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Match Result" => Ok(Self::MatchResult),
            "Hvordan bliver kampen afgjort?" => Ok(Self::HvordanBliverKampenAfgjort),
            "Winner of Tie" => Ok(Self::WinnerOfTie),
            "Total Goals Over/Under 3.5" => Ok(Self::TotalGoalsOverUnder35),
            "Total Goals Over/Under 4.5" => Ok(Self::TotalGoalsOverUnder45),
            "Total Goals Over/Under 5.5" => Ok(Self::TotalGoalsOverUnder55),
            "Total Goals Over/Under 6.5" => Ok(Self::TotalGoalsOverUnder65),
            "Handicap 1" => Ok(Self::Handicap1),
            "Handicap 2.0" => Ok(Self::Handicap20),
            "Handicap -1.0" => Ok(Self::HandicapMinus10),
            "Total Goals Over/Under 7.5" => Ok(Self::TotalGoalsOverUnder75),
            "Total Goals Over/Under 8.5" => Ok(Self::TotalGoalsOverUnder85),
            "Handicap -2.0" => Ok(Self::Handicap20),
            "Handicap -3.0" => Ok(Self::Handicap30),
            "Handicap -4.0" => Ok(Self::Handicap40),
            "Both Teams to Score" => Ok(Self::BothTeamsToScore),
            "Total Goals Over/Under 2.5" => Ok(Self::TotalGoalsOverUnder25),
            "Handicap 3.0" => Ok(Self::Handicap30),
            "Handicap -5" => Ok(Self::HandicapMinus5),
            "Handicap -6.0" => Ok(Self::HandicapMinus60),
            "Total Goals Over/Under 1.5" => Ok(Self::TotalGoalsOverUnder15),
            "Total Goals Over/Under 0.5" => Ok(Self::TotalGoalsOverUnder05),
            "Half Time Result" => Ok(Self::HalfTimeResult),
            "Handicap 4.0" => Ok(Self::Handicap40),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MarketName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MarketName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MarketName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Media"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Media\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"active\","]
#[doc = "    \"endTime\","]
#[doc = "    \"externalMediaIds\","]
#[doc = "    \"startTime\","]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"active\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"endTime\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"date-time\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"externalMediaIds\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ExternalMediaID\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"startTime\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Media {
    pub active: bool,
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(rename = "externalMediaIds")]
    pub external_media_ids: Vec<ExternalMediaId>,
    #[serde(rename = "startTime")]
    pub start_time: chrono::DateTime<chrono::offset::Utc>,
    pub url: (),
}
impl From<&Media> for Media {
    fn from(value: &Media) -> Self {
        value.clone()
    }
}
impl Media {
    pub fn builder() -> builder::Media {
        Default::default()
    }
}
#[doc = "MediaProvider"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MediaProvider\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"listingUrl\","]
#[doc = "    \"logoResource\","]
#[doc = "    \"media\","]
#[doc = "    \"mediaType\","]
#[doc = "    \"name\","]
#[doc = "    \"providerCode\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ID\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"listingUrl\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\","]
#[doc = "          \"qt-uri-protocols\": ["]
#[doc = "            \"http\","]
#[doc = "            \"https\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"logoResource\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"media\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Media\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"mediaType\": {"]
#[doc = "      \"$ref\": \"#/definitions/MediaType\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/definitions/MediaProviderName\""]
#[doc = "    },"]
#[doc = "    \"providerCode\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MediaProvider {
    pub id: Option<Id>,
    #[serde(rename = "listingUrl")]
    pub listing_url: Option<String>,
    #[serde(rename = "logoResource")]
    pub logo_resource: (),
    pub media: Vec<Media>,
    #[serde(rename = "mediaType")]
    pub media_type: MediaType,
    pub name: MediaProviderName,
    #[serde(rename = "providerCode")]
    pub provider_code: (),
}
impl From<&MediaProvider> for MediaProvider {
    fn from(value: &MediaProvider) -> Self {
        value.clone()
    }
}
impl MediaProvider {
    pub fn builder() -> builder::MediaProvider {
        Default::default()
    }
}
#[doc = "MediaProviderName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MediaProviderName\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Perform\","]
#[doc = "    \"Perform Retail\","]
#[doc = "    \"3+\","]
#[doc = "    \"3MAX\","]
#[doc = "    \"3SPO\","]
#[doc = "    \"TV2 Sport X\","]
#[doc = "    \"RTL\","]
#[doc = "    \"Img\","]
#[doc = "    \"DS\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MediaProviderName {
    Perform,
    #[serde(rename = "Perform Retail")]
    PerformRetail,
    #[serde(rename = "3+")]
    _3,
    #[serde(rename = "3MAX")]
    _3max,
    #[serde(rename = "3SPO")]
    _3spo,
    #[serde(rename = "Betradar")]
    Betradar,
    #[serde(rename = "Betradar")]
    DR1,
    #[serde(rename = "Betradar")]
    DR2,
    #[serde(rename = "See")]
    See,
    #[serde(rename = "TV2 Sport X")]
    Tv2SportX,
    #[serde(rename = "RTL")]
    Rtl,
    Img,
    #[serde(rename = "DS")]
    Ds,
    #[serde(other)]
    Unknown,
}
impl From<&MediaProviderName> for MediaProviderName {
    fn from(value: &MediaProviderName) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MediaProviderName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Perform => write!(f, "Perform"),
            Self::PerformRetail => write!(f, "Perform Retail"),
            Self::_3 => write!(f, "3+"),
            Self::_3max => write!(f, "3MAX"),
            Self::_3spo => write!(f, "3SPO"),
            Self::Betradar => write!(f, "Betradar"),
            Self::DR1 => write!(f, "DR1"),
            Self::DR2 => write!(f, "DR2"),
            Self::See => write!(f, "See"),
            Self::Tv2SportX => write!(f, "TV2 Sport X"),
            Self::Rtl => write!(f, "RTL"),
            Self::Img => write!(f, "Img"),
            Self::Ds => write!(f, "DS"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
impl std::str::FromStr for MediaProviderName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Perform" => Ok(Self::Perform),
            "Perform Retail" => Ok(Self::PerformRetail),
            "3+" => Ok(Self::_3),
            "3MAX" => Ok(Self::_3max),
            "3SPO" => Ok(Self::_3spo),
            "Betradar" => Ok(Self::Betradar),
            "DR1" => Ok(Self::DR1),
            "DR2" => Ok(Self::DR2),
            "TV2 Sport X" => Ok(Self::Tv2SportX),
            "RTL" => Ok(Self::Rtl),
            "Img" => Ok(Self::Img),
            "DS" => Ok(Self::Ds),
            "Unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MediaProviderName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MediaProviderName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MediaProviderName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "MediaType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MediaType\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"VIDEO\","]
#[doc = "    \"TELEVISION\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MediaType {
    #[serde(rename = "VIDEO")]
    Video,
    #[serde(rename = "TELEVISION")]
    Television,
}
impl From<&MediaType> for MediaType {
    fn from(value: &MediaType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Video => write!(f, "VIDEO"),
            Self::Television => write!(f, "TELEVISION"),
        }
    }
}
impl std::str::FromStr for MediaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "VIDEO" => Ok(Self::Video),
            "TELEVISION" => Ok(Self::Television),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MediaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MediaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MediaType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Outcome"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Outcome\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"active\","]
#[doc = "    \"channels\","]
#[doc = "    \"displayOrder\","]
#[doc = "    \"displayed\","]
#[doc = "    \"id\","]
#[doc = "    \"marketId\","]
#[doc = "    \"name\","]
#[doc = "    \"outcomeScore\","]
#[doc = "    \"prices\","]
#[doc = "    \"rcpAvailable\","]
#[doc = "    \"resulted\","]
#[doc = "    \"retailCode\","]
#[doc = "    \"runnerNumber\","]
#[doc = "    \"status\","]
#[doc = "    \"subType\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"active\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"channels\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Channel\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"displayOrder\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"displayed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"marketId\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"outcomeScore\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"prices\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Price\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"rcpAvailable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"resulted\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"retailCode\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"runnerNumber\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/Status\""]
#[doc = "    },"]
#[doc = "    \"subType\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SubType\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/definitions/SubTypeEnum\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    pub active: bool,
    pub channels: Vec<Channel>,
    #[serde(rename = "displayOrder")]
    pub display_order: i64,
    pub displayed: bool,
    pub id: String,
    #[serde(rename = "marketId")]
    pub market_id: String,
    pub name: String,
    #[serde(rename = "outcomeScore")]
    pub outcome_score: (),
    pub prices: Vec<Price>,
    #[serde(rename = "rcpAvailable")]
    pub rcp_available: bool,
    pub resulted: bool,
    #[serde(rename = "retailCode")]
    pub retail_code: (),
    #[serde(rename = "runnerNumber")]
    pub runner_number: i64,
    pub status: Status,
    #[serde(rename = "subType")]
    pub sub_type: Option<SubType>,
    #[serde(rename = "type")]
    pub type_: SubTypeEnum,
}
impl From<&Outcome> for Outcome {
    fn from(value: &Outcome) -> Self {
        value.clone()
    }
}
impl Outcome {
    pub fn builder() -> builder::Outcome {
        Default::default()
    }
}
#[doc = "Participant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Participant\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"active\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"role\","]
#[doc = "    \"roleCode\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"active\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"$ref\": \"#/definitions/Role\""]
#[doc = "    },"]
#[doc = "    \"roleCode\": {"]
#[doc = "      \"$ref\": \"#/definitions/Side\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/definitions/ParticipantType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Participant {
    pub active: bool,
    pub id: String,
    pub name: String,
    pub role: Role,
    #[serde(rename = "roleCode")]
    pub role_code: Side,
    #[serde(rename = "type")]
    pub type_: ParticipantType,
}
impl From<&Participant> for Participant {
    fn from(value: &Participant) -> Self {
        value.clone()
    }
}
impl Participant {
    pub fn builder() -> builder::Participant {
        Default::default()
    }
}
#[doc = "ParticipantType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ParticipantType\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"GROUP\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ParticipantType {
    #[serde(rename = "GROUP")]
    Group,
}
impl From<&ParticipantType> for ParticipantType {
    fn from(value: &ParticipantType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ParticipantType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Group => write!(f, "GROUP"),
        }
    }
}
impl std::str::FromStr for ParticipantType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "GROUP" => Ok(Self::Group),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ParticipantType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ParticipantType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ParticipantType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Period"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Period\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"clock\","]
#[doc = "    \"facts\","]
#[doc = "    \"id\","]
#[doc = "    \"incidents\","]
#[doc = "    \"order\","]
#[doc = "    \"periodIndex\","]
#[doc = "    \"periods\","]
#[doc = "    \"startTime\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"clock\": {"]
#[doc = "      \"$ref\": \"#/definitions/Clock\""]
#[doc = "    },"]
#[doc = "    \"facts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Fact\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"incidents\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {}"]
#[doc = "    },"]
#[doc = "    \"order\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"periodIndex\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"periods\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {}"]
#[doc = "    },"]
#[doc = "    \"startTime\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/definitions/PeriodType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Period {
    pub clock: Clock,
    pub facts: Vec<Fact>,
    pub id: String,
    pub incidents: Vec<::serde_json::Value>,
    pub order: i64,
    #[serde(rename = "periodIndex")]
    pub period_index: (),
    pub periods: Vec<::serde_json::Value>,
    #[serde(rename = "startTime")]
    pub start_time: chrono::DateTime<chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: PeriodType,
}
impl From<&Period> for Period {
    fn from(value: &Period) -> Self {
        value.clone()
    }
}
impl Period {
    pub fn builder() -> builder::Period {
        Default::default()
    }
}
#[doc = "PeriodType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PeriodType\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"SECOND_HALF\","]
#[doc = "    \"FIRST_HALF\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PeriodType {
    #[serde(rename = "SECOND_HALF")]
    SecondHalf,
    #[serde(rename = "FIRST_HALF")]
    FirstHalf,
}
impl From<&PeriodType> for PeriodType {
    fn from(value: &PeriodType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PeriodType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SecondHalf => write!(f, "SECOND_HALF"),
            Self::FirstHalf => write!(f, "FIRST_HALF"),
        }
    }
}
impl std::str::FromStr for PeriodType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "SECOND_HALF" => Ok(Self::SecondHalf),
            "FIRST_HALF" => Ok(Self::FirstHalf),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PeriodType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PeriodType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PeriodType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Price"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Price\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"decimal\","]
#[doc = "    \"denominator\","]
#[doc = "    \"displayOrder\","]
#[doc = "    \"handicapHigh\","]
#[doc = "    \"handicapLow\","]
#[doc = "    \"numerator\","]
#[doc = "    \"priceType\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"decimal\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"denominator\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"displayOrder\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"handicapHigh\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"handicapLow\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"numerator\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"priceType\": {"]
#[doc = "      \"$ref\": \"#/definitions/PriceType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Price {
    pub decimal: f64,
    pub denominator: i64,
    #[serde(rename = "displayOrder")]
    pub display_order: i64,
    #[serde(rename = "handicapHigh")]
    pub handicap_high: Option<String>,
    #[serde(rename = "handicapLow")]
    pub handicap_low: Option<String>,
    pub numerator: i64,
    #[serde(rename = "priceType")]
    pub price_type: PriceType,
}
impl From<&Price> for Price {
    fn from(value: &Price) -> Self {
        value.clone()
    }
}
impl Price {
    pub fn builder() -> builder::Price {
        Default::default()
    }
}
#[doc = "PriceType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PriceType\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"LP\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PriceType {
    #[serde(rename = "LP")]
    Lp,
}
impl From<&PriceType> for PriceType {
    fn from(value: &PriceType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PriceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Lp => write!(f, "LP"),
        }
    }
}
impl std::str::FromStr for PriceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "LP" => Ok(Self::Lp),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PriceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PriceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PriceType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Provider"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Provider\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"betgenius\","]
#[doc = "    \"betradar\","]
#[doc = "    \"lsports\","]
#[doc = "    \"enetpulse\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Provider {
    #[serde(rename = "betgenius")]
    Betgenius,
    #[serde(rename = "betradar")]
    Betradar,
    #[serde(rename = "lsports")]
    Lsports,
    #[serde(rename = "enetpulse")]
    Enetpulse,
}
impl From<&Provider> for Provider {
    fn from(value: &Provider) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Provider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Betgenius => write!(f, "betgenius"),
            Self::Betradar => write!(f, "betradar"),
            Self::Lsports => write!(f, "lsports"),
            Self::Enetpulse => write!(f, "enetpulse"),
        }
    }
}
impl std::str::FromStr for Provider {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "betgenius" => Ok(Self::Betgenius),
            "betradar" => Ok(Self::Betradar),
            "lsports" => Ok(Self::Lsports),
            "enetpulse" => Ok(Self::Enetpulse),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Provider {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Provider {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Provider {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "PurpleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PurpleType\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"-\","]
#[doc = "    \"L\","]
#[doc = "    \"M\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PurpleType {
    #[serde(rename = "-")]
    X,
    L,
    M,
}
impl From<&PurpleType> for PurpleType {
    fn from(value: &PurpleType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PurpleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X => write!(f, "-"),
            Self::L => write!(f, "L"),
            Self::M => write!(f, "M"),
        }
    }
}
impl std::str::FromStr for PurpleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "-" => Ok(Self::X),
            "L" => Ok(Self::L),
            "M" => Ok(Self::M),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PurpleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PurpleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PurpleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Role"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Role\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Home Team\","]
#[doc = "    \"Away Team\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Role {
    #[serde(rename = "Home Team")]
    HomeTeam,
    #[serde(rename = "Away Team")]
    AwayTeam,
}
impl From<&Role> for Role {
    fn from(value: &Role) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Role {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::HomeTeam => write!(f, "Home Team"),
            Self::AwayTeam => write!(f, "Away Team"),
        }
    }
}
impl std::str::FromStr for Role {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Home Team" => Ok(Self::HomeTeam),
            "Away Team" => Ok(Self::AwayTeam),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Role {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Side"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Side\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"HOME\","]
#[doc = "    \"AWAY\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Side {
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "AWAY")]
    Away,
}
impl From<&Side> for Side {
    fn from(value: &Side) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Side {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Home => write!(f, "HOME"),
            Self::Away => write!(f, "AWAY"),
        }
    }
}
impl std::str::FromStr for Side {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "HOME" => Ok(Self::Home),
            "AWAY" => Ok(Self::Away),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Side {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Side {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Side {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "SortCode"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SortCode\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"MTCH\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SortCode {
    #[serde(rename = "MTCH")]
    Mtch,
}
impl From<&SortCode> for SortCode {
    fn from(value: &SortCode) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SortCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mtch => write!(f, "MTCH"),
        }
    }
}
impl std::str::FromStr for SortCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "MTCH" => Ok(Self::Mtch),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for SortCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SortCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SortCode {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "State"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"State\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"RUNNING\","]
#[doc = "    \"STOPPED\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum State {
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
}
impl From<&State> for State {
    fn from(value: &State) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for State {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Running => write!(f, "RUNNING"),
            Self::Stopped => write!(f, "STOPPED"),
        }
    }
}
impl std::str::FromStr for State {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "RUNNING" => Ok(Self::Running),
            "STOPPED" => Ok(Self::Stopped),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for State {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for State {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for State {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Status"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Status\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ACTIVE\","]
#[doc = "    \"SUSPENDED\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Status {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "SUSPENDED")]
    Suspended,
}
impl From<&Status> for Status {
    fn from(value: &Status) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => write!(f, "ACTIVE"),
            Self::Suspended => write!(f, "SUSPENDED"),
        }
    }
}
impl std::str::FromStr for Status {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ACTIVE" => Ok(Self::Active),
            "SUSPENDED" => Ok(Self::Suspended),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Status {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Status {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Status {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "SubType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SubType\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"A\","]
#[doc = "    \"H\","]
#[doc = "    \"D\","]
#[doc = "    \"L\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SubType {
    A,
    H,
    D,
    L,
}
impl From<&SubType> for SubType {
    fn from(value: &SubType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SubType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::A => write!(f, "A"),
            Self::H => write!(f, "H"),
            Self::D => write!(f, "D"),
            Self::L => write!(f, "L"),
        }
    }
}
impl std::str::FromStr for SubType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "A" => Ok(Self::A),
            "H" => Ok(Self::H),
            "D" => Ok(Self::D),
            "L" => Ok(Self::L),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for SubType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SubType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SubType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "SubTypeEnum"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SubTypeEnum\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"MR\","]
#[doc = "    \"HL\","]
#[doc = "    \"MH\","]
#[doc = "    \"--\","]
#[doc = "    \"H1\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SubTypeEnum {
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "HL")]
    Hl,
    #[serde(rename = "MH")]
    Mh,
    #[serde(rename = "--")]
    X,
    H1,
}
impl From<&SubTypeEnum> for SubTypeEnum {
    fn from(value: &SubTypeEnum) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SubTypeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mr => write!(f, "MR"),
            Self::Hl => write!(f, "HL"),
            Self::Mh => write!(f, "MH"),
            Self::X => write!(f, "--"),
            Self::H1 => write!(f, "H1"),
        }
    }
}
impl std::str::FromStr for SubTypeEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "MR" => Ok(Self::Mr),
            "HL" => Ok(Self::Hl),
            "MH" => Ok(Self::Mh),
            "--" => Ok(Self::X),
            "H1" => Ok(Self::H1),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for SubTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SubTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SubTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Team"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Team\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alternativeNames\","]
#[doc = "    \"code\","]
#[doc = "    \"externalId\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"side\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alternativeNames\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AlternativeName\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"code\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"externalId\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"side\": {"]
#[doc = "      \"$ref\": \"#/definitions/Side\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Team {
    #[serde(rename = "alternativeNames")]
    pub alternative_names: Vec<AlternativeName>,
    pub code: (),
    #[serde(rename = "externalId")]
    pub external_id: (),
    pub id: (),
    pub name: String,
    pub side: Side,
    pub status: (),
}
impl From<&Team> for Team {
    fn from(value: &Team) -> Self {
        value.clone()
    }
}
impl Team {
    pub fn builder() -> builder::Team {
        Default::default()
    }
}
#[doc = "TimeBandEvent"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TimeBandEvent\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"competitionSummary\","]
#[doc = "    \"date\","]
#[doc = "    \"events\","]
#[doc = "    \"outrights\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"competitionSummary\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/CompetitionSummary\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"date\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"date-time\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"events\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Event\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"outrights\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {}"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TimeBandEvent {
    #[serde(rename = "competitionSummary")]
    pub competition_summary: Vec<CompetitionSummary>,
    pub date: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub events: Vec<Event>,
    pub outrights: Vec<::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&TimeBandEvent> for TimeBandEvent {
    fn from(value: &TimeBandEvent) -> Self {
        value.clone()
    }
}
impl TimeBandEvent {
    pub fn builder() -> builder::TimeBandEvent {
        Default::default()
    }
}
#[doc = "TimeBandEventList"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TimeBandEventList\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"errors\","]
#[doc = "    \"extensions\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"$ref\": \"#/definitions/Data\""]
#[doc = "    },"]
#[doc = "    \"errors\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {}"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TimeBandEventList {
    pub data: Data,
    pub errors: Vec<::serde_json::Value>,
    pub extensions: (),
}
impl From<&TimeBandEventList> for TimeBandEventList {
    fn from(value: &TimeBandEventList) -> Self {
        value.clone()
    }
}
impl TimeBandEventList {
    pub fn builder() -> builder::TimeBandEventList {
        Default::default()
    }
}
#[doc = "TypeClass"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TypeClass\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alternativeNames\","]
#[doc = "    \"displayOrder\","]
#[doc = "    \"fixedOddsAvailable\","]
#[doc = "    \"id\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alternativeNames\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {}"]
#[doc = "    },"]
#[doc = "    \"displayOrder\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"fixedOddsAvailable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeClass {
    #[serde(rename = "alternativeNames")]
    pub alternative_names: Vec<::serde_json::Value>,
    #[serde(rename = "displayOrder")]
    pub display_order: String,
    #[serde(rename = "fixedOddsAvailable")]
    pub fixed_odds_available: bool,
    pub id: String,
    pub name: String,
}
impl From<&TypeClass> for TypeClass {
    fn from(value: &TypeClass) -> Self {
        value.clone()
    }
}
impl TypeClass {
    pub fn builder() -> builder::TypeClass {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AlternativeName {
        type_: Result<super::AlternativeNameType, String>,
        value: Result<String, String>,
    }
    impl Default for AlternativeName {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl AlternativeName {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AlternativeNameType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AlternativeName> for super::AlternativeName {
        type Error = super::error::ConversionError;
        fn try_from(value: AlternativeName) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::AlternativeName> for AlternativeName {
        fn from(value: super::AlternativeName) -> Self {
            Self {
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Category {
        code: Result<Option<super::Code>, String>,
        display_order: Result<String, String>,
        id: Result<String, String>,
        name: Result<String, String>,
    }
    impl Default for Category {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                display_order: Err("no value supplied for display_order".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl Category {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Code>>,
            T::Error: std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn display_order<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.display_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_order: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Category> for super::Category {
        type Error = super::error::ConversionError;
        fn try_from(value: Category) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                display_order: value.display_order?,
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl From<super::Category> for Category {
        fn from(value: super::Category) -> Self {
            Self {
                code: Ok(value.code),
                display_order: Ok(value.display_order),
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Clock {
        id: Result<String, String>,
        last_update: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        offset: Result<i64, String>,
        state: Result<super::State, String>,
    }
    impl Default for Clock {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                last_update: Err("no value supplied for last_update".to_string()),
                offset: Err("no value supplied for offset".to_string()),
                state: Err("no value supplied for state".to_string()),
            }
        }
    }
    impl Clock {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn last_update<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.last_update = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_update: {}", e));
            self
        }
        pub fn offset<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.offset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for offset: {}", e));
            self
        }
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::State>,
            T::Error: std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Clock> for super::Clock {
        type Error = super::error::ConversionError;
        fn try_from(value: Clock) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                last_update: value.last_update?,
                offset: value.offset?,
                state: value.state?,
            })
        }
    }
    impl From<super::Clock> for Clock {
        fn from(value: super::Clock) -> Self {
            Self {
                id: Ok(value.id),
                last_update: Ok(value.last_update),
                offset: Ok(value.offset),
                state: Ok(value.state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Commentary {
        clock: Result<Option<super::Clock>, String>,
        facts: Result<Vec<super::Fact>, String>,
        participants: Result<Vec<super::Participant>, String>,
        periods: Result<Vec<super::Period>, String>,
    }
    impl Default for Commentary {
        fn default() -> Self {
            Self {
                clock: Err("no value supplied for clock".to_string()),
                facts: Err("no value supplied for facts".to_string()),
                participants: Err("no value supplied for participants".to_string()),
                periods: Err("no value supplied for periods".to_string()),
            }
        }
    }
    impl Commentary {
        pub fn clock<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Clock>>,
            T::Error: std::fmt::Display,
        {
            self.clock = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clock: {}", e));
            self
        }
        pub fn facts<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Fact>>,
            T::Error: std::fmt::Display,
        {
            self.facts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for facts: {}", e));
            self
        }
        pub fn participants<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Participant>>,
            T::Error: std::fmt::Display,
        {
            self.participants = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for participants: {}", e));
            self
        }
        pub fn periods<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Period>>,
            T::Error: std::fmt::Display,
        {
            self.periods = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for periods: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Commentary> for super::Commentary {
        type Error = super::error::ConversionError;
        fn try_from(value: Commentary) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                clock: value.clock?,
                facts: value.facts?,
                participants: value.participants?,
                periods: value.periods?,
            })
        }
    }
    impl From<super::Commentary> for Commentary {
        fn from(value: super::Commentary) -> Self {
            Self {
                clock: Ok(value.clock),
                facts: Ok(value.facts),
                participants: Ok(value.participants),
                periods: Ok(value.periods),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompetitionSummary {
        competition_drilldown_tag_id: Result<String, String>,
        event_count: Result<i64, String>,
        market_count: Result<Option<i64>, String>,
        type_ids: Result<Vec<String>, String>,
    }
    impl Default for CompetitionSummary {
        fn default() -> Self {
            Self {
                competition_drilldown_tag_id: Err(
                    "no value supplied for competition_drilldown_tag_id".to_string(),
                ),
                event_count: Err("no value supplied for event_count".to_string()),
                market_count: Err("no value supplied for market_count".to_string()),
                type_ids: Err("no value supplied for type_ids".to_string()),
            }
        }
    }
    impl CompetitionSummary {
        pub fn competition_drilldown_tag_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.competition_drilldown_tag_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for competition_drilldown_tag_id: {}",
                    e
                )
            });
            self
        }
        pub fn event_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.event_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_count: {}", e));
            self
        }
        pub fn market_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.market_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for market_count: {}", e));
            self
        }
        pub fn type_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.type_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_ids: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CompetitionSummary> for super::CompetitionSummary {
        type Error = super::error::ConversionError;
        fn try_from(value: CompetitionSummary) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                competition_drilldown_tag_id: value.competition_drilldown_tag_id?,
                event_count: value.event_count?,
                market_count: value.market_count?,
                type_ids: value.type_ids?,
            })
        }
    }
    impl From<super::CompetitionSummary> for CompetitionSummary {
        fn from(value: super::CompetitionSummary) -> Self {
            Self {
                competition_drilldown_tag_id: Ok(value.competition_drilldown_tag_id),
                event_count: Ok(value.event_count),
                market_count: Ok(value.market_count),
                type_ids: Ok(value.type_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Data {
        time_band_events: Result<Vec<super::TimeBandEvent>, String>,
    }
    impl Default for Data {
        fn default() -> Self {
            Self {
                time_band_events: Err("no value supplied for time_band_events".to_string()),
            }
        }
    }
    impl Data {
        pub fn time_band_events<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::TimeBandEvent>>,
            T::Error: std::fmt::Display,
        {
            self.time_band_events = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for time_band_events: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<Data> for super::Data {
        type Error = super::error::ConversionError;
        fn try_from(value: Data) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                time_band_events: value.time_band_events?,
            })
        }
    }
    impl From<super::Data> for Data {
        fn from(value: super::Data) -> Self {
            Self {
                time_band_events: Ok(value.time_band_events),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Event {
        active: Result<bool, String>,
        blurb: Result<(), String>,
        cashout_available: Result<bool, String>,
        category: Result<super::Category, String>,
        channels: Result<Vec<super::Channel>, String>,
        class: Result<super::Category, String>,
        commentary: Result<Option<super::Commentary>, String>,
        competition_drilldown_tag_id: Result<String, String>,
        display_order: Result<i64, String>,
        displayed: Result<bool, String>,
        ext_key: Result<(), String>,
        external_ids: Result<Vec<super::ExternalId>, String>,
        fixed_odds_available: Result<bool, String>,
        id: Result<String, String>,
        is_void: Result<bool, String>,
        live_betting_available: Result<bool, String>,
        live_now: Result<bool, String>,
        market_count: Result<i64, String>,
        market_counts: Result<super::MarketCounts, String>,
        markets: Result<Vec<super::Market>, String>,
        media_providers: Result<Vec<super::MediaProvider>, String>,
        meeting: Result<(), String>,
        name: Result<String, String>,
        neutral_venue: Result<bool, String>,
        popularity_order: Result<(), String>,
        race_number: Result<i64, String>,
        rcp_available: Result<bool, String>,
        resulted: Result<bool, String>,
        retail_code: Result<(), String>,
        settled: Result<bool, String>,
        sort_code: Result<super::SortCode, String>,
        sport_id: Result<String, String>,
        start_time: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        started: Result<bool, String>,
        statistics_available: Result<bool, String>,
        status: Result<super::Status, String>,
        teams: Result<Vec<super::Team>, String>,
        type_: Result<super::TypeClass, String>,
        venue: Result<(), String>,
    }
    impl Default for Event {
        fn default() -> Self {
            Self {
                active: Err("no value supplied for active".to_string()),
                blurb: Err("no value supplied for blurb".to_string()),
                cashout_available: Err("no value supplied for cashout_available".to_string()),
                category: Err("no value supplied for category".to_string()),
                channels: Err("no value supplied for channels".to_string()),
                class: Err("no value supplied for class".to_string()),
                commentary: Err("no value supplied for commentary".to_string()),
                competition_drilldown_tag_id: Err(
                    "no value supplied for competition_drilldown_tag_id".to_string(),
                ),
                display_order: Err("no value supplied for display_order".to_string()),
                displayed: Err("no value supplied for displayed".to_string()),
                ext_key: Err("no value supplied for ext_key".to_string()),
                external_ids: Err("no value supplied for external_ids".to_string()),
                fixed_odds_available: Err("no value supplied for fixed_odds_available".to_string()),
                id: Err("no value supplied for id".to_string()),
                is_void: Err("no value supplied for is_void".to_string()),
                live_betting_available: Err(
                    "no value supplied for live_betting_available".to_string()
                ),
                live_now: Err("no value supplied for live_now".to_string()),
                market_count: Err("no value supplied for market_count".to_string()),
                market_counts: Err("no value supplied for market_counts".to_string()),
                markets: Err("no value supplied for markets".to_string()),
                media_providers: Err("no value supplied for media_providers".to_string()),
                meeting: Err("no value supplied for meeting".to_string()),
                name: Err("no value supplied for name".to_string()),
                neutral_venue: Err("no value supplied for neutral_venue".to_string()),
                popularity_order: Err("no value supplied for popularity_order".to_string()),
                race_number: Err("no value supplied for race_number".to_string()),
                rcp_available: Err("no value supplied for rcp_available".to_string()),
                resulted: Err("no value supplied for resulted".to_string()),
                retail_code: Err("no value supplied for retail_code".to_string()),
                settled: Err("no value supplied for settled".to_string()),
                sort_code: Err("no value supplied for sort_code".to_string()),
                sport_id: Err("no value supplied for sport_id".to_string()),
                start_time: Err("no value supplied for start_time".to_string()),
                started: Err("no value supplied for started".to_string()),
                statistics_available: Err("no value supplied for statistics_available".to_string()),
                status: Err("no value supplied for status".to_string()),
                teams: Err("no value supplied for teams".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                venue: Err("no value supplied for venue".to_string()),
            }
        }
    }
    impl Event {
        pub fn active<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.active = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for active: {}", e));
            self
        }
        pub fn blurb<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.blurb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for blurb: {}", e));
            self
        }
        pub fn cashout_available<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.cashout_available = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for cashout_available: {}",
                    e
                )
            });
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Category>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn channels<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Channel>>,
            T::Error: std::fmt::Display,
        {
            self.channels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channels: {}", e));
            self
        }
        pub fn class<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Category>,
            T::Error: std::fmt::Display,
        {
            self.class = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for class: {}", e));
            self
        }
        pub fn commentary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Commentary>>,
            T::Error: std::fmt::Display,
        {
            self.commentary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for commentary: {}", e));
            self
        }
        pub fn competition_drilldown_tag_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.competition_drilldown_tag_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for competition_drilldown_tag_id: {}",
                    e
                )
            });
            self
        }
        pub fn display_order<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.display_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_order: {}", e));
            self
        }
        pub fn displayed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.displayed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for displayed: {}", e));
            self
        }
        pub fn ext_key<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.ext_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ext_key: {}", e));
            self
        }
        pub fn external_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ExternalId>>,
            T::Error: std::fmt::Display,
        {
            self.external_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_ids: {}", e));
            self
        }
        pub fn fixed_odds_available<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.fixed_odds_available = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for fixed_odds_available: {}",
                    e
                )
            });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn is_void<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.is_void = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_void: {}", e));
            self
        }
        pub fn live_betting_available<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.live_betting_available = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for live_betting_available: {}",
                    e
                )
            });
            self
        }
        pub fn live_now<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.live_now = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for live_now: {}", e));
            self
        }
        pub fn market_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.market_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for market_count: {}", e));
            self
        }
        pub fn market_counts<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::MarketCounts>,
            T::Error: std::fmt::Display,
        {
            self.market_counts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for market_counts: {}", e));
            self
        }
        pub fn markets<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Market>>,
            T::Error: std::fmt::Display,
        {
            self.markets = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for markets: {}", e));
            self
        }
        pub fn media_providers<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::MediaProvider>>,
            T::Error: std::fmt::Display,
        {
            self.media_providers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_providers: {}", e));
            self
        }
        pub fn meeting<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.meeting = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meeting: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn neutral_venue<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.neutral_venue = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for neutral_venue: {}", e));
            self
        }
        pub fn popularity_order<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.popularity_order = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for popularity_order: {}",
                    e
                )
            });
            self
        }
        pub fn race_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.race_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for race_number: {}", e));
            self
        }
        pub fn rcp_available<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.rcp_available = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rcp_available: {}", e));
            self
        }
        pub fn resulted<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.resulted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resulted: {}", e));
            self
        }
        pub fn retail_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.retail_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for retail_code: {}", e));
            self
        }
        pub fn settled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.settled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for settled: {}", e));
            self
        }
        pub fn sort_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SortCode>,
            T::Error: std::fmt::Display,
        {
            self.sort_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sort_code: {}", e));
            self
        }
        pub fn sport_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.sport_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sport_id: {}", e));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {}", e));
            self
        }
        pub fn started<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.started = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for started: {}", e));
            self
        }
        pub fn statistics_available<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.statistics_available = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for statistics_available: {}",
                    e
                )
            });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Status>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn teams<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Team>>,
            T::Error: std::fmt::Display,
        {
            self.teams = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for teams: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TypeClass>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn venue<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.venue = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for venue: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Event> for super::Event {
        type Error = super::error::ConversionError;
        fn try_from(value: Event) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                active: value.active?,
                blurb: value.blurb?,
                cashout_available: value.cashout_available?,
                category: value.category?,
                channels: value.channels?,
                class: value.class?,
                commentary: value.commentary?,
                competition_drilldown_tag_id: value.competition_drilldown_tag_id?,
                display_order: value.display_order?,
                displayed: value.displayed?,
                ext_key: value.ext_key?,
                external_ids: value.external_ids?,
                fixed_odds_available: value.fixed_odds_available?,
                id: value.id?,
                is_void: value.is_void?,
                live_betting_available: value.live_betting_available?,
                live_now: value.live_now?,
                market_count: value.market_count?,
                market_counts: value.market_counts?,
                markets: value.markets?,
                media_providers: value.media_providers?,
                meeting: value.meeting?,
                name: value.name?,
                neutral_venue: value.neutral_venue?,
                popularity_order: value.popularity_order?,
                race_number: value.race_number?,
                rcp_available: value.rcp_available?,
                resulted: value.resulted?,
                retail_code: value.retail_code?,
                settled: value.settled?,
                sort_code: value.sort_code?,
                sport_id: value.sport_id?,
                start_time: value.start_time?,
                started: value.started?,
                statistics_available: value.statistics_available?,
                status: value.status?,
                teams: value.teams?,
                type_: value.type_?,
                venue: value.venue?,
            })
        }
    }
    impl From<super::Event> for Event {
        fn from(value: super::Event) -> Self {
            Self {
                active: Ok(value.active),
                blurb: Ok(value.blurb),
                cashout_available: Ok(value.cashout_available),
                category: Ok(value.category),
                channels: Ok(value.channels),
                class: Ok(value.class),
                commentary: Ok(value.commentary),
                competition_drilldown_tag_id: Ok(value.competition_drilldown_tag_id),
                display_order: Ok(value.display_order),
                displayed: Ok(value.displayed),
                ext_key: Ok(value.ext_key),
                external_ids: Ok(value.external_ids),
                fixed_odds_available: Ok(value.fixed_odds_available),
                id: Ok(value.id),
                is_void: Ok(value.is_void),
                live_betting_available: Ok(value.live_betting_available),
                live_now: Ok(value.live_now),
                market_count: Ok(value.market_count),
                market_counts: Ok(value.market_counts),
                markets: Ok(value.markets),
                media_providers: Ok(value.media_providers),
                meeting: Ok(value.meeting),
                name: Ok(value.name),
                neutral_venue: Ok(value.neutral_venue),
                popularity_order: Ok(value.popularity_order),
                race_number: Ok(value.race_number),
                rcp_available: Ok(value.rcp_available),
                resulted: Ok(value.resulted),
                retail_code: Ok(value.retail_code),
                settled: Ok(value.settled),
                sort_code: Ok(value.sort_code),
                sport_id: Ok(value.sport_id),
                start_time: Ok(value.start_time),
                started: Ok(value.started),
                statistics_available: Ok(value.statistics_available),
                status: Ok(value.status),
                teams: Ok(value.teams),
                type_: Ok(value.type_),
                venue: Ok(value.venue),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExternalId {
        id: Result<String, String>,
        provider: Result<super::Provider, String>,
    }
    impl Default for ExternalId {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                provider: Err("no value supplied for provider".to_string()),
            }
        }
    }
    impl ExternalId {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn provider<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Provider>,
            T::Error: std::fmt::Display,
        {
            self.provider = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for provider: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ExternalId> for super::ExternalId {
        type Error = super::error::ConversionError;
        fn try_from(value: ExternalId) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                provider: value.provider?,
            })
        }
    }
    impl From<super::ExternalId> for ExternalId {
        fn from(value: super::ExternalId) -> Self {
            Self {
                id: Ok(value.id),
                provider: Ok(value.provider),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExternalMediaId {
        id: Result<String, String>,
        type_: Result<super::ExternalMediaIdType, String>,
    }
    impl Default for ExternalMediaId {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ExternalMediaId {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ExternalMediaIdType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ExternalMediaId> for super::ExternalMediaId {
        type Error = super::error::ConversionError;
        fn try_from(value: ExternalMediaId) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ExternalMediaId> for ExternalMediaId {
        fn from(value: super::ExternalMediaId) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Fact {
        id: Result<String, String>,
        participant_id: Result<String, String>,
        type_: Result<super::FactType, String>,
        value: Result<String, String>,
    }
    impl Default for Fact {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                participant_id: Err("no value supplied for participant_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Fact {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn participant_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.participant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for participant_id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FactType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Fact> for super::Fact {
        type Error = super::error::ConversionError;
        fn try_from(value: Fact) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                participant_id: value.participant_id?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::Fact> for Fact {
        fn from(value: super::Fact) -> Self {
            Self {
                id: Ok(value.id),
                participant_id: Ok(value.participant_id),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Market {
        active: Result<bool, String>,
        bet_in_run: Result<bool, String>,
        blurb: Result<(), String>,
        cashout_available: Result<bool, String>,
        channels: Result<Vec<super::Channel>, String>,
        collection_ids: Result<Vec<String>, String>,
        display_order: Result<i64, String>,
        display_sort_id: Result<(), String>,
        displayed: Result<bool, String>,
        event_id: Result<String, String>,
        fixed_odds_available: Result<bool, String>,
        flags: Result<Vec<super::Flag>, String>,
        group_code: Result<super::GroupCode, String>,
        handicap_value: Result<Option<f64>, String>,
        id: Result<String, String>,
        live_price_available: Result<bool, String>,
        maximum_accumulator: Result<i64, String>,
        minimum_accumulator: Result<i64, String>,
        name: Result<super::MarketName, String>,
        outcomes: Result<Vec<super::Outcome>, String>,
        rcp_available: Result<bool, String>,
        resulted: Result<(), String>,
        retail_code: Result<(), String>,
        status: Result<super::Status, String>,
        sub_type: Result<super::SubTypeEnum, String>,
        template_market_id: Result<String, String>,
        type_: Result<super::PurpleType, String>,
    }
    impl Default for Market {
        fn default() -> Self {
            Self {
                active: Err("no value supplied for active".to_string()),
                bet_in_run: Err("no value supplied for bet_in_run".to_string()),
                blurb: Err("no value supplied for blurb".to_string()),
                cashout_available: Err("no value supplied for cashout_available".to_string()),
                channels: Err("no value supplied for channels".to_string()),
                collection_ids: Err("no value supplied for collection_ids".to_string()),
                display_order: Err("no value supplied for display_order".to_string()),
                display_sort_id: Err("no value supplied for display_sort_id".to_string()),
                displayed: Err("no value supplied for displayed".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                fixed_odds_available: Err("no value supplied for fixed_odds_available".to_string()),
                flags: Err("no value supplied for flags".to_string()),
                group_code: Err("no value supplied for group_code".to_string()),
                handicap_value: Err("no value supplied for handicap_value".to_string()),
                id: Err("no value supplied for id".to_string()),
                live_price_available: Err("no value supplied for live_price_available".to_string()),
                maximum_accumulator: Err("no value supplied for maximum_accumulator".to_string()),
                minimum_accumulator: Err("no value supplied for minimum_accumulator".to_string()),
                name: Err("no value supplied for name".to_string()),
                outcomes: Err("no value supplied for outcomes".to_string()),
                rcp_available: Err("no value supplied for rcp_available".to_string()),
                resulted: Err("no value supplied for resulted".to_string()),
                retail_code: Err("no value supplied for retail_code".to_string()),
                status: Err("no value supplied for status".to_string()),
                sub_type: Err("no value supplied for sub_type".to_string()),
                template_market_id: Err("no value supplied for template_market_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Market {
        pub fn active<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.active = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for active: {}", e));
            self
        }
        pub fn bet_in_run<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.bet_in_run = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bet_in_run: {}", e));
            self
        }
        pub fn blurb<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.blurb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for blurb: {}", e));
            self
        }
        pub fn cashout_available<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.cashout_available = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for cashout_available: {}",
                    e
                )
            });
            self
        }
        pub fn channels<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Channel>>,
            T::Error: std::fmt::Display,
        {
            self.channels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channels: {}", e));
            self
        }
        pub fn collection_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.collection_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for collection_ids: {}", e));
            self
        }
        pub fn display_order<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.display_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_order: {}", e));
            self
        }
        pub fn display_sort_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.display_sort_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_sort_id: {}", e));
            self
        }
        pub fn displayed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.displayed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for displayed: {}", e));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {}", e));
            self
        }
        pub fn fixed_odds_available<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.fixed_odds_available = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for fixed_odds_available: {}",
                    e
                )
            });
            self
        }
        pub fn flags<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Flag>>,
            T::Error: std::fmt::Display,
        {
            self.flags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flags: {}", e));
            self
        }
        pub fn group_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::GroupCode>,
            T::Error: std::fmt::Display,
        {
            self.group_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_code: {}", e));
            self
        }
        pub fn handicap_value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.handicap_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for handicap_value: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn live_price_available<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.live_price_available = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for live_price_available: {}",
                    e
                )
            });
            self
        }
        pub fn maximum_accumulator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.maximum_accumulator = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for maximum_accumulator: {}",
                    e
                )
            });
            self
        }
        pub fn minimum_accumulator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.minimum_accumulator = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for minimum_accumulator: {}",
                    e
                )
            });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::MarketName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn outcomes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Outcome>>,
            T::Error: std::fmt::Display,
        {
            self.outcomes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for outcomes: {}", e));
            self
        }
        pub fn rcp_available<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.rcp_available = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rcp_available: {}", e));
            self
        }
        pub fn resulted<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.resulted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resulted: {}", e));
            self
        }
        pub fn retail_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.retail_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for retail_code: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Status>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn sub_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SubTypeEnum>,
            T::Error: std::fmt::Display,
        {
            self.sub_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub_type: {}", e));
            self
        }
        pub fn template_market_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.template_market_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for template_market_id: {}",
                    e
                )
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PurpleType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Market> for super::Market {
        type Error = super::error::ConversionError;
        fn try_from(value: Market) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                active: value.active?,
                bet_in_run: value.bet_in_run?,
                blurb: value.blurb?,
                cashout_available: value.cashout_available?,
                channels: value.channels?,
                collection_ids: value.collection_ids?,
                display_order: value.display_order?,
                display_sort_id: value.display_sort_id?,
                displayed: value.displayed?,
                event_id: value.event_id?,
                fixed_odds_available: value.fixed_odds_available?,
                flags: value.flags?,
                group_code: value.group_code?,
                handicap_value: value.handicap_value?,
                id: value.id?,
                live_price_available: value.live_price_available?,
                maximum_accumulator: value.maximum_accumulator?,
                minimum_accumulator: value.minimum_accumulator?,
                name: value.name?,
                outcomes: value.outcomes?,
                rcp_available: value.rcp_available?,
                resulted: value.resulted?,
                retail_code: value.retail_code?,
                status: value.status?,
                sub_type: value.sub_type?,
                template_market_id: value.template_market_id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Market> for Market {
        fn from(value: super::Market) -> Self {
            Self {
                active: Ok(value.active),
                bet_in_run: Ok(value.bet_in_run),
                blurb: Ok(value.blurb),
                cashout_available: Ok(value.cashout_available),
                channels: Ok(value.channels),
                collection_ids: Ok(value.collection_ids),
                display_order: Ok(value.display_order),
                display_sort_id: Ok(value.display_sort_id),
                displayed: Ok(value.displayed),
                event_id: Ok(value.event_id),
                fixed_odds_available: Ok(value.fixed_odds_available),
                flags: Ok(value.flags),
                group_code: Ok(value.group_code),
                handicap_value: Ok(value.handicap_value),
                id: Ok(value.id),
                live_price_available: Ok(value.live_price_available),
                maximum_accumulator: Ok(value.maximum_accumulator),
                minimum_accumulator: Ok(value.minimum_accumulator),
                name: Ok(value.name),
                outcomes: Ok(value.outcomes),
                rcp_available: Ok(value.rcp_available),
                resulted: Ok(value.resulted),
                retail_code: Ok(value.retail_code),
                status: Ok(value.status),
                sub_type: Ok(value.sub_type),
                template_market_id: Ok(value.template_market_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MarketCounts {
        fixed_odds: Result<i64, String>,
        total: Result<i64, String>,
    }
    impl Default for MarketCounts {
        fn default() -> Self {
            Self {
                fixed_odds: Err("no value supplied for fixed_odds".to_string()),
                total: Err("no value supplied for total".to_string()),
            }
        }
    }
    impl MarketCounts {
        pub fn fixed_odds<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.fixed_odds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fixed_odds: {}", e));
            self
        }
        pub fn total<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.total = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<MarketCounts> for super::MarketCounts {
        type Error = super::error::ConversionError;
        fn try_from(value: MarketCounts) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                fixed_odds: value.fixed_odds?,
                total: value.total?,
            })
        }
    }
    impl From<super::MarketCounts> for MarketCounts {
        fn from(value: super::MarketCounts) -> Self {
            Self {
                fixed_odds: Ok(value.fixed_odds),
                total: Ok(value.total),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Media {
        active: Result<bool, String>,
        end_time: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        external_media_ids: Result<Vec<super::ExternalMediaId>, String>,
        start_time: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        url: Result<(), String>,
    }
    impl Default for Media {
        fn default() -> Self {
            Self {
                active: Err("no value supplied for active".to_string()),
                end_time: Err("no value supplied for end_time".to_string()),
                external_media_ids: Err("no value supplied for external_media_ids".to_string()),
                start_time: Err("no value supplied for start_time".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl Media {
        pub fn active<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.active = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for active: {}", e));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {}", e));
            self
        }
        pub fn external_media_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ExternalMediaId>>,
            T::Error: std::fmt::Display,
        {
            self.external_media_ids = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for external_media_ids: {}",
                    e
                )
            });
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Media> for super::Media {
        type Error = super::error::ConversionError;
        fn try_from(value: Media) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                active: value.active?,
                end_time: value.end_time?,
                external_media_ids: value.external_media_ids?,
                start_time: value.start_time?,
                url: value.url?,
            })
        }
    }
    impl From<super::Media> for Media {
        fn from(value: super::Media) -> Self {
            Self {
                active: Ok(value.active),
                end_time: Ok(value.end_time),
                external_media_ids: Ok(value.external_media_ids),
                start_time: Ok(value.start_time),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MediaProvider {
        id: Result<Option<super::Id>, String>,
        listing_url: Result<Option<String>, String>,
        logo_resource: Result<(), String>,
        media: Result<Vec<super::Media>, String>,
        media_type: Result<super::MediaType, String>,
        name: Result<super::MediaProviderName, String>,
        provider_code: Result<(), String>,
    }
    impl Default for MediaProvider {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                listing_url: Err("no value supplied for listing_url".to_string()),
                logo_resource: Err("no value supplied for logo_resource".to_string()),
                media: Err("no value supplied for media".to_string()),
                media_type: Err("no value supplied for media_type".to_string()),
                name: Err("no value supplied for name".to_string()),
                provider_code: Err("no value supplied for provider_code".to_string()),
            }
        }
    }
    impl MediaProvider {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Id>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn listing_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.listing_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for listing_url: {}", e));
            self
        }
        pub fn logo_resource<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.logo_resource = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logo_resource: {}", e));
            self
        }
        pub fn media<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Media>>,
            T::Error: std::fmt::Display,
        {
            self.media = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media: {}", e));
            self
        }
        pub fn media_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::MediaType>,
            T::Error: std::fmt::Display,
        {
            self.media_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_type: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::MediaProviderName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn provider_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.provider_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for provider_code: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<MediaProvider> for super::MediaProvider {
        type Error = super::error::ConversionError;
        fn try_from(value: MediaProvider) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                listing_url: value.listing_url?,
                logo_resource: value.logo_resource?,
                media: value.media?,
                media_type: value.media_type?,
                name: value.name?,
                provider_code: value.provider_code?,
            })
        }
    }
    impl From<super::MediaProvider> for MediaProvider {
        fn from(value: super::MediaProvider) -> Self {
            Self {
                id: Ok(value.id),
                listing_url: Ok(value.listing_url),
                logo_resource: Ok(value.logo_resource),
                media: Ok(value.media),
                media_type: Ok(value.media_type),
                name: Ok(value.name),
                provider_code: Ok(value.provider_code),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Outcome {
        active: Result<bool, String>,
        channels: Result<Vec<super::Channel>, String>,
        display_order: Result<i64, String>,
        displayed: Result<bool, String>,
        id: Result<String, String>,
        market_id: Result<String, String>,
        name: Result<String, String>,
        outcome_score: Result<(), String>,
        prices: Result<Vec<super::Price>, String>,
        rcp_available: Result<bool, String>,
        resulted: Result<bool, String>,
        retail_code: Result<(), String>,
        runner_number: Result<i64, String>,
        status: Result<super::Status, String>,
        sub_type: Result<Option<super::SubType>, String>,
        type_: Result<super::SubTypeEnum, String>,
    }
    impl Default for Outcome {
        fn default() -> Self {
            Self {
                active: Err("no value supplied for active".to_string()),
                channels: Err("no value supplied for channels".to_string()),
                display_order: Err("no value supplied for display_order".to_string()),
                displayed: Err("no value supplied for displayed".to_string()),
                id: Err("no value supplied for id".to_string()),
                market_id: Err("no value supplied for market_id".to_string()),
                name: Err("no value supplied for name".to_string()),
                outcome_score: Err("no value supplied for outcome_score".to_string()),
                prices: Err("no value supplied for prices".to_string()),
                rcp_available: Err("no value supplied for rcp_available".to_string()),
                resulted: Err("no value supplied for resulted".to_string()),
                retail_code: Err("no value supplied for retail_code".to_string()),
                runner_number: Err("no value supplied for runner_number".to_string()),
                status: Err("no value supplied for status".to_string()),
                sub_type: Err("no value supplied for sub_type".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Outcome {
        pub fn active<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.active = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for active: {}", e));
            self
        }
        pub fn channels<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Channel>>,
            T::Error: std::fmt::Display,
        {
            self.channels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channels: {}", e));
            self
        }
        pub fn display_order<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.display_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_order: {}", e));
            self
        }
        pub fn displayed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.displayed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for displayed: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn market_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.market_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for market_id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn outcome_score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.outcome_score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for outcome_score: {}", e));
            self
        }
        pub fn prices<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Price>>,
            T::Error: std::fmt::Display,
        {
            self.prices = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prices: {}", e));
            self
        }
        pub fn rcp_available<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.rcp_available = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rcp_available: {}", e));
            self
        }
        pub fn resulted<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.resulted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resulted: {}", e));
            self
        }
        pub fn retail_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.retail_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for retail_code: {}", e));
            self
        }
        pub fn runner_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.runner_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for runner_number: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Status>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn sub_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SubType>>,
            T::Error: std::fmt::Display,
        {
            self.sub_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub_type: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SubTypeEnum>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Outcome> for super::Outcome {
        type Error = super::error::ConversionError;
        fn try_from(value: Outcome) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                active: value.active?,
                channels: value.channels?,
                display_order: value.display_order?,
                displayed: value.displayed?,
                id: value.id?,
                market_id: value.market_id?,
                name: value.name?,
                outcome_score: value.outcome_score?,
                prices: value.prices?,
                rcp_available: value.rcp_available?,
                resulted: value.resulted?,
                retail_code: value.retail_code?,
                runner_number: value.runner_number?,
                status: value.status?,
                sub_type: value.sub_type?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Outcome> for Outcome {
        fn from(value: super::Outcome) -> Self {
            Self {
                active: Ok(value.active),
                channels: Ok(value.channels),
                display_order: Ok(value.display_order),
                displayed: Ok(value.displayed),
                id: Ok(value.id),
                market_id: Ok(value.market_id),
                name: Ok(value.name),
                outcome_score: Ok(value.outcome_score),
                prices: Ok(value.prices),
                rcp_available: Ok(value.rcp_available),
                resulted: Ok(value.resulted),
                retail_code: Ok(value.retail_code),
                runner_number: Ok(value.runner_number),
                status: Ok(value.status),
                sub_type: Ok(value.sub_type),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Participant {
        active: Result<bool, String>,
        id: Result<String, String>,
        name: Result<String, String>,
        role: Result<super::Role, String>,
        role_code: Result<super::Side, String>,
        type_: Result<super::ParticipantType, String>,
    }
    impl Default for Participant {
        fn default() -> Self {
            Self {
                active: Err("no value supplied for active".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                role: Err("no value supplied for role".to_string()),
                role_code: Err("no value supplied for role_code".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Participant {
        pub fn active<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.active = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for active: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Role>,
            T::Error: std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {}", e));
            self
        }
        pub fn role_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Side>,
            T::Error: std::fmt::Display,
        {
            self.role_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_code: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParticipantType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Participant> for super::Participant {
        type Error = super::error::ConversionError;
        fn try_from(value: Participant) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                active: value.active?,
                id: value.id?,
                name: value.name?,
                role: value.role?,
                role_code: value.role_code?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Participant> for Participant {
        fn from(value: super::Participant) -> Self {
            Self {
                active: Ok(value.active),
                id: Ok(value.id),
                name: Ok(value.name),
                role: Ok(value.role),
                role_code: Ok(value.role_code),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Period {
        clock: Result<super::Clock, String>,
        facts: Result<Vec<super::Fact>, String>,
        id: Result<String, String>,
        incidents: Result<Vec<::serde_json::Value>, String>,
        order: Result<i64, String>,
        period_index: Result<(), String>,
        periods: Result<Vec<::serde_json::Value>, String>,
        start_time: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        type_: Result<super::PeriodType, String>,
    }
    impl Default for Period {
        fn default() -> Self {
            Self {
                clock: Err("no value supplied for clock".to_string()),
                facts: Err("no value supplied for facts".to_string()),
                id: Err("no value supplied for id".to_string()),
                incidents: Err("no value supplied for incidents".to_string()),
                order: Err("no value supplied for order".to_string()),
                period_index: Err("no value supplied for period_index".to_string()),
                periods: Err("no value supplied for periods".to_string()),
                start_time: Err("no value supplied for start_time".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Period {
        pub fn clock<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Clock>,
            T::Error: std::fmt::Display,
        {
            self.clock = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clock: {}", e));
            self
        }
        pub fn facts<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Fact>>,
            T::Error: std::fmt::Display,
        {
            self.facts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for facts: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn incidents<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<::serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.incidents = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for incidents: {}", e));
            self
        }
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {}", e));
            self
        }
        pub fn period_index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.period_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for period_index: {}", e));
            self
        }
        pub fn periods<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<::serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.periods = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for periods: {}", e));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PeriodType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Period> for super::Period {
        type Error = super::error::ConversionError;
        fn try_from(value: Period) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                clock: value.clock?,
                facts: value.facts?,
                id: value.id?,
                incidents: value.incidents?,
                order: value.order?,
                period_index: value.period_index?,
                periods: value.periods?,
                start_time: value.start_time?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Period> for Period {
        fn from(value: super::Period) -> Self {
            Self {
                clock: Ok(value.clock),
                facts: Ok(value.facts),
                id: Ok(value.id),
                incidents: Ok(value.incidents),
                order: Ok(value.order),
                period_index: Ok(value.period_index),
                periods: Ok(value.periods),
                start_time: Ok(value.start_time),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Price {
        decimal: Result<f64, String>,
        denominator: Result<i64, String>,
        display_order: Result<i64, String>,
        handicap_high: Result<Option<String>, String>,
        handicap_low: Result<Option<String>, String>,
        numerator: Result<i64, String>,
        price_type: Result<super::PriceType, String>,
    }
    impl Default for Price {
        fn default() -> Self {
            Self {
                decimal: Err("no value supplied for decimal".to_string()),
                denominator: Err("no value supplied for denominator".to_string()),
                display_order: Err("no value supplied for display_order".to_string()),
                handicap_high: Err("no value supplied for handicap_high".to_string()),
                handicap_low: Err("no value supplied for handicap_low".to_string()),
                numerator: Err("no value supplied for numerator".to_string()),
                price_type: Err("no value supplied for price_type".to_string()),
            }
        }
    }
    impl Price {
        pub fn decimal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.decimal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for decimal: {}", e));
            self
        }
        pub fn denominator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.denominator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for denominator: {}", e));
            self
        }
        pub fn display_order<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.display_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_order: {}", e));
            self
        }
        pub fn handicap_high<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.handicap_high = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for handicap_high: {}", e));
            self
        }
        pub fn handicap_low<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.handicap_low = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for handicap_low: {}", e));
            self
        }
        pub fn numerator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.numerator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for numerator: {}", e));
            self
        }
        pub fn price_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PriceType>,
            T::Error: std::fmt::Display,
        {
            self.price_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for price_type: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Price> for super::Price {
        type Error = super::error::ConversionError;
        fn try_from(value: Price) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                decimal: value.decimal?,
                denominator: value.denominator?,
                display_order: value.display_order?,
                handicap_high: value.handicap_high?,
                handicap_low: value.handicap_low?,
                numerator: value.numerator?,
                price_type: value.price_type?,
            })
        }
    }
    impl From<super::Price> for Price {
        fn from(value: super::Price) -> Self {
            Self {
                decimal: Ok(value.decimal),
                denominator: Ok(value.denominator),
                display_order: Ok(value.display_order),
                handicap_high: Ok(value.handicap_high),
                handicap_low: Ok(value.handicap_low),
                numerator: Ok(value.numerator),
                price_type: Ok(value.price_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Team {
        alternative_names: Result<Vec<super::AlternativeName>, String>,
        code: Result<(), String>,
        external_id: Result<(), String>,
        id: Result<(), String>,
        name: Result<String, String>,
        side: Result<super::Side, String>,
        status: Result<(), String>,
    }
    impl Default for Team {
        fn default() -> Self {
            Self {
                alternative_names: Err("no value supplied for alternative_names".to_string()),
                code: Err("no value supplied for code".to_string()),
                external_id: Err("no value supplied for external_id".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                side: Err("no value supplied for side".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl Team {
        pub fn alternative_names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::AlternativeName>>,
            T::Error: std::fmt::Display,
        {
            self.alternative_names = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for alternative_names: {}",
                    e
                )
            });
            self
        }
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn external_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.external_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_id: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn side<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Side>,
            T::Error: std::fmt::Display,
        {
            self.side = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for side: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Team> for super::Team {
        type Error = super::error::ConversionError;
        fn try_from(value: Team) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                alternative_names: value.alternative_names?,
                code: value.code?,
                external_id: value.external_id?,
                id: value.id?,
                name: value.name?,
                side: value.side?,
                status: value.status?,
            })
        }
    }
    impl From<super::Team> for Team {
        fn from(value: super::Team) -> Self {
            Self {
                alternative_names: Ok(value.alternative_names),
                code: Ok(value.code),
                external_id: Ok(value.external_id),
                id: Ok(value.id),
                name: Ok(value.name),
                side: Ok(value.side),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TimeBandEvent {
        competition_summary: Result<Vec<super::CompetitionSummary>, String>,
        date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        events: Result<Vec<super::Event>, String>,
        outrights: Result<Vec<::serde_json::Value>, String>,
        type_: Result<String, String>,
    }
    impl Default for TimeBandEvent {
        fn default() -> Self {
            Self {
                competition_summary: Err("no value supplied for competition_summary".to_string()),
                date: Err("no value supplied for date".to_string()),
                events: Err("no value supplied for events".to_string()),
                outrights: Err("no value supplied for outrights".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TimeBandEvent {
        pub fn competition_summary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::CompetitionSummary>>,
            T::Error: std::fmt::Display,
        {
            self.competition_summary = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for competition_summary: {}",
                    e
                )
            });
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn events<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Event>>,
            T::Error: std::fmt::Display,
        {
            self.events = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for events: {}", e));
            self
        }
        pub fn outrights<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<::serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.outrights = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for outrights: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TimeBandEvent> for super::TimeBandEvent {
        type Error = super::error::ConversionError;
        fn try_from(value: TimeBandEvent) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                competition_summary: value.competition_summary?,
                date: value.date?,
                events: value.events?,
                outrights: value.outrights?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::TimeBandEvent> for TimeBandEvent {
        fn from(value: super::TimeBandEvent) -> Self {
            Self {
                competition_summary: Ok(value.competition_summary),
                date: Ok(value.date),
                events: Ok(value.events),
                outrights: Ok(value.outrights),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TimeBandEventList {
        data: Result<super::Data, String>,
        errors: Result<Vec<::serde_json::Value>, String>,
        extensions: Result<(), String>,
    }
    impl Default for TimeBandEventList {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                errors: Err("no value supplied for errors".to_string()),
                extensions: Err("no value supplied for extensions".to_string()),
            }
        }
    }
    impl TimeBandEventList {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Data>,
            T::Error: std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn errors<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<::serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.errors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for errors: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TimeBandEventList> for super::TimeBandEventList {
        type Error = super::error::ConversionError;
        fn try_from(value: TimeBandEventList) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                errors: value.errors?,
                extensions: value.extensions?,
            })
        }
    }
    impl From<super::TimeBandEventList> for TimeBandEventList {
        fn from(value: super::TimeBandEventList) -> Self {
            Self {
                data: Ok(value.data),
                errors: Ok(value.errors),
                extensions: Ok(value.extensions),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeClass {
        alternative_names: Result<Vec<::serde_json::Value>, String>,
        display_order: Result<String, String>,
        fixed_odds_available: Result<bool, String>,
        id: Result<String, String>,
        name: Result<String, String>,
    }
    impl Default for TypeClass {
        fn default() -> Self {
            Self {
                alternative_names: Err("no value supplied for alternative_names".to_string()),
                display_order: Err("no value supplied for display_order".to_string()),
                fixed_odds_available: Err("no value supplied for fixed_odds_available".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl TypeClass {
        pub fn alternative_names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<::serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.alternative_names = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for alternative_names: {}",
                    e
                )
            });
            self
        }
        pub fn display_order<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.display_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_order: {}", e));
            self
        }
        pub fn fixed_odds_available<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.fixed_odds_available = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for fixed_odds_available: {}",
                    e
                )
            });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TypeClass> for super::TypeClass {
        type Error = super::error::ConversionError;
        fn try_from(value: TypeClass) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                alternative_names: value.alternative_names?,
                display_order: value.display_order?,
                fixed_odds_available: value.fixed_odds_available?,
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl From<super::TypeClass> for TypeClass {
        fn from(value: super::TypeClass) -> Self {
            Self {
                alternative_names: Ok(value.alternative_names),
                display_order: Ok(value.display_order),
                fixed_odds_available: Ok(value.fixed_odds_available),
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
}
