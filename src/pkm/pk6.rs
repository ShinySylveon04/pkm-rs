use super::{pkx::Pkx, poke_crypto, types};
use core::convert::TryInto;
use no_std_io::Reader;
use safe_transmute::TriviallyTransmutable;

pub type Pk6Bytes = [u8; Pk6::STORED_SIZE];

pub struct Pk6 {
    data: Pk6Bytes,
}

impl Pk6 {
    pub const STORED_SIZE: usize = 232;
    pub const BLOCK_SIZE: usize = 56;

    pub fn new(data: [u8; Pk6::STORED_SIZE]) -> Self {
        let seed_bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let seed = u32::from_le_bytes(seed_bytes);
        Self {
            data: poke_crypto::decrypt::<{ Pk6::STORED_SIZE }, { Pk6::BLOCK_SIZE }>(data, seed),
        }
    }
}

impl Reader for Pk6 {
    fn get_slice(&self) -> &[u8] {
        &self.data
    }
}

impl Pkx for Pk6 {
    fn encryption_constant(&self) -> u32 {
        self.default_read_le(0x00)
    }

    fn species(&self) -> types::Species {
        self.default_read_le::<u16>(0x08).into()
    }

    fn tid(&self) -> u16 {
        self.default_read_le(0x0C)
    }

    fn sid(&self) -> u16 {
        self.default_read_le(0x0E)
    }

    fn ability(&self) -> types::Ability {
        let ability: u8 = self.default_read(0x14);
        (ability as u16).into()
    }

    fn ability_number(&self) -> types::AbilityNumber {
        self.default_read::<u8>(0x15).into()
    }

    fn pid(&self) -> u32 {
        self.default_read_le(0x18)
    }

    fn nature(&self) -> types::Nature {
        self.default_read::<u8>(0x1C).into()
    }

    fn gender(&self) -> types::Gender {
        let byte = self.default_read::<u8>(0x1D);
        ((byte >> 1) & 3).into()
    }

    fn evs(&self) -> types::Stats {
        types::Stats {
            hp: self.default_read(0x1E),
            atk: self.default_read(0x1F),
            def: self.default_read(0x20),
            spa: self.default_read(0x21),
            spd: self.default_read(0x22),
            spe: self.default_read(0x23),
        }
    }

    fn move1(&self) -> types::Move {
        self.default_read::<u16>(0x5A).into()
    }

    fn move2(&self) -> types::Move {
        self.default_read::<u16>(0x5C).into()
    }

    fn move3(&self) -> types::Move {
        self.default_read::<u16>(0x5E).into()
    }

    fn move4(&self) -> types::Move {
        self.default_read::<u16>(0x60).into()
    }

    fn iv32(&self) -> u32 {
        self.default_read_le(0x74)
    }

    fn ht_friendship(&self) -> u32 {
        self.default_read(0xA2)
    }

    fn ot_friendship(&self) -> u32 {
        self.default_read(0xCA)
    }

