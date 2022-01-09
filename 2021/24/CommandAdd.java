class CommandAdd extends Command {

	CommandAdd(String[] tokens) {
		super(tokens);
	}

	@Override
	void exec(long[] reg) {
		reg[registerIdx] += secondOperand.getValue(reg);
	}
}