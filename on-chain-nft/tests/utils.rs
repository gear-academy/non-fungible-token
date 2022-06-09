use gear_lib::non_fungible_token::token::*;
use gstd::prelude::*;
use gtest::{Program, RunResult, System};
use on_chain_nft_io::*;
const USERS: &[u64] = &[3, 4, 5];

pub fn init_nft(sys: &System) {
    sys.init_logger();
    let nft = Program::current(sys);

    let mut layers = BTreeMap::new();
    let first_layer = vec![
        String::from(
        "PHN2ZyBoZWlnaHQ9JzIxMCcgd2lkdGg9JzUwMCc+PHBvbHlnb24gcG9pbnRzPScxMDAsMTAgNDAsMTk4IDE5MCw3OCAxMCw3OCAxNjAsMTk4JyBzdHlsZT0nZmlsbDpsaW1lO3N0cm9rZTpwdXJwbGU7c3Ryb2tlLXdpZHRoOjU7ZmlsbC1ydWxlOm5vbnplcm87Jy8+PC9zdmc+",
        ),
        String::from(
            "PHN2ZyBoZWlnaHQ9JzIxMCcgd2lkdGg9JzUwMCc+PHBvbHlnb24gcG9pbnRzPScxMDAsMTAgNDAsMTk4IDE5MCw3OCAxMCw3OCAxNjAsMTk4JyBzdHlsZT0nZmlsbDpibHVlO3N0cm9rZTpyZWQ7c3Ryb2tlLXdpZHRoOjU7ZmlsbC1ydWxlOm5vbnplcm87Jy8+PC9zdmc+",
        )
    ];
    let second_layer = vec![
        String::from(
            "PHN2ZyBoZWlnaHQ9JzMwJyB3aWR0aD0nMjAwJz48dGV4dCB4PScwJyB5PScxNScgZmlsbD0ncmVkJz5PbiBDaGFpbiBORlQ8L3RleHQ+PC9zdmc+"
        ),
        String::from(
            "PHN2ZyBoZWlnaHQ9JzMwJyB3aWR0aD0nMjAwJz48dGV4dCB4PScwJyB5PScxNScgZmlsbD0nZ3JlZW4nPk9uIENoYWluIE5GVDwvdGV4dD48L3N2Zz4="
        )
    ];
    layers.insert(1, first_layer);
    layers.insert(2, second_layer);
    let res = nft.send(
        USERS[0],
        InitOnChainNFT {
            name: String::from("OnChainToken"),
            symbol: String::from("OCT"),
            base_uri: String::from(""),
            royalties: None,
            base_image: String::from("<svg height='100' width='100'><circle cx='50' cy='50' r='40' stroke='black' stroke-width='3' fill='red' /></svg>"),
            layers,
        },
    );

    assert!(res.log().is_empty());
}

pub fn mint(nft: &Program, member: u64, description: Vec<ItemId>) -> RunResult {
    nft.send(
        member,
        OnChainNFTAction::Mint {
            token_metadata: TokenMetadata {
                name: "CryptoKitty".to_string(),
                description: "Description".to_string(),
                media: "http://".to_string(),
                reference: "http://".to_string(),
            },
            description,
        },
    )
}

pub fn burn(nft: &Program, member: u64, token_id: u64) -> RunResult {
    nft.send(
        member,
        OnChainNFTAction::Burn {
            token_id: token_id.into(),
        },
    )
}

pub fn transfer(nft: &Program, from: u64, to: u64, token_id: u64) -> RunResult {
    nft.send(
        from,
        OnChainNFTAction::Transfer {
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn approve(nft: &Program, from: u64, to: u64, token_id: u64) -> RunResult {
    nft.send(
        from,
        OnChainNFTAction::Approve {
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}