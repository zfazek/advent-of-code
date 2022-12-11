import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Main202211 {
	static List<Monkey> monkeys = new ArrayList<>();
	static Monkey monkey;
	static long den = 1;

	public static void main(String[] args) throws IOException {
		long start = System.currentTimeMillis();
		List<String> input = Files.readAllLines(Paths.get("2022/11/11.txt"));
		for (int i = 0; i < input.size(); ++i) {
			String line = input.get(i);
			if (line.isEmpty()) {
				continue;
			}
			char c2 = line.charAt(2);
			char c7 = line.charAt(7);
			if (c2 == 'S') {
				String[] tokens = line.split(": ");
				monkey = new Monkey();
				String[] tokens1 = tokens[1].split(", ");
				List<Long> items = new ArrayList<>();
				for (int j = 0; j < tokens1.length; j++) {
					items.add(Long.parseLong(tokens1[j]));
					monkey.items = items;
				}
			} else if (c2 == 'O') {
				String[] tokens = line.trim().split(" ");
				monkey.op = tokens[4];
				if (tokens[5].equals("old")) {
					monkey.opn = 0;
				} else {
					monkey.opn = Integer.parseInt(tokens[5]);
				}
			} else if (c2 == 'T') {
				String[] tokens = line.trim().split(" ");
				monkey.test = Integer.parseInt(tokens[3]);
				den *= monkey.test;
			} else if (c2 == ' ' && c7 == 't') {
				String[] tokens = line.trim().split(" ");
				monkey.t = Integer.parseInt(tokens[5]);
			} else if (c2 == ' ' && c7 == 'f') {
				String[] tokens = line.trim().split(" ");
				monkey.f = Integer.parseInt(tokens[5]);
				monkeys.add(monkey);
			}
		}
		long max = 0;
		for (int i = 0; i < 10000; i++) {
			for (Monkey monkey : monkeys) {
				monkey.n += monkey.items.size();
				for (int j = 0; j < monkey.items.size(); j++) {
					long v = monkey.items.get(j);
					long op;
					if (monkey.opn == 0) {
						op = v;
					} else {
						op = monkey.opn;
					}
					if (monkey.op.charAt(0) == '+') {
						v += op;
					} else {
						v *= op;
					}
					// v /= 3;
					v = v % den;
					if (v > max) {
						max = v;
					}
					if (v % monkey.test == 0) {
						monkeys.get(monkey.t).items.add(v);
					} else {
						monkeys.get(monkey.f).items.add(v);
					}
				}
				monkey.items.clear();
			}
		}
		int n = monkeys.size();
		List<Integer> arr = new ArrayList<>();
		for (int i = 0; i < n; i++) {
			arr.add(monkeys.get(i).n);
		}
		Collections.sort(arr);
		long ans = (long)(arr.get(n - 1)) * arr.get(n - 2);
		System.out.println(ans);
		long end = System.currentTimeMillis();
		System.out.format("%.3f sec\n", (end - start) / 1000.0);
	}
}

class Monkey {
	List<Long> items = new ArrayList<>();
	String op;
	int opn;
	int test;
	int t;
	int f;
	int n = 0;

	@Override
	public String toString() {
		return String.format("%s %s %d %d %d %d: %d", items, op, opn, test, t, f, n);
	}
}