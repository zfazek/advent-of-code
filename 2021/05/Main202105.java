import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main202105 {

	static int N = 1000;
	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2021/05/05.txt"));
		one(lines);
		two(lines);
	}

	private static void one(List<String> lines) {
		int[][] arr = new int[N][N];
		for (String line : lines) {
			String[] tokens = line.split(" -> ");
			String[] first = tokens[0].split(",");
			String[] second = tokens[1].split(",");
			int x1 = Integer.parseInt(first[0]);
			int y1 = Integer.parseInt(first[1]);
			int x2 = Integer.parseInt(second[0]);
			int y2 = Integer.parseInt(second[1]);
			if (x1 == x2) {
				for (int i = Math.min(y1, y2); i <= Math.max(y1, y2); i++) {
					arr[x1][i]++;
				}
			} else if (y1 == y2) {
				for (int i = Math.min(x1, x2); i <= Math.max(x1, x2); i++) {
					arr[i][y1]++;
				}
			}
		}
		long n = 0;
		for (int x = 0; x < N; x++) {
			for (int y = 0; y < N; y++) {
				if (arr[x][y] >= 2) {
					n++;
				}
			}
		}
		System.out.println(n);
	}

	private static void two(List<String> lines) {
		int[][] arr = new int[N][N];
		for (String line : lines) {
			String[] tokens = line.split(" -> ");
			String[] first = tokens[0].split(",");
			String[] second = tokens[1].split(",");
			int x1 = Integer.parseInt(first[0]);
			int y1 = Integer.parseInt(first[1]);
			int x2 = Integer.parseInt(second[0]);
			int y2 = Integer.parseInt(second[1]);
			if (x1 == x2) {
				for (int i = Math.min(y1, y2); i <= Math.max(y1, y2); i++) {
					arr[x1][i]++;
				}
			} else if (y1 == y2) {
				for (int i = Math.min(x1, x2); i <= Math.max(x1, x2); i++) {
					arr[i][y1]++;
				}
			} else if (Math.abs(x2 - x1) == Math.abs(y2 - y1)) {
				if (x1 < x2) {
					for (int x = x1; x <= x2; x++) {
						if (y1 < y2) {
							arr[x][y1 + x - x1]++;
						} else {
							arr[x][y1 - x + x1]++;
						}
					}
				} else {
					for (int x = x2; x <= x1; x++) {
						if (y1 < y2) {
							arr[x][y2 - x + x2]++;
						} else {
							arr[x][y2 + x - x2]++;
						}
					}
				}
			}
		}
		long n = 0;
		for (int x = 0; x < N; x++) {
			for (int y = 0; y < N; y++) {
				if (arr[x][y] >= 2) {
					n++;
				}
			}
		}
		System.out.println(n);
	}
}
