use crate::avr::core_type::CoreType;
use crate::avr::instruction::Instruction;
use crate::avr::instructions::adc::Adc;
use crate::avr::instructions::add::Add;
use crate::avr::instructions::adiw::Adiw;
use crate::avr::instructions::and::And;
use crate::avr::instructions::andi::Andi;
use crate::avr::instructions::asr::Asr;
use crate::avr::instructions::bclr::Bclr;
use crate::avr::instructions::bld::Bld;
use crate::avr::instructions::brbc::Brbc;
use crate::avr::instructions::brbs::Brbs;
use crate::avr::instructions::bset::Bset;
use crate::avr::instructions::bst::Bst;
use crate::avr::instructions::call16::Call16;
use crate::avr::instructions::call22::Call22;
use crate::avr::instructions::cbi::Cbi;
use crate::avr::instructions::com::Com;
use crate::avr::instructions::cp::Cp;
use crate::avr::instructions::cpc::Cpc;
use crate::avr::instructions::cpi::Cpi;
use crate::avr::instructions::cpse::Cpse;
use crate::avr::instructions::dec::Dec;
use crate::avr::instructions::eor::Eor;
use crate::avr::instructions::fmul::Fmul;
use crate::avr::instructions::fmuls::Fmuls;
use crate::avr::instructions::fmulsu::Fmulsu;
use crate::avr::instructions::icall16::Icall16;
use crate::avr::instructions::icall22::Icall22;
use crate::avr::instructions::ijmp::Ijmp;
use crate::avr::instructions::in_io::In;
use crate::avr::instructions::inc::Inc;
use crate::avr::instructions::jmp::Jmp;
use crate::avr::instructions::lac::Lac;
use crate::avr::instructions::las::Las;
use crate::avr::instructions::lat::Lat;
use crate::avr::instructions::ld::Ld;
use crate::avr::instructions::ld_dec::LdDec;
use crate::avr::instructions::ld_inc::LdInc;
use crate::avr::instructions::lddy::Lddy;
use crate::avr::instructions::lddy_dec::LddyDec;
use crate::avr::instructions::lddy_inc::LddyInc;
use crate::avr::instructions::lddyq::Lddyq;
use crate::avr::instructions::lddz::Lddz;
use crate::avr::instructions::lddz_dec::LddzDec;
use crate::avr::instructions::lddz_inc::LddzInc;
use crate::avr::instructions::lddzq::Lddzq;
use crate::avr::instructions::ldi::Ldi;
use crate::avr::instructions::lds::Lds;
use crate::avr::instructions::lds_avrc::LdsAvrc;
use crate::avr::instructions::lpm::Lpm;
use crate::avr::instructions::lpm_rd::LpmRd;
use crate::avr::instructions::lpm_rd_inc::LpmRdInc;
use crate::avr::instructions::lsr::Lsr;
use crate::avr::instructions::mov::Mov;
use crate::avr::instructions::movw::Movw;
use crate::avr::instructions::mul::Mul;
use crate::avr::instructions::muls::Muls;
use crate::avr::instructions::mulsu::Mulsu;
use crate::avr::instructions::neg::Neg;
use crate::avr::instructions::nop::Nop;
use crate::avr::instructions::or::Or;
use crate::avr::instructions::ori::Ori;
use crate::avr::instructions::out_io::Out;
use crate::avr::instructions::pop::Pop;
use crate::avr::instructions::push::Push;
use crate::avr::instructions::rcall16::Rcall16;
use crate::avr::instructions::rcall22::Rcall22;
use crate::avr::instructions::ret16::Ret16;
use crate::avr::instructions::ret22::Ret22;
use crate::avr::instructions::reti16::Reti16;
use crate::avr::instructions::reti22::Reti22;
use crate::avr::instructions::rjmp::Rjmp;
use crate::avr::instructions::ror::Ror;
use crate::avr::instructions::sbc::Sbc;
use crate::avr::instructions::sbci::Sbci;
use crate::avr::instructions::sbi::Sbi;
use crate::avr::instructions::sbic::Sbic;
use crate::avr::instructions::sbis::Sbis;
use crate::avr::instructions::sbiw::Sbiw;
use crate::avr::instructions::sbr::Sbr;
use crate::avr::instructions::sbrc::Sbrc;
use crate::avr::instructions::sbrs::Sbrs;
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
      return match core_type {
        CoreType::Bits16 => Box::new(Call16::new(opcode, next_opcode)),
        CoreType::Bits22 => Box::new(Call22::new(opcode, next_opcode)),
      };
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
      return match core_type {
        CoreType::Bits16 => Box::new(Icall16::new()),
        CoreType::Bits22 => Box::new(Icall22::new()),
      };
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

    let is_lsr = opcode & 0b1111_1110_0000_1111 == 0b1001_0100_0000_0110;
    if is_lsr {
      return Box::new(Lsr::new(opcode));
    }

    let is_mov = opcode & 0b1111_1100_0000_0000 == 0b0010_1100_0000_0000;
    if is_mov {
      return Box::new(Mov::new(opcode));
    }

    let is_movw = opcode & 0b1111_1111_0000_0000 == 0b0000_0001_0000_0000;
    if is_movw {
      return Box::new(Movw::new(opcode));
    }

    let is_mul = opcode & 0b1111_1100_0000_0000 == 0b1001_1100_0000_0000;
    if is_mul {
      return Box::new(Mul::new(opcode));
    }

    let is_muls = opcode & 0b1111_1111_0000_0000 == 0b0000_0010_0000_0000;
    if is_muls {
      return Box::new(Muls::new(opcode));
    }

    let is_mulsu = opcode & 0b1111_1111_1000_1000 == 0b0000_0011_0000_0000;
    if is_mulsu {
      return Box::new(Mulsu::new(opcode));
    }

    let is_neg = opcode & 0b1111_1110_0000_1111 == 0b1001_0100_0000_0001;
    if is_neg {
      return Box::new(Neg::new(opcode));
    }

    let is_nop = opcode == 0x00;
    if is_nop {
      return Box::new(Nop::new());
    }

    let is_or = opcode & 0b1111_1100_0000_0000 == 0b0010_1000_0000_0000;
    if is_or {
      return Box::new(Or::new(opcode));
    }

    let is_ori = opcode & 0b1111_0000_0000_0000 == 0b0110_0000_0000_0000;
    if is_ori {
      return Box::new(Ori::new(opcode));
    }

    let is_out = opcode & 0b1111_1000_0000_0000 == 0b1011_1000_0000_0000;
    if is_out {
      return Box::new(Out::new(opcode));
    }

    let is_pop = opcode & 0b1111_1110_0000_1111 == 0b1001_0000_0000_1111;
    if is_pop {
      return Box::new(Pop::new(opcode));
    }

    let is_push = opcode & 0b1111_1110_0000_1111 == 0b1001_0010_0000_1111;
    if is_push {
      return Box::new(Push::new(opcode));
    }

    let is_rcall = opcode & 0b1111_0000_0000_0000 == 0b1101_0000_0000_0000;
    if is_rcall {
      return match core_type {
        CoreType::Bits16 => Box::new(Rcall16::new(opcode)),
        CoreType::Bits22 => Box::new(Rcall22::new(opcode)),
      };
    }

    let is_ret = opcode == 0x9508;
    if is_ret {
      return match core_type {
        CoreType::Bits16 => Box::new(Ret16::new()),
        CoreType::Bits22 => Box::new(Ret22::new()),
      };
    }

    let is_reti = opcode == 0x9518;
    if is_reti {
      return match core_type {
        CoreType::Bits16 => Box::new(Reti16::new()),
        CoreType::Bits22 => Box::new(Reti22::new()),
      };
    }

    let is_rjmp = opcode & 0b1111_0000_0000_0000 == 0b1100_0000_0000_0000;
    if is_rjmp {
      return Box::new(Rjmp::new(opcode));
    }

    let is_ror = opcode & 0b1111_1110_0000_1111 == 0b1001_0100_0000_0111;
    if is_ror {
      return Box::new(Ror::new(opcode));
    }

    let is_sbc = opcode & 0b1111_1100_0000_0000 == 0b0000_1000_0000_0000;
    if is_sbc {
      return Box::new(Sbc::new(opcode));
    }

    let is_sbci = opcode & 0b1111_0000_0000_0000 == 0b0100_0000_0000_0000;
    if is_sbci {
      return Box::new(Sbci::new(opcode));
    }

    let is_sbi = opcode & 0b1111_1111_0000_0000 == 0b1001_1010_0000_0000;
    if is_sbi {
      return Box::new(Sbi::new(opcode));
    }

    let is_sbic = opcode & 0b1111_1111_0000_0000 == 0b1001_1001_0000_0000;
    if is_sbic {
      return Box::new(Sbic::new(opcode, next_opcode, &self.opcode));
    }

    let is_sbis = opcode & 0b1111_1111_0000_0000 == 0b1001_1011_0000_0000;
    if is_sbis {
      return Box::new(Sbis::new(opcode, next_opcode, &self.opcode));
    }

    let is_sbiw = opcode & 0b1111_1111_0000_0000 == 0b1001_0111_0000_0000;
    if is_sbiw {
      return Box::new(Sbiw::new(opcode));
    }

    let is_sbr = opcode & 0b1111_0000_0000_0000 == 0b0110_0000_0000_0000;
    if is_sbr {
      return Box::new(Sbr::new(opcode));
    }

    let is_sbrc = opcode & 0b1111_1110_0000_1000 == 0b1111_1100_0000_1000;
    if is_sbrc {
      return Box::new(Sbrc::new(opcode, next_opcode, &self.opcode));
    }

    let is_sbrs = opcode & 0b1111_1110_0000_1000 == 0b1111_1110_0000_0000;
    if is_sbrs {
      return Box::new(Sbrs::new(opcode, next_opcode, &self.opcode));
    }

    panic!("Unknown opcode: 0x{:04x}", opcode);
  }
}
