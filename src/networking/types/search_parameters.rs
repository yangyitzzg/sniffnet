use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;

/// Used to express the search filters applied to GUI inspect page
#[derive(Clone, Debug, Default, Hash)]
pub struct SearchParameters {
    /// IP address (source)
    pub address_src: String,
    /// Transport port (source)
    pub port_src: String,
    /// IP address (destination)
    pub address_dst: String,
    /// Transport port (destination)
    pub port_dst: String,
    /// Protocol
    pub proto: String,
    /// Application protocol
    pub app_proto: String,
    /// Country
    pub country: String,
    /// Domain
    pub domain: String,
    /// Autonomous System name
    pub as_name: String,
    /// Whether to display only favorites
    pub only_favorites: bool,
}

impl SearchParameters {
    pub fn match_entry(
        &self,
        key: &AddressPortPair,
        value: &InfoAddressPortPair,
        r_dns_host: Option<&(String, Host)>,
        is_favorite: bool,
    ) -> bool {
        // if a host-related filter is active and this address has not been resolved yet => false
        if r_dns_host.is_none() && self.is_some_host_filter_active() {
            return false;
        }

        for filter_input_type in FilterInputType::ALL {
            if !filter_input_type.matches_entry(self, key, value, r_dns_host) {
                return false;
            }
        }

        // check favorites filter
        if self.only_favorites && !is_favorite {
            return false;
        }

        // if arrived at this point all filters are satisfied
        true
    }

    pub fn is_some_host_filter_active(&self) -> bool {
        self.only_favorites
            || !self.country.is_empty()
            || !self.as_name.is_empty()
            || !self.domain.is_empty()
    }

    pub fn reset_host_filters(&self) -> Self {
        Self {
            country: String::new(),
            domain: String::new(),
            as_name: String::new(),
            only_favorites: false,
            ..self.clone()
        }
    }
}

#[derive(Copy, Clone)]
pub enum FilterInputType {
    AddressSrc,
    PortSrc,
    AddressDst,
    PortDst,
    Proto,
    AppProto,
    Country,
    Domain,
    AsName,
}

impl FilterInputType {
    pub const ALL: [FilterInputType; 9] = [
        Self::AddressSrc,
        Self::PortSrc,
        Self::AddressDst,
        Self::PortDst,
        Self::Proto,
        Self::AppProto,
        Self::Country,
        Self::Domain,
        Self::AsName,
    ];

    pub fn matches_entry(
        self,
        search_params: &SearchParameters,
        key: &AddressPortPair,
        value: &InfoAddressPortPair,
        r_dns_host: Option<&(String, Host)>,
    ) -> bool {
        let filter_value = self.current_value(search_params).to_lowercase();

        if filter_value.is_empty() {
            return true;
        }

        let entry_value = self.entry_value(key, value, r_dns_host).to_lowercase();

        if let Some(stripped_filter) = filter_value.strip_prefix('=') {
            return entry_value.eq(stripped_filter);
        }

        match self {
            FilterInputType::AddressSrc
            | FilterInputType::AddressDst
            | FilterInputType::Domain
            | FilterInputType::AsName => entry_value.contains(&filter_value),
            FilterInputType::PortSrc
            | FilterInputType::PortDst
            | FilterInputType::Proto
            | FilterInputType::AppProto
            | FilterInputType::Country => entry_value.starts_with(&filter_value),
        }
    }

    pub fn current_value(self, search_params: &SearchParameters) -> &str {
        match self {
            FilterInputType::AddressSrc => &search_params.address_src,
            FilterInputType::PortSrc => &search_params.port_src,
            FilterInputType::AddressDst => &search_params.address_dst,
            FilterInputType::PortDst => &search_params.port_dst,
            FilterInputType::Proto => &search_params.proto,
            FilterInputType::AppProto => &search_params.app_proto,
            FilterInputType::Country => &search_params.country,
            FilterInputType::Domain => &search_params.domain,
            FilterInputType::AsName => &search_params.as_name,
        }
    }

    pub fn entry_value(
        self,
        key: &AddressPortPair,
        value: &InfoAddressPortPair,
        r_dns_host: Option<&(String, Host)>,
    ) -> String {
        match self {
            FilterInputType::AddressSrc => key.address1.to_string(),
            FilterInputType::PortSrc => {
                if let Some(port) = key.port1 {
                    port.to_string()
                } else {
                    "-".to_string()
                }
            }
            FilterInputType::AddressDst => key.address2.to_string(),
            FilterInputType::PortDst => {
                if let Some(port) = key.port2 {
                    port.to_string()
                } else {
                    "-".to_string()
                }
            }
            FilterInputType::Proto => key.protocol.to_string(),
            FilterInputType::AppProto => value.app_protocol.to_string(),
            FilterInputType::Country => r_dns_host.unwrap().1.country.to_string(),
            FilterInputType::Domain => r_dns_host.unwrap().0.to_string(),
            FilterInputType::AsName => r_dns_host.unwrap().1.asn.name.to_string(),
        }
    }

    pub fn clear_search(self, search_params: &SearchParameters) -> SearchParameters {
        match self {
            FilterInputType::AddressSrc => SearchParameters {
                address_src: String::new(),
                ..search_params.clone()
            },
            FilterInputType::PortSrc => SearchParameters {
                port_src: String::new(),
                ..search_params.clone()
            },
            FilterInputType::AddressDst => SearchParameters {
                address_dst: String::new(),
                ..search_params.clone()
            },
            FilterInputType::PortDst => SearchParameters {
                port_dst: String::new(),
                ..search_params.clone()
            },
            FilterInputType::Proto => SearchParameters {
                proto: String::new(),
                ..search_params.clone()
            },
            FilterInputType::AppProto => SearchParameters {
                app_proto: String::new(),
                ..search_params.clone()
            },
            FilterInputType::Domain => SearchParameters {
                domain: String::new(),
                ..search_params.clone()
            },
            FilterInputType::Country => SearchParameters {
                country: String::new(),
                ..search_params.clone()
            },
            FilterInputType::AsName => SearchParameters {
                as_name: String::new(),
                ..search_params.clone()
            },
        }
    }

    pub fn new_search(
        self,
        search_params: &SearchParameters,
        new_value: String,
    ) -> SearchParameters {
        match self {
            FilterInputType::AddressSrc => SearchParameters {
                address_src: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::PortSrc => SearchParameters {
                port_src: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::AddressDst => SearchParameters {
                address_dst: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::PortDst => SearchParameters {
                port_dst: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::Proto => SearchParameters {
                proto: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::AppProto => SearchParameters {
                app_proto: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::Domain => SearchParameters {
                domain: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::Country => SearchParameters {
                country: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::AsName => SearchParameters {
                as_name: new_value,
                ..search_params.clone()
            },
        }
    }
}
