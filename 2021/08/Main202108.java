import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeSet;

/*
 * digits: segments length
 * 0:      012456:   6
 * 1:      25:       2 #
 * 2:      02346:    5
 * 3:      02356:    5
 * 4:      1235:     4 #
 * 5:      01356:    5
 * 6:      013456:   6
 * 7:      025:      3 #
 * 8:      0123456:  7 #
 * 9:      012356:   6
 *
 * segments: lengths
 * 0: 6, 5, 3, 7
 * 1: 6, 4, 5, 7
 * 2: 6, 2, 5, 4, 3, 7
 * 3: 6, 5, 4, 7
 * 4: 6, 5, 7
 * 5: 6, 2, 5, 4, 3, 7
 * 6: 6, 5, 7
 */

public class Main202108 {
	static Map<Integer, Set<Integer>> segmentsLengths = new HashMap<>();
	static Map<Integer, Set<Character>> segmentsChars = new HashMap<>();
	static List<Digit> digitList = new ArrayList<>();
	static int[] digitsResult = new int[10];

	public static void main(String[] args) throws IOException {
		segmentsLengths.put(0, new TreeSet<>(Arrays.asList(6, 5, 3, 7)));
		segmentsLengths.put(1, new TreeSet<>(Arrays.asList(6, 4, 5, 7)));
		segmentsLengths.put(2, new TreeSet<>(Arrays.asList(6, 2, 5, 4, 3, 7)));
		segmentsLengths.put(3, new TreeSet<>(Arrays.asList(6, 5, 4, 7)));
		segmentsLengths.put(4, new TreeSet<>(Arrays.asList(6, 5, 7)));
		segmentsLengths.put(5, new TreeSet<>(Arrays.asList(6, 2, 5, 4, 3, 7)));
		segmentsLengths.put(6, new TreeSet<>(Arrays.asList(6, 5, 7)));
		segmentsChars.put(0, new TreeSet<>());
		segmentsChars.put(1, new TreeSet<>());
		segmentsChars.put(2, new TreeSet<>());
		segmentsChars.put(3, new TreeSet<>());
		segmentsChars.put(4, new TreeSet<>());
		segmentsChars.put(5, new TreeSet<>());
		segmentsChars.put(6, new TreeSet<>());
		digitList.add(new Digit(0, new TreeSet<>(Arrays.asList(0, 1, 2, 4, 5, 6))));
		digitList.add(new Digit(1, new TreeSet<>(Arrays.asList(2, 5))));
		digitList.add(new Digit(2, new TreeSet<>(Arrays.asList(0, 2, 3, 4, 6))));
		digitList.add(new Digit(3, new TreeSet<>(Arrays.asList(0, 2, 3, 5, 6))));
		digitList.add(new Digit(4, new TreeSet<>(Arrays.asList(1, 2, 3, 5))));
		digitList.add(new Digit(5, new TreeSet<>(Arrays.asList(0, 1, 3, 5, 6))));
		digitList.add(new Digit(6, new TreeSet<>(Arrays.asList(0, 1, 3, 4, 5, 6))));
		digitList.add(new Digit(7, new TreeSet<>(Arrays.asList(0, 2, 5))));
		digitList.add(new Digit(8, new TreeSet<>(Arrays.asList(0, 1, 2, 3, 4, 5, 6))));
		digitList.add(new Digit(9, new TreeSet<>(Arrays.asList(0, 1, 2, 3, 5, 6))));
		List<String> input = Files.readAllLines(Paths.get("2021/08/08.txt"));
		one(input);
		two(input);
	}

	private static void one(List<String> input) {
		long n = 0;
		for (String line : input) {
			String[] tokens = line.split(" \\| ");
			String[] patterns = tokens[1].split(" ");
			for (String pattern : patterns) {
				int length = pattern.length();
				if (length == 2 || length == 3 || length == 4 || length == 7) {
					n++;
				}
			}
		}
		System.out.println(n);
	}

	private static String sortString(String str) {
		char[] temp = str.toCharArray();
		Arrays.sort(temp);
		return new String(temp);
	}

