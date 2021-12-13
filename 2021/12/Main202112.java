import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeSet;

public class Main202112 {
	static Map<String, Set<String>> edges = new HashMap<>();
	static Set<String> distinctPaths = new TreeSet<>();
	static Set<String> visited = new TreeSet<>();
	static String currentPath;
	static String caveTwice;
	static boolean visitedOnce;

	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Paths.get("2021/12/12.txt"));
		long start = System.currentTimeMillis();
		for (String line : input) {
			String[] tokens = line.split("-");
			String a = tokens[0];
			String b = tokens[1];
			if (!a.equals("end") && !b.equals("start")) {
				if (edges.containsKey(a)) {
				} else {
					edges.put(a, new TreeSet<>());
				}
				edges.get(a).add(b);
			}
			if (!b.equals("end") && !a.equals("start")) {
				if (edges.containsKey(b)) {
				} else {
					edges.put(b, new TreeSet<>());
				}
				edges.get(b).add(a);
			}
		}
		currentPath = "";
		findCavesA("start");
		System.out.println(distinctPaths.size());

		distinctPaths.clear();
		for (String edge : edges.keySet()) {
			if (!edge.equals("end") && !edge.equals("start")) {
				char firstLetter = edge.charAt(0);
				if (firstLetter >= 'a' && firstLetter <= 'z') {
					caveTwice = edge;
					visitedOnce = false;
					findCavesB("start");
				}
			}
		}
		long end = System.currentTimeMillis();
		System.out.println(distinctPaths.size());
		System.out.format("%d ms\n", end - start);
	}

	private static void findCavesA(String edge) {
		if (edge.equals("start")) {
			currentPath = "start";
			visited.clear();
		}
		for (String target : edges.get(edge)) {
			if (visited.contains(target)) {
				continue;
			}
			if (!target.equals("end")) {
				char firstLetter = target.charAt(0);
				if (firstLetter >= 'a' && firstLetter <= 'z') {
					visited.add(target);
				}
				currentPath += "," + target;
				findCavesA(target);
			} else {
				currentPath += "," + target;
				distinctPaths.add(currentPath);
			}
			visited.remove(target);
			currentPath = currentPath.substring(0, currentPath.lastIndexOf(","));
		}
	}
	private static void findCavesB(String edge) {
		if (edge.equals("start")) {
			currentPath = "start";
			visited.clear();
		}
		for (String target : edges.get(edge)) {
			if (visited.contains(target)) {
				continue;
			}
			if (!target.equals("end")) {
				char firstLetter = target.charAt(0);
				if (firstLetter >= 'a' && firstLetter <= 'z') {
					if (target.equals(caveTwice)) {
						if (visitedOnce) {
							visited.add(target);
						} else {
							visitedOnce = true;
						}
					} else {
						visited.add(target);
					}
				}
				currentPath += "," + target;
				findCavesB(target);
			} else {
				currentPath += "," + target;
				distinctPaths.add(currentPath);
			}
			if (target.equals(caveTwice)) {
				if (visited.contains(target)) {
					visitedOnce = true;
				} else {
					visitedOnce = false;
				}
			}
			visited.remove(target);
			currentPath = currentPath.substring(0, currentPath.lastIndexOf(","));
		}
	}
}