    fn language(&self) -> types::Language {
        self.default_read::<u8>(0xE3).into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pk6Data(Pk6Bytes);

// This is safe because the bytes in Pk6Data can be anything
unsafe impl TriviallyTransmutable for Pk6Data {}

impl From<Pk6Data> for Pk6 {
    fn from(data: Pk6Data) -> Self {
        Self::new(data.0)
    }
}

impl Default for Pk6Data {
    fn default() -> Self {
        Self([0; Pk6::STORED_SIZE])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_EKX: Pk6Bytes = [
        0x80, 0x5c, 0x86, 0x02, 0x00, 0x00, 0xd6, 0x41, 0x20, 0x0e, 0x56, 0x4f, 0xaa, 0xf1, 0xf4,
        0x2f, 0xa5, 0x9e, 0xcc, 0xfe, 0x8b, 0xf2, 0x32, 0x20, 0x51, 0xd1, 0x99, 0xdd, 0x42, 0xd2,
        0x55, 0xe5, 0x05, 0x1f, 0x85, 0x2a, 0x62, 0xe2, 0x2a, 0x14, 0x5a, 0x21, 0x96, 0xdb, 0x76,
        0x2e, 0xd6, 0x4e, 0x72, 0xa0, 0x72, 0x08, 0xa0, 0x2b, 0x59, 0x35, 0xf9, 0x56, 0xba, 0xc6,
        0x92, 0x55, 0x0c, 0x01, 0xf9, 0x2b, 0xdb, 0x58, 0xbd, 0x84, 0x5a, 0xc9, 0x94, 0x77, 0x96,
        0x72, 0x1d, 0x5b, 0x13, 0xd1, 0x8a, 0x7b, 0x7e, 0x07, 0x93, 0xec, 0xe2, 0x81, 0x08, 0x4b,
        0x13, 0xfa, 0xda, 0x5f, 0x4a, 0x6c, 0x0a, 0xcb, 0x50, 0x90, 0xb9, 0x48, 0x37, 0x99, 0x68,
        0x9b, 0x51, 0xe9, 0xe7, 0x1b, 0xfe, 0x80, 0xcb, 0x56, 0xad, 0x23, 0xb8, 0x56, 0x50, 0x60,
        0x47, 0xf4, 0x59, 0x27, 0xee, 0x49, 0xb3, 0x76, 0xcb, 0xa7, 0xef, 0x77, 0xe7, 0x59, 0xdb,
        0xd8, 0xe9, 0x1e, 0x4e, 0xe9, 0xf5, 0xa9, 0xf3, 0xb7, 0x77, 0x93, 0x7c, 0x45, 0x86, 0x5e,
        0xef, 0x41, 0x3f, 0x0d, 0xb1, 0xb6, 0x66, 0xf2, 0xd8, 0x86, 0x98, 0x64, 0xf2, 0xf2, 0x7f,
        0x4b, 0x86, 0xf6, 0x46, 0xda, 0x44, 0x7f, 0xec, 0x75, 0x34, 0xd4, 0xcd, 0x58, 0x4b, 0x7a,
        0x33, 0x21, 0x3e, 0xdf, 0x68, 0xb1, 0xe9, 0xbd, 0x55, 0x11, 0x91, 0x28, 0x53, 0x6e, 0xfb,
        0x5a, 0xc1, 0xcf, 0x38, 0x72, 0xec, 0x04, 0xd1, 0xac, 0xe1, 0x8c, 0x5a, 0x51, 0x30, 0xb4,
        0x8b, 0xa4, 0xec, 0x45, 0xbc, 0x43, 0x6d, 0x14, 0xb8, 0x8e, 0x93, 0x80, 0x91, 0x1e, 0x91,
        0xca, 0x14, 0xb7, 0xdf, 0xf2, 0xb3, 0x26,
    ];

    #[test]
    fn should_decrypt() {
        let result: Pk6Bytes = [
            0x80, 0x5c, 0x86, 0x02, 0x00, 0x00, 0xd6, 0x41, 0x84, 0x00, 0x18, 0x01, 0x56, 0xf6,
            0x42, 0xc8, 0x40, 0x42, 0x0f, 0x00, 0x96, 0x04, 0x00, 0x00, 0x23, 0x0f, 0x37, 0x31,
            0x03, 0x04, 0xfc, 0x00, 0x06, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x3f, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x64, 0x00, 0x61, 0x00,
            0x6d, 0x00, 0x61, 0x00, 0x6e, 0x00, 0x74, 0x00, 0x20, 0x00, 0x36, 0x00, 0x49, 0x00,
            0x56, 0x00, 0x73, 0x00, 0x00, 0x00, 0x90, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x10, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xbf, 0x45, 0x00, 0x56, 0x00, 0x92, 0xe0,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x2c, 0x31, 0x0a, 0x12, 0x2c, 0x31,
            0x10, 0x31, 0x00, 0x31, 0x00, 0x00, 0x00, 0x00, 0x46, 0x00, 0x03, 0x04, 0x00, 0x00,
            0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x00, 0x69, 0x00, 0x74, 0x00,
            0x74, 0x00, 0x6f, 0x00, 0x20, 0x00, 0x69, 0x00, 0x73, 0x00, 0x20, 0x00, 0x92, 0xe0,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 0x03, 0x07, 0x0f, 0x97, 0x00, 0x02, 0x00,
            0x00, 0x00, 0x0c, 0x0c, 0x19, 0x00, 0x00, 0x00, 0x94, 0x00, 0x0b, 0x1e, 0x00, 0x18,
            0x12, 0x0a, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00,
        ];

        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.get_slice(), result)
    }

    #[test]
    fn pk6_data_size_should_be_232() {
        assert_eq!(core::mem::size_of::<Pk6Data>(), Pk6::STORED_SIZE);
    }

    #[test]
    fn should_read_species() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.species(), types::Species::Ditto);
    }

