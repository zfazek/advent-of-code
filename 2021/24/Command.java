abstract class Command {
	protected int registerIdx;
	protected SecondOperand secondOperand;

	Command(String[] tokens) {
		if (tokens.length == 3) {
			if (new String("wxyz").indexOf(tokens[2]) != -1) {
				secondOperand = new Register(tokens[2].charAt(0) - 'w');
			} else {
				secondOperand = new Value(Integer.parseInt(tokens[2]));
			}
		}
		registerIdx = tokens[1].charAt(0) - 'w';
	}

	abstract void exec(long[] reg);
}