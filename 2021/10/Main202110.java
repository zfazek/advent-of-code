import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Stack;

public class Main202110 {
	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Paths.get("2021/10/10.txt"));
		long start = System.currentTimeMillis();
		long sum1 = 0;
		List<Long> totalScores = new ArrayList<>();
		String opens = "([{<";
		String closes = ")]}>";
		int[] scores = {3, 57, 1197, 25137};
		for (String line : input) {
			long s2 = 0;
			Stack<Character> stack = new Stack<>();
			boolean incomplete = true;
			for (int i = 0; i < line.length(); i++) {
				char c = line.charAt(i);
				int idx = opens.indexOf(c);
				if (idx > -1) {
					stack.push(c);
				} else {
					idx = closes.indexOf(c);
					char s = stack.peek();
					if (opens.indexOf(s) == idx) {
						stack.pop();
					} else {
						sum1 += scores[idx];
						incomplete = false;
						break;
					}
				}
			}
			if (incomplete) {
				Object[] arr = stack.toArray();
				for (int i = arr.length - 1; i >= 0; i--) {
					int s = opens.indexOf((char)arr[i]) + 1;
					s2 = s2 * 5 + s;
				}
				totalScores.add(s2);
			}
		}
		System.out.println(sum1);
		Collections.sort(totalScores);
		System.out.println(totalScores.get(totalScores.size() / 2));
		long end = System.currentTimeMillis();
		System.out.println(end - start + " ms");
	}
}
