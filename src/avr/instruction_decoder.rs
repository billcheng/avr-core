use crate::avr::operations::lpm_rd_inc::LpmRdInc;
use crate::avr::core_type::CoreType;
use crate::avr::operation::Instruction;
use crate::avr::operations::adc::Adc;
use crate::avr::operations::add::Add;
use crate::avr::operations::adiw::Adiw;
use crate::avr::operations::and::And;
use crate::avr::operations::andi::Andi;
use crate::avr::operations::asr::Asr;
use crate::avr::operations::bclr::Bclr;
use crate::avr::operations::bld::Bld;
use crate::avr::operations::brbc::Brbc;
use crate::avr::operations::brbs::Brbs;
use crate::avr::operations::bset::Bset;
use crate::avr::operations::bst::Bst;
use crate::avr::operations::call::Call;
use crate::avr::operations::cbi::Cbi;
use crate::avr::operations::com::Com;
use crate::avr::operations::cp::Cp;
use crate::avr::operations::cpc::Cpc;
use crate::avr::operations::cpi::Cpi;
use crate::avr::operations::cpse::Cpse;
use crate::avr::operations::dec::Dec;
use crate::avr::operations::eor::Eor;
use crate::avr::operations::fmul::Fmul;
use crate::avr::operations::fmuls::Fmuls;
use crate::avr::operations::fmulsu::Fmulsu;
use crate::avr::operations::icall::Icall;
use crate::avr::operations::ijmp::Ijmp;
use crate::avr::operations::in_io::In;
use crate::avr::operations::inc::Inc;
use crate::avr::operations::jmp::Jmp;
use crate::avr::operations::lac::Lac;
use crate::avr::operations::las::Las;
use crate::avr::operations::lat::Lat;
use crate::avr::operations::ld::Ld;
use crate::avr::operations::ld_dec::LdDec;
use crate::avr::operations::ld_inc::LdInc;
use crate::avr::operations::lddy::Lddy;
use crate::avr::operations::lddy_dec::LddyDec;
use crate::avr::operations::lddy_inc::LddyInc;
use crate::avr::operations::lddyq::Lddyq;
use crate::avr::operations::lddz::Lddz;
use crate::avr::operations::lddz_dec::LddzDec;
use crate::avr::operations::lddz_inc::LddzInc;
use crate::avr::operations::lddzq::Lddzq;
use crate::avr::operations::ldi::Ldi;
use crate::avr::operations::lds::Lds;
use crate::avr::operations::lds_avrc::LdsAvrc;
use crate::avr::operations::lpm::Lpm;
use crate::avr::operations::lpm_rd::LpmRd;
use crate::avr::util::opcode_size::Opcode;
use std::rc::Rc;

pub struct InstructionDecoder {
  opcode: Rc<Opcode>,
}

impl InstructionDecoder {
  pub fn new() -> Self {
    Self {
      opcode: Rc::new(Opcode::new()),
    }
  }

