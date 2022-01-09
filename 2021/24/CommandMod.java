class CommandMod extends Command {

	CommandMod(String[] tokens) {
		super(tokens);
	}

	@Override
	void exec(long[] reg) {
		reg[registerIdx] %= secondOperand.getValue(reg);

	}
}