	private static void two(List<String> input) {
		long sum = 0;
		for (String line : input) {
			String[] tokens = line.split(" \\| ");
			String[] patterns = tokens[0].split(" ");
			for (int i = 0; i < patterns.length; i++) {
				patterns[i] = sortString(patterns[i]);
			}
			for (String pattern : patterns) {
				int length = pattern.length();
				if (length == 2) {
					for (int i = 0; i < length; i++) {
						char c = pattern.charAt(i);
						segmentsChars.get(2).add(c);
						segmentsChars.get(5).add(c);
					}
				} else if (length == 3) {
					for (int i = 0; i < length; i++) {
						char c = pattern.charAt(i);
						segmentsChars.get(0).add(c);
						segmentsChars.get(2).add(c);
						segmentsChars.get(5).add(c);
					}
				} else if (length == 4) {
					for (int i = 0; i < length; i++) {
						char c = pattern.charAt(i);
						segmentsChars.get(1).add(c);
						segmentsChars.get(2).add(c);
						segmentsChars.get(3).add(c);
						segmentsChars.get(5).add(c);
					}
				}
			}
			char[] arr = new char[7];
			for (int i = 0; i < digitsResult.length; i++) {
				digitsResult[i] = -1;
			}
			foo(arr, 0, patterns);
			String[] values = tokens[1].split(" ");
			for (int i = 0; i < values.length; i++) {
				values[i] = sortString(values[i]);
			}
			int d = 1000;
			for (String value : values) {
				for (int i = 0; i < patterns.length; i++) {
					if (value.equals(patterns[i])) {
						sum += d * digitsResult[i];
						d /= 10;
					}
				}
			}
		}
		System.out.println(sum);
	}

	private static String print(char[] arr) {
		String ret = "";
		for (int i = 0; i < arr.length; i++) {
			ret += arr[i] + " ";
		}
		return ret;
	}

	private static void printDigitResult() {
		for (int i = 0; i < digitsResult.length; i++) {
			System.out.format("%d ", digitsResult[i]);
		}
		System.out.println();
	}

	private static boolean foo1(char[] arr, int idx, String[] patterns) {
		if (idx >= patterns.length) {
			return true;
		}
		for (int d = 0; d < 10; d++) {
			boolean alreadyUseddigit = false;
			for (int j = 0; j < idx; j++) {
				if (digitsResult[j] == d) {
					alreadyUseddigit = true;
					break;
				}
			}
			if (alreadyUseddigit) {
				continue;
			}
			digitsResult[idx] = d;
			if (isCorrect1(arr, patterns)) {
				boolean res = foo1(arr, idx + 1, patterns);
				if (res) {
					return true;
				}
			}
		}
		digitsResult[idx] = -1;
		return false;
	}

	private static boolean isCorrect1(char[] arr, String[] patterns) {
		for (int i = 0; i < digitsResult.length; i++) {
			int d = digitsResult[i];
			if (d == -1) {
				break;
			}
			String pattern = patterns[i];
			int length = pattern.length();
			Digit digit = digitList.get(d);
			if (length != digit.segments.size()) {
				return false;
			}
			for (int idx : digit.segments) {
				if (pattern.indexOf(arr[idx]) == -1) {
					return false;
				}
			}
		}
		return true;
	}

	private static boolean foo(char[] arr, int idx, String[] patterns) {
		if (idx >= arr.length) {
			for (int i = 0; i < digitsResult.length; i++) {
				digitsResult[i] = -1;
			}
			boolean res = foo1(arr, 0, patterns);
			if (res) {
				//				System.out.println("BINGO");
				//				System.out.println(print(arr));
				//				printDigitResult();
			}
			return res;
		}
		for (char c = 'a'; c <= 'g'; c++) {
			boolean alreadyUsedLetter = false;
			for (int j = 0; j < idx; j++) {
				if (arr[j] == c) {
					alreadyUsedLetter = true;
					break;
				}
			}
			if (alreadyUsedLetter) {
				continue;
			}
			arr[idx] = c;
			//			System.out.println(print(arr));
			if (isCorrect(arr, patterns)) {
				boolean res = foo(arr, idx + 1, patterns);
				if (res) {
					return true;
				}
			}
		}
		arr[idx] = 0;
		return false;
	}

	private static boolean isCorrect(char[] arr, String[] patterns) {
		for (int i = 0; i < arr.length; i++) {
			char c = arr[i];
			if (c == 0) {
				break;
			}
			//			System.out.format("i: %d %c ", i, c);
			if (segmentsChars.get(i).size() > 0 && !segmentsChars.get(i).contains(c)) {
				//				System.out.println(" -> false1");
				return false;
			}
			Set<Integer> lengths = segmentsLengths.get(i);
			Set<Integer> foundLengths = new TreeSet<>();
			for (String pattern : patterns) {
				//				System.out.print(pattern + " ");
				int length = pattern.length();
				int idx = pattern.indexOf(c);
				if (idx >= 0 && !lengths.contains(length)) {
					//					System.out.println(" -> false2");
					return false;
				}
				if (idx >= 0) {
					foundLengths.add(length);
				}
			}
			for (int l : lengths) {
				if (!foundLengths.contains(l)) {
					//					System.out.println(" -> false3");
					return false;
				}
			}
			//			System.out.println();
		}
		return true;
	}
}

class Digit {
	int id;
	Set<Integer> segments;

	Digit(int id, Set<Integer> segments) {
		this.id = id;
		this.segments = segments;
	}
}