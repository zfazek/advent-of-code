
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

public class Main201507 {

	static Map<String, String> wires = new TreeMap<>();
	static Map<String, Integer> dp = new TreeMap<>();

	private static int mod(int n) {
		return n % 65536;
	}

	private static int foo(String wire) {
		if (dp.containsKey(wire)) {
			return dp.get(wire);
		}
		if (!wires.containsKey(wire)) {
			try {
				int r = Integer.parseInt(wire);
				dp.put(wire, r);
				return r;
			} catch(NumberFormatException e) {
				System.out.format("%s not found, returning 0\n", wire);
				return 0;
			}
		}
		String cmd = wires.get(wire);
		if (cmd.contains("AND")) {
			String[] t = cmd.split(" AND ");
			String s1 = t[0];
			String s2 = t[1];
			int r =  foo(s1) & foo(s2);
			dp.put(wire, r);
			return r;
		} else if (cmd.contains("OR")) {
			String[] t = cmd.split(" OR ");
			String s1 = t[0];
			String s2 = t[1];
			int r =  foo(s1) | foo(s2);
			dp.put(wire, r);
			return r;
		} else if (cmd.contains("NOT")) {
			String s1 = cmd.substring(4);
			int result = ~foo(s1);
			if (result < 0) {
				result+= 65536;
			}
			int r =  result;
			dp.put(wire, r);
			return r;
		} else if (cmd.contains("LSHIFT")) {
			String[] t = cmd.split(" LSHIFT ");
			String s1 = t[0];
			String s2 = t[1];
			int r =  mod(foo(s1) << Integer.parseInt(s2));
			dp.put(wire, r);
			return r;
		} else if (cmd.contains("RSHIFT")) {
			String[] t = cmd.split(" RSHIFT ");
			String s1 = t[0];
			String s2 = t[1];
			int r =  foo(s1) >> Integer.parseInt(s2);
			dp.put(wire, r);
			return r;
		} else {
			try {
				int r =  Integer.parseInt(cmd);				
				dp.put(wire, r);
				return r;
			} catch(NumberFormatException e) {
				int r =  foo(cmd);
				dp.put(wire, r);
				return r;
			}
		}
	}

	private static void one(List<String> lines) {
		long startTime = System.currentTimeMillis();
		for (String line : lines) {
			String[] tokens = line.split(" -> ");
			String cmd = tokens[0];
			String key = tokens[1];
			wires.put(key, cmd);
		}
		long stopTime = System.currentTimeMillis();
		{String key = "a"; System.out.format("%s: %s\n", key, foo(key));}
		System.out.format("Elapsed time: %.2f sec\n", (stopTime - startTime) / 1000.0);
	}

	private static void two(List<String> lines) {
		dp.clear();
		wires.clear();
		long startTime = System.currentTimeMillis();
		for (String line : lines) {
			String[] tokens = line.split(" -> ");
			String cmd = tokens[0];
			String key = tokens[1];
			if (key.equals("b")) {
				wires.put(key, "956");
			} else {
				wires.put(key, cmd);
			}
		}
		long stopTime = System.currentTimeMillis();
		{String key = "a"; System.out.format("%s: %s\n", key, foo(key));}
		System.out.format("Elapsed time: %.2f sec\n", (stopTime - startTime) / 1000.0);
		
	}
	
	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("07.txt"));
		one(lines);
		two(lines);
	}

}
