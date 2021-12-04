import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;


public class Main202104 {
	static Table table;
	static List<Table> tables = new ArrayList<>();

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2021/04/04.txt"));
		one(lines);
	}

	private static void one(List<String> lines) {
		List<String> numbers = new ArrayList<>(Arrays.asList(lines.get(0).split(",")));
		for (int i = 2; i < lines.size(); i++) {
			int y = (i - 2) % 6;
			if (y == 0) {
				table = new Table();
			}
			String[] tokens = lines.get(i).trim().split("\\s+");
			if (tokens.length == 5) {
				for (int x = 0; x < 5; x++) {
					table.m[y][x] = Integer.parseInt(tokens[x]);
				}
			} else {
				tables.add(table);
			}
		}
		tables.add(table);

		for (String number : numbers) {
			int n = Integer.parseInt(number);
			move(n);
		}
	}

	private static void move(int n) {
		Iterator<Table> it = tables.iterator();
		while (it.hasNext()) {
			Table table = it.next();
			table.move(n);
			if (table.isWon()) {
				System.out.println(n * table.getScore());
				it.remove();
			}
		}
	}
}

class Table {

	int[][] m = new int[5][5];

	void move(int n) {
		for (int y = 0; y < 5; y++) {
			for (int x = 0; x < 5; x++) {
				if (m[y][x] == n) {
					m[y][x] = -1;
				}
			}
		}
	}

	boolean isWon() {
		for (int y = 0; y < 5; y++) {
			boolean allPicked = true;
			for (int x = 0; x < 5; x++) {
				if (m[y][x] != -1) {
					allPicked = false;
				}
			}
			if (allPicked) {
				return true;
			}
		}
		for (int y = 0; y < 5; y++) {
			boolean allPicked = true;
			for (int x = 0; x < 5; x++) {
				if (m[x][y] != -1) {
					allPicked = false;
				}
			}
			if (allPicked) {
				return true;
			}
		}
		return false;
	}

	int getScore() {
		int score = 0;
		for (int y = 0; y < 5; y++) {
			for (int x = 0; x < 5; x++) {
				if (m[y][x] != -1) {
					score += m[y][x];
				}
			}
		}
		return score;
	}

	@Override
	public String toString() {
		String ret = "";
		for (int y = 0; y < 5; y++) {
			for (int x = 0; x < 5; x++) {
				ret += String.format("%3d", m[y][x]);
			}
			ret += ",";
		}
		return ret;
	}
}