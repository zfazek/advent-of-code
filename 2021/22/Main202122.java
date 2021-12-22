import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main202122 {
	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Paths.get("2021/22/22.txt"));
		final int N = 50;
		final int SIZE = 2 * N + 1;
		int[][][] arr = new int[SIZE][SIZE][SIZE];
		for (String line : input) {
			String[] tokens = line.split(" ");
			String cmd = tokens[0];
			String[] coords = tokens[1].split(",");
			int x1 = Integer.parseInt(coords[0].split("=")[1].split("\\.\\.")[0]);
			int x2 = Integer.parseInt(coords[0].split("=")[1].split("\\.\\.")[1]);
			int y1 = Integer.parseInt(coords[1].split("=")[1].split("\\.\\.")[0]);
			int y2 = Integer.parseInt(coords[1].split("=")[1].split("\\.\\.")[1]);
			int z1 = Integer.parseInt(coords[2].split("=")[1].split("\\.\\.")[0]);
			int z2 = Integer.parseInt(coords[2].split("=")[1].split("\\.\\.")[1]);
			int v = 0;
			if (cmd.equals("on")) {
				v = 1;
			}
			for (int i = -N; i <= N; i++) {
				for (int j = -N; j <= N; j++) {
					for (int k = -N; k <= N; k++) {
						int i1 = i + N;
						int j1 = j + N;
						int k1 = k + N;
						if (i >= x1 && i <= x2 && j >= y1 && j <= y2 && k >= z1 && k <= z2) {
							arr[i1][j1][k1] = v;
						}
					}
				}
			}
		}
		long n = 0;
		for (int i = -N; i <= N; i++) {
			for (int j = -N; j <= N; j++) {
				for (int k = -N; k <= N; k++) {
					int i1 = i + N;
					int j1 = j + N;
					int k1 = k + N;
					if (arr[i1][j1][k1] == 1) {
						n++;
					}
				}
			}
		}
		System.out.println(n);
	}
}
