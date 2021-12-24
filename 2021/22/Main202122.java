import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.ListIterator;

public class Main202122 {
	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Paths.get("2021/22/22aa.txt"));
		//		one(input);
		two(input);
	}

	@SuppressWarnings("unused")
	private static void one(List<String> input) {
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

	private static void two(List<String> input) {
		List<Pos> positions = new ArrayList<>();
		for (String line : input) {
			System.out.println(line);
			String[] tokens = line.split(" ");
			String cmd = tokens[0];
			String[] coords = tokens[1].split(",");
			int x1 = Integer.parseInt(coords[0].split("=")[1].split("\\.\\.")[0]);
			int x2 = Integer.parseInt(coords[0].split("=")[1].split("\\.\\.")[1]);
			//			int y1 = Integer.parseInt(coords[1].split("=")[1].split("\\.\\.")[0]);
			//			int y2 = Integer.parseInt(coords[1].split("=")[1].split("\\.\\.")[1]);
			//			int z1 = Integer.parseInt(coords[2].split("=")[1].split("\\.\\.")[0]);
			//			int z2 = Integer.parseInt(coords[2].split("=")[1].split("\\.\\.")[1]);
			if (cmd.equals("on") && positions.isEmpty()) {
				positions.add(new Pos(x1, x2));
				continue;
			}
			if (cmd.equals("off") && positions.isEmpty()) {
				continue;
			}
			ListIterator<Pos> it = positions.listIterator();
			while (it.hasNext()) {
				if (cmd.equals("on")) {
					Pos p = it.next();
					if ((x1 >= p.from && x1 <= p.end && x2 >= p.end) ||
							(x1 <= p.from && x2 >= p.from && x2 <= p.end)) {
						p.from = Math.min(x1, p.from);
						p.end = Math.max(x2, p.end);
						continue;
					}
					if (x2 <= p.from || x1 >= p.end) {
						it.add(new Pos(x1, x2));
						continue;
					}
				} else {

				}
			}
		}
		long n = 0;
		for (Pos p : positions) {
			System.out.println(p);
			n += p.end - p.from + 1;
		}
		System.out.println(n);
	}
}

class Pos {
	int from;
	int end;

	Pos(int from, int end) {
		this.from = from;
		this.end = end;
	}

	@Override
	public String toString() {
		return String.format("%d %d", from, end);
	}
}