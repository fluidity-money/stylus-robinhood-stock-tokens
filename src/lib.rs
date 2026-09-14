#![no_std]

use bobcat_cd::address;

pub use stylus_chainlink_price_feeds::PriceFeed;

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
    pub const fn addr(self) -> [u8; 20] {
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

    pub const fn oracle(self) -> PriceFeed {
        match self {
            Self::Aapl => PriceFeed::AaplUsd,
            Self::Amd => PriceFeed::AmdUsd,
            Self::Amzn => PriceFeed::AmznUsd,
            Self::Asml => PriceFeed::AsmlUsd,
            Self::Baba => PriceFeed::BabaUsd,
            Self::Clsk => PriceFeed::ClskUsd,
            Self::Coin => PriceFeed::CoinUsd,
            Self::Crcl => PriceFeed::CrclUsd,
            Self::Crwv => PriceFeed::CrwvUsd,
            Self::Dell => PriceFeed::DellUsd,
            Self::Ewy => PriceFeed::EwyUsd,
            Self::Gme => PriceFeed::GmeUsd,
            Self::Googl => PriceFeed::GooglUsd,
            Self::Intc => PriceFeed::IntcUsd,
            Self::Ionq => PriceFeed::IonqUsd,
            Self::Meta => PriceFeed::MetaUsd,
            Self::Msft => PriceFeed::MsftUsd,
            Self::Mstr => PriceFeed::MstrUsd,
            Self::Mu => PriceFeed::MuUsd,
            Self::Nbis => PriceFeed::NbisUsd,
            Self::Nvda => PriceFeed::NvdaUsd,
            Self::Orcl => PriceFeed::OrclUsd,
            Self::Pltr => PriceFeed::PltrUsd,
            Self::Qqq => PriceFeed::QqqUsd,
            Self::Rgti => PriceFeed::RgtiUsd,
            Self::Rklb => PriceFeed::RklbUsd,
            Self::Sgov => PriceFeed::SgovUsd,
            Self::Slv => PriceFeed::SlvUsd,
            Self::Sndk => PriceFeed::SndkUsd,
            Self::Spcx => PriceFeed::SpcxUsd,
            Self::Spy => PriceFeed::SpyUsd,
            Self::Tsla => PriceFeed::TslaUsd,
            Self::Tsm => PriceFeed::TsmUsd,
            Self::Usar => PriceFeed::UsarUsd,
            Self::Uso => PriceFeed::UsoUsd,
        }
    }

    pub const fn oracle_addr(self) -> [u8; 20] {
        self.oracle().addr()
    }
}

impl From<StockToken> for [u8; 20] {
    fn from(token: StockToken) -> Self {
        token.addr()
    }
}

/// Price lookup through `stylus-chainlink-price-feeds`' Bobcat backend.
#[cfg(feature = "bobcat-sdk")]
pub mod bobcat {
    use bobcat_maths::U;
    use stylus_chainlink_price_feeds::get_latest_round_data_opt;

    use crate::StockToken;

    pub fn price(token: StockToken) -> Option<U> {
        get_latest_round_data_opt(token.oracle())
    }
}

#[cfg(feature = "stylus-sdk")]
pub mod stylus {
    use stylus_chainlink_price_feeds::{ErrGetLatestRoundData, get_latest_round_data};
    use stylus_sdk::{
        alloy_primitives::U256,
        prelude::{Host, StaticCallContext},
    };

    use crate::StockToken;

    pub fn price<H, C>(host: &H, ctx: C, token: StockToken) -> Result<U256, ErrGetLatestRoundData>
    where
        H: Host + ?Sized,
        C: StaticCallContext,
    {
        get_latest_round_data(host, ctx, token.oracle())
    }
}

#[cfg(feature = "stylus-sdk")]
pub use stylus::price;

#[cfg(all(feature = "bobcat-sdk", not(feature = "stylus-sdk")))]
pub use bobcat::price;
