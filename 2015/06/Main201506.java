
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main201506 {

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2015/06.txt"));
		final int N = 1000;
		a(lines, N);
		b(lines, N);

	}

	private static void a(List<String> lines, final int N) {
		int[][] t = new int[N][N];
		for (String line : lines) {
			//			System.out.println(line);
			String[] tokens = line.split(" ");
			if (tokens[0].equals("toggle")) {
				String[] from = tokens[1].split(",");
				String[] to = tokens[3].split(",");
				int fromX = Integer.parseInt(from[0]);
				int fromY = Integer.parseInt(from[1]);
				int toX = Integer.parseInt(to[0]);
				int toY = Integer.parseInt(to[1]);
				for (int i = fromX; i <= toX; i++) {
					for (int j = fromY; j <= toY; j++) {
						t[i][j] = 1 - t[i][j];
					}
				}
			} else {
				String[] from = tokens[2].split(",");
				String[] to = tokens[4].split(",");
				int fromX = Integer.parseInt(from[0]);
				int fromY = Integer.parseInt(from[1]);
				int toX = Integer.parseInt(to[0]);
				int toY = Integer.parseInt(to[1]);
				int value = 1;
				if (tokens[1].equals("off")) {
					value = 0;
				}
				for (int i = fromX; i <= toX; i++) {
					for (int j = fromY; j <= toY; j++) {
						t[i][j] = value;
					}
				}
			}
		}
		long n = 0;
		for (int i = 0; i < N; i++) {
			for (int j = 0; j < N; j++) {
				n += t[i][j];
			}
		}
		System.out.println(n);
	}

	private static void b(List<String> lines, final int N) {
		int fromX, fromY, toX, toY;
		int[][] t = new int[N][N];
		for (String line : lines) {
			//			System.out.println(line);
			String[] tokens = line.split(" ");
			int value = 2;
			if (tokens[0].equals("toggle")) {
				String[] from = tokens[1].split(",");
				String[] to = tokens[3].split(",");
				fromX = Integer.parseInt(from[0]);
				fromY = Integer.parseInt(from[1]);
				toX = Integer.parseInt(to[0]);
				toY = Integer.parseInt(to[1]);
			} else {
				String[] from = tokens[2].split(",");
				String[] to = tokens[4].split(",");
				fromX = Integer.parseInt(from[0]);
				fromY = Integer.parseInt(from[1]);
				toX = Integer.parseInt(to[0]);
				toY = Integer.parseInt(to[1]);
				if (tokens[1].equals("off")) {
					value = -1;
				} else {
					value = 1;
				}
			}
			for (int i = fromX; i <= toX; i++) {
				for (int j = fromY; j <= toY; j++) {
					t[i][j] += value;
					if (t[i][j] < 0) {
						t[i][j] = 0;
					}
				}
			}
		}
		long n = 0;
		for (int i = 0; i < N; i++) {
			for (int j = 0; j < N; j++) {
				n += t[i][j];
			}
		}
		System.out.println(n);
	}
}