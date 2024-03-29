import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main202120 {
	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Paths.get("2021/20/20.txt"));
		long start = System.currentTimeMillis();
		String algorithm = input.get(0);
		int T = 50;
		int border = T;
		int arrIdx = 0;
		int size = Math.max(input.size() - 2, input.get(2).length()) + 2 * border;
		char[][][] arr = new char[size][size][2];
		for (int i = 0; i < arr.length; i++) {
			for (int j = 0; j < arr[0].length; j++) {
				arr[i][j][arrIdx] = '.';
			}
		}

		for (int i = 2; i < input.size(); i++) {
			for (int j = 0; j < input.get(i).length(); j++) {
				arr[i - 2 + border][j + border][arrIdx] = input.get(i).charAt(j);
			}
		}
		for (int t = 0; t < T; t++) {
			for (int i = 0; i < arr.length; i++) {
				for (int j = 0; j < arr[0].length; j++) {
					StringBuilder idxStr = new StringBuilder();
					char c;
					for (int di = -1; di < 2; di++) {
						for (int dj = -1; dj < 2; dj++) {
							if (i + di < 0 || i + di >= arr.length || j + dj < 0 || j + dj >= arr[0].length) {
								c = arr[0][0][arrIdx];
							} else {
								c = arr[i + di][j + dj][arrIdx];
							}
							if (c == '.') {
								idxStr.append('0');
							} else {
								idxStr.append('1');
							}
						}
					}
					int idx = Integer.parseInt(idxStr.toString(), 2);
					arr[i][j][1 - arrIdx] = algorithm.charAt(idx);
				}
			}
			arrIdx = 1 - arrIdx;
		}
		long n = 0;
		for (int i = 0; i < arr.length; i++) {
			for (int j = 0; j < arr[0].length; j++) {
				if (arr[i][j][arrIdx] == '#') {
					n++;
				}
			}
		}
		long end = System.currentTimeMillis();
		System.out.format("%d ms\n", end - start);
		System.out.println(n);
	}

	@SuppressWarnings("unused")
	private static void print(char[][] arr) {
		for (int i = 0; i < arr.length; i++) {
			for (int j = 0; j < arr[0].length; j++) {
				System.out.print(arr[i][j]);
			}
			System.out.println();
		}
		System.out.println();
	}
}
