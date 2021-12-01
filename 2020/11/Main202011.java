import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class Main202011 {
	static final char FLOOR = '.';
	static final char EMPTY = 'L';
	static final char OCCUPIED = '#';

	static List<String> lines;
	static List<String> lines1 = new ArrayList<>();

	public static void main(String[] args) throws IOException {
		lines = Files.readAllLines(Paths.get("2020/11/11.txt"));
		// print();
		while (move(lines, lines1)) {
			// print();
		}
		int n = getNumberOfOccupied();
		System.out.println(n);
		// print();
		while (move2(lines, lines1)) {
			// print();
		}
		n = getNumberOfOccupied();
		System.out.println(n);
	}

	private static int getNumberOfOccupied() {
		int acc = 0;
		for (String line : lines) {
			for (int i = 0; i < line.length(); i++) {
				if (line.charAt(i) == OCCUPIED) {
					acc++;
				}
			}
		}
		return acc;
	}

	@SuppressWarnings("unused")
	private static void print() {
		for (String line : lines) {
			System.out.println(line);
		}
		System.out.println();
	}

	@SuppressWarnings("unused")
	private static boolean move(List<String> lines, List<String> lines1) {
		lines1.clear();
		for (int i = 0; i < lines.size(); i++) {
			String line = "";
			for (int j = 0; j < lines.get(0).length(); j++) {
				final char c = lines.get(i).charAt(j);
				if (c == FLOOR) {
					line += FLOOR;
					continue;
				}
				if (c == EMPTY) {
					final int n = getNumberOfNeighbors(i, j);
					if (n == 0) {
						line += OCCUPIED;
					} else {
						line += EMPTY;
					}
					continue;
				}
				final int n = getNumberOfNeighbors(i, j);
				if (n >= 4) {
					line += EMPTY;
				} else {
					line += OCCUPIED;
				}
			}
			lines1.add(line);
		}
		boolean differed = false;
		for (int i = 0; i < lines.size(); i++) {
			if (!lines.get(i).equals(lines1.get(i))) {
				differed = true;
				break;
			}
		}
		lines.clear();
		for (String line : lines1) {
			lines.add(line);
		}
		return differed;
	}

	private static boolean move2(List<String> lines, List<String> lines1) {
		lines1.clear();
		for (int i = 0; i < lines.size(); i++) {
			String line = "";
			for (int j = 0; j < lines.get(0).length(); j++) {
				final char c = lines.get(i).charAt(j);
				if (c == FLOOR) {
					line += FLOOR;
					continue;
				}
				if (c == EMPTY) {
					final int n = getNumberOfNeighbors2(i, j);
					if (n == 0) {
						line += OCCUPIED;
					} else {
						line += EMPTY;
					}
					continue;
				}
				final int n = getNumberOfNeighbors2(i, j);
				if (n >= 5) {
					line += EMPTY;
				} else {
					line += OCCUPIED;
				}
			}
			lines1.add(line);
		}
		boolean differed = false;
		for (int i = 0; i < lines.size(); i++) {
			if (!lines.get(i).equals(lines1.get(i))) {
				differed = true;
				break;
			}
		}
		lines.clear();
		for (String line : lines1) {
			lines.add(line);
		}
		return differed;
	}

	private static int getNumberOfNeighbors2(int i, int j) {
		final int columns = lines.get(0).length();
		final int rows = lines.size();
		int acc = 0;
		for (int y = -1; y < 2; y++) {
			for (int x = -1; x < 2; x++) {
				if (x == 0 && y == 0) {
					continue;
				}
				int posX;
				int posY;
				int k = 0;
				do {
					k++;
					posX = j + k * x;
					posY = i + k * y;
				} while (posX >= 0 && posX < columns && posY >= 0 && posY < rows
						&& lines.get(posY).charAt(posX) == FLOOR);
				if (posX >= 0 && posX < columns && posY >= 0 && posY < rows
						&& lines.get(posY).charAt(posX) == OCCUPIED) {
					acc++;
				}
			}
		}
		return acc;
	}

	private static int getNumberOfNeighbors(int i, int j) {
		final int columns = lines.get(0).length();
		final int rows = lines.size();
		int acc = 0;
		if (i > 0 && j > 0 && lines.get(i - 1).charAt(j - 1) == OCCUPIED)
			acc++;
		if (i > 0 && lines.get(i - 1).charAt(j) == OCCUPIED)
			acc++;
		if (i > 0 && j < columns - 1 && lines.get(i - 1).charAt(j + 1) == OCCUPIED)
			acc++;
		if (j > 0 && lines.get(i).charAt(j - 1) == OCCUPIED)
			acc++;
		if (j < columns - 1 && lines.get(i).charAt(j + 1) == OCCUPIED)
			acc++;
		if (i < rows - 1 && j > 0 && lines.get(i + 1).charAt(j - 1) == OCCUPIED)
			acc++;
		if (i < rows - 1 && lines.get(i + 1).charAt(j) == OCCUPIED)
			acc++;
		if (i < rows - 1 && j < columns - 1 && lines.get(i + 1).charAt(j + 1) == OCCUPIED)
			acc++;
		return acc;
	}

}
