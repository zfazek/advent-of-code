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
	static List<Integer> results = new ArrayList<>();

	public static void main(String[] args) throws IOException {
		String file = Files.readString(Paths.get("2021/04/04a.txt"));
		List<String> boards = new ArrayList<>(Arrays.asList(file.split("\\r\\n\\r\\n")));
		List<String> numbers = Arrays.asList(boards.get(0).split(","));
		boards.remove(0);
		for (String board : boards) {
			String[] nums = board.trim().split("\\s+");
			Table table = new Table();
			for (int i = 0; i < nums.length; i++) {
				table.m[i / Table.SIZE][i % Table.SIZE] = Integer.parseInt(nums[i]);
			}
			tables.add(table);
		}

		for (String number : numbers) {
			int n = Integer.parseInt(number);
			draw(n);
		}
		if (!results.isEmpty()) {
			System.out.println(results.get(0));
			System.out.println(results.get(results.size() - 1));
		}
	}

	private static void draw(int n) {
		Iterator<Table> it = tables.iterator();
		while (it.hasNext()) {
			Table table = it.next();
			table.draw(n);
			if (table.isWon()) {
				results.add(n * table.getScore());
				it.remove();
			}
		}
	}
}

class Table {
	static final int SIZE = 5;

	int[][] m = new int[SIZE][SIZE];

	void draw(int n) {
		for (int y = 0; y < SIZE; y++) {
			for (int x = 0; x < SIZE; x++) {
				if (m[y][x] == n) {
					m[y][x] = -1;
				}
			}
		}
	}

	boolean isWon() {
		for (int y = 0; y < SIZE; y++) {
			boolean allPicked = true;
			for (int x = 0; x < SIZE; x++) {
				if (m[y][x] != -1) {
					allPicked = false;
				}
			}
			if (allPicked) {
				return true;
			}
		}
		for (int y = 0; y < SIZE; y++) {
			boolean allPicked = true;
			for (int x = 0; x < SIZE; x++) {
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
		for (int y = 0; y < SIZE; y++) {
			for (int x = 0; x < SIZE; x++) {
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
		for (int y = 0; y < SIZE; y++) {
			for (int x = 0; x < SIZE; x++) {
				ret += String.format("%3d", m[y][x]);
			}
			ret += "\n";
		}
		return ret;
	}
}