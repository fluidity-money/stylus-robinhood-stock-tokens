#![no_std]

use bobcat_cd::address;

use bobcat_maths::U;

pub use stylus_chainlink_price_feeds::{ChainlinkPriceFeed, robinhood::Robinhood};

pub(crate) use bobcat_interfaces::{eip20::*, robinhood_stock_tokens::*};

pub type Address = [u8; 20];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Eip712Domain {
    pub fields: u8,
    pub name: &'static str,
    pub version: &'static str,
    pub verifying_contract: Address,
    pub salt: U,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StockToken {
    Aapl,
    Amd,
    Amzn,
    Asml,
    Baba,
    Clsk,
    Coin,
    Crcl,
    Crwv,
    Dell,
    Ewy,
    Gme,
    Googl,
    Intc,
    Ionq,
    Meta,
    Msft,
    Mstr,
    Mu,
    Nbis,
    Nvda,
    Orcl,
    Pltr,
    Qqq,
    Rgti,
    Rklb,
    Sgov,
    Slv,
    Sndk,
    Spcx,
    Spy,
    Tsla,
    Tsm,
    Usar,
    Uso,
}

impl StockToken {
    /// Returns the canonical token contract address on Robinhood Chain.
    pub const fn addr(self) -> Address {
        match self {
            Self::Aapl => address!(b"aF3D76f1834A1d425780943C99Ea8A608f8a93f9"),
            Self::Amd => address!(b"86923f96303D656E4aa86D9d42D1e57ad2023fdC"),
            Self::Amzn => address!(b"12f190a9F9d7D37a250758b26824B97CE941bF54"),
            Self::Asml => address!(b"47F93d52cBeC7C6D2CfC080e154002370a60dAEA"),
            Self::Baba => address!(b"ad25Ac6C84D497db898fa1E8387bf6Af3532a1c4"),
            Self::Clsk => address!(b"cBB95BBF36099d34dA091dc6Fa6F49EfA257Cee3"),
            Self::Coin => address!(b"6330D8C3178a418788dF01a47479c0ce7CCF450b"),
            Self::Crcl => address!(b"dF0992E440dD0be65BD8439b609d6D4366bf1CB5"),
            Self::Crwv => address!(b"5f10A1C971B69e47e059e1dC91901B59b3fB49C3"),
            Self::Dell => address!(b"941AE714EC6D8130c7B75d67160Ca08f1e7d11Dd"),
            Self::Ewy => address!(b"7f0aBeF0C07280F82c6a08ead09dEd6BAE2C13Fc"),
            Self::Gme => address!(b"1b0E319c6A659F002271B69dB8A7df2F911c153E"),
            Self::Googl => address!(b"2e0847E8910a9732eB3fb1bb4b70a580ADAD4FE3"),
            Self::Intc => address!(b"c72b96e0E48ecd4DC75E1e45396e26300BC39681"),
            Self::Ionq => address!(b"558378E000D634A36593E338eBacdd6207640EfE"),
            Self::Meta => address!(b"c0D6457C16Cc70d6790Dd43521C899C87ce02f35"),
            Self::Msft => address!(b"e93237C50D904957Cf27E7B1133b510C669c2e74"),
            Self::Mstr => address!(b"ec262a75e413fAfD0dF80480274532C79D42da09"),
            Self::Mu => address!(b"fF080c8ce2E5feadaCa0Da81314Ae59D232d4afD"),
            Self::Nbis => address!(b"9D9c6684F596F66a64C030B93A886D51Fd4D7931"),
            Self::Nvda => address!(b"d0601CE157Db5bdC3162BbaC2a2C8aF5320D9EEC"),
            Self::Orcl => address!(b"b0992820E760d836549ba69BC7598b4af75dEE03"),
            Self::Pltr => address!(b"894E1EC2D74FFE5AEF8Dc8A9e84686acCB964F2A"),
            Self::Qqq => address!(b"D5f3879160bc7c32ebb4dC785F8a4F505888de68"),
            Self::Rgti => address!(b"284358abc07F9359f19f4b5b4aC91901Be2597Ba"),
            Self::Rklb => address!(b"3b14C39E89D60D627b42a1A4CA45b5bb45Fc12e2"),
            Self::Sgov => address!(b"92FD66527192E3e61d4DDd13322Aa222DE86F9B5"),
            Self::Slv => address!(b"411eFb0E7f985935DAec3D4C3ebaEa0d0AD7D89f"),
            Self::Sndk => address!(b"B90A19fF0Af67f7779afF50A882A9CfF42446400"),
            Self::Spcx => address!(b"4a0E65A3EcceC6dBe60AE065F2e7bb85Fae35eEa"),
            Self::Spy => address!(b"117cc2133c37B721F49dE2A7a74833232B3B4C0C"),
            Self::Tsla => address!(b"322F0929c4625eD5bAd873c95208D54E1c003b2d"),
            Self::Tsm => address!(b"58FfE4a942d3885bAa22D7520691F611EF09e7AA"),
            Self::Usar => address!(b"d917B029C761D264c6A312BBbcDA868658eF86a6"),
            Self::Uso => address!(b"a30FA36Db767ad9eD3f7a60fC79526fB4d56D344"),
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Aapl => "Apple • Robinhood Token",
            Self::Amd => "AMD • Robinhood Token",
            Self::Amzn => "Amazon • Robinhood Token",
            Self::Asml => "ASML Holding NV • Robinhood Token",
            Self::Baba => "Alibaba • Robinhood Token",
            Self::Clsk => "CleanSpark • Robinhood Token",
            Self::Coin => "Coinbase • Robinhood Token",
            Self::Crcl => "Circle Internet Group • Robinhood Token",
            Self::Crwv => "CoreWeave • Robinhood Token",
            Self::Dell => "Dell • Robinhood Token",
            Self::Ewy => "iShares MSCI South Korea fund • Robinhood Token",
            Self::Gme => "GameStop • Robinhood Token",
            Self::Googl => "Alphabet Class A • Robinhood Token",
            Self::Intc => "Intel • Robinhood Token",
            Self::Ionq => "IonQ • Robinhood Token",
            Self::Meta => "Meta Platforms • Robinhood Token",
            Self::Msft => "Microsoft • Robinhood Token",
            Self::Mstr => "Strategy Inc. • Robinhood Token",
            Self::Mu => "Micron Technology • Robinhood Token",
            Self::Nbis => "Nebius Group • Robinhood Token",
            Self::Nvda => "NVIDIA • Robinhood Token",
            Self::Orcl => "Oracle • Robinhood Token",
            Self::Pltr => "Palantir Technologies • Robinhood Token",
            Self::Qqq => "Invesco QQQ • Robinhood Token",
            Self::Rgti => "Rigetti Computing • Robinhood Token",
            Self::Rklb => "Rocket Lab Corporation • Robinhood Token",
            Self::Sgov => "iShares 0-3 Month Treasury Bond • Robinhood Token",
            Self::Slv => "iShares Silver Trust • Robinhood Token",
            Self::Sndk => "Sandisk Corporation • Robinhood Token",
            Self::Spcx => {
                "Space Exploration Technologies Corp. Class A Common Stock • Robinhood Token"
            }
            Self::Spy => "SPDR S&P 500 ETF Trust • Robinhood Token",
            Self::Tsla => "Tesla • Robinhood Token",
            Self::Tsm => "Taiwan Semiconductor Manufacturing • Robinhood Token",
            Self::Usar => "USA Rare Earth • Robinhood Token",
            Self::Uso => "United States Oil Fund • Robinhood Token",
        }
    }

    pub const fn decimals(self) -> u8 {
        18
    }

    pub const fn oracle(self) -> Robinhood {
        match self {
            Self::Aapl => Robinhood::AaplUsd,
            Self::Amd => Robinhood::AmdUsd,
            Self::Amzn => Robinhood::AmznUsd,
            Self::Asml => Robinhood::AsmlUsd,
            Self::Baba => Robinhood::BabaUsd,
            Self::Clsk => Robinhood::ClskUsd,
            Self::Coin => Robinhood::CoinUsd,
            Self::Crcl => Robinhood::CrclUsd,
            Self::Crwv => Robinhood::CrwvUsd,
            Self::Dell => Robinhood::DellUsd,
            Self::Ewy => Robinhood::EwyUsd,
            Self::Gme => Robinhood::GmeUsd,
            Self::Googl => Robinhood::GooglUsd,
            Self::Intc => Robinhood::IntcUsd,
            Self::Ionq => Robinhood::IonqUsd,
            Self::Meta => Robinhood::MetaUsd,
            Self::Msft => Robinhood::MsftUsd,
            Self::Mstr => Robinhood::MstrUsd,
            Self::Mu => Robinhood::MuUsd,
            Self::Nbis => Robinhood::NbisUsd,
            Self::Nvda => Robinhood::NvdaUsd,
            Self::Orcl => Robinhood::OrclUsd,
            Self::Pltr => Robinhood::PltrUsd,
            Self::Qqq => Robinhood::QqqUsd,
            Self::Rgti => Robinhood::RgtiUsd,
            Self::Rklb => Robinhood::RklbUsd,
            Self::Sgov => Robinhood::SgovUsd,
            Self::Slv => Robinhood::SlvUsd,
            Self::Sndk => Robinhood::SndkUsd,
            Self::Spcx => Robinhood::SpcxUsd,
            Self::Spy => Robinhood::SpyUsd,
            Self::Tsla => Robinhood::TslaUsd,
            Self::Tsm => Robinhood::TsmUsd,
            Self::Usar => Robinhood::UsarUsd,
            Self::Uso => Robinhood::UsoUsd,
        }
    }

    pub fn oracle_addr(self) -> Address {
        self.oracle().addr()
    }

    pub const fn eip712_domain(self) -> Eip712Domain {
        match self {
            Self::Aapl => Eip712Domain {
                fields: 0x0f,
                name: "Apple • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"af3d76f1834a1d425780943c99ea8a608f8a93f9"),
                salt: U::ZERO,
            },
            Self::Amd => Eip712Domain {
                fields: 0x0f,
                name: "AMD • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"86923f96303d656e4aa86d9d42d1e57ad2023fdc"),
                salt: U::ZERO,
            },
            Self::Amzn => Eip712Domain {
                fields: 0x0f,
                name: "Amazon • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"12f190a9f9d7d37a250758b26824b97ce941bf54"),
                salt: U::ZERO,
            },
            Self::Asml => Eip712Domain {
                fields: 0x0f,
                name: "ASML Holding NV • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"47f93d52cbec7c6d2cfc080e154002370a60daea"),
                salt: U::ZERO,
            },
            Self::Baba => Eip712Domain {
                fields: 0x0f,
                name: "Alibaba • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"ad25ac6c84d497db898fa1e8387bf6af3532a1c4"),
                salt: U::ZERO,
            },
            Self::Clsk => Eip712Domain {
                fields: 0x0f,
                name: "CleanSpark • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"cbb95bbf36099d34da091dc6fa6f49efa257cee3"),
                salt: U::ZERO,
            },
            Self::Coin => Eip712Domain {
                fields: 0x0f,
                name: "Coinbase • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"6330d8c3178a418788df01a47479c0ce7ccf450b"),
                salt: U::ZERO,
            },
            Self::Crcl => Eip712Domain {
                fields: 0x0f,
                name: "Circle Internet Group • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"df0992e440dd0be65bd8439b609d6d4366bf1cb5"),
                salt: U::ZERO,
            },
            Self::Crwv => Eip712Domain {
                fields: 0x0f,
                name: "CoreWeave • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"5f10a1c971b69e47e059e1dc91901b59b3fb49c3"),
                salt: U::ZERO,
            },
            Self::Dell => Eip712Domain {
                fields: 0x0f,
                name: "Dell • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"941ae714ec6d8130c7b75d67160ca08f1e7d11dd"),
                salt: U::ZERO,
            },
            Self::Ewy => Eip712Domain {
                fields: 0x0f,
                name: "iShares MSCI South Korea fund • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"7f0abef0c07280f82c6a08ead09ded6bae2c13fc"),
                salt: U::ZERO,
            },
            Self::Gme => Eip712Domain {
                fields: 0x0f,
                name: "GameStop • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"1b0e319c6a659f002271b69db8a7df2f911c153e"),
                salt: U::ZERO,
            },
            Self::Googl => Eip712Domain {
                fields: 0x0f,
                name: "Alphabet Class A • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"2e0847e8910a9732eb3fb1bb4b70a580adad4fe3"),
                salt: U::ZERO,
            },
            Self::Intc => Eip712Domain {
                fields: 0x0f,
                name: "Intel • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"c72b96e0e48ecd4dc75e1e45396e26300bc39681"),
                salt: U::ZERO,
            },
            Self::Ionq => Eip712Domain {
                fields: 0x0f,
                name: "IonQ • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"558378e000d634a36593e338ebacdd6207640efe"),
                salt: U::ZERO,
            },
            Self::Meta => Eip712Domain {
                fields: 0x0f,
                name: "Meta Platforms • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"c0d6457c16cc70d6790dd43521c899c87ce02f35"),
                salt: U::ZERO,
            },
            Self::Msft => Eip712Domain {
                fields: 0x0f,
                name: "Microsoft • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"e93237c50d904957cf27e7b1133b510c669c2e74"),
                salt: U::ZERO,
            },
            Self::Mstr => Eip712Domain {
                fields: 0x0f,
                name: "Strategy Inc. • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"ec262a75e413fafd0df80480274532c79d42da09"),
                salt: U::ZERO,
            },
            Self::Mu => Eip712Domain {
                fields: 0x0f,
                name: "Micron Technology • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"ff080c8ce2e5feadaca0da81314ae59d232d4afd"),
                salt: U::ZERO,
            },
            Self::Nbis => Eip712Domain {
                fields: 0x0f,
                name: "Nebius Group • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"9d9c6684f596f66a64c030b93a886d51fd4d7931"),
                salt: U::ZERO,
            },
            Self::Nvda => Eip712Domain {
                fields: 0x0f,
                name: "NVIDIA • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"d0601ce157db5bdc3162bbac2a2c8af5320d9eec"),
                salt: U::ZERO,
            },
            Self::Orcl => Eip712Domain {
                fields: 0x0f,
                name: "Oracle • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"b0992820e760d836549ba69bc7598b4af75dee03"),
                salt: U::ZERO,
            },
            Self::Pltr => Eip712Domain {
                fields: 0x0f,
                name: "Palantir Technologies • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"894e1ec2d74ffe5aef8dc8a9e84686accb964f2a"),
                salt: U::ZERO,
            },
            Self::Qqq => Eip712Domain {
                fields: 0x0f,
                name: "Invesco QQQ • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"d5f3879160bc7c32ebb4dc785f8a4f505888de68"),
                salt: U::ZERO,
            },
            Self::Rgti => Eip712Domain {
                fields: 0x0f,
                name: "Rigetti Computing • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"284358abc07f9359f19f4b5b4ac91901be2597ba"),
                salt: U::ZERO,
            },
            Self::Rklb => Eip712Domain {
                fields: 0x0f,
                name: "Rocket Lab Corporation • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"3b14c39e89d60d627b42a1a4ca45b5bb45fc12e2"),
                salt: U::ZERO,
            },
            Self::Sgov => Eip712Domain {
                fields: 0x0f,
                name: "iShares 0-3 Month Treasury Bond • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"92fd66527192e3e61d4ddd13322aa222de86f9b5"),
                salt: U::ZERO,
            },
            Self::Slv => Eip712Domain {
                fields: 0x0f,
                name: "iShares Silver Trust • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"411efb0e7f985935daec3d4c3ebaea0d0ad7d89f"),
                salt: U::ZERO,
            },
            Self::Sndk => Eip712Domain {
                fields: 0x0f,
                name: "Sandisk Corporation • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"b90a19ff0af67f7779aff50a882a9cff42446400"),
                salt: U::ZERO,
            },
            Self::Spcx => Eip712Domain {
                fields: 0x0f,
                name: "Space Exploration Technologies Corp. Class A Common Stock • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"4a0e65a3eccec6dbe60ae065f2e7bb85fae35eea"),
                salt: U::ZERO,
            },
            Self::Spy => Eip712Domain {
                fields: 0x0f,
                name: "SPDR S&P 500 ETF Trust • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"117cc2133c37b721f49de2a7a74833232b3b4c0c"),
                salt: U::ZERO,
            },
            Self::Tsla => Eip712Domain {
                fields: 0x0f,
                name: "Tesla • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"322f0929c4625ed5bad873c95208d54e1c003b2d"),
                salt: U::ZERO,
            },
            Self::Tsm => Eip712Domain {
                fields: 0x0f,
                name: "Taiwan Semiconductor Manufacturing • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"58ffe4a942d3885baa22d7520691f611ef09e7aa"),
                salt: U::ZERO,
            },
            Self::Usar => Eip712Domain {
                fields: 0x0f,
                name: "USA Rare Earth • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"d917b029c761d264c6a312bbbcda868658ef86a6"),
                salt: U::ZERO,
            },
            Self::Uso => Eip712Domain {
                fields: 0x0f,
                name: "United States Oil Fund • Robinhood Token",
                version: "1",
                verifying_contract: address!(b"a30fa36db767ad9ed3f7a60fc79526fb4d56d344"),
                salt: U::ZERO,
            },
        }
    }

    pub fn terms(self) -> &'static str {
        "https://robinhood.com/stocktoken/rhj"
    }

    pub fn domain_separator(self) -> U {
        match self {
            Self::Aapl => U::const_from_hex(
                b"ddc20599e9f8b3f5f24eea08e26e58b564138e2a2a0dbd21bf8374048ebf4424",
            )
            .unwrap(),
            Self::Amd => U::const_from_hex(
                b"4f7414a49b594198b251cf4472210c75949ec78b3769b8ead702acc0609fee17",
            )
            .unwrap(),
            Self::Amzn => U::const_from_hex(
                b"9e4222dc88867f1329aa6acc5a1aa81b45bf4132906300ba82ff43ec14a61a01",
            )
            .unwrap(),
            Self::Asml => U::const_from_hex(
                b"06494358a2f8735929cde930daf03cfab8148f85bbe47500e77acdd4eb9206e3",
            )
            .unwrap(),
            Self::Baba => U::const_from_hex(
                b"726b1c341c9f212157b71f35c757b0276c188355ef1e1d36318c9e3cf7f17aee",
            )
            .unwrap(),
            Self::Clsk => U::const_from_hex(
                b"876668158311b1ab24ea818c972c201c837e078ff0e67a41bdc119e7e14c2b0b",
            )
            .unwrap(),
            Self::Coin => U::const_from_hex(
                b"3864acb9626f80b5eab61355dde07a0ed372439c073052a64d235be0e4550156",
            )
            .unwrap(),
            Self::Crcl => U::const_from_hex(
                b"4c54d092a713ecb31538b6310161eafafe84be23a34125c02f714ff6c1787b27",
            )
            .unwrap(),
            Self::Crwv => U::const_from_hex(
                b"4aa7f796561126f7fae63f6b694c9b1869158506da30459c367ac1462e3285f4",
            )
            .unwrap(),
            Self::Dell => U::const_from_hex(
                b"8fc7daf61243cc7aac591b00d86e7b5787164c246f33fcc9847ab7ca42ec40c3",
            )
            .unwrap(),
            Self::Ewy => U::const_from_hex(
                b"03a48248d12094755fb9d44ec881fd4ec746e483aad1d14ae5249f38d3e485ff",
            )
            .unwrap(),
            Self::Gme => U::const_from_hex(
                b"136826e77a7b9207850ddc1f9b91462a328bd7bd1f1ba283f2e5340acc545e03",
            )
            .unwrap(),
            Self::Googl => U::const_from_hex(
                b"e4ce632a06b61144c31debb5cc23eec6986efdf45f8d3d89999b573fa2d231fc",
            )
            .unwrap(),
            Self::Intc => U::const_from_hex(
                b"85d74cda4a25cd675da552b3629bfa6eddfed98be3a18aea9ec916af69d53b27",
            )
            .unwrap(),
            Self::Ionq => U::const_from_hex(
                b"c00517054ac490ed7bce3e7fee22be16a4e46f3b34a79e4788aea83262b43191",
            )
            .unwrap(),
            Self::Meta => U::const_from_hex(
                b"5e515e21dee007a77d5f9d11e943ea0c69890552610fcf75a938700339567d92",
            )
            .unwrap(),
            Self::Msft => U::const_from_hex(
                b"fe5532dffb5037a32acae81fc1a5a61964cfdb985b27fa09d884577c922443b6",
            )
            .unwrap(),
            Self::Mstr => U::const_from_hex(
                b"3bb1a8a59dcefb3849920b51c09fb1457850ba40331872ff67f1aac713aa2d0c",
            )
            .unwrap(),
            Self::Mu => U::const_from_hex(
                b"9ff9c5233d52b3a66de0b121a7ba0d3e5fbee472de9e2f528ddfde83c2659414",
            )
            .unwrap(),
            Self::Nbis => U::const_from_hex(
                b"58842ac1dba751de2b3a68b0985fdee45268c408755b3d30a87de9a14ed8db57",
            )
            .unwrap(),
            Self::Nvda => U::const_from_hex(
                b"9561b23bbb0b6a2c7eecb765b6ae196568c31251e7086d435234d3017abcf6f7",
            )
            .unwrap(),
            Self::Orcl => U::const_from_hex(
                b"dc34df1f13cf1dff2e88b69ad2fde88334f497b09f98a6c8efbb2c187f7b02bb",
            )
            .unwrap(),
            Self::Pltr => U::const_from_hex(
                b"e82099e80d9d8c01951d96841c85652b71b80feb5cf1e1bd25c87d22d14f13b7",
            )
            .unwrap(),
            Self::Qqq => U::const_from_hex(
                b"0d742aa0bc660a58e10d0b613abeb775e4c6ce80c31919e718f74190a6eed95e",
            )
            .unwrap(),
            Self::Rgti => U::const_from_hex(
                b"edd1fa6716b64970ae051556e59bd0d0c241b2ab62f1f6af2c47a34ff8ebfc10",
            )
            .unwrap(),
            Self::Rklb => U::const_from_hex(
                b"99473716d76d7559f7921bf687bf151f676affa69dfb1ccf7f3857c893304ad2",
            )
            .unwrap(),
            Self::Sgov => U::const_from_hex(
                b"325c877912a82763ee194675f1477d9e520ee632778d19ec3c6dd4a4592ac7bd",
            )
            .unwrap(),
            Self::Slv => U::const_from_hex(
                b"a098536349e39e26371e37bfa1fbc1cae5db238b779741cbd5b62d3cd5b15f9c",
            )
            .unwrap(),
            Self::Sndk => U::const_from_hex(
                b"1612568f77316762559269c41b79f1f679d3af7c9f86c1a632ff88d07799d0ef",
            )
            .unwrap(),
            Self::Spcx => U::const_from_hex(
                b"0690c9efe70baa4d30911aebe56cd6fb3ae53c5202bc1f39c7b155d399c682cf",
            )
            .unwrap(),
            Self::Spy => U::const_from_hex(
                b"9664225da5421089c71fcde532a8a45100a788b0adc7c45dbb4fe72320cfb527",
            )
            .unwrap(),
            Self::Tsla => U::const_from_hex(
                b"aed03ce8fb70819d92adddf9b9dd4fbb2f32f0e37b6d5937b441c17549d83ed8",
            )
            .unwrap(),
            Self::Tsm => U::const_from_hex(
                b"55c66118aa4278aab657b61ac4017ac63c4f8149da74e37b087e8fb47ce4df74",
            )
            .unwrap(),
            Self::Usar => U::const_from_hex(
                b"9479467d8d45a26a2d00d75edac17a1e086c8386770b22effaa3fc89b7ca9b20",
            )
            .unwrap(),
            Self::Uso => U::const_from_hex(
                b"0b13e14566ffdd0691d584222002bbec8a1230846e7ae88971c1072ae5fbc174",
            )
            .unwrap(),
        }
    }
}

