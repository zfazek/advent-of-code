class CommandEql extends Command {

	CommandEql(String[] tokens) {
		super(tokens);
	}

	@Override
	void exec(long[] reg) {
		reg[registerIdx] = reg[registerIdx] == secondOperand.getValue(reg) ? 1 : 0;
	}
}