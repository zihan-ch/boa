use crate::{bytecompiler::ByteCompiler, vm::Opcode};

use boa_ast::statement::Block;

impl ByteCompiler<'_, '_> {
    /// Compile a [`Block`] `boa_ast` node
    pub(crate) fn compile_block(
        &mut self,
        block: &Block,
        use_expr: bool,
        configurable_globals: bool,
    ) {
        self.context.push_compile_time_environment(false);
        let push_env = self.emit_opcode_with_two_operands(Opcode::PushDeclarativeEnvironment);

        self.create_script_decls(block.statement_list(), configurable_globals);
        self.compile_statement_list(block.statement_list(), use_expr, configurable_globals);

        let (num_bindings, compile_environment) = self.context.pop_compile_time_environment();
        let index_compile_environment = self.push_compile_environment(compile_environment);
        self.patch_jump_with_target(push_env.0, num_bindings as u32);
        self.patch_jump_with_target(push_env.1, index_compile_environment as u32);

        self.emit_opcode(Opcode::PopEnvironment);
    }
}
