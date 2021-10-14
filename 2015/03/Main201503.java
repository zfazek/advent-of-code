
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Main201503 {

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2015/03.txt"));
		Set<House> houses = new HashSet<>();
		for (String line : lines) {
			houses.clear();
			int x = 0;
			int y = 0;
			houses.add(new House(x, y));
			for (int i = 0; i < line.length(); i++) {
				char c = line.charAt(i);
				switch (c) {
				case '^':
					y++;
					houses.add(new House(x, y));
					break;
				case 'v':
					y--;
					houses.add(new House(x, y));
					break;
				case '>':
					x++;
					houses.add(new House(x, y));
					break;
				case '<':
					x--;
					houses.add(new House(x, y));
					break;
				}
			}
			System.out.println(houses.size());
		}		
		for (String line : lines) {
			houses.clear();
			int[] xs = new int[2];
			int[] ys = new int[2];
			houses.add(new House(xs[0], ys[0]));
			houses.add(new House(xs[1], ys[1]));
			for (int i = 0; i < line.length(); i++) {
				char c = line.charAt(i);
				switch (c) {
				case '^':
					ys[i % 2]++;
					houses.add(new House(xs[i % 2], ys[i % 2]));
					break;
				case 'v':
					ys[i % 2]--;
					houses.add(new House(xs[i % 2], ys[i % 2]));
					break;
				case '>':
					xs[i % 2]++;
					houses.add(new House(xs[i % 2], ys[i % 2]));
					break;
				case '<':
					xs[i % 2]--;
					houses.add(new House(xs[i % 2], ys[i % 2]));
					break;
				}
			}
			System.out.println(houses.size());
		}
	}
}

class House {
	House(int x, int y) {
		this.x = x;
		this.y = y;
	}

	@Override
	public int hashCode() {
		return 10000 * x + y;
	}

	@Override
	public boolean equals(Object o) {
		if (o == this) return true;
		if (!(o instanceof House)) {
			return false;
		}

		House house = (House)o;
		return this.x == house.x && this.y == house.y;
	}

	int x;
	int y;
}