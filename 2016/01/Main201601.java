
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;

public class Main201601 {

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2016/01.txt"));
//		one(lines);
		two(lines);
	}

	private static void one(List<String> lines) {
		int x = 0;
		int y = 0;
		int dir = 90;
		for (String line : lines) {
			String[] tokens = line.split(", ");
			for (String token : tokens) {
				char c = token.charAt(0);
				int s = Integer.parseInt(token.substring(1));
				if (c == 'L') {
					dir = (dir + 90) % 360;
				} else {
					dir = dir - 90;
					if (dir < 0) {
						dir += 360;
					}
				}
				if (dir == 0) {
					x += s;
				} else if (dir == 90) {
					y -= s;
				} else if (dir == 180) {
					x -= s;
				} else {
					y += s;
				}
				//				System.out.format("%s: x: %d, y: %d, dir: %d\n", token, x, y, dir);
			}
		}
		System.out.println(x + y);
	}

	private static void two(List<String> lines) {
		Map<Integer, Set<Integer>> visited = new TreeMap<>();
		int x = 0;
		int y = 0;
		int dir = 90;
		for (String line : lines) {
			String[] tokens = line.split(", ");
			for (String token : tokens) {
				char c = token.charAt(0);
				int s = Integer.parseInt(token.substring(1));
				if (c == 'L') {
					dir = (dir + 90) % 360;
				} else {
					dir = dir - 90;
					if (dir < 0) {
						dir += 360;
					}
				}
				if (dir == 0) {
					for (int i = 0; i < s; i++) {
						x++;
						move(visited, x, y, dir, token);
					}
				} else if (dir == 90) {
					for (int i = 0; i < s; i++) {
						y--;
						move(visited, x, y, dir, token);
					}
				} else if (dir == 180) {
					for (int i = 0; i < s; i++) {
						x--;
						move(visited, x, y, dir, token);
					}
				} else {
					for (int i = 0; i < s; i++) {
						y++;
						move(visited, x, y, dir, token);
					}
				}
			}
		}
	}

	private static void move(Map<Integer, Set<Integer>> visited, int x, int y, int dir, String token) {
		if (visited.containsKey(x) && visited.get(x).contains(y)) {
			System.out.format("%s: x: %d, y: %d, dir: %d\n", token, x, y, dir);
			System.out.println(x + y);
			System.exit(0);
		} else {
			if (!visited.containsKey(x)) {
				visited.put(x, new TreeSet<>());
			}
			visited.get(x).add(y);
		}
	}

}