  pub fn get(&self, core_type: &CoreType, opcode: u16, next_opcode: u16) -> Box<dyn Instruction> {
    let is_adc = opcode & 0b1111_1100_0000_0000 == 0b0001_1100_0000_0000;
    if is_adc {
      return Box::new(Adc::new(opcode));
    }

    let is_add = opcode & 0b1111_1100_0000_0000 == 0b0000_1100_0000_0000;
    if is_add {
      return Box::new(Add::new(opcode));
    }

    let is_adiw = opcode & 0b1111_1111_0000_0000 == 0b1001_0110_0000_0000;
    if is_adiw {
      return Box::new(Adiw::new(opcode));
    }

    let is_and = opcode & 0b1111_1100_0000_0000 == 0b0010_0000_0000_0000;
    if is_and {
      return Box::new(And::new(opcode));
    }

    let is_andi = opcode & 0b1111_0000_0000_0000 == 0b0111_0000_0000_0000;
    if is_andi {
      return Box::new(Andi::new(opcode));
    }

    let is_asr = opcode & 0b1111_1110_0000_1111 == 0b1001_0100_0000_0101;
    if is_asr {
      return Box::new(Asr::new(opcode));
    }

    let is_bclr = opcode & 0b1111_1111_1000_1111 == 0b1001_0100_1000_1000;
    if is_bclr {
      return Box::new(Bclr::new(opcode));
    }

    let is_bld = opcode & 0b1111_1111_1000_1111 == 0b1111_1000_0000_0000;
    if is_bld {
      return Box::new(Bld::new(opcode));
    }

    let is_brbc = opcode & 0b1111_1100_0000_0000 == 0b1111_0100_0000_0000;
    if is_brbc {
      return Box::new(Brbc::new(opcode));
    }

    let is_brbs = opcode & 0b1111_1100_0000_0000 == 0b1111_0000_0000_0000;
    if is_brbs {
      return Box::new(Brbs::new(opcode));
    }

    let is_bset = opcode & 0b1111_1111_1000_1111 == 0b1001_0100_0000_1000;
    if is_bset {
      return Box::new(Bset::new(opcode));
    }

    let is_bst = opcode & 0b1111_1110_0000_1000 == 0b1111_1010_0000_0000;
    if is_bst {
      return Box::new(Bst::new(opcode));
    }

    let is_call = opcode & 0b1111_1110_0000_1110 == 0b1001_010_0000_1110;
    if is_call {
      return Box::new(Call::new(core_type, opcode, next_opcode));
    }

    let is_cbi = opcode & 0b1111_1111_0000_0000 == 0b1001_1000_0000_0000;
    if is_cbi {
      return Box::new(Cbi::new(opcode));
    }

    let is_com = opcode & 0b1111_1110_0000_1111 == 0b1001_0100_0000_0000;
    if is_com {
      return Box::new(Com::new(opcode));
    }

    let is_cp = opcode & 0b1111_1100_0000_0000 == 0b0001_0100_0000_0000;
    if is_cp {
      return Box::new(Cp::new(opcode));
    }

    let is_cpc = opcode & 0b1111_1100_0000_0000 == 0b0000_0100_0000_0000;
    if is_cpc {
      return Box::new(Cpc::new(opcode));
    }

    let is_cpi = opcode & 0b1111_0000_0000_0000 == 0b0011_0000_0000_0000;
    if is_cpi {
      return Box::new(Cpi::new(opcode));
    }

    let is_cpse = opcode & 0b1111_1100_0000_0000 == 0b0001_0000_0000_0000;
    if is_cpse {
      return Box::new(Cpse::new(opcode, next_opcode, &self.opcode));
    }

    let is_dec = opcode & 0b1111_1110_0000_1111 == 0b1001_0100_0000_1010;
    if is_dec {
      return Box::new(Dec::new(opcode));
    }

    let is_eor = opcode & 0b1111_1100_0000_0000 == 0b0010_0100_0000_0000;
    if is_eor {
      return Box::new(Eor::new(opcode));
    }

    let is_fmul = opcode & 0b1111_1111_1000_1000 == 0b0000_0011_0000_1000;
    if is_fmul {
      return Box::new(Fmul::new(opcode));
    }

    let is_fmuls = opcode & 0b1111_1111_1000_1000 == 0b0000_0011_1000_0000;
    if is_fmuls {
      return Box::new(Fmuls::new(opcode));
    }

    let is_fmulsu = opcode & 0b1111_1111_1000_1000 == 0b0000_0011_1000_1000;
    if is_fmulsu {
      return Box::new(Fmulsu::new(opcode));
    }

    let is_icall = opcode == 0b1001_0101_0000_1001;
    if is_icall {
      return Box::new(Icall::new(core_type));
    }

    let is_ijmp = opcode == 0b1001_0100_0000_1001;
    if is_ijmp {
      return Box::new(Ijmp::new());
    }

    let is_io = opcode & 0b1111_1000_0000_0000 == 0b1011_0000_0000_0000;
    if is_io {
      return Box::new(In::new(opcode));
    }

    let is_inc = opcode & 0b1111_1110_0000_1111 == 0b1001_0100_0000_0011;
    if is_inc {
      return Box::new(Inc::new(opcode));
    }

    let is_jmp = opcode & 0b1111_1110_0000_1110 == 0b1001_0100_0000_1100;
    if is_jmp {
      return Box::new(Jmp::new(opcode, next_opcode));
    }

    let is_lac = opcode & 0b1111_1110_0000_1111 == 0b1001_0010_0000_0110;
    if is_lac {
      return Box::new(Lac::new(opcode));
    }

    let is_las = opcode & 0b1111_1110_0000_1111 == 0b1001_0010_0000_0101;
    if is_las {
      return Box::new(Las::new(opcode));
    }

    let is_lat = opcode & 0b1111_1110_0000_1111 == 0b1001_0010_0000_0111;
    if is_lat {
      return Box::new(Lat::new(opcode));
    }

    let is_ld = opcode & 0b1111_1110_0000_1111 == 0b1001_0000_0000_1100;
    if is_ld {
      return Box::new(Ld::new(opcode));
    }

    let is_ld_inc = opcode & 0b1111_1110_0000_1111 == 0b1001_0000_0000_1101;
    if is_ld_inc {
      return Box::new(LdInc::new(opcode));
    }

    let is_ld_dec = opcode & 0b1111_1110_0000_1111 == 0b1001_0000_0000_1110;
    if is_ld_dec {
      return Box::new(LdDec::new(opcode));
    }

    let is_ldd = opcode & 0b1111_1110_0000_1111 == 0b1000_0000_0000_1000;
    if is_ldd {
      return Box::new(Lddy::new(opcode));
    }

    let is_lddinc = opcode & 0b1111_1110_0000_1111 == 0b1001_0000_0000_1001;
    if is_lddinc {
      return Box::new(LddyInc::new(opcode));
    }

    let is_ldddec = opcode & 0b1111_1110_0000_1111 == 0b1001_0000_0000_1010;
    if is_ldddec {
      return Box::new(LddyDec::new(opcode));
    }

    let is_lddq = opcode & 0b1101_0010_0000_1000 == 0b1000_0000_0000_1000;
    if is_lddq {
      return Box::new(Lddyq::new(opcode));
    }

    let is_lddz = opcode & 0b1111_1110_0000_1111 == 0b1000_0000_0000_0000;
    if is_lddz {
      return Box::new(Lddz::new(opcode));
    }

    let is_lddzinc = opcode & 0b1111_1110_0000_1111 == 0b1000_0000_0000_0001;
    if is_lddzinc {
      return Box::new(LddzInc::new(opcode));
    }

    let is_lddzdec = opcode & 0b1111_1110_0000_1111 == 0b1000_0000_0000_0010;
    if is_lddzdec {
      return Box::new(LddzDec::new(opcode));
    }

    let is_lddzq = opcode & 0b1111_1110_0000_1111 == 0b1000_0000_0000_0000;
    if is_lddzq {
      return Box::new(Lddzq::new(opcode));
    }

    let is_ldi = opcode & 0b1111_0000_0000_0000 == 0b1110_0000_0000_0000;
    if is_ldi {
      return Box::new(Ldi::new(opcode));
    }

    let is_lds = opcode & 0b1111_1110_0000_1111 == 0b1001_0000_0000_0000;
    if is_lds {
      return Box::new(Lds::new(opcode, next_opcode));
    }

    let is_ldsavrc = opcode & 0b1111_1000_0000_0000 == 0b1010_0000_0000_0000;
    if is_ldsavrc {
      return Box::new(LdsAvrc::new(opcode));
    }

    let is_lpm = opcode == 0b1001_0101_1100_1000;
    if is_lpm {
      return Box::new(Lpm::new());
    }

    let is_lpm_rd = opcode & 0b1111_1110_0000_1111 == 0b1001_0000_0000_0100;
    if is_lpm_rd {
      return Box::new(LpmRd::new(opcode));
    }

    let is_lpm_rd_inc = opcode & 0b1111_1110_0000_1111 == 0b1001_0000_0000_0101;
    if is_lpm_rd_inc {
      return Box::new(LpmRdInc::new(opcode));
    }

    panic!("Unknown opcode: 0x{:04x}", opcode);
  }
}
