package main;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Main202014 {


	private static void foo1() throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("14.txt"));
		long maskAnd = 0;
		long maskOr = 0;
		Map<Integer, Long> numbers = new HashMap<>();
		for (String line : lines) {
			String[] tokens = line.split(" = ");
			final String cmd = tokens[0];
			if (cmd.startsWith("mem")) {
				int addr = Integer.parseInt(cmd.substring(4, cmd.length() - 1));
				long value = Long.parseLong(tokens[1]);
				value &= maskAnd;
				value |= maskOr;
				numbers.put(addr, value);
			} else {
				final String pattern = tokens[1];
				final String maskAndStr = pattern.replace('X', '1');
				final String maskOrStr = pattern.replace('X', '0');
				maskAnd = Long.parseLong(maskAndStr, 2);
				maskOr = Long.parseLong(maskOrStr, 2);
			}
		}
		long n = 0;
		for (int addr : numbers.keySet()) {
			n += numbers.get(addr);
		}
		System.out.println(n);
	}

	private static void foo2() throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("14.txt"));
		Map<Long, Integer> numbers = new HashMap<>();
		String mask = "";
		for (String line : lines) {
			String[] tokens = line.split(" = ");
			final String cmd = tokens[0];
			if (cmd.startsWith("mem")) {
				int addr = Integer.parseInt(cmd.substring(4, cmd.length() - 1));
				int value = Integer.parseInt(tokens[1]);
				List<Long> addrs = getAddrs(mask, addr);
				for (Long a : addrs) {
					numbers.put(a, value);
				}
			} else {
				mask = tokens[1];
			}
		}
		long n = 0;
		for (long addr : numbers.keySet()) {
			n += numbers.get(addr);
		}
		System.out.println(n);
	}

	private static List<Long> getAddrs(final String maskOrig, int addr) {
		List<Long> addrs = new ArrayList<>();
		final String addrStr = String.format("%36s", Integer.toBinaryString(addr)).replace(' ', '0');
//		System.out.println(addrStr);
		int n = 0;
		for (int i = 0; i < maskOrig.length(); i++) {
			if (maskOrig.charAt(i) == 'X') {
				n++;
			}
		}
		String maskNew = "";
		for (int i = 0; i < maskOrig.length(); i++) {
			if (maskOrig.charAt(i) == 'X') {
				maskNew += "X";
			} else if (maskOrig.charAt(i) == '0') {
				maskNew += addrStr.charAt(i);
			} else {
				maskNew += "1";
			}
		}
		for (int i = 0; i < (int)Math.pow(2, n); i++) {
			final String num = String.format("%" + n + "s", Integer.toBinaryString(i)).replace(' ', '0');
			String maskStr = maskNew;
			int idx = 0;
			for (int j = 0; j < n; j++) {
				idx = maskStr.indexOf('X', idx);
				maskStr = maskStr.substring(0, idx) + num.charAt(j) + maskStr.substring(idx + 1);
				idx++;
			}
//			System.out.println(maskNew);
			long mask = Long.parseLong(maskStr, 2);
//			System.out.println(mask);
			addrs.add(mask);
		}
		return addrs;
	}

	public static void main(String[] args) throws IOException {
		foo1();
		foo2();
	}

}