    #[test]
    fn should_read_pid() {
        let pkx = Pk6::new(TEST_EKX);
        let pid = 0x31370F23;
        assert_eq!(pkx.pid(), pid)
    }

    #[test]
    fn should_read_tid() {
        let pkx = Pk6::new(TEST_EKX);
        let tid = 63062;
        assert_eq!(pkx.tid(), tid)
    }

    #[test]
    fn should_read_sid() {
        let pkx = Pk6::new(TEST_EKX);
        let sid = 51266;
        assert_eq!(pkx.sid(), sid)
    }

    #[test]
    fn should_read_tsv() {
        let pkx = Pk6::new(TEST_EKX);
        let tsv = 0993;
        assert_eq!(pkx.tsv(), tsv)
    }

    #[test]
    fn should_read_psv() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.psv(), 0993)
    }

    #[test]
    fn should_read_nature() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.nature(), types::Nature::Adamant)
    }

    #[test]
    fn should_read_minted_nature() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.minted_nature(), types::Nature::Adamant)
    }

    #[test]
    fn should_read_ability() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.ability(), types::Ability::Imposter)
    }

    #[test]
    fn should_read_ability_number() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.ability_number(), types::AbilityNumber::Hidden)
    }

    #[test]
    fn should_read_hidden_power() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.hidden_power(), types::HiddenPower::Dark)
    }

    #[test]
    fn should_read_language() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.language(), types::Language::French)
    }

    #[test]
    fn should_read_gender() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.gender(), types::Gender::Genderless)
    }

    #[test]
    fn should_read_move1() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.move1(), types::Move::Transform)
    }

    #[test]
    fn should_read_move2() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.move2(), types::Move::None)
    }

    #[test]
    fn should_read_move3() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.move3(), types::Move::None)
    }

    #[test]
    fn should_read_move4() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.move4(), types::Move::None)
    }

    #[test]
    fn should_read_ivs() {
        let pkx = Pk6::new(TEST_EKX);
        let stats = types::Stats {
            hp: 31,
            atk: 31,
            def: 31,
            spa: 31,
            spd: 31,
            spe: 31,
        };
        assert_eq!(pkx.ivs(), stats)
    }

    #[test]
    fn should_read_evs() {
        let pkx = Pk6::new(TEST_EKX);
        let stats = types::Stats {
            hp: 252,
            atk: 0,
            def: 6,
            spa: 252,
            spd: 0,
            spe: 0,
        };
        assert_eq!(pkx.evs(), stats)
    }

    #[test]
    fn should_read_ot_friendship() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.ot_friendship(), 0)
    }

    #[test]
    fn should_read_ht_friendship() {
        let pkx = Pk6::new(TEST_EKX);
        assert_eq!(pkx.ht_friendship(), 0)
    }
}
