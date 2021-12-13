import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Main202113 {
	static int sizeX, sizeY;

	public static void main(String[] args) throws IOException {
		String input = Files.readString(Paths.get("2021/13/13.txt"));
		long start = System.currentTimeMillis();
		String[] parts = input.split("\\r\\n\\r\\n");
		String[] dots = parts[0].split("\\r\\n");
		String[] cmds = parts[1].split("\\r\\n");
		sizeX = Integer.MIN_VALUE;
		sizeY = Integer.MIN_VALUE;
		for (String dot : dots) {
			String[] line = dot.split(",");
			int x = Integer.parseInt(line[0]);
			int y = Integer.parseInt(line[1]);
			sizeX = Math.max(x, sizeX);
			sizeY = Math.max(y, sizeY);
		}

		sizeX++;
		sizeY++;
		boolean[][] arr = new boolean[sizeY][sizeX];
		for (String dot : dots) {
			String[] line = dot.split(",");
			int x = Integer.parseInt(line[0]);
			int y = Integer.parseInt(line[1]);
			arr[y][x] = true;
		}
		for (String line : cmds) {
			String[] tokens = line.split(" ");
			String[] cmd = tokens[2].split("=");
			String dir = cmd[0];
			int idx = Integer.parseInt(cmd[1]);
			fold(arr, dir, idx);
			System.out.println(getNumberOfDots(arr));
		}
		long end = System.currentTimeMillis();
		print(arr);
		System.out.println(end - start + " ms");
	}

	private static int getNumberOfDots(boolean[][] arr) {
		int sum = 0;
		for (int i = 0; i < sizeY; i++) {
			for (int j = 0; j < sizeX; j++) {
				if (arr[i][j]) {
					sum++;
				}
			}
		}
		return sum;
	}

	private static void fold(boolean[][] arr, String dir, int idx) {
		if (dir.equals("x")) {
			foldX(arr, idx);
		} else {
			foldY(arr, idx);
		}
	}

	private static void foldY(boolean[][] arr, int idx) {
		for (int i = 0; i <= idx; i++) {
			for (int j = 0; j < sizeX; j++) {
				if (i == idx) {
					arr[i][j] = false;
				} else {
					int from = 2 * idx - i;
					if (from < sizeY) {
						arr[i][j] |= arr[from][j];
						arr[from][j] = false;
					}
				}
			}
		}
		sizeY = idx;
	}

	private static void foldX(boolean[][] arr, int idx) {
		for (int i = 0; i < sizeY; i++) {
			for (int j = 0; j <= idx; j++) {
				if (j == idx) {
					arr[i][j] = false;
				} else {
					int from = 2 * idx - j;
					if (from < sizeX) {
						arr[i][j] |= arr[i][from];
						arr[i][from] = false;
					}
				}
			}
		}
		sizeX = idx;
	}

	private static void print(boolean[][] arr) {
		for (int i = 0; i < sizeY; i++) {
			for (int j = 0; j < sizeX; j++) {
				char c = arr[i][j] ? '#' : '.';
				System.out.print(c);
			}
			System.out.println();
		}
		System.out.println();
	}
}
