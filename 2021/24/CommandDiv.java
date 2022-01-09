class CommandDiv extends Command {

	CommandDiv(String[] tokens) {
		super(tokens);
	}

	@Override
	void exec(long[] reg) {
		reg[registerIdx] /= secondOperand.getValue(reg);
	}
}