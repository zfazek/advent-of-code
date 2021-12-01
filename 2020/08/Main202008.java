import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Main202008 {

	private static int a(List<String> lines) {
		Set<Integer> visited = new HashSet<Integer>();
		int acc = 0;
		int idx = 0;
		while (!visited.contains(idx)) {
			final String line = lines.get(idx);
			if (line.startsWith("acc")) {
				visited.add(idx);
				final int n = Integer.parseInt(line.substring(4));
				acc += n;
				idx++;
			} else if (line.startsWith("jmp")) {
				visited.add(idx);
				final int n = Integer.parseInt(line.substring(4));
				idx += n;
			} else {
				visited.add(idx);
				idx++;
			}
		}
		return acc;
	}

	private static String swap(String line) {
		if (line.startsWith("nop")) {
			return "jmp " + line.substring(4);
		} else if (line.startsWith("jmp")) {
			return "nop " + line.substring(4);
		}
		return line;
	}

	private static int b(List<String> lines) {
		for (int i = 0; i < lines.size(); i++) {
			Set<Integer> visited = new HashSet<Integer>();
			int acc = 0;
			int idx = 0;
			lines.set(i, swap(lines.get(i)));
			while (!visited.contains(idx) && idx >= 0 && idx < lines.size()) {
				final String line = lines.get(idx);
				if (line.startsWith("acc")) {
					visited.add(idx);
					final int n = Integer.parseInt(line.substring(4));
					acc += n;
					idx++;
				} else if (line.startsWith("jmp")) {
					visited.add(idx);
					final int n = Integer.parseInt(line.substring(4));
					idx += n;
				} else {
					visited.add(idx);
					idx++;
				}
			}
			if (idx == lines.size()) {
				return acc;
			}
			lines.set(i, swap(lines.get(i)));
		}
		return -1;
	}

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2020/08/08.txt"));
		System.out.println(a(lines));
		System.out.println(b(lines));
	}
}
