class Value extends SecondOperand {

	Value(int value) {
		super(value);
	}

	@Override
	long getValue(long[] reg) {
		return valueOrRegister;
	}
}