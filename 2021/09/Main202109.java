import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Main202109 {

	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Paths.get("2021/09/09.txt"));
		one(input);
		two(input);
	}

	private static void one(List<String> input) {
		long sum = 0;
		int[][] arr = new int[input.size() + 2][input.get(0).length() + 2];
		for (int i = 0; i < input.size() + 2; i++) {
			for (int j = 0; j < input.get(0).length() + 2; j++) {
				arr[i][j] = 9;
			}
		}
		for (int i = 0; i < input.size(); i++) {
			String line = input.get(i);
			for (int j = 0; j < line.length(); j++) {
				int c = line.charAt(j);
				arr[i + 1][j + 1] = c - '0';
			}
		}
		for (int i = 0; i < input.size(); i++) {
			for (int j = 0; j < input.get(i).length(); j++) {
				int v = arr[i + 1][j + 1];
				int v1 = arr[i][j + 1];
				int v2 = arr[i + 2][j + 1];
				int v3 = arr[i + 1][j];
				int v4 = arr[i + 1][j + 2];
				if (v < v1 && v < v2 && v < v3 && v < v4) {
					sum += v + 1;
				}
			}
		}
		System.out.println(sum);
	}

	private static void two(List<String> input) {
		List<Integer> basins = new ArrayList<>();
		int[][] arr = new int[input.size() + 2][input.get(0).length() + 2];
		for (int i = 0; i < input.size() + 2; i++) {
			for (int j = 0; j < input.get(0).length() + 2; j++) {
				arr[i][j] = 9;
			}
		}
		for (int i = 0; i < input.size(); i++) {
			String line = input.get(i);
			for (int j = 0; j < line.length(); j++) {
				int c = line.charAt(j);
				arr[i + 1][j + 1] = c - '0';
			}
		}
		//        printArr(input, arr);
		for (int i = 0; i < input.size() + 2; i++) {
			for (int j = 0; j < input.get(0).length() + 2; j++) {
				int c = arr[i][j];
				if (c < 9) {
					int s = foo(arr, i, j);
					basins.add(s);
				}
			}
		}
		Collections.sort(basins);
		int idx = basins.size() - 2;
		System.out.println(basins.get(idx - 1) * basins.get(idx) * basins.get(idx + 1));
	}

	private static void printArr(List<String> input, int[][] arr) {
		for (int i = 0; i < input.size() + 2; i++) {
			for (int j = 0; j < input.get(0).length() + 2; j++) {
				System.out.format("%d ", arr[i][j]);
			}
			System.out.println();
		}
	}

	private static int foo(int[][] arr, int i, int j) {
		if (arr[i][j] == 9) {
			return 0;
		}
		arr[i][j] = 9;
		return 1 + foo(arr, i - 1, j) + foo(arr, i + 1, j) + foo(arr, i, j - 1) + foo(arr, i, j + 1);
	}
}
