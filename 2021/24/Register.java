class Register extends SecondOperand {

	Register(int opB) {
		super(opB);
	}

	@Override
	long getValue(long[] reg) {
		return reg[valueOrRegister];
	}
}