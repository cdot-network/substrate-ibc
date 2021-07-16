use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;

pub mod primitive {
    use ibc::ics02_client::height::Height as IbcHeight;
    use ibc::ics24_host::identifier::ClientId as IbcClientId;
    use ibc::ics02_client::client_type::ClientType as IbcClientType;

    use codec::{Decode, Encode};
    use sp_runtime::RuntimeDebug;

    #[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug)]
    pub struct Height {
        /// Previously known as "epoch"
        pub revision_number: u64,

        /// The height of a block
        pub revision_height: u64,
    }

    impl From<IbcHeight> for Height {
        fn from(IbcHeight{revision_number, revision_height} : IbcHeight) -> Self {
            Height {
                revision_number,
                revision_height,
            }
        }
    }

    #[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug)]
    pub enum ClientType {
        Tendermint = 1,
    }

    impl From<IbcClientType> for ClientType {
        fn from(value: IbcClientType) -> Self {
            match value {
                IbcClientType::Tendermint => ClientType::Tendermint,
                _ => unreachable!()
            }
        }
    }

    #[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug)]
    pub struct ClientId(String);

    impl From<IbcClientId> for ClientId {
        fn from(value: IbcClientId) -> Self {
            let value = value.as_str();
            Self(value.to_string())
        }
    }
}

pub mod client_event {
    use ibc::ics02_client::events::CreateClient as IbcCreateClient;
    use ibc::ics02_client::events::Attributes as IbcAttributes;



    use codec::{Decode, Encode};
    use sp_runtime::RuntimeDebug;
    use super::primitive::Height;
    use super::primitive::ClientType;
    use super::primitive::ClientId;

    #[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug)]
    pub struct Attributes {
        pub height: Height,
        pub client_id: ClientId,
        pub client_type: ClientType,
        pub consensus_height: Height,
    }

    impl From<IbcAttributes> for Attributes {
        fn from(IbcAttributes{height, client_id, client_type, consensus_height}: IbcAttributes) -> Self {
            Attributes {
                height: height.into(),
                client_id: client_id.into(),
                client_type: client_type.into(),
                consensus_height: consensus_height.into(),
            }
        }
    }

    #[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug)]
    pub struct CreateClient(Attributes);

    impl From<IbcCreateClient> for CreateClient {
        fn from(value : IbcCreateClient) -> Self {
            match value {
                IbcCreateClient(value) => {
                    CreateClient(value.into())
                }
                _ => unreachable!()
            }
        }
    }

}


