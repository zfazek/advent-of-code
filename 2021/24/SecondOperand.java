abstract class SecondOperand {
	protected int valueOrRegister;

	SecondOperand(int value) {
		this.valueOrRegister = value;
	}

	abstract long getValue(long[] reg);
}