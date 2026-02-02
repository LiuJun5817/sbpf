c_asm::ADD32_IMM  if !self.executable.get_sbpf_version().move_memory_instruction_classes() => {
    self.emit_address_translation(None, Value::RegisterPlusConstant64(dst, insn.off as i64, true), 4, Some(Value::Constant64(insn.imm, true)));
},
ebpf::ST_DW_IMM if !self.executable.get_sbpf_version().move_memory_instruction_classes() => {
    self.emit_address_translation(None, Value::RegisterPlusConstant64(dst, insn.off as i64, true), 8, Some(Value::Constant64(insn.imm, true)));
},

// 像在isabelle中定义systhx里的内容一样