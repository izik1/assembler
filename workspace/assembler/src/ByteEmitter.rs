// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ByteEmitter
{
	instruction_pointer: usize,
	end_instruction_pointer: usize,
}

impl ByteEmitter
{
	#[inline(always)]
	pub(crate) fn new(executable_anonymous_memory_map: & mut ExecutableAnonymousMemoryMap) -> Self
	{
		let instruction_pointer = executable_anonymous_memory_map.address as usize;
		let length = executable_anonymous_memory_map.length;
		
		Self
		{
			instruction_pointer,
			end_instruction_pointer: instruction_pointer + length,
		}
	}
	
	#[inline(always)]
	pub(crate) fn emit_bytes(&mut self, bytes: &[u8])
	{
		let length = bytes.len();
		
		if length == 0
		{
			return
		}
		else if length == 1
		{
			self.emit_u8(unsafe { *bytes.get_unchecked(0) });
			return
		}
		
		debug_assert!(self.instruction_pointer + length <= self.end_instruction_pointer, "Not enough space to emit '{}' byte(s)", length);
		unsafe
		{
			copy_nonoverlapping(bytes.as_ptr(), self.instruction_pointer as *mut u8, length);
			self.instruction_pointer += length;
		}
	}
	
	#[inline(always)]
	pub(crate) fn emit_prefix_group2_for_segment_register(&mut self, segment_register: SegmentRegister)
	{
		use self::SegmentRegister::*;
		
		let segment_register_byte = match segment_register
		{
			ES => 0x26,
			CS => 0x2E,
			SS => 0x36,
			DS => 0x3E,
			FS => 0x64,
			GS => 0x65,
		};
		self.emit_u8(segment_register_byte)
	}
	
	#[inline(always)]
	pub(crate) fn emit_2_byte_vex_prefix(&mut self, r_bit: u8, vvvv: impl Register, l: u8, pp: u8)
	{
		self.emit_u8(0xC5);
		self.emit_u8((r_bit | ((!vvvv.index()) << 3) & 0x78) | (l << 2) | pp);
	}
	
	#[inline(always)]
	pub(crate) fn emit_3_byte_vex_prefix(&mut self, r_bit: u8, x_bit: u8, b_bit: u8, mmmmm: u8, w: u8, vvvv: impl Register, l: u8, pp: u8)
	{
		self.emit_u8(0xC5);
		self.emit_u8(r_bit | x_bit | b_bit | mmmmm);
		self.emit_u8((w << 7) | ((!vvvv.index() << 3) & 0x78) | (l << 2) | pp);
	}
	
	#[inline(always)]
	pub(crate) fn emit_u8_if_not_zero(&mut self, byte: u8)
	{
		if byte != 0x00
		{
			self.emit_u8(byte)
		}
	}
	
	#[inline(always)]
	pub(crate) fn emit_u8(&mut self, emit: u8)
	{
		const Size: usize = 1;
		debug_assert!(self.instruction_pointer + Size <= self.end_instruction_pointer, "Not enough space to emit an u8");
		unsafe { *(self.instruction_pointer as *mut u8) = emit };
		self.instruction_pointer += Size;
	}
	
	#[inline(always)]
	pub(crate) fn emit_u16(&mut self, emit: u16)
	{
		const Size: usize = 2;
		debug_assert!(self.instruction_pointer + Size <= self.end_instruction_pointer, "Not enough space to emit an u16");
		unsafe { *(self.instruction_pointer as *mut u16) = emit.to_le() };
		self.instruction_pointer += Size;
	}
	
	#[inline(always)]
	pub(crate) fn emit_u32(&mut self, emit: u32)
	{
		const Size: usize = 4;
		debug_assert!(self.instruction_pointer + Size <= self.end_instruction_pointer, "Not enough space to emit an u32");
		unsafe { *(self.instruction_pointer as *mut u32) = emit.to_le() };
		self.instruction_pointer += Size;
	}
	
	#[inline(always)]
	pub(crate) fn emit_u64(&mut self, emit: u64)
	{
		const Size: usize = 8;
		debug_assert!(self.instruction_pointer + Size <= self.end_instruction_pointer, "Not enough space to emit an u64");
		unsafe { *(self.instruction_pointer as *mut u64) = emit.to_le() };
		self.instruction_pointer += Size;
	}
	
	#[inline(always)]
	pub(crate) fn emit_u128(&mut self, emit: u128)
	{
		const Size: usize = 16;
		debug_assert!(self.instruction_pointer + Size <= self.end_instruction_pointer, "Not enough space to emit an u128");
		unsafe { *(self.instruction_pointer as *mut u128) = emit.to_le() };
		self.instruction_pointer += Size;
	}
}
