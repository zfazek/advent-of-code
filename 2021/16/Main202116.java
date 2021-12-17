import java.io.IOException;
import java.math.BigInteger;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.NoSuchElementException;

public class Main202116 {
	static int versions;
	static int idx;

	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Paths.get("2021/16/16.txt"));
		for (String line : input) {
			versions = 0;
			System.out.println();
			System.out.println(line);
			String binary = new BigInteger("1" + line, 16).toString(2).substring(1);
			System.out.println(binary);
			int packetLength = binary.length();
			idx = 0;
			System.out.format("packet length: %d\n", packetLength);
			long value = decodePacket(binary, packetLength);
			System.out.format("versions: %d\n", versions);
			System.out.format("result: %d\n", value);
		}
	}

	private static int decodeBits(String binary, int n) {
		String version = binary.substring(idx, idx + n);
		idx += n;
		return Integer.parseInt(version, 2);
	}

	private static long decodeNumber(String binary) {
		String digit;
		String number = "";
		do {
			digit = binary.substring(idx, idx + 5);
			idx += 5;
			number += digit.substring(1, 5);
		} while (digit.charAt(0) == '1');
		long res = Long.parseLong(number, 2);
		return res;
	}

	private static long decodePacket(String binary, int packetEndIdx) {
		versions += decodeBits(binary, 3);
		int type = decodeBits(binary, 3);
		System.out.format("type: %d, ", type);
		if (type == 4) {
			long number = decodeNumber(binary);
			System.out.format("number: %d\n", number);
			return number;
		} else {
			String lengthType = binary.substring(idx, idx + 1);
			idx += 1;
			List<Long> nums = new ArrayList<>();
			if (lengthType.equals("0")) {
				int subPacketsLength = Integer.parseInt(binary.substring(idx, idx + 15), 2);
				idx += 15;
				int subPacketEndIdx = idx + subPacketsLength;
				do {
					long v = decodePacket(binary, subPacketEndIdx);
					nums.add(v);
				} while (idx < subPacketEndIdx);
				long value = calculateValue(type, nums);
				return value;
			} else {
				int count = Integer.parseInt(binary.substring(idx, idx + 11), 2);
				idx += 11;
				for (int i = 0; i < count; i++) {
					long v = decodePacket(binary, packetEndIdx);
					nums.add(v);
				}
				long value = calculateValue(type, nums);
				return value;
			}
		}
	}

	private static long calculateValue(int type, List<Long> numbers) {
		if (type == 0) {
			return numbers.stream().mapToLong(Long::longValue).sum();
		} else if (type == 1) {
			long p = 1;
			for (long n : numbers) {
				p *= n;
			}
			return p;
		} else if (type == 2) {
			return numbers.stream().mapToLong(v -> v).min().orElseThrow(NoSuchElementException::new);
		} else if (type == 3) {
			return numbers.stream().mapToLong(v -> v).max().orElseThrow(NoSuchElementException::new);
		} else if (type == 5) {
			if (numbers.get(0) > numbers.get(1)) {
				return 1;
			}
			return 0;
		} else if (type == 6) {
			if (numbers.get(0) < numbers.get(1)) {
				return 1;
			}
			return 0;
		} else if (type == 7) {
			if (numbers.get(0) == numbers.get(1)) {
				return 1;
			}
			return 0;
		}
		return 0;
	}

}