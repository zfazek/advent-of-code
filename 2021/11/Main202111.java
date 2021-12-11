import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main202111 {
	static long flashes = 0;

	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Paths.get("2021/11/11.txt"));
		long start = System.currentTimeMillis();
		int N = input.size();
		int[][] arr = new int[N][N];
		for (int i = 0; i < input.size(); i++) {
			for (int j = 0; j < input.get(i).length(); j++) {
				arr[i][j] = Integer.parseInt(input.get(i).charAt(j) + "");
			}
		}
		int step = 1;
		while (true) {
			increaseByOne(arr, N);
			while (hasFlash(arr, N)) {
				loop: for (int i = 0; i < N; i++) {
					for (int j = 0; j < N; j++) {
						if (arr[i][j] > 9) {
							flash(arr, N, i, j);
							break loop;
						}
					}
				}
			}
			if (step == 100) {
				System.out.println(flashes);
			}
			if (isEnd(arr, N)) {
				break;
			}
			step++;
		}
		System.out.println(step);
		long end = System.currentTimeMillis();
		System.out.format("%d ms\n", end - start);
	}

	private static boolean isEnd(int[][] arr, int N) {
		for (int i = 0; i < N; i++) {
			for (int j = 0; j < N; j++) {
				if (arr[i][j] != 0) {
					return false;
				}
			}
		}
		return true;
	}

	private static void flash(int[][] arr, int N, int y, int x) {
		flashes++;
		arr[y][x] = 0;
		for (int dy = -1; dy < 2; dy++) {
			for (int dx = -1; dx < 2; dx++) {
				if (dy == 0 && dx == 0) {
					continue;
				}
				int newX = x + dx;
				int newY = y + dy;
				if (newX >= 0 && newX < N && newY >= 0 && newY < N) {
					if (arr[newY][newX] > 0) {
						arr[newY][newX]++;
					}
				}
			}
		}
	}

	private static void increaseByOne(int[][] arr, int N) {
		for (int i = 0; i < N; i++) {
			for (int j = 0; j < N; j++) {
				arr[i][j] = (arr[i][j] + 1);
			}
		}
	}

	private static boolean hasFlash(int[][] arr, int N) {
		for (int i = 0; i < N; i++) {
			for (int j = 0; j < N; j++) {
				if (arr[i][j] > 9) {
					return true;
				}
			}
		}
		return false;
	}

	@SuppressWarnings("unused")
	private static void print(int[][] arr, int N) {
		for (int i = 0; i < N; i++) {
			for (int j = 0; j < N; j++) {
				System.out.format("%3d", arr[i][j]);
			}
			System.out.println();
		}
		System.out.println();
	}
}
