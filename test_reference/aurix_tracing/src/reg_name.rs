/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.8.0 on Tue, 30 Jun 2026 15:56:11 +0000

//! Contains perfect hash function that maps form raw addresses to
//! a string containing the names of all registers that point to an address.
//!
//! When using tracing feature to record accesses to registers, the exact
//! API path, though which a specific address was accessed gets lost.
//! This poses a problem when recorded register accesses contain accesses
//! to unexpected registers. [`reg_name_from_addr`] can be used to make
//! logs of raw register accesses more readable to humans by providing a list
//! of names of registers that alias a specific physical address.
//!
use phf::phf_map;

/// Get a &str name of a register given it's address.
pub fn reg_name_from_addr(addr: u64) -> Option<&'static &'static str> {
    REGISTER_NAMES.get(&addr)
}

static REGISTER_NAMES: phf::Map<u64, &'static str> = phf_map! {
  0x40010000u64 => "
      TIMER.bitfield_reg(),
      TIMER.bitfield_reg_alt_group(),
      TIMER.cluster1().cluster1().nestedreg(),
      TIMER.cluster1().hssl()[0].ch()[0].hsslxcoky(),
      TIMER.cluster1().cr(),
      TIMER.clusterdim()[0].cr(),
    ",
  0x40010004u64 => "
      TIMER.sr(),
      TIMER.cluster1().hssl()[0].ch()[1].hsslxcoky(),
    ",
  0x40010010u64 => "
      TIMER.int(),
    ",
  0x40010020u64 => "
      TIMER.nobitfield_reg(),
    ",
  0x40010024u64 => "
      TIMER.r#match(),
    ",
  0x40010028u64 => "
      TIMER.prescale_rd(),
    ",
  0x4001002cu64 => "
      TIMER.prescale_wr(),
    ",
  0x40010050u64 => "
      TIMER.arrayreg()[0],
    ",
  0x40010054u64 => "
      TIMER.arrayreg()[1],
    ",
  0x40010058u64 => "
      TIMER.arrayreg()[2],
    ",
  0x4001005cu64 => "
      TIMER.arrayreg()[3],
    ",
  0x40010060u64 => "
      TIMER.register64bit(),
    ",
  0x40012000u64 => "
      TIMER.timer(),
    ",
  0x40010100u64 => "
      TIMER.clusterdim()[1].cr(),
    ",
  0x40010200u64 => "
      TIMER.clusterdim()[2].cr(),
    ",
  0x40010300u64 => "
      TIMER.clusterdim()[3].cr(),
    ",
  0x50000000u64 => "
      UART[0].reg1_()[0],
      UART[0].uart().uart(),
    ",
  0x50000004u64 => "
      UART[0].reg1_()[1],
    ",
  0x50000100u64 => "
      UART[0].regbitfieldraw(),
    ",
  0x50000104u64 => "
      UART[0].reg16bitenum(),
    ",
  0x50000106u64 => "
      UART[0].reg8bitraw(),
    ",
  0x50000108u64 => "
      UART[0].reg16bitraw(),
    ",
  0x50000110u64 => "
      UART[0].reg32bitraw(),
    ",
  0x50000200u64 => "
      UART[0].regenumvalue(),
    ",
  0x50001004u64 => "
      UART[1].reg1_()[0],
      UART[1].uart().uart(),
    ",
  0x50001008u64 => "
      UART[1].reg1_()[1],
    ",
  0x50001104u64 => "
      UART[1].regbitfieldraw(),
    ",
  0x50001108u64 => "
      UART[1].reg16bitenum(),
    ",
  0x5000110au64 => "
      UART[1].reg8bitraw(),
    ",
  0x5000110cu64 => "
      UART[1].reg16bitraw(),
    ",
  0x50001114u64 => "
      UART[1].reg32bitraw(),
    ",
  0x50001204u64 => "
      UART[1].regenumvalue(),
    ",
  0x50002008u64 => "
      UART[2].reg1_()[0],
      UART[2].uart().uart(),
    ",
  0x5000200cu64 => "
      UART[2].reg1_()[1],
    ",
  0x50002108u64 => "
      UART[2].regbitfieldraw(),
    ",
  0x5000210cu64 => "
      UART[2].reg16bitenum(),
    ",
  0x5000210eu64 => "
      UART[2].reg8bitraw(),
    ",
  0x50002110u64 => "
      UART[2].reg16bitraw(),
    ",
  0x50002118u64 => "
      UART[2].reg32bitraw(),
    ",
  0x50002208u64 => "
      UART[2].regenumvalue(),
    ",
  0x60000000u64 => "
      FOO.r#in(),
    ",
  0x70000000u64 => "
      ESCAPE_TEST.register(),
    ",
  0xa1000u64 => "
      DERIVED_TEST.baseregister(),
    ",
  0xa1002u64 => "
      DERIVED_TEST.derivedregister(),
    ",
  0xa1004u64 => "
      DERIVED_TEST.derivedfromfaraway(),
    ",
  0xa0000u64 => "
      DERIVED_TEST.basecluster().reg1(),
    ",
  0xa0004u64 => "
      DERIVED_TEST.basecluster().reg2(),
    ",
  0x70100000u64 => "
      P_33.i2c2().reg1(),
    ",
  0x70100004u64 => "
      P_33.i2c2().reg2(),
    ",
  0x70300000u64 => "
      HAS_HEADER_STRUCT.i2c2().reg1(),
    ",
  0x70300004u64 => "
      HAS_HEADER_STRUCT.i2c2().reg2(),
    ",
  0x70700000u64 => "
      DIM_INDEX_PERI.clust()[0].areg()[0],
    ",
  0x70700004u64 => "
      DIM_INDEX_PERI.clust()[0].areg()[1],
    ",
  0x70700008u64 => "
      DIM_INDEX_PERI.clust()[0].areg()[2],
    ",
  0x7070000cu64 => "
      DIM_INDEX_PERI.clust()[0].breg()[0],
    ",
  0x70700010u64 => "
      DIM_INDEX_PERI.clust()[0].breg()[1],
    ",
  0x70700014u64 => "
      DIM_INDEX_PERI.clust()[0].breg()[2],
    ",
  0x70700018u64 => "
      DIM_INDEX_PERI.clust()[0].creg()[0],
    ",
  0x7070001cu64 => "
      DIM_INDEX_PERI.clust()[0].creg()[1],
    ",
  0x70700020u64 => "
      DIM_INDEX_PERI.clust()[0].creg()[2],
    ",
  0x70700030u64 => "
      DIM_INDEX_PERI.clust()[1].areg()[0],
    ",
  0x70700034u64 => "
      DIM_INDEX_PERI.clust()[1].areg()[1],
    ",
  0x70700038u64 => "
      DIM_INDEX_PERI.clust()[1].areg()[2],
    ",
  0x7070003cu64 => "
      DIM_INDEX_PERI.clust()[1].breg()[0],
    ",
  0x70700040u64 => "
      DIM_INDEX_PERI.clust()[1].breg()[1],
    ",
  0x70700044u64 => "
      DIM_INDEX_PERI.clust()[1].breg()[2],
    ",
  0x70700048u64 => "
      DIM_INDEX_PERI.clust()[1].creg()[0],
    ",
  0x7070004cu64 => "
      DIM_INDEX_PERI.clust()[1].creg()[1],
    ",
  0x70700050u64 => "
      DIM_INDEX_PERI.clust()[1].creg()[2],
    ",
  0x70700060u64 => "
      DIM_INDEX_PERI.clust()[2].areg()[0],
    ",
  0x70700064u64 => "
      DIM_INDEX_PERI.clust()[2].areg()[1],
    ",
  0x70700068u64 => "
      DIM_INDEX_PERI.clust()[2].areg()[2],
    ",
  0x7070006cu64 => "
      DIM_INDEX_PERI.clust()[2].breg()[0],
    ",
  0x70700070u64 => "
      DIM_INDEX_PERI.clust()[2].breg()[1],
    ",
  0x70700074u64 => "
      DIM_INDEX_PERI.clust()[2].breg()[2],
    ",
  0x70700078u64 => "
      DIM_INDEX_PERI.clust()[2].creg()[0],
    ",
  0x7070007cu64 => "
      DIM_INDEX_PERI.clust()[2].creg()[1],
    ",
  0x70700080u64 => "
      DIM_INDEX_PERI.clust()[2].creg()[2],
    ",
};
