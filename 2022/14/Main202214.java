import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class Main202214 {
	static final int N = 2000;
	static char arr[][] = new char[N][N];
	static int minX = Integer.MAX_VALUE;
	static int maxX = 0;
	static int minY = Integer.MAX_VALUE;
	static int maxY = 0;

	static void init() {
		for (int i = 0; i < arr.length; i++) {
			for (int j = 0; j < arr[0].length; j++) {
				arr[i][j] = '.';
			}
		}
	}

	static void print() {
		int offset = 4;
		for (int i = minY - offset; i <= maxY + offset; i++) {
			for (int j = minX - offset; j <= maxX + offset; j++) {
				System.out.format("%c", arr[i][j]);
			}
			System.out.println();
		}
		System.out.println();
	}

	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Paths.get("2022/14/14.txt"));
		init();
		for (int i = 0; i < input.size(); ++i) {
			String line = input.get(i);
			// System.out.println(line);
			String[] tokens = line.split(" -> ");
			List<Point> ps = new ArrayList<>();
			for (int j = 0; j < tokens.length; j++) {
				String[] points = tokens[j].split(",");
				int x = Integer.parseInt(points[0]) + 500;
				int y = Integer.parseInt(points[1]);
				minX = Math.min(minX, x);
				maxX = Math.max(maxX, x);
				minY = Math.min(minY, y);
				maxY = Math.max(maxY, y);
				ps.add(new Point(x, y));
			}
			for (int j = 0; j < ps.size() - 1; j++) {
				Point p1 = ps.get(j);
				Point p2 = ps.get(j + 1);
				int dx = 0;
				int dy = 0;
				if (p1.x < p2.x) {
					dx = 1;
				} else if (p1.x > p2.x) {
					dx = -1;
				}
				if (p1.y < p2.y) {
					dy = 1;
				} else if (p1.y > p2.y) {
					dy = -1;
				}
				int x = p1.x;
				int y = p1.y;
				if (dy == 0) {
					while (x != p2.x) {
						arr[y][x] = '#';
						x += dx;
					}
					arr[y][x] = '#';
				} else if (dx == 0) {
					while (y != p2.y) {
						arr[y][x] = '#';
						y += dy;
					}
					arr[y][x] = '#';
				}
			}
		}
		// System.out.format("%d %d %d %d\n", minX, maxX, minY, maxY);
		// print();
		one();
		two();
	}

	static void one() {
		int ans = 0;
		while (true) {
			int x = 1000;
			int y = 0;
			while (true) {
				if (y > maxY) {
					break;
				}
				boolean moved = false;
				if (arr[y + 1][x] == '.') {
					y++;
					moved = true;
				}
				if (!moved && arr[y + 1][x - 1] == '.') {
					y++;
					x--;
					moved = true;
				}
				if (!moved && arr[y + 1][x + 1] == '.') {
					y++;
					x++;
					moved = true;
				}
				if (!moved) {
					break;
				}
			}
			// System.out.println(y + " " + x);
			arr[y][x] = 'O';
			// print();
			if (y > maxY) {
				break;
			}
			ans++;
		}
		System.out.println(ans);
	}

	static void two() {
		int ans = 0;
		for (int i = 0; i < N; i++) {
			for (int j = 0; j < N; j++) {
				if (arr[i][j] == 'O') {
					arr[i][j] = '.';
				}
			}
		}
		for (int i = 0; i < N; i++) {
			arr[maxY + 2][i] = '#';
		}
		maxY += 2;
		while (true) {
			// print();
			int x = 1000;
			int y = 0;
			if (arr[y][x] == 'O') {
				break;
			}
			while (true) {
				if (y > maxY) {
					break;
				}
				boolean moved = false;
				if (arr[y + 1][x] == '.') {
					y++;
					moved = true;
				}
				if (!moved && arr[y + 1][x - 1] == '.') {
					y++;
					x--;
					moved = true;
				}
				if (!moved && arr[y + 1][x + 1] == '.') {
					y++;
					x++;
					moved = true;
				}
				if (!moved) {
					break;
				}
			}
			// System.out.println(y + " " + x);
			arr[y][x] = 'O';
			// print();
			if (y > maxY) {
				break;
			}
			ans++;
		}
		System.out.println(ans);
	}
}

class Point {
	int x;
	int y;

	Point(int x, int y) {
		this.x = x;
		this.y = y;
	}
}