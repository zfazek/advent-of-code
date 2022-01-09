import java.util.Objects;

class Registers {
	private int i;
	private long w, x, y, z;

	@Override
	public int hashCode() {
		return Objects.hash(i, w, x, y, z);
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if (obj == null)
			return false;
		if (getClass() != obj.getClass())
			return false;
		Registers other = (Registers)obj;
		return i == other.i && w == other.w && x == other.x && y == other.y && z == other.z;
	}

	Registers(int i, long w, long x, long y, long z) {
		this.i = i;
		this.w = w;
		this.x = x;
		this.y = y;
		this.z = z;
	}

	@Override
	public String toString() {
		return String.format("i: %d, [%d, %d, %d, %d]", i, w, x, y, z);
	}
}