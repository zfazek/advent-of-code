import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main201606 {

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2016/06/06.txt"));
		one(lines);
		two(lines);
	}

	private static void one(List<String> lines) {
		if (lines.isEmpty()) {
			return;
		}
		int lineLength = lines.get(0).length();
		int[][] arr = new int[lineLength][26];
		for (String line : lines) {
			for (int i = 0; i < line.length(); i++) {
				arr[i][line.charAt(i) - 'a']++;
			}
		}
		for (int i = 0; i < lineLength; i++) {
			int max = Integer.MIN_VALUE;
			char c = ' ';
			for (int j = 0; j < 26; j++) {
				if (arr[i][j] > max) {
					c = (char) ('a' + j);
					max = arr[i][j];
				}
			}
			System.out.print(c);
		}
		System.out.println();
	}

	private static void two(List<String> lines) {
		if (lines.isEmpty()) {
			return;
		}
		int lineLength = lines.get(0).length();
		int[][] arr = new int[lineLength][26];
		for (String line : lines) {
			for (int i = 0; i < line.length(); i++) {
				arr[i][line.charAt(i) - 'a']++;
			}
		}
		for (int i = 0; i < lineLength; i++) {
			int min = Integer.MAX_VALUE;
			char c = ' ';
			for (int j = 0; j < 26; j++) {
				if (arr[i][j] > 0 && arr[i][j] < min) {
					c = (char) ('a' + j);
					min = arr[i][j];
				}
			}
			System.out.print(c);
		}
		System.out.println();
	}


}