impl From<StockToken> for Address {
    fn from(token: StockToken) -> Self {
        token.addr()
    }
}

#[cfg(feature = "bobcat-sdk")]
pub mod bobcat {
    use bobcat_maths::U;

    use bobcat_call::{call_bool, static_call_bool_opt, static_call_word_opt};

    use stylus_chainlink_price_feeds::{
        get_latest_round_data_opt, get_latest_round_data_split_opt,
    };

    use super::*;

    #[cfg(feature = "alloc")]
    extern crate alloc;

    pub fn price(t: StockToken) -> Option<U> {
        get_latest_round_data_opt(t.oracle())
    }

    pub fn price_split(t: StockToken) -> Option<(U, U)> {
        get_latest_round_data_split_opt(t.oracle())
    }

    pub fn paused(t: StockToken) -> Option<bool> {
        static_call_bool_opt(t.addr(), &SEL_PAUSED, u64::MAX)
    }

    pub fn token_paused(t: StockToken) -> Option<bool> {
        static_call_bool_opt(t.addr(), &SEL_TOKEN_PAUSED, u64::MAX)
    }

    pub fn oracle_paused(t: StockToken) -> Option<bool> {
        static_call_bool_opt(t.addr(), &SEL_ORACLE_PAUSED, u64::MAX)
    }

