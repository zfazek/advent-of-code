package main;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;

public class Main202018 {
	final static String FILE_NAME = "18.txt";

	private static void foo1() throws IOException {
		List<String> lines = Files.readAllLines(Paths.get(FILE_NAME));
		long acc = 0;
		for (String line : lines) {
			acc += getResult(line, 1);
		}
		System.out.println(acc);
	}

	private static long getResult(String line, final int mode) {
		long acc = 0;
		while (true) {
//			System.out.println(line);
			final int p1 = line.lastIndexOf('(');
			if (p1 == -1) {
				if (mode == 1) {
					acc += getResultWithoutParentheses1(line);
				} else {
					acc += getResultWithoutParentheses2(line);
				}
				break;
			}
			final int p2 = line.indexOf(')', p1);
			final String innerStr = line.substring(p1 + 1, p2);
			if (mode == 1) {
				final long innerResult = getResultWithoutParentheses1(innerStr);
				line = line.substring(0, p1) + innerResult + line.substring(p2 + 1);
			} else {
				final long innerResult = getResultWithoutParentheses2(innerStr);
				line = line.substring(0, p1) + innerResult + line.substring(p2 + 1);
			}
		}
		return acc;
	}

	private static long getResultWithoutParentheses1(String line) {
		String[] tokens = line.split(" ");
		long acc = Integer.parseInt(tokens[0]);
		boolean sum = false;
//		System.out.println(line);
		for (int i = 1; i < tokens.length; i++) {
			final String token = tokens[i];
//			System.out.format("i: %d, token:%s:\n", i, token);
			if (token.equals("+")) {
				sum = true;
			} else if (token.equals("*")) {
					sum = false;
			} else {
				int num = Integer.parseInt(token);
				if (sum) {
					acc += num;
				} else {
					acc *= num;
				}
			}
		}
		return acc;
	}	private static void foo2() throws IOException {
		List<String> lines = Files.readAllLines(Paths.get(FILE_NAME));
		long acc = 0;
		for (String line : lines) {
//			System.out.println(line);
			acc += getResult(line, 2);
		}
		System.out.println(acc);
	}

	private static long getResultWithoutParentheses2(String line) {
		List<String> tokens = new LinkedList<>(Arrays.asList(line.split(" ")));
//		System.out.println(line);
		for (int i = 1; i < tokens.size(); i++) {
			final String token = tokens.get(i);
			if (token.equals(" ")) {
				continue;
			}
//			System.out.format("i: %d, token:%s:\n", i, token);
			if (token.equals("+")) {
				final int num = Integer.parseInt(tokens.get(i - 1)) + Integer.parseInt(tokens.get(i + 1));
				tokens.set(i - 1, " ");
				tokens.set(i, " ");
				tokens.set(i + 1, "" + num);
			}
		}
		Iterator<String> it = tokens.iterator();
		while (it.hasNext()) {
			String s = it.next();
			if (s.equals(" ")) {
				it.remove();
			}
		}
		long acc = Integer.parseInt(tokens.get(0));
		for (int i = 1; i < tokens.size(); i++) {
			final String token = tokens.get(i);
//			System.out.format("i: %d, token:%s:\n", i, token);
			if (token.equals("*")) {
				continue;
			}
			acc *= Integer.parseInt(token);
		}
		return acc;
	}

	public static void main(String[] args) throws IOException {
		foo1();
		foo2();
	}
}
