class CommandMul extends Command {

	CommandMul(String[] tokens) {
		super(tokens);
	}

	@Override
	void exec(long[] reg) {
		reg[registerIdx] *= secondOperand.getValue(reg);
	}
}