    pub fn ui_multiplier(t: StockToken) -> Option<U> {
        static_call_word_opt(t.addr(), &SEL_UI_MULTIPLIER, u64::MAX, 0)
    }

    pub fn new_ui_multiplier(t: StockToken) -> Option<U> {
        static_call_word_opt(t.addr(), &SEL_NEW_UI_MULTIPLIER, u64::MAX, 0)
    }

    pub fn transfer(t: StockToken, recipient: Address, amt: U) -> bool {
        call_bool(
            t.addr(),
            &make_fn_transfer(recipient, &amt),
            &U::ZERO,
            u64::MAX,
        )
    }

    pub fn transfer_from(t: StockToken, from: Address, recipient: Address, amt: U) -> bool {
        call_bool(
            t.addr(),
            &make_fn_transfer_from(from, recipient, &amt),
            &U::ZERO,
            u64::MAX,
        )
    }

    pub fn permit(
        t: StockToken,
        owner: Address,
        spender: Address,
        value: U,
        deadline: U,
        v: u8,
        r: U,
        s: U,
    ) -> bool {
        call_bool(
            t.addr(),
            &make_fn_permit(owner, spender, &value, &deadline, v, &r, &s),
            &U::ZERO,
            u64::MAX,
        )
    }
}

#[cfg(feature = "stylus-sdk")]
pub mod stylus {
    extern crate alloc;

    use alloc::vec::Vec;

    use stylus_chainlink_price_feeds::{
        ErrGetLatestRoundData, ErrGetLatestRoundDataReason, get_latest_round_data,
        get_latest_round_data_split,
    };

    use core::fmt::{Display, Formatter, Result as FmtResult};

    use stylus_sdk::{
        alloy_primitives::{Address as StylusAddr, FixedBytes, U256},
        call,
        prelude::{
            Host, MutatingCallContext, StaticCallContext, calls::errors::Error as StylusError,
        },
    };

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum ErrContractCallReason {
        Revert,
        BadRd,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ErrContractCall(Vec<u8>, ErrContractCallReason);

    impl Display for ErrContractCallReason {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "{self:?}")
        }
    }

    impl core::error::Error for ErrContractCallReason {}

    pub fn price<H, C>(host: &H, ctx: C, t: StockToken) -> Result<U256, ErrContractCall>
    where
        H: Host + ?Sized,
        C: StaticCallContext,
    {
        get_latest_round_data(host, ctx, t.oracle()).map_err(|ErrGetLatestRoundData(x, v)| {
            match (x, v) {
                (x, ErrGetLatestRoundDataReason::Revert) => {
                    ErrContractCall(x, ErrContractCallReason::Revert)
                }
                (x, ErrGetLatestRoundDataReason::BadRd) => {
                    ErrContractCall(x, ErrContractCallReason::BadRd)
                }
            }
        })
    }

    pub fn price_split<H, C>(
        host: &H,
        ctx: C,
        t: StockToken,
    ) -> Result<(U256, U256), ErrContractCall>
    where
        H: Host + ?Sized,
        C: StaticCallContext,
    {
        get_latest_round_data_split(host, ctx, t.oracle()).map_err(|ErrGetLatestRoundData(x, v)| {
            match (x, v) {
                (x, ErrGetLatestRoundDataReason::Revert) => {
                    ErrContractCall(x, ErrContractCallReason::Revert)
                }
                (x, ErrGetLatestRoundDataReason::BadRd) => {
                    ErrContractCall(x, ErrContractCallReason::BadRd)
                }
            }
        })
    }

    fn static_call<H, C>(
        cd: &[u8],
        host: &H,
        ctx: C,
        t: StockToken,
    ) -> Result<Vec<u8>, ErrContractCall>
    where
        H: Host + ?Sized,
        C: StaticCallContext,
    {
        call::static_call(host, ctx, StylusAddr::from(t.addr()), cd).map_err(|v| match v {
            StylusError::Revert(v) => ErrContractCall(v, ErrContractCallReason::Revert),
            _ => unimplemented!(),
        })
    }

    pub fn paused<H, C>(host: &H, ctx: C, t: StockToken) -> Result<bool, ErrContractCall>
    where
        H: Host + ?Sized,
        C: StaticCallContext,
    {
        Ok(static_call(&SEL_PAUSED, host, ctx, t)?[31] != 0)
    }

    pub fn token_paused<H, C>(host: &H, ctx: C, t: StockToken) -> Result<bool, ErrContractCall>
    where
        H: Host + ?Sized,
        C: StaticCallContext,
    {
        Ok(static_call(&SEL_TOKEN_PAUSED, host, ctx, t)?[31] != 0)
    }

    pub fn oracle_paused<H, C>(host: &H, ctx: C, t: StockToken) -> Result<bool, ErrContractCall>
    where
        H: Host + ?Sized,
        C: StaticCallContext,
    {
        Ok(static_call(&SEL_ORACLE_PAUSED, host, ctx, t)?[31] != 0)
    }

    pub fn ui_multiplier<H, C>(host: &H, ctx: C, t: StockToken) -> Result<U256, ErrContractCall>
    where
        H: Host + ?Sized,
        C: StaticCallContext,
    {
        let rd = static_call(&SEL_UI_MULTIPLIER, host, ctx, t)?;
        U256::try_from_be_slice(&rd).ok_or(ErrContractCall(rd, ErrContractCallReason::BadRd))
    }

    pub fn new_ui_multiplier<H, C>(host: &H, ctx: C, t: StockToken) -> Result<U256, ErrContractCall>
    where
        H: Host + ?Sized,
        C: StaticCallContext,
    {
        let rd = static_call(&SEL_NEW_UI_MULTIPLIER, host, ctx, t)?;
        U256::try_from_be_slice(&rd).ok_or(ErrContractCall(rd, ErrContractCallReason::BadRd))
    }

    fn call<H, C>(cd: &[u8], host: &H, ctx: C, t: StockToken) -> Result<Vec<u8>, ErrContractCall>
    where
        H: Host + ?Sized,
        C: MutatingCallContext,
    {
        call::call(host, ctx, StylusAddr::from(t.addr()), cd).map_err(|v| match v {
            StylusError::Revert(v) => ErrContractCall(v, ErrContractCallReason::Revert),
            _ => unimplemented!(),
        })
    }

    pub fn transfer<H, C>(
        host: &H,
        ctx: C,
        t: StockToken,
        StylusAddr(recipient): StylusAddr,
        amt: U256,
    ) -> Result<U256, ErrContractCall>
    where
        H: Host + ?Sized,
        C: MutatingCallContext,
    {
        let amt: [u8; 32] = amt.to_be_bytes();
        let rd = call(&make_fn_transfer(*recipient, &amt.into()), host, ctx, t)?;
        U256::try_from_be_slice(&rd).ok_or(ErrContractCall(rd, ErrContractCallReason::BadRd))
    }

    pub fn transfer_from<H, C>(
        host: &H,
        ctx: C,
        t: StockToken,
        StylusAddr(from): StylusAddr,
        StylusAddr(recipient): StylusAddr,
        amt: U256,
    ) -> Result<U256, ErrContractCall>
    where
        H: Host + ?Sized,
        C: MutatingCallContext,
    {
        let amt: [u8; 32] = amt.to_be_bytes();
        let rd = call(
            &make_fn_transfer_from(*from, *recipient, &amt.into()),
            host,
            ctx,
            t,
        )?;
        U256::try_from_be_slice(&rd).ok_or(ErrContractCall(rd, ErrContractCallReason::BadRd))
    }

    pub fn permit<H, C>(
        host: &H,
        ctx: C,
        t: StockToken,
        StylusAddr(owner): StylusAddr,
        StylusAddr(spender): StylusAddr,
        value: U256,
        deadline: U256,
        v: u8,
        FixedBytes(r): FixedBytes<32>,
        FixedBytes(s): FixedBytes<32>,
    ) -> Result<U256, ErrContractCall>
    where
        H: Host + ?Sized,
        C: MutatingCallContext,
    {
        let value: [u8; 32] = value.to_be_bytes();
        let deadline: [u8; 32] = deadline.to_be_bytes();
        let rd = call(
            &make_fn_permit(
                *owner,
                *spender,
                &value.into(),
                &deadline.into(),
                v,
                &r.into(),
                &s.into(),
            ),
            host,
            ctx,
            t,
        )?;
        U256::try_from_be_slice(&rd).ok_or(ErrContractCall(rd, ErrContractCallReason::BadRd))
    }
}

#[cfg(feature = "stylus-sdk")]
pub use stylus::*;

#[cfg(all(feature = "bobcat-sdk", not(feature = "stylus-sdk")))]
pub use bobcat::*;
